$libuv = Convert-Path "$PSScriptRoot\..\ext\libuv"
$gyp = Convert-Path "$PSScriptRoot\..\ext\gyp"
$haywire = Convert-Path "$PSScriptRoot\..\ext\Haywire"

Copy-Item $libuv "$haywire\lib\libuv" -Recurse -Force 
Copy-Item $gyp "$haywire\lib\libuv\build" -Recurse -Force