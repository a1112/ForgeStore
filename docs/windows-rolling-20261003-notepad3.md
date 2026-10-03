# Windows 滚动适配：Notepad3 7.26.602.1（2026-10-03）

本轮完成 Notepad3 **7.26.602.1** 固定来源与许可核对、托管 Inno 安装、真实中文文本编辑/另存/新进程显式重开，以及签名本地市场更新、回滚和 Core/Store 重启。累计完成 **19 / 1000 款独立 Windows 应用**。目录 v19 的 21 项中两项是内部夹具；10 项既有阻塞候选保持。安装成功或窗口出现没有单独计入完成。

[官方固定发布](https://github.com/rizonesoft/Notepad3/releases/tag/RELEASE_7.26.602.1)发布时间 2026-06-02T11:26:58Z，非预发布，GitHub 未标记 immutable。标签对应源码 commit `f0b1b6544bee6b40aeeed050990e3737fd1af712`。固定 `Notepad3_7.26.602.1_x64_Setup.exe` 为 4,882,616 字节，SHA-256 `9cf68b38bcc1aa679050b9069174ab222383ba004c37bdd47056cafc71626be9`，与官方资产 digest 一致。固定字节和源码 commit 已记录，不宣称发布不可变。

[固定 Build/Docs/License.txt](https://raw.githubusercontent.com/rizonesoft/Notepad3/f0b1b6544bee6b40aeeed050990e3737fd1af712/Build/Docs/License.txt)含 Notepad3/MiniPath BSD 三条款、Scintilla 通知和 bundled grepWin GPL v3 通知。市场 BSD-3-Clause 指主应用，各组件条款继续适用；MiniPath/grepWin 不另计完成。安装后的 Docs/License.txt 为 3,407 字节，SHA-256 `8d6739ff11b904bd12d5534d8cc5da43f30f13fe2db9b903eeb3a3f73e55cd93`；**仅将 CRLF 转为 LF** 后为 3,371 字节、SHA-256 `bdf8ba94fb60b92a5a6b6219028d45d5f3b9d38495e3bc59f61781bb69e6efff`，与固定源文件一致，原始字节不相同。其他固定许可参考已列出，未宣称全组件对应源码审计。

安装标记为 Inno Setup 6.7.0；安装器 PE stub 为 i386，实际 launcher 为 x86_64。原始托管安装 `job-1790992599487-1` 使用 `/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /DIR=C:\Notepad3 /TASKS= /LOG`，正常退出 0，代次 `gen-job-1790992599487-1` ready。`/TASKS=` 是请求参数，安装任务和注册表结果未单独审计。没有手工解包替代安装。launcher `Notepad3/Notepad3.exe` 为 5,289,272 字节，SHA-256 `92bc17ffc6300d7c73b4ca735f65e510dda7243a490cedef14385cf142d57a96`。GUI About 实际显示 Notepad3 (x64) 7.26.602.1。

原始输入 C:\ForgeQA\input\输入.txt 为 UTF-8 无 BOM、CRLF，96 字节，SHA-256 `dd66aa9435cdb568fa28ac3639b031f5f991bc306a7caf8f7ea6929b85e635be`，包括中文“中文路径与内容验证”、value=17 和 replace_target=old。output 在 GUI 前为空。`job-1790992656124-2` 在文件选择器显式打开输入，读取真实内容，Ctrl+H literal Replace All 将 old 改为 originalpassed（实际显示替换一处），通过 File > Save As/F6 保存 np3original.txt，正常退出 0。输出按预期替换逐字节核对，原始输入保持。

首次重开 `job-1790992881868-3` 正确标题/状态已出现，但截图早于编辑区重绘且进程已关闭；保留其正常退出记录，**不据此认定可见内容验收**。随后另一个新进程 `job-1790992963770-4` 用文件选择器显式打开输出，实际观察中文、value=17 和 originalpassed 后截图，正常退出 0。输入与输出字节均不变。

| GUI 输出 | 字节数 | SHA-256 |
| --- | ---: | --- |
| np3original.txt | 107 | `33e67eb5ca58ba4afad555b85ab620832e22e590756c748edcde105a6ec16476` |
| np3market.txt | 107 | `1b5265a02f37a5835247199665e970ea2bb70f09be7eaf2ad2913feebd05a1b0` |

签名目录 v19 使用既有 root 1，timestamp/snapshot/targets 均为 28，到期 2026-10-29。tuftool 0.17 从上一代可信根执行 `clone --metadata-only` 返回 0；23 个公开 JSON 字节清单、17 份验收收据及旧目标 pin 保持。原始收据 SHA-256 `24bfeeb755bcbf3940c73d54a7e1e017dd1830dd1502b123c97734cddaa68efd` 冻结，后续生命周期另记。激活前 53 条完整 Store 记录和旧已安装项均保持。

市场 verified-cache 更新 `b1a8ea8d-3403-4efc-91b1-9ece73c27143` 成功，Core 安装 `job-1790993245462-5` 得到新代次 `gen-job-1790993245462-5`。新输入独立准备，104 字节，SHA-256 `7dc8a49b3d0d78065f16bba5f1b15fe434987ddea5d66fe8583f9562efcc9289`，含“市场代次中文内容验证”、value=21 和 marketold，output 为空，未复制原输出。`job-1790993285234-6` 实际 GUI 将 marketold 改为 marketpassed（替换一处），另存 np3market.txt 并退出 0。`job-1790993451080-7` 新进程显式打开更新输出，真实中文/ASCII 可见后截图并退出 0。预期替换、UTF-8 无 BOM、CRLF 和输入不变均经字节核对；launcher 与许可 pin 保持。

市场回滚 `33c4b1b9-ac22-484e-8bc8-6232ff56fd94` 成功，现有 newest-other-ready 规则选回原代次，两个 ready 代次均保留，没有额外 Core 切换。`job-1790993540911-8` 新进程显式打开原始 np3original.txt，观察中文、value=17 和 originalpassed，截图后正常退出 0。

Core/Store 重启后，**142 个完整终态 Core 作业、55 条完整 Store 数据库记录、全部代次与选择、已安装列表、目录、配置和服务约束保持一致**。本轮前 134 个 Core 作业和 53 条市场记录逐条不变；55 条市场记录规范 JSON SHA-256 `b69c16abdc8f24347c62a61649f569b837d97ec50f6172dee9a1f5fea2a54f9e`。两套输入/输出和安装许可字节保持。运行二进制仍为 managed-MSI-v10 / Store managed-MSI-v2，Store NoNewPrivileges=yes、ProtectSystem=full、PrivateTmp=yes 保持。

信任根 SHA-256 始终为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。23 个公开目录源文件逐字节不变；缓存解析后完整文档及签名保持，不对缓存序列化字节作断言。latestKnownTime 从 `2026-10-03T02:07:23.935421003Z` 单调推进至 `2026-10-03T02:14:11.096982156Z`，没有信任重置、降级或绕过。

保留的测试台问题包括 Save As 实际快捷键 F6、首次重开截图早于重绘以及重启后首次 Core readiness probe 的暂时 ENOENT。最后一项经有界重试等到服务就绪，完整状态比较通过，没有重复重启或重置信任。八张公开截图来自实际完整 GUI 截图。现有授权登录被自行复用，没有移除密码或改变认证，密码和签名密钥不入库。

本轮无需新增应用或 Core/Store 行为修补。验收仅覆盖所列 UTF-8 中文文本文件工作流、缓存同版本更新及代次回滚。正则、多语言语法高亮、其他编码、大文件、打印、加密、bundled 工具工作流、客户机在线下载、全新机器注册、跨版本升级、远端安装包再分发、AuthentiCode、可复现构建和完整组件/安装器/任务审计均未验证。

证据入口：[固定来源与许可](evidence/2026-10-03-notepad3-source.json)、[配方](evidence/2026-10-03-notepad3-recipe.json)、[原始验收收据](evidence/windows-notepad3-20261003.acceptance.json)、[后端验证](evidence/2026-10-03-notepad3-backend.json)、[市场生命周期](evidence/2026-10-03-notepad3-publication.json)、[测试台失败记录](evidence/2026-10-03-notepad3-failures.json)、[累计台账](windows-1000-progress.json)。继续选择未完成且官方固定安装包、版本和许可可核对的独立应用。
