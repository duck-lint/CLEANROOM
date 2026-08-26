Set-StrictMode -Version Latest

Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class CleanroomPower {
    [DllImport("kernel32.dll")]
    public static extern uint SetThreadExecutionState(uint flags);
}
'@

$continuous = [uint32]0x80000000
$systemRequired = [uint32]0x00000001
$displayRequired = [uint32]0x00000002
[void][CleanroomPower]::SetThreadExecutionState($continuous -bor $systemRequired -bor $displayRequired)
try {
    while ($true) { Start-Sleep -Seconds 30 }
} finally {
    [void][CleanroomPower]::SetThreadExecutionState($continuous)
}
