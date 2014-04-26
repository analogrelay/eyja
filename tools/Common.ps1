function Write-Banner($banner) {
    Write-Host -ForegroundColor Green "*** $banner ***"
}

function Write-Info($info) {
    Write-Host -ForegroundColor Green $info
}

function exec($cmd) {
    Write-Host -ForegroundColor Magenta "exec> " -NoNewLine
    Write-Host "$cmd $args"
    &$cmd @args
}