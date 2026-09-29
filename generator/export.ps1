param([string]$Code = "1234", [int]$Cols = 4, [int]$Rows = 4, [int]$Scale = 2)
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$gen = Join-Path $root "target\release\capgen.exe"
$gif = Join-Path $env:TEMP "cap_export.gif"
$png = Join-Path $env:TEMP ("cap_export_" + $Code + ".png")
& $gen $Code $gif 20 30 | Out-Null
Add-Type -AssemblyName System.Drawing
$img = [System.Drawing.Image]::FromFile($gif)
$dim = New-Object System.Drawing.Imaging.FrameDimension $img.FrameDimensionsList[0]
$total = $img.GetFrameCount($dim)
$need = $Cols * $Rows
$w = $img.Width * $Scale
$h = $img.Height * $Scale
$bmp = New-Object System.Drawing.Bitmap ($w * $Cols), ($h * $Rows)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::NearestNeighbor
for ($i = 0; $i -lt $need; $i++) {
    $f = [int][math]::Floor($i * ($total - 1) / ($need - 1))
    $img.SelectActiveFrame($dim, $f) | Out-Null
    $x = ($i % $Cols) * $w
    $y = [int][math]::Floor($i / $Cols) * $h
    $g.DrawImage($img, $x, $y, $w, $h)
}
$g.Dispose()
$bmp.Save($png, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp.Dispose(); $img.Dispose()
Remove-Item $gif -ErrorAction SilentlyContinue
"sprite: $png  (code=$Code, frames $need of $total, {0}x{1})" -f ($w * $Cols), ($h * $Rows)