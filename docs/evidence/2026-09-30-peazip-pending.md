# PeaZip 11.3.0 — installed, functional roundtrip verified, GUI review pending

The fixed official Windows x64 EXE is 11,391,512 bytes with SHA-256 `cb52763da39ef44f6b8ce3619eb76817aa8b2e4c3518c406dc5cc2857bcaeb76`. Downloaded bytes agree with both the official release asset digest and the official SHA256.txt line. The vendor identifies LGPLv3; component notices still apply. Source and hash evidence: `2026-09-30-peazip-source.json` and `2026-09-30-peazip-official-hash.json`.

Managed install job `job-1790755724792-27` succeeded in generation `gen-job-1790755724792-27` using the official Inno installer. Launch job `job-1790756115185-28` opened the real Windows file manager and later exited normally; its final status is succeeded. This exit does not by itself grant functional acceptance.

## Real GUI workflow and file evidence

Two input fixtures were created before GUI actions: a 740-byte ASCII file and a 53-byte UTF-8 file named `中文文件.txt`. Through the application's Add screen, the input directory was compressed to `rolling-peazip-input.zip`. Through its Extract screen, the ZIP was extracted into `Documents/roundtrip`. The 648-byte ZIP has SHA-256 `854154463b68d23b7aef6ca98a73a871e6e9e7a6fc65660dbabbbb7b381a24ae`. Archive CRC checks passed. Both ZIP members and both GUI-extracted files match their originals byte for byte, including the Chinese filename. See `2026-09-30-peazip-files.json`.

An initial verification attempt ran before the extraction dialog had been submitted and returned FileNotFoundError. This was a premature harness check: Return in the output field selected its text rather than submitting. After clicking the actual confirmation button, extraction completed and the subsequent byte comparison passed. This is not recorded as an application extraction failure.

## Remaining GUI defect and investigation

The confirmation and cancellation buttons on Add, Extract and Settings render as solid black rectangles, obscuring their labels. Other file-manager text, icons and controls render normally. Task invocation works, but this usability defect prevents full GUI acceptance and publication.

Inspection of the exact 11.3.0 vendor source identifies these controls as Lazarus `TBitBtn`: `ButtonArchiveOK`, `ButtonArchiveCancel`, `ButtonExtOk`, `ButtonExtCancel`, `ButtonOptOK` and `ButtonOptCancel`. Settings resource definitions contain ordinary `ok` and `cancel` captions. This narrows investigation to button rendering/configuration; it does not establish a root cause or verified fix. Source retrieval from raw.githubusercontent.com initially failed with ConnectionResetError; the official GitHub contents/blob API succeeded without weakening TLS validation. The `peach.pas` bytes have SHA-256 `b906cee33383a539da46414b40deeb4b4db1cb8781e9925480e0849199f101c8`.

Theme settings were inspected without changing them. The VM later locked. noVNC was not in view-only mode, but ordinary keys, a drag gesture and its explicit Escape control did not expose the login form. Reloading the same page reconnected and retained the lock screen. SSH remained available and the managed PeaZip job was still running. The user subsequently confirmed that they had unlocked the desktop, and the Settings screen with black button captions was observed again. No authentication bypass, VM restart or security-setting change was performed.

## Classic-theme trial

Lazarus 4.0's Windows `TBitBtn` renderer uses buffered composited theme text. Wine 11.14's DrawThemeTextEx implementation reports flags other than text-color and font-property as unsupported. Combined with the affected control type and unaffected ordinary labels, this suggests an incompatibility in themed caption rendering; it is an inference from the source comparison, not a Wine patch diagnosis.

First, launch job `job-1790758244217-29` requested `WINEDLLOVERRIDES=uxtheme=`. The actual process environment contained the service preset `mscoree,mshtml=` and uxtheme remained mapped. Buttons stayed black. This did not exercise the intended DLL-disable hypothesis. The application was then closed normally and that job succeeded. See `2026-09-30-peazip-theme-launch.json`, `2026-09-30-peazip-theme-status.json` and `2026-09-30-peazip-theme-trial.jpg`.

With the job terminal, a private backup of the PeaZip prefix registry was created outside the repository. Exactly one appearance value, ThemeManager/ThemeActive, was changed from `1` to `0` in that prefix. No DLL, runtime binary, application executable or security setting was changed. Managed launch job `job-1790758534698-30` then displayed legible **OK / Cancel** captions in Add. See `2026-09-30-peazip-theme-config-trial.json`, `2026-09-30-peazip-theme-config-launch.json`, and the before/after screenshots. uxtheme remains loaded; this trial avoids active visual theming, rather than disabling the DLL.

A post-fix archive path `rolling-peazip-input-fixed.zip` was entered through the GUI. The initial click did not submit the dialog: a backend file check found no output, and returning focus showed the same populated Add screen. Investigation found screenshot/click scaling differences after the browser view size changed. This was an incomplete harness action, not an observed compressor failure. Later the display became inactive; the noVNC Tab control exposed the login form, and normal authentication restored the desktop. Credentials were not saved in repository files.

After restoring desktop input, clicking Add's actual OK button created the new 648-byte ZIP. Its SHA-256 equals the original archive, CRC checks pass, and both members match their original bytes. The first post-fix Extract attempt was stopped by PeaZip's command-concatenation guard. Preserve `2026-09-30-peazip-fixed-extract-warning.jpg` and `2026-09-30-peazip-fixed-files-before-retry.json`: the archive existed but the expected extraction directory did not. The guard was not disabled. Reopening Extract and appending `roundtripfixed` to the application's default Documents output directory then succeeded. The GUI displayed both extracted files, including `中文文件.txt`; both extracted files match their originals byte for byte. See `2026-09-30-peazip-fixed-roundtrip.jpg` and `2026-09-30-peazip-fixed-files.json`. The cause of the rejected command/path remains unresolved; a successful retry does not establish that a hyphen alone caused it.

The classic-theme launch exited normally, and a fresh service query reports succeeded with no remaining PeaZip process; see `2026-09-30-peazip-theme-config-terminal.json`. The current generation has the trial appearance setting. Automatic application in fresh installation/update generations has **not** been implemented or validated. A closed application-profile design is proposed at CompatForge/docs/plans/2026-09-30-peazip-classic-theme-design.md. No signed acceptance or market activation was attempted.

Technical sources: [Lazarus 4.0 button renderer](https://gitlab.com/freepascal.org/lazarus/lazarus/-/raw/lazarus_4_0/lcl/interfaces/win32/win32wsbuttons.pp), [Wine 11.14 theme drawing](https://raw.githubusercontent.com/wine-mirror/wine/wine-11.14/dlls/uxtheme/draw.c).

## Counting and market state

PeaZip is **installed** and its specified ZIP file workflow passed. It is **not fully GUI accepted**, has no signed acceptance receipt, and is **not published**. Distinct accepted local-market applications remain **10/1000**. Candidate-v10 and its existing TUF trust root were not changed.

Sources: [official Windows download](https://peazip.github.io/peazip-64bit.html), [fixed release](https://github.com/peazip/PeaZip/releases/tag/11.3.0), [source control definitions](https://github.com/peazip/PeaZip/blob/11.3.0/peazip-sources/dev/peach.lfm), [source implementation](https://github.com/peazip/PeaZip/blob/11.3.0/peazip-sources/dev/peach.pas).


## Approved profile implementation and deployment gate

The user approved the classic profile design on 2026-09-30. The implementation now carries a closed profile in the reviewed recipe, prepared launch plan and signed delivery schema. Install submission explicitly expects either default or classic appearance and checks it before staging. Trusted registry tooling is configured separately from historical runtime bindings, so adding this policy does not rewrite existing generation runtime snapshots.

Regression tests exposed and then fixed two authorization/cleanup defects: direct serialized-plan authorization must match the trusted tool path and digest, and auxiliary process-tree cleanup errors must retain their cleanup stage so an unsafe prefix lease stays poisoned. The tool uses an absolute, digest-verified runtime executable from its own directory with builtin-only DLL loading. Linux domain/service/orchestrator/process regression completed successfully; retained output includes 107 process tests and 76 service tests (two isolated service helpers ignored). Windows Store backend/catalogue tests passed 13 and 7 tests respectively; formatting was corrected and checked. A read-only query using the real pinned Wine registry utility returned ThemeActive REG_SZ 0. These are implementation checks, not a fresh-install GUI acceptance.

Independent review confirmed that the deployed baseline synchronously prepares a process before it registers an active cancellation owner. The market likewise awaits submission before it receives the runtime job ID. Preparing cancellation/shutdown and the installer spawn boundary must be repaired and tested before this profile is deployed. No new service binary has been activated, no candidate-v11 acceptance has been signed, and the accepted count remains 10/1000. See `2026-09-30-peazip-profile-review.json` for retained checks and the remaining gates.
