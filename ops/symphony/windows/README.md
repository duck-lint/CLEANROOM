# CLEANROOM Windows controls

These small PowerShell launchers are the Windows-facing lifecycle boundary for
the external Symphony pilot. They do not implement scheduling or replace
Symphony. Symphony's `/api/v1/state` is the runtime authority for running,
blocked, and retrying issue state.

Run `Install-Cleanroom-Controls.ps1` once from Windows PowerShell. It creates
Desktop and Start-menu shortcuts for:

- Start CLEANROOM Work
- Check CLEANROOM
- Finish CLEANROOM Work
- Stop CLEANROOM Now
- Finish CLEANROOM and Shut Down PC

The launchers wake `Ubuntu-24.04`, source Symphony's token only inside the WSL
launch shell, start the official detached binary, and never pass the token to
Codex. They use the loopback dashboard at `127.0.0.1:4040`; operators do not
need to remember that address because Start and Check open it automatically.

The host state file is `%LOCALAPPDATA%\\CLEANROOM\\control-state.json`. It
contains lifecycle markers and process IDs only, never credentials. A separate
hidden keep-awake process holds a Windows execution-state request and is
released by the normal Finish path. No Windows startup task is installed, so
Symphony remains stopped after reboot until Start is explicitly invoked.

Finish drains the one-issue pilot by waiting for either durable completion or a
durable Symphony blocked state. It does not dispatch a replacement issue after
Finish is requested. Stop Now is the only path allowed to interrupt active
work, and it requires a visible confirmation.

Notifications are convenience only. The GitHub `symphony:human` label and
Symphony workpad remain authoritative. A blocked notification includes the
GitHub issue URL.
