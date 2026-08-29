. (Join-Path $PSScriptRoot 'Cleanroom-Control.Common.ps1')

try {
    $output = @(Invoke-PilotAction -Action 'status')
    $message = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ }) -join "`n"
    $statusLine = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ } | Select-Object -First 1)
    if ($statusLine -match '^(WORKING|FINISHING|NEEDS YOU|IDLE)') {
        Open-CleanroomDashboard
    }
    if ($statusLine -match '^(WORKING|FINISHING|NEEDS YOU)') {
        $kind = 'Warning'
    } elseif ($statusLine -match '^(IDLE|STOPPED)') {
        $kind = 'Information'
    } else {
        $kind = 'Warning'
    }
    Show-CleanroomMessage -Title 'CLEANROOM status' -Message $message -Kind $kind
} catch {
    Invoke-ControlFailure $_.Exception.Message
}
