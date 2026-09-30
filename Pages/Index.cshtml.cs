using System.Diagnostics;
using System.Security.Cryptography;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.Extensions.Caching.Memory;

namespace AiCapcha.Pages
{
    public class IndexModel : PageModel
    {
        private static readonly TimeSpan ChallengeTtl = TimeSpan.FromMinutes(5);

        private readonly IMemoryCache _cache;
        private readonly IConfiguration _config;
        private readonly IWebHostEnvironment _env;
        private readonly int _digits;
        private readonly double _fps;

        public IndexModel(IMemoryCache cache, IConfiguration config, IWebHostEnvironment env)
        {
            _cache = cache;
            _config = config;
            _env = env;
            _digits = int.TryParse(_config["CapGen:Digits"], out var d) && d > 0 ? d : 4;
            _fps = double.TryParse(_config["CapGen:Fps"], out var f) && f > 0 ? f : 20;
        }

        public string ChallengeId { get; private set; } = "";

        [BindProperty]
        public string? Answer { get; set; }

        public void OnGet() => ChallengeId = CreateChallengeData();

        // новый код
        public JsonResult OnGetNew() => new(new { id = CreateChallengeData() });

        // поток кадров: отдаём по одному в реальном времени, второй раз запросить нельзя
        public async Task<IActionResult> OnGetStream(string id, CancellationToken ct)
        {
            if (string.IsNullOrEmpty(id)
                || !_cache.TryGetValue(id, out ChallengeData? c) || c is null
                || !c.TryBeginStream())
            {
                return NotFound();
            }

            try
            {
                Response.ContentType = "application/octet-stream";
                Response.Headers["Cache-Control"] = "no-store";
                Response.Headers["X-Content-Type-Options"] = "nosniff";

                await using var fs = System.IO.File.OpenRead(c.RawPath);
                var head = new byte[13 + 255];
                await fs.ReadExactlyAsync(head.AsMemory(0, 13), ct);
                int w = head[4] | (head[5] << 8);
                int h = head[6] | (head[7] << 8);
                int frames = BitConverter.ToInt32(head, 8);
                int pal = head[12];
                await fs.ReadExactlyAsync(head.AsMemory(13, pal), ct);
                await Response.Body.WriteAsync(head.AsMemory(0, 13 + pal), ct);
                await Response.Body.FlushAsync(ct);

                var frame = new byte[w * h];
                var sw = Stopwatch.StartNew();
                for (int i = 0; i < frames; i++)
                {
                    await fs.ReadExactlyAsync(frame, ct);
                    await Response.Body.WriteAsync(frame, ct);
                    await Response.Body.FlushAsync(ct);
                    var wait = (long)((i + 1) * 1000.0 / _fps) - sw.ElapsedMilliseconds;
                    if (wait > 0) await Task.Delay((int)wait, ct);
                }
            }
            catch (OperationCanceledException) { /* клиент отключился */ }
            finally
            {
                TryDelete(c.RawPath);
            }

            return new EmptyResult();
        }

        // проверка: хэшируем введённый код и сверяем с сохранённым хэшем
        public JsonResult OnPostCheck(string? answer, string? id)
        {
            if (!string.IsNullOrEmpty(id)
                && _cache.TryGetValue(id, out ChallengeData? c) && c is not null
                && Matches(answer, c))
            {
                _cache.Remove(id);
                return new JsonResult(new { solved = true });
            }

            return new JsonResult(new
            {
                solved = false,
                message = "Неверно, попробуйте ещё раз.",
                id = CreateChallengeData()
            });
        }

        private bool Matches(string? answer, ChallengeData c)
        {
            if (string.IsNullOrEmpty(answer)) return false;
            var input = answer.Trim();
            if (input.Length != _digits) return false;
            var hash = Hash(c.Salt, input);
            return CryptographicOperations.FixedTimeEquals(hash, c.Hash);
        }

        private string CreateChallengeData()
        {
            var code = string.Concat(Enumerable.Range(0, _digits)
                .Select(_ => (char)('0' + Random.Shared.Next(10))));

            var salt = RandomNumberGenerator.GetBytes(16);
            var challenge = new ChallengeData
            {
                Salt = salt,
                Hash = Hash(salt, code),
                RawPath = RunGenerator(code)
            };

            var id = Guid.NewGuid().ToString("N");
            var options = new MemoryCacheEntryOptions { AbsoluteExpirationRelativeToNow = ChallengeTtl };
            options.RegisterPostEvictionCallback((_, value, _, _) =>
            {
                if (value is ChallengeData cd) TryDelete(cd.RawPath);
            });
            _cache.Set(id, challenge, options);
            return id;
        }

        private static void TryDelete(string path)
        {
            try { if (System.IO.File.Exists(path)) System.IO.File.Delete(path); } catch { }
        }

        private static byte[] Hash(byte[] salt, string code)
        {
            var codeBytes = System.Text.Encoding.UTF8.GetBytes(code);
            var buf = new byte[salt.Length + codeBytes.Length];
            Buffer.BlockCopy(salt, 0, buf, 0, salt.Length);
            Buffer.BlockCopy(codeBytes, 0, buf, salt.Length, codeBytes.Length);
            return SHA256.HashData(buf);
        }

        // генерируем сырые кадры во временный файл, его же потом отдаёт OnGetStream
        private string RunGenerator(string code)
        {
            var exe = _config["CapGen:Path"] ?? "generator/target/release/capgen.exe";
            if (!Path.IsPathRooted(exe))
            {
                exe = Path.GetFullPath(Path.Combine(_env.ContentRootPath, exe));
            }
            if (!System.IO.File.Exists(exe) && exe.EndsWith(".exe", StringComparison.OrdinalIgnoreCase))
            {
                var alt = exe.Substring(0, exe.Length - 4);
                if (System.IO.File.Exists(alt)) exe = alt;
            }
            if (!System.IO.File.Exists(exe))
            {
                throw new FileNotFoundException($"Не найден генератор: {exe}. Соберите generator (cargo build --release).");
            }

            var fps = _config["CapGen:Fps"] ?? "20";
            var seconds = _config["CapGen:Seconds"] ?? "30";
            var mode = _config["CapGen:Mode"] ?? "classic";
            var tmp = Path.Combine(Path.GetTempPath(), $"cap_{Guid.NewGuid():N}.bin");

            try
            {
                var psi = new ProcessStartInfo(exe)
                {
                    RedirectStandardError = true,
                    UseShellExecute = false,
                    CreateNoWindow = true
                };
                psi.ArgumentList.Add(code);
                psi.ArgumentList.Add(tmp);
                psi.ArgumentList.Add(fps);
                psi.ArgumentList.Add(seconds);
                psi.ArgumentList.Add(mode);

                using var proc = Process.Start(psi)!;
                if (!proc.WaitForExit(30_000))
                {
                    proc.Kill(true);
                    throw new TimeoutException("Генератор кадров не успел завершиться.");
                }
                if (proc.ExitCode != 0 || !System.IO.File.Exists(tmp))
                {
                    throw new InvalidOperationException($"Генератор завершился с кодом {proc.ExitCode}.");
                }

                return tmp;
            }
            catch
            {
                TryDelete(tmp);
                throw;
            }
        }

        private sealed class ChallengeData
        {
            private int _streamed;

            public required byte[] Salt { get; init; }
            public required byte[] Hash { get; init; }
            public required string RawPath { get; init; }

            public bool TryBeginStream() => Interlocked.Exchange(ref _streamed, 1) == 0;
        }
    }
}
