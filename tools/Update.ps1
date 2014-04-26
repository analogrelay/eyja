. "$PSScriptRoot\Common.ps1"

pushd "$RepoRoot\ext"
try {
    $repos = @(
        "https://github.com/kellabyte/Haywire")

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
                if(git status | where { $_ -like "*working directory clean"}) {
                    exec git checkout master
                    exec git pull origin master
                } else {
                    throw "There are changes in the $_ repository! You must commit them or revert them"
                }
            }
            finally {
                popd
            }
        }
    }
} finally {
    popd
}