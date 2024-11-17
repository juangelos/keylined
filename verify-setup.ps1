#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

function Test-Command {
    param (
        [string]$Name,
        [string]$Command,
        [string]$ExpectedPattern
    )
    
    Write-Host "Checking $Name... " -NoNewline
    
    try {
        $output = Invoke-Expression $Command
        if ($output -match $ExpectedPattern) {
            Write-Host "OK" -ForegroundColor Green
            return $true
        }
    }
    catch {
        Write-Host "Failed" -ForegroundColor Red
        Write-Host $_.Exception.Message
        return $false
    }
    
    Write-Host "Version mismatch" -ForegroundColor Yellow
    return $false
}

$checks = @(
    @{
        Name = "Git"
        Command = "git --version"
        Pattern = "git version 2\.[4-9][0-9]"
    },
    @{
        Name = "Rust"
        Command = "rustc --version"
        Pattern = "rustc 1\.7[5-9]\.[0-9]+"
    },
    @{
        Name = "Cargo"
        Command = "cargo --version"
        Pattern = "cargo 1\.7[5-9]\.[0-9]+"
    },
    @{
        Name = "PowerShell"
        Command = "$PSVersionTable.PSVersion.ToString()"
        Pattern = "7\.[3-9]\.[0-9]+"
    },
    @{
        Name = "LLVM"
        Command = "clang --version"
        Pattern = "clang version 1[5-9]\."
    }
)

$allPassed = $true
foreach ($check in $checks) {
    if (-not (Test-Command @check)) {
        $allPassed = $false
    }
}

# Check Rust components
Write-Host "`nChecking Rust components..."
$components = @("clippy", "rustfmt", "rust-analyzer", "rust-src")
foreach ($component in $components) {
    Write-Host "Checking $component... " -NoNewline
    $installed = rustup component list | Select-String "^$component"
    if ($installed) {
        Write-Host "OK" -ForegroundColor Green
    } else {
        Write-Host "Missing" -ForegroundColor Red
        $allPassed = $false
    }
}

if (-not $allPassed) {
    Write-Host "`nSome checks failed. Please review the requirements and try again." -ForegroundColor Red
    exit 1
}

Write-Host "`nAll checks passed!" -ForegroundColor Green
