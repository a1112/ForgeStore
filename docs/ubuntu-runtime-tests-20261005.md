# Isolated Ubuntu application runtime — 2026-10-05

The separate Ubuntu 26.04 ordinary-user Wayland session runs the reviewed native
package service, ForgeStore v2, Plasma, snapd with strict AppArmor confinement,
and user Flatpak. This report separates source checks from application acceptance.
Windows remains 22/1000; Linux samples never increase that count.

The Store Linux release SHA256 is
`4e53cd24444d3dd50542519914355e79c54fe24b0e76e53965d203f4519d6f35`.
Its exact source `34ab5a723d7c28b12ad76910e14e2a1440cd2763` passed Linux
fmt/check/test/Clippy locked offline, 67 tests with two existing ignored tests.
The real Qt UI compiled on Ubuntu Qt 6.10 and passed both offscreen tests; actual
ordinary-user GUI screenshots are recorded separately. Ubuntu unit packaging
commit `c99a670b3a537b415fd733b087ae5955796796a6` passed five packaging tests
on Windows and Linux and an independent review.

## Initial candidate channel and current acceptance

`catalogue/ubuntu-candidate-v2/tuf` has schema 2 and three compatibility-unknown
entries. It is local discovery metadata, not accepted market publication.
Root version 1 SHA256 is
`80407a2440177841da7fbd3e02a4757cc071ec49100d5d0762da58034be2831a`;
role version 2 expires `2026-11-05T00:00:00Z`. Its private signing key stays
outside the repositories. Windows trust is unchanged. The first candidate's
mixed-case internal AppID failed schema validation; the second corrected only
the internal ID while retaining the real case-sensitive Flatpak reference.
Real Linux daemon checks accepted v2 and rejected target/signature tampering.

| Candidate | Fixed upstream identity | Acceptance |
| --- | --- | --- |
| Kate | Ubuntu `kate`, `4:25.12.3-0ubuntu1`, amd64; artifact 2749472 bytes, SHA256 `a0f42af31e4bc1fa2d449029b3eb8229f822c236f90dc54950b130a575ca03aa` | GUI accepted; published in accepted-v4 |
| GNOME Calculator | Strict Snap `gnome-calculator`, ID `J8OcDPQ0JM8dbvk29HRqpWVI9kBw0atG`, revision 972 | GUI accepted; published in accepted-v4 |
| GNOME Text Editor | `app/org.gnome.TextEditor/x86_64/stable`, flathub, commit `76ecd1d6e9ecfa26f6939c54eec4e9db3491fce949c8808bdee9fb55758a1a91` | Two runtime-download timeouts; not installed or GUI accepted |

## Observed failures and recovery

Kate's first GUI job failed guest HTTPS decoding. The verified official artifact
was subsequently seeded into the ordinary user's digest cache with private
permissions; this is not a successful guest download. A normal administrator
Polkit prompt accepted authorization, then the native transaction failed.
Both original jobs remain durable. After bounded root-private diagnostics and
graceful-stop draining were reviewed and deployed, a third new GUI job identified
APT's `_apt` UID 42 transition failing with EPERM and HTTPS method exit 112.
The exact diagnostic is `evidence/ubuntu-20261005/kate-native-sandbox-diagnostic.json`.

The minimal reviewed systemd correction retains ambient CAP_SETUID while leaving
no-new-privileges, other restrictions, root policy and Polkit intact. Package-free
RED/GREEN tests proved temporary credential switching and zero capabilities after
permanent `_apt` downgrade. The fourth GUI job `065e62f9-7830-44cd-a076-a636bab54167` completed with APT exit 0, independently observed `_apt` HTTPS, the exact installed version and an empty dpkg audit. All three earlier failures remain recorded.

Flatpak GUI job `e4992255-9d17-4d03-8a26-29034f58c0b7` downloaded runtime data
continuously, then reached the explicit 1800-second deadline. The ordinary-user
child stopped; actual app list was empty and the reviewed commit absent. The
failure and 177134629-byte repository cache remain. See
`evidence/ubuntu-20261005/flatpak-download-timeout.jpg`.

Read-only network comparisons found variable official HTTPS throughput on both
outer builder and guest. They did not isolate a guest MTU or TLS fault. No source,
certificate verification, GPG verification, firewall, or trust check was disabled.
Snap GUI job `cd5de4d2-4ea1-42de-bee4-f9b25b3341af` passed normal administrator Polkit authorization and completed the exact strict revision 972, version 48.1. Its GUI sample is accepted and subsequently published in accepted-v4. No active native child was cancelled or erased to release the sequential queue.

`docs/ubuntu-progress.json` records candidate, installed, GUI-accepted and accepted
publication separately. Kate passed saved-file/reopen, uninstall/reinstall and service/session persistence. Snap passed calculation function, GTK4 native Wayland with enforced AppArmor, and nine basic window actions. Both are published in accepted-v4. Flatpak function and further lifecycle coverage remain separate runtime gates.
The Ubuntu profile also remains `deploymentReady:false`: package inventory is not
an independently reviewed dependency/license closure.


## Kate GUI and lifecycle acceptance

The ordinary UID 1000 used the real Kate 25.12.3 Wayland application to create and
save `Documents/forge-kate-ubuntu-20261005.txt`: 95 bytes, SHA256
`b1581df97bf8af2a33906306020d058b8d9470db0dcfbed2ad3b8cde6377143d`.
After closing all Kate processes, a new menu launch reopened the file through the
GUI file dialog. About dialogs confirmed version and LGPL v2 licensing.
Maximize, restore, move, resize, minimize, task restore, fullscreen, fullscreen
restore and close have separate actual screenshots. Pointer titlebar drag failed
and activated Undo; Redo/save restored the unchanged file. Successful move and
resize used KWin's supported keyboard modes, rather than claiming that drag passed.

Market uninstall job `75282aa5-4b6c-43f1-8164-80d5a6a422b7` removed Kate and
its installed menu entry while preserving the test document; fixed-version
reinstall `9d5e11a2-5ca8-40b0-984e-b84d990af9f1` succeeded. Controlled native
and Store service restarts preserved five complete native journals byte for byte
and all seven complete Store rows. Normal GUI logout/login changed SDDM session
21 to 181, with a new Wayland session leader and a new Kate process 181296.
The file reopened visibly, its bytes remained exact, all seven rows remained
identical, and the Never dim/off settings persisted. This is session and service
restart acceptance; a cold VM reboot remains separate.

The original raw TUF cache hash-equality assertion failed and is retained. Full
signed JSON content and signatures still exactly match the immutable pinned
public source for all four roles. Both changed raw hashes are reproduced by
changing only HashMap field order; `tuf-cache-serialization-order.json` records
the before/after proof. Normalized JSON hashes are not TUF OLPC signature digests.
The trusted-root bytes remain fixed, without resetting the datastore or deleting
earlier failures. Kate is GUI-accepted; it is not counted as accepted publication
until a reviewed signed channel is loaded and observed in the real local market.


Flatpak retry `cdc80c3a-709e-4e45-9be9-510b1a621f5c` was submitted through
the real GUI and queued behind the running Snap transaction; the first failure
and runtime cache are retained. The previous single-connection native service exceeded the snapshot probe
deadline during an active root transaction, temporarily showing native backends
unavailable and an empty installed list. This retained observation did not mean
Kate was removed. Reviewed native commit `bbb857de319e46feddbb4040ee397a14ada9e475`
was deployed without restarting Store or the user session. It preserved all six
complete root journals byte for byte and all nine Store job IDs. The unit,
policy, CLI and Polkit payload hashes stayed unchanged; read-only identity
queries correctly reported both installed native applications. Flatpak retry
`cdc80c3a-709e-4e45-9be9-510b1a621f5c` was still running at 15:39 UTC and later
failed at the explicit 1800-second operation deadline. It was not manually
killed by the parent. A final bounded ordinary-user read-only check found no
Flatpak process, no installed user applications and 197183037 bytes of retained
repository cache. The fixed reviewed app commit was absent. Both timeout jobs
and all nine terminal Store rows remain recorded in
`evidence/ubuntu-20261005/flatpak-retry-timeout-readback.json`. Cache growth does
not count as installation or GUI acceptance.

The prepared `ubuntu-accepted-v3` has only Kate marked tested. It retains the
original root and increments the three online roles to version 3. The lifecycle
proof is also a signed target. An independent review verified all four RSA-PSS
signatures using the TUF OLPC payload, the role hash/length chain, 31 public
evidence attachments and the source/staged proof bytes. Real Linux test-root
preflight accepts this catalogue and rejects catalogue/targets-signature
tampering; its first invocation stopped at a stale test binary path, corrected
to the unchanged deployed binary. This preparation is not activation in the
actual market. Active and queued jobs must be preserved before activation.


## Strict Snap GUI sample

GNOME Calculator revision 972/version 48.1 passed real `6*7=42` and
`12.5*8=100` calculations. Maximize, restore, move, resize, minimize, task
restore, fullscreen and exit fullscreen have separate screenshots. Close is
independently proved by process disappearance. The original close screenshot
captured the preceding frame; `snap-close-evidence-correction.json` preserves it
and binds a later process-free readback and confirmed desktop screenshot. The
first batched move input did not move the window; its screenshot remains, and
separate keyboard input later succeeded. After closing PID 202486, a menu
launch created PID 253637 and a fresh `6*7=42` calculation passed. History was
empty on reopen; persistent calculation history is not claimed. Both process
readbacks identify GTK4, the native Wayland client and enforced Snap AppArmor.

The official fixed artifact is 2150400 bytes with SHA3-384
`4b14f277af5be9803bf166b0f3fb77563dc7b8a5b7df325ba74af3b5137c9c0a913ed90636b9f4bab82ddfbaafa4ea72`.
The first ordinary-user hash attempt raised PermissionError on the root-owned
0600 file. This harness permission failure is retained; bounded root read-only
maintenance verified the bytes without relaxing permissions. Official Snap
metadata and the actual About dialog agree on GPL version 3 or later.
`snap-calculator-gui-acceptance.json` hashes all 17 screenshots and related
public evidence; `native-journal-readonly-state.json` records actual complete
root-journal preservation, fixed identity and the nine durable Store jobs.

The menu entry refreshed and launched the fixed Snap successfully, but its
menu icon appeared generic. The exported desktop file names
`org.gnome.Calculator` as its icon; host theme resolution remains a cosmetic
follow-up. This observation does not change the measured calculator functions.

The pre-publication snapshot has three candidates, two installed and two
GUI-accepted samples, with zero accepted publications. Kate's original lifecycle
proof and its 31-file
manifest remain immutable. The prepared accepted-v3 channel also remains
unchanged and inactive. Snap uninstall/reinstall and session or cold-reboot
persistence, Flatpak GUI acceptance, and the reviewed Ubuntu dependency/license
closure remain outside this sample. Ubuntu stays `deploymentReady:false` and
Windows stays 22/1000.

## Actual accepted-market activation

The separate `ubuntu-accepted-v4` channel is now loaded by the real UID1000
service and GUI. It contains only the tested Kate and Snap calculator, with
hash-bound immutable GUI proofs. The original root bytes and inode remain;
the online roles advance from 2 to 4 and expire on 2026-11-05. Version 3 remains
a prepared, inactive historical channel. Actual preflight accepted both entries
and rejected catalogue and signature tampering; independent signature and
evidence review approved the release.

Two path-permission guards rejected activation before service stop: Ubuntu's
`~/.local/state` and the earlier private-maintenance directory were mode 0775.
Their original inodes remain after removing group write from the first (0755)
and making the latter private (0700). Subsequent activation held a SQLite
admission barrier until the Store stopped, made private full state/cache and
coherent database backups, appended the new public targets/versioned roles,
and atomically replaced the timestamp. No package job was cancelled or trust
cache downgraded.

After normal Store restart, every field in all nine job rows is unchanged,
all six native records retain their exact bytes, both fixed installed identities
remain visible, and all four cached complete metadata documents including
signatures match the selected public roles. Raw cache-byte equality is not
claimed. The GUI shows two entries and the calculator's tested state and proof
pin. `ubuntu-market-v4-publication.json` binds these observations and screenshots.
The ledger therefore records two accepted publications, with both Flatpak
timeouts still retained and Ubuntu `deploymentReady:false`.

### Restore the Windows rolling profile

After all nine Ubuntu jobs reached a terminal state, Ubuntu was normally powered off. The preserved Arch/Windows VM was started using its inspected existing wrapper and entered through normal SDDM login. `arch-rolling-restoration.json` verifies all 170 core jobs and every field in 61 Store rows, configurations/generations, both TuxPaint save generations, power settings, original root 1 and roles 31. All 26 signed source files keep exact bytes. Cached signed content and signatures match the checkpoint; raw cache equality is false and is not claimed. Windows stays at 22/1000. The obsolete-helper preflight failure and pre-login missing socket are retained; neither changed package state or trust. The Ubuntu disk and failed Flatpak cache remain available for later work.
