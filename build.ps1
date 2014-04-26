param(
    [string]$Configuration = "debug",
    [switch]$UpdateDependencies)

. "$PSScriptRoot\tools\Common.ps1"

$env:GYP_MSVS_VERSION="2013"

& "$PSScriptRoot\tools\UpdateDependencies.ps1" -Update:$UpdateDependencies
exec python "$PSScriptRoot\ext\gyp\gyp_main.py" "--depth=." "-Icommon.gypi" "-Dlibrary=static_library" "-Duv_library=static_library" "-Dtarget_arch=x64" "--build=$Configuration" "$PSScriptRoot\root.gyp" | Out-Host