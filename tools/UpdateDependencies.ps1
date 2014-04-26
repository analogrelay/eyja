param([switch]$Update)

. "$PSScriptRoot\Common.ps1"

pushd "$RepoRoot\ext"
try {
    $repos = @(
        "https://chromium.googlesource.com/external/gyp")

    $repos | ForEach-Object {
        $repo = $_;
        if($_ -is [array]) {
            $repo = $_[0];
            $folder = $_[1];
        } else {
            $folder = [IO.Path]::GetFileNameWithoutExtension($_);
        }

        if(!(Test-Path $folder)) {
            Write-Banner "Cloning $repo"
            exec git clone $repo $folder
        }
        elseif($Update) {
            Write-Banner "Updating $repo"
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