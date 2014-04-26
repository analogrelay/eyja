. "$PSScriptRoot\Common.ps1"

$libuv = Convert-Path "$PSScriptRoot\..\ext\libuv"
$gyp = Convert-Path "$PSScriptRoot\..\ext\gyp"
$haywire = Convert-Path "$PSScriptRoot\..\ext\Haywire"

exec cp $libuv "$haywire\lib\libuv" -Recurse -Force 
exec cp $gyp "$haywire\lib\libuv\build" -Recurse -Force