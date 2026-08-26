. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    $display = Get-CleanroomDisplayState
    if (-not $display.Safe) {
        Add-Type -AssemblyName System.Windows.Forms
        $answer = [System.Windows.Forms.MessageBox]::Show(
            "Emergency stop will interrupt active CLEANROOM work. The issue may remain open or incomplete.`n`n$($display.Text)`n`nStop immediately?",
            'STOP CLEANROOM NOW', 'YesNo', [System.Windows.Forms.MessageBoxIcon]::Warning)
        if ($answer -ne [System.Windows.Forms.DialogResult]::Yes) { exit 0 }
    }
    $state = Read-ControlState
    Stop-Symphony -Force | Out-Null
    if ($null -ne $state) {
        Stop-ChildProcess $state.symphony_pid
        Stop-ChildProcess $state.monitor_pid
        Stop-ChildProcess $state.keep_awake_pid
    }
    Remove-ControlState
    Show-CleanroomNotification -Title 'CLEANROOM emergency stop' -Message 'CLEANROOM was stopped immediately. Work may be incomplete; this was not the normal Finish path.'
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
