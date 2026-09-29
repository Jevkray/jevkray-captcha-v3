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

        public IndexModel(IMemoryCache cache, IConfiguration config, IWebHostEnvironment env)
        {
            _cache = cache;
            _config = config;
            _env = env;
            _digits = int.TryParse(_config["CapGen:Digits"], out var d) && d > 0 ? d : 4;
        }

        public string ChallengeId { get; private set; } = "";

        [BindProperty]
        public string? Answer { get; set; }

        public void OnGet() => ChallengeId = CreateChallengeData();

        // новый код
        public JsonResult OnGetNew() => new(new { id = CreateChallengeData() });

        // сама гифка
        public IActionResult OnGetGif(string id)
        {
            if (!string.IsNullOrEmpty(id) && _cache.TryGetValue(id, out ChallengeData? c) && c is not null)
            {
                return File(c.Gif, "image/gif");
            }
            return NotFound();
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
                Gif = RunGenerator(code)
            };

            var id = Guid.NewGuid().ToString("N");
            _cache.Set(id, challenge, ChallengeTtl);
            return id;
        }

        private static byte[] Hash(byte[] salt, string code)
        {
            var codeBytes = System.Text.Encoding.UTF8.GetBytes(code);
            var buf = new byte[salt.Length + codeBytes.Length];
            Buffer.BlockCopy(salt, 0, buf, 0, salt.Length);
            Buffer.BlockCopy(codeBytes, 0, buf, salt.Length, codeBytes.Length);
            return SHA256.HashData(buf);
        }

        private byte[] RunGenerator(string code)
        {
            var exe = _config["CapGen:Path"] ?? "generator/target/release/capgen.exe";
            if (!Path.IsPathRooted(exe))
            {
                exe = Path.GetFullPath(Path.Combine(_env.ContentRootPath, exe));
            }
            if (!System.IO.File.Exists(exe) && exe.EndsWith(".exe", StringComparison.OrdinalIgnoreCase))`r`n            {`r`n                var alt = exe.Substring(0, exe.Length - 4);`r`n                if (System.IO.File.Exists(alt)) exe = alt;`r`n            }`r`n            if (!System.IO.File.Exists(exe))
            {
                throw new FileNotFoundException($"Не найден генератор: {exe}. Соберите generator (cargo build --release).");
            }

            var fps = _config["CapGen:Fps"] ?? "30";
            var seconds = _config["CapGen:Seconds"] ?? "30";
            var tmp = Path.Combine(Path.GetTempPath(), $"cap_{Guid.NewGuid():N}.gif");

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

                using var proc = Process.Start(psi)!;
                if (!proc.WaitForExit(30_000))
                {
                    proc.Kill(true);
                    throw new TimeoutException("Генератор GIF не успел завершиться.");
                }
                if (proc.ExitCode != 0 || !System.IO.File.Exists(tmp))
                {
                    throw new InvalidOperationException($"Генератор завершился с кодом {proc.ExitCode}.");
                }

                return System.IO.File.ReadAllBytes(tmp);
            }
            finally
            {
                if (System.IO.File.Exists(tmp)) System.IO.File.Delete(tmp);
            }
        }

        private sealed class ChallengeData
        {
            public required byte[] Salt { get; init; }
            public required byte[] Hash { get; init; }
            public required byte[] Gif { get; init; }
        }
    }
}
