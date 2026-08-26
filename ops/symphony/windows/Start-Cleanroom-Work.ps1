. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    Ensure-Ollama
    $authorized = @(Get-AuthorizedIssueNumbers)
    if ($authorized.Count -gt 1) {
        throw "More than one open symphony:auto issue is authorized (#$($authorized -join ', #')). Keep the pilot limited to one issue before starting."
    }

    $existing = Get-RuntimeSnapshot
    if ($null -ne $existing) {
        Open-CleanroomDashboard
        $display = Get-CleanroomDisplayState
        Show-CleanroomNotification -Title 'CLEANROOM is already running' -Message $display.Text
        exit 0
    }

    $null = Invoke-WslScript 'true'
    $symphonyProcess = Start-Symphony
    Wait-ForDashboard
    $keepAwake = Start-KeepAwake
    $monitor = Start-Process -FilePath (Get-CleanroomPowerShell) -WindowStyle Hidden -PassThru -ArgumentList @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', (Join-Path $PSScriptRoot 'Monitor-Cleanroom.ps1'))
    $snapshot = Get-RuntimeSnapshot
    $issue = Get-FirstIssueNumber $snapshot
    Write-ControlState -Lifecycle running -LastIssue $issue -SymphonyPid $symphonyProcess.Id -KeepAwakePid $keepAwake.Id -MonitorPid $monitor.Id
    Open-CleanroomDashboard
    Show-CleanroomNotification -Title 'CLEANROOM is running' -Message 'The Symphony dashboard is open. Windows should remain on.'
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
