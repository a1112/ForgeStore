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

## Candidate channel

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
| Kate | Ubuntu `kate`, `4:25.12.3-0ubuntu1`, amd64; artifact 2749472 bytes, SHA256 `a0f42af31e4bc1fa2d449029b3eb8229f822c236f90dc54950b130a575ca03aa` | GUI accepted; accepted publication pending |
| GNOME Calculator | Strict Snap `gnome-calculator`, ID `J8OcDPQ0JM8dbvk29HRqpWVI9kBw0atG`, revision 972 | Pending |
| GNOME Text Editor | `app/org.gnome.TextEditor/x86_64/stable`, flathub, commit `76ecd1d6e9ecfa26f6939c54eec4e9db3491fce949c8808bdee9fb55758a1a91` | Pending |

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
Snap GUI job `cd5de4d2-4ea1-42de-bee4-f9b25b3341af` passed normal administrator Polkit authorization and is running upstream change 2. It is not installed or GUI-accepted yet. No active native child is cancelled or erased to release the sequential queue.

`docs/ubuntu-progress.json` records candidate, installed, GUI-accepted and accepted
publication separately. Kate passed saved-file/reopen, uninstall/reinstall and service/session persistence. Snap and Flatpak function, GTK/CSD interoperability, broader window coverage and accepted publication remain required runtime gates.
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
and runtime cache are retained. During an active root transaction the current
single-connection native service can exceed the snapshot probe deadline, so the
Store temporarily reports native backends unavailable and an empty installed
list. This is an observed responsiveness limitation, not evidence that Kate was
removed. Its exact package was independently observed after the completed
reinstall/session test. This limitation remains under investigation.

The prepared `ubuntu-accepted-v3` has only Kate marked tested. It retains the
original root and increments the three online roles to version 3. The lifecycle
proof is also a signed target. An independent review verified all four RSA-PSS
signatures using the TUF OLPC payload, the role hash/length chain, 31 public
evidence attachments and the source/staged proof bytes. Real Linux test-root
preflight accepts this catalogue and rejects catalogue/targets-signature
tampering; its first invocation stopped at a stale test binary path, corrected
to the unchanged deployed binary. This preparation is not activation in the
actual market. Active and queued jobs must be preserved before activation.
