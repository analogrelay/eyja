. "$PSScriptRoot\Common.ps1"

pushd "$RepoRoot\ext"
try {
    $repos = @(
        "https://github.com/kellabyte/Haywire",
        "https://github.com/joyent/libuv",
        "https://chromium.googlesource.com/external/gyp")

    $repos | ForEach-Object {
        $folder = [IO.Path]::GetFileNameWithoutExtension($_)
        if(!(Test-Path $folder)) {
            Write-Banner "Cloning $_"
            exec git clone $_
        }
        else {
            Write-Banner "Updating $_"
            pushd $folder
            try {
                exec git checkout -f master
                exec git pull origin master
            }
            finally {
                popd
            }
        }
    }
} finally {
    popd
}