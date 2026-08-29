. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

Add-Type -AssemblyName System.Windows.Forms
$choice = [System.Windows.Forms.MessageBox]::Show(
    "Emergency stop will terminate CLEANROOM Symphony immediately.`r`n`r`nActive work may remain incomplete and the current issue may remain open.`r`n`r`nStop CLEANROOM now?",
    'Emergency stop CLEANROOM',
    [System.Windows.Forms.MessageBoxButtons]::YesNo,
    [System.Windows.Forms.MessageBoxIcon]::Warning,
    [System.Windows.Forms.MessageBoxDefaultButton]::Button2)
if ($choice -ne [System.Windows.Forms.DialogResult]::Yes) {
    exit 0
}

try {
    $output = @(Invoke-PilotAction -Action 'stop-now')
    $message = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ }) -join "`n"
    Show-CleanroomMessage -Title 'CLEANROOM emergency stop' -Message $message -Kind Warning
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
