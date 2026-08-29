param([switch]$ShutdownAfter)

. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    Show-CleanroomMessage -Title 'CLEANROOM is finishing' -Message 'CLEANROOM is finishing — current work will be allowed to finish before Symphony stops.' -Kind Warning
    $output = @(Invoke-PilotAction -Action 'finish')
    $message = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ }) -join "`n"
    Show-CleanroomMessage -Title 'CLEANROOM finished' -Message $message
    if ($ShutdownAfter) {
        Start-Sleep -Seconds 2
        shutdown.exe /s /t 0 /d p:4:1 /c 'CLEANROOM finished safely'
    }
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
