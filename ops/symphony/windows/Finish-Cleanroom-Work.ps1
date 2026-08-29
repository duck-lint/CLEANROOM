param([switch]$ShutdownAfter)

. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    $output = @(Invoke-PilotAction -Action 'stop')
    $message = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ }) -join "`n"
    Show-CleanroomMessage -Title 'CLEANROOM finished' -Message $message
    Show-CleanroomNotification -Title 'CLEANROOM finished' -Message 'CLEANROOM finished - SAFE TO SHUT DOWN'
    if ($ShutdownAfter) {
        Start-Sleep -Seconds 2
        shutdown.exe /s /t 0 /d p:4:1 /c 'CLEANROOM finished safely'
    }
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
