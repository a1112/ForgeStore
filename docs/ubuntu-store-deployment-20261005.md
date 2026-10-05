# Ubuntu Store deployment, 2026-10-05

The independent Ubuntu 26.04.1 VM received the already verified ForgeStore daemon and Qt UI. This record is the pre-installation baseline: the three catalogue entries were candidates with `compatibility.status=unknown` and `evidence=null`, with no installed records or Store jobs. It does not count GUI acceptance or publication. Application installation and GUI testing are separate subsequent work. The old Arch/Windows VM, Windows 22/1000 ledger and candidate-v22 trust state were not changed.

## Verified payloads

| Payload | Source and actual SHA-256 |
| --- | --- |
| `/usr/bin/forge-store-service` | Rust commit `34ab5a723d7c28b12ad76910e14e2a1440cd2763`; `4e53cd24444d3dd50542519914355e79c54fe24b0e76e53965d203f4519d6f35` (16,956,536 bytes) |
| `/usr/bin/forge-store-ui` | Built from `33f6b593c3ca1535948fff57f3a416bd28903cec`; all 12 UI source files matched commit `34ab5a7`; `f1892e3b0add68b49bb4a66fc8591d640b9beda0c78e5d1cbbb71d9c1de2ff88` (89,360 bytes) |
| Ubuntu public TUF root | root version 1, `80407a2440177841da7fbd3e02a4757cc071ec49100d5d0762da58034be2831a` |
| Ubuntu catalogue target | `1dc2541178d5fa587a72ce0fa87f891e74dfcf204b215f18345e3acba3036424` (schema 2, 2,060 bytes) |

Ubuntu Qt packages actually installed were Core/Gui `6.10.2+dfsg-7` and Qml/Quick/QuickControls2 `6.10.2+dfsg-3`. This differs from the prior Arch package versions listed in the unchanged UI manifest; the Ubuntu deployment receipt records the observed versions separately.

The daemon actually loaded the independent `ubuntu-candidate-v2` TUF profile before system installation. Its three entries were `org.kde.kate` (UbuntuDeb), `org.gnome.calculator` (strict Snap), and `org.gnome.texteditor` (Flatpak). The Flatpak upstream reference retains `org.gnome.TextEditor` case. Independent timestamp/snapshot/targets roles were version 2, expiring 2026-11-05; root and existing production trust were retained.

The first Ubuntu candidate-v1 failed the actual daemon parser because its AppEntry ID used uppercase `TextEditor`; that failure was retained, and the publisher signed role version 2 instead of deleting state or rewinding an accepted version. Negative copied profiles were also rejected: a larger catalogue target hit TUF's signed size bound, altered targets metadata hit the snapshot hash bound, and an altered timestamp signature produced `SignatureThreshold { threshold: 1, valid: 0 }`. Only public JSON metadata was staged or installed. Signing material and passwords were not read or deployed.

## Ordinary-user service and API

The service runs as the desktop user `forge`, UID/GID 1000, through enabled `systemctl --user` unit `forge-store.service`. Its executable is `/usr/bin/forge-store-service`; state is `~/.local/state/forge-store`, including the durable queue and accepted TUF datastore. The socket is `/run/user/1000/forge-store/store.sock`, owned by 1000:1000 with mode 0600. A real API snapshot verified schema 2 and the three unknown candidates before any application operation. The desktop entry `/usr/share/applications/forge-store.desktop` uses `Exec=forge-store-ui`; UI reads `${XDG_RUNTIME_DIR}/forge-store/store.sock` from the normal desktop environment. No extra socket variable is needed.

The first deployed user unit kept legacy `ProtectSystem=full` and `PrivateTmp=yes`. systemd's unprivileged filesystem isolation created a user namespace whose UID map contained only `1000 -> 1000`; the external root native-service peer no longer appeared as UID 0, so the Store's strict `SO_PEERCRED` check rejected it. The failure, original unit and UID-map evidence were retained. An Ubuntu-only user drop-in set `ProtectSystem=no`, `PrivateTmp=no` and `PrivateUsers=no`, preserving `NoNewPrivileges=yes` and mode-0077 state creation. After a queue-empty restart the daemon used the initial UID namespace, and real snapshot probes reported APT, strict Snap and Flatpak available. Root-peer verification, native administrator policy and Polkit requirements were not relaxed. This reduces user-service filesystem isolation to preserve authenticated access to the independently privileged native service.

The native daemon unit is `forge-native-packaged.service`, with fixed root-owned socket `/run/forge-native-package/forge-native-package.sock`. Snap capability briefly reported unavailable while snapd was dormant; a separate native-service investigation confirmed ordinary socket activation and strict AppArmor. Store keeps real probe failures visible instead of claiming a backend is available.

## Packaging correction

`packaging/forge-store.service` remains the default Arch fixture unit, unchanged. `scripts/build_bundle.py --profile ubuntu` selects the independent `packaging/forge-store-ubuntu.service`, installed under the same service name. It explicitly stays in the initial user namespace, preserves `NoNewPrivileges=yes` and `UMask=0077`, and has no fixture-provisioning pre-command. The default `--profile arch` retains legacy configuration; unknown profiles are rejected. The options select service configuration only: TUF inputs still require independent reviewed Ubuntu metadata, and the OS remains responsible for native policy and approved remotes.

The profile tests first observed the Ubuntu bundle incorrectly contain the Arch namespace settings and an unknown profile silently pass. After the change the new bundle selection tests and existing verified-install tests passed. These configuration changes do not alter the deployed daemon/UI binary hashes. Full private preflight, failure, installation-payload and API receipts are retained outside Git; no credential or signing key is included in this document.
