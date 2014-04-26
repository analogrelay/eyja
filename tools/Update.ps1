. "$PSScriptRoot\..\tools\Common.ps1"
exec pushd $PSScriptRoot

$repos = @(
    "https://github.com/kellabyte/Haywire")

$repos | ForEach-Object {
    Write-Banner "** Checking $_ **"
    $folder = [IO.Path]::GetFileNameWithoutExtension($_)
    if(!(Test-Path $folder)) {
        Write-Info "Cloning..."
        exec git clone $_
    }
    else {
        Write-Info "Updating..."
        exec pushd $_
        exec git checkout master
        exec git pull origin master
        exec popd
    }
}

exec popd