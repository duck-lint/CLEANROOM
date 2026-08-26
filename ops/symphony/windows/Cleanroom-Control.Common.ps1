Set-StrictMode -Version Latest

$script:ControlRoot = Join-Path $env:LOCALAPPDATA 'CLEANROOM'
$script:StatePath = Join-Path $script:ControlRoot 'control-state.json'
$script:Distro = 'Ubuntu-24.04'
$script:DashboardUrl = 'http://127.0.0.1:4040'
$script:StateUrl = "$script:DashboardUrl/api/v1/state"
$script:OllamaUrl = 'http://127.0.0.1:11434'
$script:GitHubRepo = 'duck-lint/CLEANROOM'
$script:GitHubIssueBaseUrl = 'https://github.com/duck-lint/CLEANROOM/issues'

function Invoke-WslScript {
    param([Parameter(Mandatory)][string]$Script)

    $output = @(& wsl.exe --distribution $script:Distro --exec bash -lc $Script 2>&1)
    if ($LASTEXITCODE -ne 0) {
        $detail = ($output | ForEach-Object { $_.ToString().Replace([char]0, '').Trim() } | Where-Object { $_ }) -join ' '
        if (-not $detail) { $detail = 'no diagnostic was returned' }
        throw "Ubuntu WSL command failed: $detail"
    }
    return $output
}

function Test-Endpoint {
    param([Parameter(Mandatory)][string]$Url)

    try {
        $null = Invoke-WebRequest -Uri $Url -Method Get -TimeoutSec 3 -UseBasicParsing -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

function Get-RuntimeSnapshot {
    try {
        return Invoke-RestMethod -Uri $script:StateUrl -Method Get -TimeoutSec 3 -ErrorAction Stop
    } catch {
        return $null
    }
}

function Get-Entries {
    param($Snapshot, [Parameter(Mandatory)][string]$Name)

    if ($null -eq $Snapshot) { return @() }
    $value = $Snapshot.$Name
    if ($null -eq $value) { return @() }
    return @($value)
}

function Get-IssueNumber {
    param($Entry)

    foreach ($property in @('issue_identifier', 'issue_number', 'identifier', 'issue_id')) {
        $value = [string]$Entry.$property
        if ($value -match '(?i)(?:GH-)?(\d+)') { return $Matches[1] }
    }
    return '?'
}

function Get-FirstIssueNumber {
    param($Snapshot)

    foreach ($name in @('blocked', 'running', 'retrying')) {
        $entry = (Get-Entries $Snapshot $name | Select-Object -First 1)
        if ($null -ne $entry) { return Get-IssueNumber $entry }
    }
    $state = Read-ControlState
    if ($null -ne $state -and $state.last_issue) { return [string]$state.last_issue }
    return '?'
}

function Read-ControlState {
    if (-not (Test-Path -LiteralPath $script:StatePath)) { return $null }
    try {
        return Get-Content -LiteralPath $script:StatePath -Raw | ConvertFrom-Json -ErrorAction Stop
    } catch {
        throw "CLEANROOM control state is unreadable at $script:StatePath. Do not shut down Windows until Check CLEANROOM succeeds."
    }
}

function Write-ControlState {
    param(
        [Parameter(Mandatory)][ValidateSet('running', 'finishing')][string]$Lifecycle,
        [string]$LastIssue,
        [int]$SymphonyPid = 0,
        [int]$KeepAwakePid = 0,
        [int]$MonitorPid = 0
    )

    New-Item -ItemType Directory -Path $script:ControlRoot -Force | Out-Null
    [ordered]@{
        lifecycle = $Lifecycle
        last_issue = $LastIssue
        symphony_pid = $SymphonyPid
        keep_awake_pid = $KeepAwakePid
        monitor_pid = $MonitorPid
        updated_at = [DateTime]::UtcNow.ToString('o')
    } | ConvertTo-Json | Set-Content -LiteralPath $script:StatePath -Encoding utf8
}

function Remove-ControlState {
    if (Test-Path -LiteralPath $script:StatePath) {
        Remove-Item -LiteralPath $script:StatePath -Force
    }
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
        # A balloon is a convenience fallback only; it never owns lifecycle state.
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

function Get-CleanroomDisplayState {
    $state = Read-ControlState
    $snapshot = Get-RuntimeSnapshot
    $blocked = @(Get-Entries $snapshot 'blocked')
    $running = @(Get-Entries $snapshot 'running')
    $retrying = @(Get-Entries $snapshot 'retrying')
    $issue = Get-FirstIssueNumber $snapshot

    if ($null -eq $snapshot) {
        if ($null -ne $state -and $state.lifecycle -eq 'finishing') {
            return [pscustomobject]@{ Text = "FINISHING #$issue - DO NOT SHUT DOWN YET"; Safe = $false; Snapshot = $null; Issue = $issue }
        }
        if ($null -ne $state -and $state.lifecycle -eq 'running') {
            return [pscustomobject]@{ Text = "WORKING ON #$issue - DO NOT SHUT DOWN"; Safe = $false; Snapshot = $null; Issue = $issue }
        }
        if (-not (Test-SymphonyProcess)) {
            return [pscustomobject]@{ Text = 'STOPPED - SAFE TO SHUT DOWN'; Safe = $true; Snapshot = $null; Issue = $null }
        }
        return [pscustomobject]@{ Text = 'WORKING ON #? - DO NOT SHUT DOWN'; Safe = $false; Snapshot = $null; Issue = '?' }
    }

    if ($blocked.Count -gt 0) {
        return [pscustomobject]@{ Text = "NEEDS YOU #$issue - WORK SAFELY PAUSED"; Safe = $true; Snapshot = $snapshot; Issue = $issue }
    }
    if ($null -ne $state -and $state.lifecycle -eq 'finishing' -and ($running.Count -gt 0 -or $retrying.Count -gt 0)) {
        return [pscustomobject]@{ Text = "FINISHING #$issue - DO NOT SHUT DOWN YET"; Safe = $false; Snapshot = $snapshot; Issue = $issue }
    }
    if ($running.Count -gt 0 -or $retrying.Count -gt 0) {
        return [pscustomobject]@{ Text = "WORKING ON #$issue - DO NOT SHUT DOWN"; Safe = $false; Snapshot = $snapshot; Issue = $issue }
    }
    if ($null -ne $state -and $state.lifecycle -eq 'finishing') {
        return [pscustomobject]@{ Text = "FINISHING #$issue - DO NOT SHUT DOWN YET"; Safe = $false; Snapshot = $snapshot; Issue = $issue }
    }
    return [pscustomobject]@{ Text = 'IDLE - SAFE TO STOP'; Safe = $true; Snapshot = $snapshot; Issue = $null }
}

function Test-SymphonyProcess {
    return $null -ne (Get-NetTCPConnection -LocalPort 4040 -State Listen -ErrorAction SilentlyContinue | Select-Object -First 1)
}

function Ensure-Ollama {
    if (Test-Endpoint "$script:OllamaUrl/api/version") { return }

    $service = Get-Service -Name 'Ollama' -ErrorAction SilentlyContinue
    if ($null -ne $service -and $service.Status -ne 'Running') {
        try { Start-Service -Name 'Ollama' -ErrorAction Stop } catch { throw 'Ollama is installed but could not be started. Start Ollama from Windows, then try Start CLEANROOM Work again.' }
    } elseif ($null -eq $service) {
        $command = Get-Command ollama.exe -ErrorAction SilentlyContinue
        if ($null -eq $command) { throw 'Ollama is not available. Install or start Ollama, then try Start CLEANROOM Work again.' }
        try { Start-Process -FilePath $command.Source -ArgumentList 'serve' -WindowStyle Hidden -ErrorAction Stop | Out-Null } catch { throw 'Ollama was found but could not be started. Start Ollama from Windows, then try again.' }
    }

    $deadline = [DateTime]::UtcNow.AddSeconds(45)
    while ([DateTime]::UtcNow -lt $deadline) {
        if (Test-Endpoint "$script:OllamaUrl/api/version") { return }
        Start-Sleep -Seconds 1
    }
    throw 'Ollama did not become reachable at its configured local endpoint within 45 seconds.'
}

function Get-AuthorizedIssueNumbers {
    $scriptText = @'
set -euo pipefail
source "$HOME/.config/symphony/github-token.env"
curl -fsS -H "Accept: application/vnd.github+json" -H "Authorization: Bearer $SYMPHONY_GITHUB_TOKEN" "https://api.github.com/repos/duck-lint/CLEANROOM/issues?state=open&per_page=100" |
python3 -c 'import json,sys; items=[x for x in json.load(sys.stdin) if "pull_request" not in x and any(str(label.get("name", "")).strip().lower()=="symphony:auto" for label in x.get("labels", []))]; print("\n".join(str(x["number"]) for x in items))'
'@
    return @(Invoke-WslScript $scriptText | ForEach-Object { $_.ToString().Trim() } | Where-Object { $_ -match '^\d+$' })
}

function Start-KeepAwake {
    $scriptPath = Join-Path $PSScriptRoot 'Keep-CleanroomAwake.ps1'
    $powershell = Get-CleanroomPowerShell
    return Start-Process -FilePath $powershell -WindowStyle Hidden -PassThru -ArgumentList @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', $scriptPath)
}

function Get-CleanroomPowerShell {
    $command = Get-Command pwsh.exe -ErrorAction SilentlyContinue
    if ($null -eq $command) { $command = Get-Command powershell.exe -ErrorAction Stop }
    return $command.Source
}

function Stop-ChildProcess {
    param([int]$ProcessId)
    if ($ProcessId -gt 0) {
        $process = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
        if ($null -ne $process) { Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue }
    }
}

function Start-Symphony {
    $scriptText = @'
set -euo pipefail
state_dir="$HOME/.local/state/cleanroom-symphony"
mkdir -p "$state_dir" "$HOME/symphony-logs/cleanroom/log"
if [ -f "$state_dir/pid" ] && kill -0 "$(cat "$state_dir/pid")" 2>/dev/null; then
  echo already-running
  exit 10
fi
source "$HOME/.config/symphony/github-token.env"
printf '%s' "$$" > "$state_dir/pid"
exec "$HOME/symphony/bin/symphony-v0.0.2-linux_x86_64" \
  --i-understand-that-this-will-be-running-without-the-usual-guardrails \
  --logs-root "$HOME/symphony-logs/cleanroom/log" \
  --port 4040 "$HOME/symphony/cleanroom-pilot/WORKFLOW.md" \
  >"$HOME/symphony-logs/cleanroom/launcher.log" 2>&1
'@
    $stdoutPath = Join-Path $script:ControlRoot 'wsl.stdout.log'
    $stderrPath = Join-Path $script:ControlRoot 'wsl.stderr.log'
    New-Item -ItemType Directory -Path $script:ControlRoot -Force | Out-Null
    # Start-Process reconstructs its argument list as a Windows command line.
    # Passing the multiline Bash program directly therefore loses its argv
    # boundary. Encode only the launcher text so WSL receives one command.
    $encodedScript = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($scriptText))
    $encodedCommand = "echo $encodedScript | base64 -d | bash"
    $quotedCommand = '"' + $encodedCommand.Replace('"', '\\"') + '"'
    $process = Start-Process -FilePath 'wsl.exe' -WindowStyle Hidden -PassThru `
        -RedirectStandardOutput $stdoutPath -RedirectStandardError $stderrPath `
        -ArgumentList @('--distribution', $script:Distro, '--exec', 'bash', '-lc', $quotedCommand)
    Start-Sleep -Milliseconds 500
    if ($process.HasExited) {
        $detail = Get-Content -LiteralPath $stderrPath -Raw -ErrorAction SilentlyContinue
        if (-not $detail) { $detail = 'the WSL launcher exited before Symphony became available' }
        throw "Symphony could not be started: $detail"
    }
    return $process
}

function Stop-Symphony {
    param([switch]$Force)

    $signal = if ($Force) { 'KILL' } else { 'TERM' }
    $scriptText = @'
set -euo pipefail
state_dir="$HOME/.local/state/cleanroom-symphony"
pid_file="$state_dir/pid"
if [ ! -f "$pid_file" ]; then exit 0; fi
pid="$(cat "$pid_file")"
if [ -n "$pid" ] && [ -r "/proc/$pid/cmdline" ] && tr '\0' ' ' < "/proc/$pid/cmdline" | grep -Fq '/home/duck-lint/symphony/cleanroom-pilot/WORKFLOW.md'; then
  kill -SIGNAL "$pid"
  if [ 'SIGNAL' = 'TERM' ]; then
    # Burrito/BEAM shutdown can exceed the ordinary idle polling interval.
    # Keep the boundary fail-closed, but give a graceful TERM enough time.
    for _ in $(seq 1 45); do
      kill -0 "$pid" 2>/dev/null || break
      sleep 1
    done
    if kill -0 "$pid" 2>/dev/null; then echo graceful-stop-timeout; exit 12; fi
  fi
fi
rm -f "$pid_file"
echo stopped
'@
    $scriptText = $scriptText.Replace('SIGNAL', $signal)
    return Invoke-WslScript $scriptText
}

function Wait-ForDashboard {
    param([int]$TimeoutSeconds = 45)
    $deadline = [DateTime]::UtcNow.AddSeconds($TimeoutSeconds)
    while ([DateTime]::UtcNow -lt $deadline) {
        if (Test-Endpoint $script:StateUrl) { return }
        Start-Sleep -Seconds 1
    }
    throw 'Symphony did not expose its dashboard state within 45 seconds. It was not marked safe to shut down.'
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
