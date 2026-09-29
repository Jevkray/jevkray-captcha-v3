param([int]$N = 100, [int]$Fps = 20, [int]$Seconds = 30)
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$gen = Join-Path $root "target\release\capgen.exe"
$sol = Join-Path $root "target\release\solve.exe"
$dir = Join-Path $env:TEMP "capset_eval"
if (!(Test-Path $dir)) { New-Item -ItemType Directory -Path $dir | Out-Null }
$rng = New-Object System.Random
$exact = 0; $dok = 0; $dtot = 0
$rows = @()
for ($i = 1; $i -le $N; $i++) {
    $code = -join (1..4 | ForEach-Object { $rng.Next(0, 10) })
    $gif = Join-Path $dir "c$i.gif"
    & $gen $code $gif $Fps $Seconds | Out-Null
    $out = (& $sol $gif 2>$null) -join ''
    $pred = [regex]::Match($out, 'code=(\d{4})').Groups[1].Value
    if ($pred -eq $code) { $exact++ }
    for ($k = 0; $k -lt 4; $k++) {
        if ($pred.Length -eq 4 -and $pred[$k] -eq $code[$k]) { $dok++ }
        $dtot++
    }
    $rows += "$i,$code,$pred"
    if ($i % 25 -eq 0) { Write-Host "  $i/$N exact=$exact" }
}
$rows | Set-Content (Join-Path $dir "results.csv")
Write-Host ("EXACT = {0}/{1} ({2}%)  DIGIT_ACC = {3}%" -f $exact, $N, [math]::Round(100.0 * $exact / $N, 1), [math]::Round(100.0 * $dok / $dtot, 1))