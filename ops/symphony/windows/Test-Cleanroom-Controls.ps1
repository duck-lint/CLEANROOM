Set-StrictMode -Version Latest
$root = $PSScriptRoot

function Assert-Contains {
    param([string]$Text, [string]$Needle, [string]$Description)
    if ($Text.IndexOf($Needle, [System.StringComparison]::OrdinalIgnoreCase) -lt 0) {
        throw "Control assertion failed: $Description"
    }
}

$common = Get-Content (Join-Path $root 'Cleanroom-Control.Common.ps1') -Raw
$check = Get-Content (Join-Path $root 'Check-Cleanroom.ps1') -Raw
$stopNow = Get-Content (Join-Path $root 'Stop-Cleanroom-Now.ps1') -Raw
$finish = Get-Content (Join-Path $root 'Finish-Cleanroom-Work.ps1') -Raw

Assert-Contains $common '/home/duck-lint/symphony/scripts/project.py' 'deployed pilot command is used'
Assert-Contains $common "'finish'" 'finish is an allowed pilot action'
if ($common -match 'symphony-pilot source|PROJECT-REPOS\\symphony-pilot|github-token\.env|cleanroom-symphony') {
    throw 'Control assertion failed: obsolete source, credential, or legacy state path remains'
}

$confirmation = $stopNow.IndexOf('MessageBoxDefaultButton]::Button2', [System.StringComparison]::OrdinalIgnoreCase)
$invocation = $stopNow.IndexOf("Invoke-PilotAction -Action 'stop-now'", [System.StringComparison]::OrdinalIgnoreCase)
if ($confirmation -lt 0 -or $invocation -lt 0 -or $confirmation -gt $invocation) {
    throw 'Control assertion failed: Stop Now is not confirmation-gated'
}
Assert-Contains $finish "Invoke-PilotAction -Action 'finish'" 'Finish delegates to pilot finish'
if ($finish -match "Invoke-PilotAction -Action 'stop'") {
    throw 'Control assertion failed: Finish still delegates to stop'
}
Assert-Contains $check "'^(WORKING|FINISHING|NEEDS YOU)'" 'unsafe states use warning mapping'
Assert-Contains $check "'^(IDLE|STOPPED)'" 'safe states use information mapping'
Assert-Contains $check '^(WORKING|FINISHING|NEEDS YOU|IDLE)' 'dashboard opens only for running states'

Write-Output 'CLEANROOM Windows control assertions passed.'
