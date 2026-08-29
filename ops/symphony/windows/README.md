# CLEANROOM Windows controls

These small PowerShell launchers are thin Windows adapters for the reusable
`symphony-pilot` project lifecycle. They do not implement scheduling or
replace Symphony. The pilot is the authority for process, credential,
workflow, dashboard, and state handling.

Run `Install-Cleanroom-Controls.ps1` once from Windows PowerShell. It creates
Desktop and Start-menu shortcuts for:

- Start CLEANROOM Work
- Check CLEANROOM
- Finish CLEANROOM Work
- Stop CLEANROOM Now
- Finish CLEANROOM and Shut Down PC

The adapters invoke the atomically deployed pilot command at
`/home/duck-lint/symphony/scripts/project.py` inside `Ubuntu-24.04`, using the
deployed CLEANROOM profile at `/home/duck-lint/symphony/profile.toml`. They never load
credentials, choose a workflow, or maintain a Windows PID/state system. They
use the loopback dashboard at `127.0.0.1:4040`; Start and Check open it when
the pilot reports a running service.

Check delegates to pilot `status`. Finish delegates to pilot `finish`, which
drains active work before stopping; Stop Now is the explicit emergency action
after a Windows Yes/No confirmation. Shutdown delegates to Finish and is
invoked only after pilot reports a safe stopped result. No Windows startup task
is installed, so Symphony remains stopped after reboot until Start is explicitly
invoked.

Generic pilot notifications are convenience only. The GitHub `symphony:human`
label and Symphony workpad remain authoritative. A blocked notification
includes the GitHub issue URL.
