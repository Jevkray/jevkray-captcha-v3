param([Parameter(Mandatory=$true)][string]$Gif, [string]$Out = "", [int]$Cols=4, [int]$Rows=4, [int]$Scale=2, [int]$From=-1, [int]$To=-1)
Add-Type -AssemblyName System.Drawing
$img = [System.Drawing.Image]::FromFile((Resolve-Path $Gif))
$dim = New-Object System.Drawing.Imaging.FrameDimension $img.FrameDimensionsList[0]
$total = $img.GetFrameCount($dim)
if ($From -lt 0) { $From = 0 }
if ($To -lt 0) { $To = $total - 1 }
$need = $Cols * $Rows
$w = $img.Width * $Scale
$h = $img.Height * $Scale
$bmp = New-Object System.Drawing.Bitmap ($w * $Cols), ($h * $Rows)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::NearestNeighbor
$g.Clear([System.Drawing.Color]::FromArgb(0,0,0))
for ($i = 0; $i -lt $need; $i++) {
    $f = $From + [int][math]::Floor($i * ($To - $From) / [math]::Max($need - 1, 1))
    if ($f -ge $total) { $f = $total - 1 }
    $img.SelectActiveFrame($dim, $f) | Out-Null
    $g.DrawImage($img, (($i % $Cols) * $w), ([int][math]::Floor($i / $Cols) * $h), $w, $h)
}
$g.Dispose()
if ($Out -eq "") { $Out = [System.IO.Path]::ChangeExtension($Gif, ".png") }
$bmp.Save($Out, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp.Dispose(); $img.Dispose()
"sprite: $Out (frames $From..$To of $total)"