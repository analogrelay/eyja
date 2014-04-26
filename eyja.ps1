$eyja = Join-Path $PSScriptRoot "builds\windows\debug\eyja\eyja.exe"
if(!(Test-Path $eyja)) {
    throw "Eyja has not been built, use build.ps1 to build it"
}
&$eyja @args