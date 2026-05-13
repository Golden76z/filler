<#
.SYNOPSIS
  Build the native GUI replay app (`visualizer-gui`) on the Windows host.

.DESCRIPTION
  The GUI binary uses eframe/egui and runs as a real desktop window, so it
  must be built on the host (not inside the Docker container, which has no
  display). The audit binary keeps building inside Docker as before — they
  are completely independent.

  Prerequisites:
   * Rust toolchain on Windows. Install once from https://rustup.rs/ and
     reopen PowerShell.

.EXAMPLE
  .\build-gui.ps1
  .\target\release\visualizer-gui.exe .\game.log
#>
[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$SolutionDir = $PSScriptRoot
Set-Location -LiteralPath $SolutionDir

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust toolchain not found." -ForegroundColor Yellow
    Write-Host "Install it once from https://rustup.rs/ (run rustup-init.exe), then"
    Write-Host "reopen PowerShell and re-run this script." -ForegroundColor Yellow
    exit 1
}

Write-Host "Building visualizer-gui (release, gui feature)..." -ForegroundColor Cyan
cargo build --release --features gui --bin visualizer-gui
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$exe = Join-Path $SolutionDir "target\release\visualizer-gui.exe"
if (Test-Path -LiteralPath $exe) {
    Write-Host ""
    Write-Host "Built: $exe" -ForegroundColor Green
    Write-Host "Run:   .\target\release\visualizer-gui.exe .\game.log"
    Write-Host "       (or launch it with no args and drop a game.log on the window)"
} else {
    Write-Host "Build finished but the .exe was not found at $exe" -ForegroundColor Yellow
    exit 1
}
