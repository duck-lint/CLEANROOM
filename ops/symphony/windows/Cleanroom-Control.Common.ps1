Set-StrictMode -Version Latest

# This file is only a Windows adapter. Symphony lifecycle state, credentials,
# workflow selection, and process ownership belong to symphony-pilot.
$script:Distro = 'Ubuntu-24.04'
$script:PilotProfile = '/home/duck-lint/symphony/profile.toml'
$script:DashboardUrl = 'http://127.0.0.1:4040'

function Get-PilotSourceWslPath {
    $projectRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..\..\..\symphony-pilot'))
    if (-not (Test-Path -LiteralPath (Join-Path $projectRoot 'scripts\project.py'))) {
        throw "The symphony-pilot source was not found beside CLEANROOM at $projectRoot."
    }

    $path = @(& wsl.exe --distribution $script:Distro --exec wslpath -a -u $projectRoot 2>&1)
    if ($LASTEXITCODE -ne 0 -or $path.Count -eq 0) {
        throw 'Ubuntu WSL could not translate the symphony-pilot source path.'
    }
    return [string]$path[-1].Trim()
}

function Invoke-PilotAction {
    param(
        [Parameter(Mandatory)]
        [ValidateSet('start', 'status', 'stop', 'stop-now', 'test')]
        [string]$Action
    )

    $source = Get-PilotSourceWslPath
    $arguments = @(
        '--distribution', $script:Distro,
        '--exec', 'python3',
        "$source/scripts/project.py",
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

function Show-CleanroomNotification {
    param(
        [Parameter(Mandatory)][string]$Title,
        [Parameter(Mandatory)][string]$Message,
        [string]$Url
    )

    try {
        Add-Type -AssemblyName System.Runtime.WindowsRuntime -ErrorAction SilentlyContinue
        $null = [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime]
        $xml = New-Object Windows.Data.Xml.Dom.XmlDocument
        $escapedTitle = [System.Security.SecurityElement]::Escape($Title)
        $escapedMessage = [System.Security.SecurityElement]::Escape($Message)
        $launch = if ($Url) { [System.Security.SecurityElement]::Escape($Url) } else { '' }
        $xml.LoadXml("<toast launch='$launch'><visual><binding template='ToastGeneric'><text>$escapedTitle</text><text>$escapedMessage</text></binding></visual></toast>")
        $toast = [Windows.UI.Notifications.ToastNotification]::new($xml)
        [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('CLEANROOM').Show($toast)
        return
    } catch {
        try {
            Add-Type -AssemblyName System.Windows.Forms
            $icon = New-Object System.Windows.Forms.NotifyIcon
            $icon.Icon = [System.Drawing.SystemIcons]::Information
            $icon.Visible = $true
            $icon.ShowBalloonTip(8000, $Title, $Message, [System.Windows.Forms.ToolTipIcon]::Info)
            Start-Sleep -Seconds 8
            $icon.Dispose()
        } catch {
            Write-Warning "CLEANROOM notification unavailable: $Title - $Message"
        }
    }
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
    Show-CleanroomNotification -Title 'CLEANROOM control needs attention' -Message $Message
    exit 1
}
