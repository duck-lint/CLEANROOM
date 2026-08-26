. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    $display = Get-CleanroomDisplayState
    Open-CleanroomDashboard
    Show-CleanroomMessage -Title 'CLEANROOM status' -Message $display.Text -Kind $(if ($display.Safe) { 'Information' } else { 'Warning' })
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
