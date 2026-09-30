# WinSCP 6.5.7: installed, SFTP acceptance blocked

Count remains 7/1000. No WinSCP acceptance receipt, catalogue entry or market publication was created.

## Source and installation

[Official downloads](https://winscp.net/eng/downloads.php), [release checksums](https://winscp.net/download/WinSCP-6.5.7-ReadMe.txt), [license](https://winscp.net/eng/docs/license). Program GPL-3.0-or-later; installer, icons and trademarks have separate notices/restrictions.

Fixed official SourceForge Setup EXE: 12,361,264 bytes, SHA-256 `65103dad94a907bf8b7b9b8db8174dfb60010237d9125db066f34d89817bcd99`, matched vendor checksum before installation. Wine 11.14, normal UID 1000. `/CURRENTUSER /SP- /VERYSILENT /SUPPRESSMSGBOXES /NORESTART` installation job `job-1790730879400-6` succeeded, generation `gen-job-1790730879400-6`. Launcher `users/forge/AppData/Local/Programs/WinSCP/WinSCP.exe`, SHA-256 `ac0644af380d90f666ad4cacdd8f8c0178cf97a86e50c5e40cfd065a292a2445`.

Initial submission encountered maximum parallel jobs. The sole pending PortableApps license job was cancelled; its evidence and artifact remain intact. No license acceptance was performed.

## Real GUI result and retained failures

Prepared local upload and remote download UTF-8 fixtures with Chinese text. Tested SFTP to the existing isolated VM at `forge@127.0.0.1`. Host key was derived from the builder's already trusted public known_hosts entry, fingerprint `SHA256:s172wpAv8B/u+KxOYbDWewuY2d/Fv2ueiir8Mi0sGrs`.

First connection failed because the test invocation supplied the OpenSSH `SHA256:` prefix inside WinSCP's fingerprint format. Retained screenshot `2026-09-30-winscp-hostkey-format.jpg`. Corrected the string representation (same pinned key; no wildcard, trust bypass or unknown-key acceptance). [Command-line documentation](https://winscp.net/eng/docs/commandline) describes `/hostkey` and `/ini=nul`; the latter prevents persistent configuration.

The corrected GUI connection passed host identity verification but failed authentication: **No supported authentication methods available (server sent: publickey)**. Screenshot `2026-09-30-winscp-auth-blocker.jpg`. Existing SSH private keys were not copied into the application; password authentication was not enabled. No upload/download/edit completed. Both launch jobs report `succeeded` despite the GUI authentication error, so process status alone must never count this application. Terminal job statuses are in `2026-09-30-winscp-jobs.json`.

Next requirement: an authorized SFTP test endpoint with usable authentication, followed by GUI upload, download, edit and checksum verification, then signed local publication. Installation alone is incomplete.
