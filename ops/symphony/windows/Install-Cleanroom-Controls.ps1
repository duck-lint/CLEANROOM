Set-StrictMode -Version Latest

$scriptRoot = $PSScriptRoot
$powershellCommand = Get-Command pwsh.exe -ErrorAction SilentlyContinue
if ($null -eq $powershellCommand) { $powershellCommand = Get-Command powershell.exe -ErrorAction Stop }
$powershell = $powershellCommand.Source
$shell = New-Object -ComObject WScript.Shell
$desktop = [Environment]::GetFolderPath('Desktop')
$startMenu = Join-Path ([Environment]::GetFolderPath('Programs')) 'CLEANROOM'
New-Item -ItemType Directory -Path $startMenu -Force | Out-Null

$controls = @(
    @{ Name = 'Start CLEANROOM Work'; Script = 'Start-Cleanroom-Work.ps1'; Description = 'Start the CLEANROOM Symphony pilot' },
    @{ Name = 'Check CLEANROOM'; Script = 'Check-Cleanroom.ps1'; Description = 'Show the current CLEANROOM lifecycle state' },
    @{ Name = 'Finish CLEANROOM Work'; Script = 'Finish-Cleanroom-Work.ps1'; Description = 'Drain and finish CLEANROOM safely' },
    @{ Name = 'Stop CLEANROOM Now'; Script = 'Stop-Cleanroom-Now.ps1'; Description = 'Emergency stop for CLEANROOM work' },
    @{ Name = 'Finish CLEANROOM and Shut Down PC'; Script = 'Finish-Cleanroom-And-Shutdown.ps1'; Description = 'Finish CLEANROOM, then shut down Windows' }
)

foreach ($control in $controls) {
    $targetScript = Join-Path $scriptRoot $control.Script
    foreach ($folder in @($desktop, $startMenu)) {
        $shortcutPath = Join-Path $folder "$($control.Name).lnk"
        $shortcut = $shell.CreateShortcut($shortcutPath)
        $shortcut.TargetPath = $powershell
        $shortcut.Arguments = "-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File `"$targetScript`""
        $shortcut.WorkingDirectory = $scriptRoot
        $shortcut.Description = $control.Description
        $shortcut.IconLocation = "$env:SystemRoot\System32\WindowsPowerShell\v1.0\powershell.exe,0"
        $shortcut.Save()
    }
}

Write-Output "Installed $($controls.Count) CLEANROOM controls in the Desktop and Start-menu CLEANROOM folder."
