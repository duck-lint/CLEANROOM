param([switch]$ShutdownAfter)

. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

function Finish-Now {
    param($State)

    Stop-Symphony | Out-Null
    Stop-ChildProcess $State.symphony_pid
    Stop-ChildProcess $State.monitor_pid
    Stop-ChildProcess $State.keep_awake_pid
    Remove-ControlState
    Show-CleanroomNotification -Title 'CLEANROOM finished' -Message 'CLEANROOM finished - SAFE TO SHUT DOWN'
    if ($ShutdownAfter) {
        Start-Sleep -Seconds 2
        shutdown.exe /s /t 0 /d p:4:1 /c 'CLEANROOM finished safely'
    }
}

try {
    $state = Read-ControlState
    $snapshot = Get-RuntimeSnapshot
    if ($null -eq $state -and $null -eq $snapshot) {
        Show-CleanroomNotification -Title 'CLEANROOM already stopped' -Message 'CLEANROOM finished - SAFE TO SHUT DOWN'
        if ($ShutdownAfter) { shutdown.exe /s /t 0 /d p:4:1 /c 'CLEANROOM already stopped' }
        exit 0
    }

    $issue = Get-FirstIssueNumber $snapshot
    $keepAwakePid = if ($null -ne $state) { [int]$state.keep_awake_pid } else { 0 }
    $monitorPid = if ($null -ne $state) { [int]$state.monitor_pid } else { 0 }
    $symphonyPid = if ($null -ne $state) { [int]$state.symphony_pid } else { 0 }
    Write-ControlState -Lifecycle finishing -LastIssue $issue -SymphonyPid $symphonyPid -KeepAwakePid $keepAwakePid -MonitorPid $monitorPid
    Show-CleanroomNotification -Title 'CLEANROOM is finishing' -Message "FINISHING #$issue - DO NOT SHUT DOWN YET"
    Open-CleanroomDashboard

    while ($true) {
        $snapshot = Get-RuntimeSnapshot
        if ($null -eq $snapshot) {
            throw 'The Symphony dashboard became unreachable while finishing. The stopping boundary is not proven; Windows must remain on.'
        }
        $blocked = @(Get-Entries $snapshot 'blocked')
        $running = @(Get-Entries $snapshot 'running')
        $retrying = @(Get-Entries $snapshot 'retrying')
        if ($blocked.Count -gt 0) {
            $blockedIssue = Get-IssueNumber ($blocked | Select-Object -First 1)
            Show-CleanroomNotification -Title 'CLEANROOM needs you' -Message "Issue #$blockedIssue is durably human-blocked; work is safely paused." -Url "$script:GitHubIssueBaseUrl/$blockedIssue"
            Finish-Now (Read-ControlState)
            exit 0
        }
        if ($running.Count -eq 0 -and $retrying.Count -eq 0) {
            Finish-Now (Read-ControlState)
            exit 0
        }
        Start-Sleep -Seconds 5
    }
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
