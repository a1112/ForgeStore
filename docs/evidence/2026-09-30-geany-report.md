# Geany 2.1 Windows rolling acceptance

Accepted in the isolated ForgeOS v12 local market; cumulative distinct accepted Windows applications: 8 / 1000. This is not a remote public market release.

## Source and installation

The fixed official [2.1.0 release](https://github.com/geany/geany/releases/tag/2.1.0) provides `geany-2.1_setup.exe`, 28,220,816 bytes. Downloaded SHA-256 matches the official GitHub asset digest: `50c7835b31bc736d0b5e5f894aa8ec08a95185873fb64d29a95727d2859107d8`. The source receipt records the stable URL and redirect host without temporary signed query strings. Tagged [COPYING](https://raw.githubusercontent.com/geany/geany/2.1.0/COPYING) contains GPL version 2; installed Geany and bundled component notice paths/digests are recorded in `2026-09-30-geany-final-file.json`. Existing installed notices remain intact. No public binary redistribution was performed.

Initial installation job `job-1790745047807-10` succeeded, but GUI text was severely clipped. A trial launch with `PANGOCAIRO_BACKEND=fc` restored readable menus, editor text and Chinese characters. Fontconfig backend selection is documented in the [GNOME Pango discussion](https://discourse.gnome.org/t/how-to-find-out-which-os-specific-backends-are-in-use/15980). This is a verified application configuration workaround; it does not establish a low-level Wine font defect's root cause.

The subsequent candidate definition update did not change the existing immutable installed generation. Job `job-1790745879939-13` omitted the option and reproduced the display failure. Registry readback, launch plan and selected process environment are retained in `2026-09-30-geany-generation-diagnostic.json`. Deployed service source resolves launchers from the installed generation definition. Configured reinstall `job-1790746389457-14` succeeded and created `gen-job-1790746389457-14`; regular launch `job-1790746442149-15` carried the option without a request-time font override. No generation definition or trust state was edited in place.

## Real GUI workflow

Through the visible noVNC desktop, Chinese text was pasted, a word was entered, Find/Replace changed `after` to `passed`, and the file was saved. File content was read independently from disk. Browser bulk typing/clipboard attempts did not inject the proposed full Python fixture; only actual observed Chinese and ASCII text is claimed. No compilation is claimed.

The configured generation reopened the prior GUI output copied into its Documents directory as a test fixture. It displayed `你好世界` and `passed` correctly. Further GUI editing added `ready`, then Save wrote the UTF-8 file: `你好世界\npassed\nready\n`, 26 bytes, SHA-256 `7e40375d4cb1c1f8519353c2503ce102222ef27ae4d3a8c7969c0d70c78be1e5`. Screenshots distinguish the initial font failure, trial, configured reopen and configured save. The copied fixture is not counted as a new application or as a GUI write.

## Signed local Store validation

Candidate v8 uses targets/snapshot/timestamp version 17 and preserves trusted root SHA-256 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`. It contains 10 catalogue entries, including two internal fixtures excluded from the distinct Windows count. The Geany acceptance receipt is a signed target.

Activation initially hit an overly strict checker assertion that installed IDs remain identical: publishing the new entry correctly made the already installed Geany generation visible in the Store. Post-activation snapshot retained all previous apps and jobs and added Geany. This checker failure is documented, not treated as a service failure.

Verified-cache Store update `5c6124b5-ccf0-4604-8ab5-f8887a9a1730` and rollback `9db2a0e2-5b8e-42d8-8ae7-e939023de998` both succeeded. Store restart preserved installed state and job history. The GUI-edited file digest was unchanged after rollback. Rollback reopen job `job-1790747021033-17` selected the configured generation and retained the font option.

Fresh-machine recipe registration, guest online downloading, compiler/debugger/plugin features and cross-version upgrade remain unverified. Signing key and password are not stored in source. Earlier failure evidence and pending applications remain preserved.
