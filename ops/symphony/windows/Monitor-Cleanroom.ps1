. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

$seen = @{}
while (Test-Path -LiteralPath $script:StatePath) {
    try {
        $snapshot = Get-RuntimeSnapshot
        foreach ($entry in (Get-Entries $snapshot 'blocked')) {
            $number = Get-IssueNumber $entry
            if (-not $seen.ContainsKey($number)) {
                $seen[$number] = $true
                Show-CleanroomNotification `
                    -Title 'CLEANROOM needs you' `
                    -Message "Issue #$number is durably human-blocked. Work is safely paused." `
                    -Url "$script:GitHubIssueBaseUrl/$number"
            }
        }
    } catch {
        # Notification failure or a temporary dashboard failure never changes state.
    }
    Start-Sleep -Seconds 5
}
