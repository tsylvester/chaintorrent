$ErrorActionPreference = 'Continue'
$logFile = Join-Path $PSScriptRoot 'rust-checks.log'

$checks = @(
    @{ Name = 'cargo check --workspace --all-targets --all-features'; Run = { cargo check --workspace --all-targets --all-features } },
    @{ Name = 'cargo clippy --workspace --all-targets --all-features'; Run = { cargo clippy --workspace --all-targets --all-features } },
    @{ Name = 'cargo fmt --all --check'; Run = { cargo fmt --all --check } },
    @{ Name = 'cargo deny check'; Run = { cargo deny check } },
    @{ Name = 'cargo test --workspace --all-targets --all-features'; Run = { cargo test --workspace --all-targets --all-features } }
)

"Rust checks started: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss zzz')" |
    Set-Content -LiteralPath $logFile -Encoding utf8

foreach ($check in $checks) {
    "`r`n> $($check.Name)`r`n" |
        Add-Content -LiteralPath $logFile -Encoding utf8

    & $check.Run 2>&1 |
        Out-File -LiteralPath $logFile -Append -Encoding utf8

    $exitCode = $LASTEXITCODE
    "`r`n[exit code: $exitCode]`r`n" |
        Add-Content -LiteralPath $logFile -Encoding utf8
}

Write-Host "Checks finished. Full output: $logFile"