# MSI candidates — 2026-09-30

LibreOffice recipe 26.2.4 direct URL and its mirror list now return HTTP 404. The official download page links current 26.8.0 x86-64 MSI; official mirror list declares 374906880 bytes and SHA-256 4aa6c6e1895f4055104effcb556bd3362d20c6ad707c149543304f395ef9db95. License page identifies MPL-2.0 and component-specific notices. Writer/Calc/Impress workflows remain untested.

Audacity official Windows page links 4.0.0 x86_64 MSI and SHA-256 2aecc44d28a004d15ae7c23c099f232ba8c5b3e6b19ebdbaf7d6c596316dc5b3. Official GitHub release asset agrees on digest and reports 49532928 bytes. LICENSE.txt at the exact Audacity-4.0.0 tag identifies GPLv3 with component-specific licenses. No audio import/edit/export or project save/reopen acceptance yet.

Both require managed MSI execution. The deployed service currently inspects install input as PE, as demonstrated by the preserved JASP failure. The existing MSI validator is non-executing. The proposed Prepared Install integration is not implemented; no workaround raw Wine invocation, CAB extraction or portable substitution was performed. These candidates are not counted as installed, GUI accepted or published.

Sources: https://www.libreoffice.org/download/ ; https://download.documentfoundation.org/libreoffice/stable/26.8.0/win/x86_64/LibreOffice_26.8.0_Win_x86-64.msi.mirrorlist ; https://www.libreoffice.org/licenses/ ; https://www.audacityteam.org/download/windows/ ; https://github.com/audacity/audacity/releases/tag/Audacity-4.0.0 ; https://raw.githubusercontent.com/audacity/audacity/Audacity-4.0.0/LICENSE.txt . Download session 81849 has terminated. Audacity completed and its exact size, SHA-256 and compound-file magic match the pinned metadata. LibreOffice failed with ConnectionResetError at mirror.nevacloud.com; the partial file remains unverified. See 2026-09-30-msi-source-artifacts.json and 2026-09-30-msi-download-progress.json. No install or GUI acceptance is inferred from either download result.

After that session terminated, a bounded retry used the BFSU HTTPS mirror, first verifying that the exact mirror URL was listed by the official source. It returned HTTP 403 before any package bytes were written. This second failure is preserved in `2026-09-30-libreoffice-mirror-retry.json`; it did not replace the original partial file or produce a verified installer.
# 2026-10-03 后续状态

托管 MSI 已实现，并通过 Qalculate! 5.12.0 的安装、真实 GUI 持久文件、签名本地市场缓存更新、回滚及重启验收。详见 [本轮报告](../windows-rolling-20261003-qalculate.md)。上文是 2026-09-30 的历史状态，失败记录保留；其中“未实现”不代表当前状态。Audacity 待逐应用复测，LibreOffice 下载阻塞未解除。Core MSI 上限现为 2 GiB；ForgeStore 上限仍为 1 GiB，JASP 包仍超过市场上限。
