. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    $output = @(Invoke-PilotAction -Action 'status')
    Open-CleanroomDashboard
    $message = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ }) -join "`n"
    Show-CleanroomMessage -Title 'CLEANROOM status' -Message $message
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
