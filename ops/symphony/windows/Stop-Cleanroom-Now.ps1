. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    $output = @(Invoke-PilotAction -Action 'stop-now')
    $message = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ }) -join "`n"
    Show-CleanroomMessage -Title 'CLEANROOM emergency stop' -Message $message -Kind Warning
    Show-CleanroomNotification -Title 'CLEANROOM emergency stop' -Message 'CLEANROOM was stopped immediately. Work may be incomplete; this was not the normal Finish path.'
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
