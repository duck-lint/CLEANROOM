. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    $null = Invoke-PilotAction -Action 'start'
    Open-CleanroomDashboard
    Show-CleanroomNotification -Title 'CLEANROOM is running' -Message 'The Symphony dashboard is open. Windows should remain on.'
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
