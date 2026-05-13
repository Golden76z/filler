<#
.SYNOPSIS
  Run a bash command inside the `filler` Docker image with this `solution/` folder mounted at `/filler/solution`.

.DESCRIPTION
  Works from Windows PowerShell without hand-escaping volume paths. Use `-Interactive` when you need a TTY
  (e.g. `visualizer`, or an interactive shell). Use `-Build` to (re)build the image from `../docker_image`.

.EXAMPLE
  .\docker-filler.ps1 -Build -Command 'cd /filler/solution && cargo build --release'

.EXAMPLE
  .\docker-filler.ps1 -Command 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'

.EXAMPLE
  .\docker-filler.ps1 -Interactive -Command 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
#>
[CmdletBinding()]
param(
  [Parameter(Mandatory = $false)]
  [string] $Command,

  [switch] $Build,
  [switch] $Interactive,

  [string] $Image = "filler"
)

$ErrorActionPreference = "Stop"

$SolutionDir = $PSScriptRoot
$RepoRoot = Split-Path -Parent $SolutionDir
$DockerContext = Join-Path $RepoRoot "docker_image"

if (-not (Test-Path -LiteralPath $DockerContext)) {
  Write-Error "Expected docker build context at: $DockerContext"
}

if ($Build) {
  docker build -t $Image -- $DockerContext
}

$Vol = ((Resolve-Path -LiteralPath $SolutionDir).Path) -replace "\\", "/"

$dockerArgs = @("run", "--rm", "--entrypoint", "bash")
if ($Interactive) {
  $dockerArgs += @("-i", "-t")
}
$dockerArgs += @("-v", "${Vol}:/filler/solution", $Image)

if (-not $PSBoundParameters.ContainsKey("Command") -or [string]::IsNullOrWhiteSpace($Command)) {
  if ($Interactive) {
    $Command = "bash"
  }
  else {
    Write-Host @"
Usage:
  Record a game log (no TTY needed):
    .\docker-filler.ps1 -Command 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'

  Replay with ANSI visualizer (needs TTY):
    .\docker-filler.ps1 -Interactive -Command 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'

  Open a shell in the container:
    .\docker-filler.ps1 -Interactive

  First-time image build:
    .\docker-filler.ps1 -Build -Interactive
"@
    exit 2
  }
}

$dockerArgs += @("-c", $Command)
& docker @dockerArgs
exit $LASTEXITCODE
