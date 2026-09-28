# ForgeStore v12 isolated VM acceptance

Date: 2026-09-29 (Asia/Shanghai). This report covers the first signed,
locally provisioned catalogue of five reviewed entries. The test VM is a
disposable overlay; the pre-existing user VM and its overlay were not changed.

## Reproducible inputs

| Item | Identity |
| --- | --- |
| ForgeStore source | `ce20930a7291807326ace742fa6d126e3b2c6a23` |
| ForgeOS integration source | `f92f08f` |
| CompatForge debug base image | SHA-256 `e6d862887d58335256ab1144a548d7f2140a9d8b6733e9eaddeb3558a34b4741` |
| Store bundle | `/srv/forge-apps-build/forge-store-bundle-v12.tar.gz`, SHA-256 `9ec9c668c12882350a2f2a8bd14aa02eaf0d5fc168026d9f8ced58395a752a9f` |
| Final immutable raw image | `/srv/forge-apps-fast/images/forgeos-store-final-v12.raw`, SHA-256 `59715e4b286d40d4597f7dadca3d2866b4d836608568c06e6b88d67f5ab546ca` |
| Test instance | `/srv/forge-apps-fast/lab/apps-v12-final` (QCOW2 overlay on the above raw image) |

The bundle has 114 verified files. The independent installer receipt is
[forge-store-v12-install-receipt.json](forge-store-v12-install-receipt.json),
and the image builder receipt is
[forgeos-store-final-v12.raw.json](forgeos-store-final-v12.raw.json).
The product raw image keeps `auth_admin` for package management and XFCE as
its recovery/default session. Only the disposable overlay enables temporary
SSH access and KWin autologin for this acceptance run.

## Real VM results

| Path | Operation and observed result |
| --- | --- |
| `.forgepkg` | `org.forgeos.storefixture` installed at 1.0.0 (job `76838f2a`), advanced under signed catalogue v2 to 2.0.0 (job `01b07f4a`), then rolled back to the original digest (job `526c4821`). All jobs succeeded. |
| Flatpak | Signed local remote installed fixture 1.0.0 (job `e6c2c044`; `flatpak run` printed `ForgeStore Flatpak fixture 1.0.0`), updated it to 2.0.0 (job `006f843e`; run printed 2.0.0), uninstalled it (job `fd906a01`), then installed it again (job `cceef233`). All jobs succeeded. |
| CompatForge Windows app | Exact verified upstream 7-Zip 26.01 installer (SHA-256 `d64a0468f5b5b0b0fc5b2188450bcd655b70809d97b1c4535f2884635094377d`) was placed in the test guest cache; ForgeStore independently verified and installed it (job `461e33ad`). The installed list reported `7zip`, version 26.01. |
| Persistence | After removing the acceptance-only Polkit rule and a normal VM shutdown/start, the service reported all three backends available, the three installed entries remained, and the above job records remained successful. |
| Visible UI | The Chinese Qt/QML window rendered without clipped card actions. The market and installed views are captured in [final desktop](forge-store-v12-final.png) and [installed applications](forge-store-v12-installed.png). The live isolated view is forwarded to host loopback port 6094. |

The v11 diagnostic run showed the previous mapped-root fixture rejection was
resolved. Independent review then found and corrected a narrower mount policy
gap: v12 requires both the effective `/usr` mount and the effective fixture
mount to be read-only before accepting overflow UID 65534. A test for
`/usr rw` with a read-only fixture submount failed before the fix and passed
after it. The normal UID 0 path, fixed fixture path, safe ancestor ownership,
single-UID mapping, `NoNewPrivs=1`, zero effective capabilities, signed size,
and signed SHA-256 remain checked. Windows and Linux Rust suites, the
offline bundle installation, and the unchanged ForgeOS integration checks
passed.

The temporary `00-forge-store-v12-acceptance.rules` file was verified absent
after the last reboot. The test overlay retains loopback-only SSH and KWin
autologin so the desktop can be viewed; neither is present in the immutable
product image.

## Known limits

- The current signed catalogue contains three reviewed Windows applications
  and two local Linux/package fixtures. It is not a public remote app store.
- The Flatpak fixture omits version metadata, so the installed view displays
  `未知` despite successful execution of version 2.0.0.
- After rollback to a package version no longer in the current signed
  catalogue, the installed view displays the verified digest prefix instead
  of guessing a version label.
- During this VM test, KDE PowerDevil intercepted QEMU's ACPI power-key
  request. Stopping PowerDevil in the disposable guest before requesting ACPI
  shutdown allowed a normal, exit-code-zero shutdown. This host-control path
  is separate from the app market and was not changed in the product image.
