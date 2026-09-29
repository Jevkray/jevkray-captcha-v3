param([Parameter(Mandatory=$true)][string]$Csv)
# CSV: строки "code,answer" (4 цифры)
$rows = Get-Content $Csv | Where-Object { $_ -match ',' }
$exact = 0; $dok = 0; $dtot = 0; $n = 0
foreach ($r in $rows) {
    $p = $r.Split(','); if ($p.Length -lt 2) { continue }
    $code = $p[0].Trim(); $ans = $p[1].Trim(); $n++
    if ($ans -eq $code) { $exact++ }
    for ($k = 0; $k -lt 4; $k++) {
        if ($ans.Length -eq 4 -and $ans[$k] -eq $code[$k]) { $dok++ }
        $dtot++
    }
}
$digit = if ($dtot -gt 0) { 100.0 * $dok / $dtot } else { 0 }
$exactPct = if ($n -gt 0) { 100.0 * $exact / $n } else { 0 }
"examples={0}  exact={1} ({2:N1}%)  per-digit={3:N1}%" -f $n, $exact, $exactPct, $digit
foreach ($k in 1..4) {
    $p = [math]::Pow(10, -(4 - $k))
    $succ = 100.0 * (1 - [math]::Pow(1 - $p, 100))
    "if model reads {0} of 4 digits reliably -> success in 100 attempts: {1:N1}%" -f $k, $succ
}