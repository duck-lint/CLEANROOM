Set-StrictMode -Version Latest

# This file is only a Windows adapter. Symphony lifecycle state, credentials,
# workflow selection, and process ownership belong to symphony-pilot.
$script:Distro = 'Ubuntu-24.04'
$script:PilotProfile = '/home/duck-lint/symphony/profile.toml'
$script:PilotCommand = '/home/duck-lint/symphony/scripts/project.py'
$script:DashboardUrl = 'http://127.0.0.1:4040'

function Invoke-PilotAction {
    param(
        [Parameter(Mandatory)]
        [ValidateSet('start', 'status', 'stop', 'stop-now', 'finish', 'test')]
        [string]$Action
    )

    $arguments = @(
        '--distribution', $script:Distro,
        '--exec', 'python3',
        $script:PilotCommand,
        '--profile', $script:PilotProfile,
        $Action
    )
    $output = @(& wsl.exe @arguments 2>&1)
    if ($LASTEXITCODE -ne 0) {
        $detail = ($output | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ }) -join ' '
        if (-not $detail) { $detail = 'no diagnostic was returned' }
        throw "symphony-pilot $Action failed: $detail"
    }
    return $output
}

function Show-CleanroomMessage {
    param(
        [Parameter(Mandatory)][string]$Title,
        [Parameter(Mandatory)][string]$Message,
        [ValidateSet('Information', 'Warning', 'Error')][string]$Kind = 'Information'
    )

    try {
        Add-Type -AssemblyName System.Windows.Forms
        $icon = [System.Windows.Forms.MessageBoxIcon]::$Kind
        [void][System.Windows.Forms.MessageBox]::Show($Message, $Title, 'OK', $icon)
    } catch {
        Write-Output "$Title`n$Message"
    }
}

function Open-CleanroomDashboard {
    Start-Process $script:DashboardUrl | Out-Null
}

function Invoke-ControlFailure {
    param([Parameter(Mandatory)][string]$Message)
    Show-CleanroomMessage -Title 'CLEANROOM control could not complete' -Message $Message -Kind Error
    exit 1
}
