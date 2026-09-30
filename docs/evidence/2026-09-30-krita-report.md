# Krita 5.3.4 Windows acceptance — 2026-09-30

Official fixed KDE installer (165,547,784 bytes) matched the official mirror-list SHA-256. NSIS /S installation succeeded in the isolated ForgeOS v12 / Wine 11.14 environment. Launcher uses Program Files/Krita (x64)/bin/krita.exe.

GUI workflow opened a generated all-white 256×256 PNG, drew two black brush strokes, saved PNG, saved native Krita KRZ, closed and reopened KRZ through CompatForge. Decoded PNG contains 15,949 black pixels and 48,716 white pixels; its digest differs from the blank input. The native ZIP CRC, application/x-krita mimetype, 5.3.4 document XML, dimensions and paint-layer payload were checked. The reopened GUI retained the X painting and Background paint layer.

A first verification harness incorrectly required mergedimage.png in the KRZ archive and raised KeyError. The actual KRZ contains maindoc.xml, preview.png and layer tiles, without mergedimage.png. The harness was corrected to validate the observed KRZ structure; it does not claim independently decoded KRZ tiles or pixel equivalence. This was a test harness assumption failure, not an application defect. No application code patch was needed.

Signed candidate-v10 has metadata version 19 and 12 entries, including 10 independent Windows applications and 2 excluded internal fixtures. Existing trusted root digest remained 345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e. Verified-cache Store update c3e1874c-55ed-4853-a952-1696e9957b16 and rollback 46098f55-c0b3-403f-a700-27a137c30efe both succeeded. Saved KRZ digest remained intact after rollback. Store restart preserved installed entries and jobs.

Acceptance is limited to this isolated local market. Guest online download, fresh-machine recipe registration, remote public distribution, tablets, animation, advanced formats and cross-version upgrade remain unverified. The application license basis uses COPYING from shared source tag v6.0.4; the official release identifies 5.3.4 as the Qt5 build of that shared codebase.

Evidence: windows-krita-20260930.acceptance.json; 2026-09-30-krita-source.json; 2026-09-30-krita-files.json; 2026-09-30-krita-publication.json; saved and reopened JPG screenshots. Passwords and signing private keys are excluded.
