# TeXworks 0.6.11：有限文本编辑与本地市场验收

2026-10-11，TeXworks 完成正式原代次和独立市场代次的小型文本编辑、保存、新进程重开，以及预缓存同版本更新、精确回滚和 Core/Store 正常重启。累计 **31/1000** 款独立 Windows 应用，未解决候选仍为 12 项。本轮验收的是有限文本编辑流程，**TeX 编译、排版和 PDF 预览未验收**。没有修改 Core、Store 或全局运行时上下文。

## 固定来源、安装与许可

官方 [0.6.11 发布页](https://github.com/TeXworks/texworks/releases/tag/release-0.6.11) 的安装包为 `TeXworks-win10-setup-0.6.11-202602100835-git_7951fd8.exe`，29,600,489 字节，SHA-256 `24b7bccbf04ad136618216f0427f39fa6a60ef6163879db00fd8c17ab6b4b2e8`。实际下载与官方 GitHub 资产摘要一致。

固定源码提交为 [7951fd8b942b083d937f8d6019826eca94461dc5](https://github.com/TeXworks/texworks/tree/7951fd8b942b083d937f8d6019826eca94461dc5)，归档 11,510,362 字节，SHA-256 `f18553724289a9db38c98bca7340825cc4da8dfb30e8443132053fcd45755067`。固定主程序源码声明 GPL-2.0-or-later；安装 COPYING 为 18,326 字节，源码 COPYING 为 17,987 字节，339 处 CRLF 转为 LF 后相同，原字节不同。Qt、Poppler、Hunspell、SyncTeX、Lua、图标及完整捆绑组件许可闭包未核验；Authenticode 和可重复构建未核验。市场许可及摘要明确注明上述范围。[固定来源收据](evidence/2026-10-11-texworks-source.json) 保持原字节和原阶段标签。

安装参数为 `/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /TASKS= /DIR=C:\TeXworks`，实际路径含一个反斜杠。固定 Inno 源码用于核对任务参数，没有进一步核验全部安装后文件关联注册项。启动器未指定工作目录，使用既有默认行为。正式安装 `job-1791697342464-1` 正常退出 0，代次 `gen-job-1791697342464-1` 就绪。冻结定义摘要为 `e8f58560b728fea611d781570560a47c3aea1f25af9de948b80b787475390d22`；安装 `TeXworks.exe` 为 92,099,024 字节，SHA-256 `08ffd34b020ed2bee490c703544659393fcc54007c09703dd81a67b209c95a3c`。

## 正式 GUI 与独立市场代次

原代次打开中文名 `C:\ForgeQA\input\原稿.tex`，输入为 147 字节 UTF-8/LF，包含既有中文内容和 `STATUS=BEFORE`。实际 GUI 普通替换为 `STATUS=AFTER`，追加 ASCII 注释，再使用 GUI Save As 保存到 ASCII 路径 `C:\ForgeQA\output\original-edited.tex`。输出为 172 字节，SHA-256 `cc15a8dbaaef36d65734b9bf5c4ce8e365e4b822cb1a4c16659f1b2b90d6039b`。正常退出后，新正式进程显式重新打开该文件并核对内容，再正常退出。[原 GUI 收据](evidence/2026-10-11-texworks-gui.json)、[实际输出](evidence/2026-10-11-texworks/original-edited.tex) 和 [原签名收据](evidence/windows-texworks-20261011.acceptance.json) 保留独立市场待执行的历史标签，不追改为完成。

Store IPC 使用已核验的本地缓存提交同版本更新 `b7b88332-1b9f-4941-afbb-dd7b264ad52b`，Core 安装 `job-1791697756399-4` 成功，形成独立市场代次 `gen-job-1791697756399-4`。定义、程序和运行时摘要与原代次一致，测试前该代次没有验收输出。

市场代次打开另一个中文名 `市场稿.tex`，158 字节 UTF-8/LF。实际执行相同普通替换、另一条 ASCII 注释追加和 GUI Save As；输出 `market-edited.tex` 为 181 字节，SHA-256 `5f7985da41e8c3102cbdef3b24c9b0c2ef1bc6681fd18d3494d1ea6d3ea00d67`。独立新进程显式重开并核对；About 实际显示 0.6.11、修订 7951fd8 和 Qt 6.10.1。两个原始输入保持原字节。输出由 GUI 写入，检查助手只读核验，没有代写验收结果。[市场 GUI 与截图](evidence/2026-10-11-texworks-market-gui.json)、[实际市场输出](evidence/2026-10-11-texworks/market-edited.tex) 记录完整流程。

## 签名市场、回滚与重启

`candidate-v32` 保留原 root 1，在线 targets/snapshot/timestamp 为 41，33 条目录项包含 31 款独立 Windows 应用和 2 个内部夹具。64 个实际签名目标包含旧 62 个目标及新安装包/收据，除目录本体增加 TeXworks 外旧目标字节保持；37 个公共目录文件已核对。tuftool 0.17 使用明确的既有 root 正常验证并下载目录、安装包和收据，没有关闭签名或有效期检查。旧 root SHA-256 为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`，原字节保持；全部旧签名目录及 MPC-HC 原证明和更正证明保留。实际 Store 激活确认版本、有限范围摘要、许可和证据引用。[激活收据](evidence/2026-10-11-texworks-activation.json) 保留激活时的阶段，后续完成另记。

回滚前完整 **234 Core/78 Store** 检查点先保存并读回，再提交 Store IPC 精确回滚 `d5f94d0f-43bd-4833-be96-1d76d8d46fa6`，成功恢复 `gen-job-1791697342464-1`。回滚后 **234/79** 旧完整字段及全部代次记录保留。新正式 GUI 任务 `job-1791697979365-7` 显式重开原输出、核对并正常退出。原代次、市场代次和回滚后共五个正式 GUI 进程均正常 Quit 退出 0，并有 wineserver 停止事件。更新与回滚通过 Store IPC 提交，不声称 GUI 按钮功能验收。

Core 和 Store 顺序正常重启，重启前后均为 **235 Core/79 条完整 Store SQLite 行**。全部旧任务字段、完整市场行、代次记录、正式配置、信任文档、防护、受测既有固定数据及本轮输入/输出、程序、许可和 Wine 注册表固定摘要核对通过，信任时间不倒退。文件核对覆盖固定验收数据，不声称遍历全部应用文件系统。

当前私有完整检查点为 `texworks-20261011/after-restart.json`，SHA-256 `00eef406a2a64a8e2a44cc7fb22d8682bd2b0e813ff3c4293341665294ba4db2`；完整内容不入库。最新只读助手为 `L:/project/FOS/.transfer/rolling-current-state-texworks-20261011.py`，通过已有严格主机密钥校验的 `rolling-guest-20261010.py` 在客体内执行，不能直接在 Windows 宿主运行。旧固定总数助手只代表历史。[恢复收据](evidence/2026-10-11-texworks-runtime-recovery.json)、[当前核对](evidence/2026-10-11-texworks-current-state.json) 和 [发布收据](evidence/2026-10-11-texworks-publication.json) 区分各阶段的实际结果。

## 失败记录与验收边界

首次 GUI 输入 `%` 时测试按键助手实际输入了 `5`，截图 02 保留该错误。保存最终证明前，实际 GUI 删除并使用 Shift+5 修正，重新保存后按期望字节核对通过；这是测试输入问题，不归因为应用缺陷。截图 04 在新进程打开过渡时拍摄，不作为重开通过证明；截图 05 才是实际加载完成的原代次重开证明。两张旧截图均保持原字节。[失败记录](evidence/2026-10-11-texworks-failures.json) 另保留只读路径发现问题，以及最终只读助手误在宿主运行、因缺少客体库停止的记录。正确使用严格 SSH 客体调用后，完整状态核对通过；失败和正确重试之间没有依赖变更。本轮没有安装、启动或完整检查点失败；所有依赖变更均在前置核验通过后执行。

仅两个固定小型 UTF-8/LF 输入、上述普通替换、ASCII 编辑、ASCII 输出路径 Save As、显式新进程重开和本地生命周期验收。直接中文键盘输入、其他编码、大型文件、模板、脚本、拼写检查、SyncTeX、TeX 编译/排版、PDF 预览及高级功能未验收。客体联网下载、新机器注册、配置迁移与跨版本升级未验收。

Core 仍为固定 `compatforge-cli-launcher-cwd-v11-r1`，SHA-256 `4dcf174ea80934e2d87fa47fbe17fee3fb308e00d7ff820c2e8c4aa06f4f8818`；Store 仍为 managed-msi-v2，旧 Core v10 保留，Provider v2 未部署或 GUI 验收。认证和锁屏配置未修改，不自动息屏设置保持。Ubuntu 仍独立记录 Kate 与 Snap 计算器两项；Flatpak Text Editor 两次下载超时、未安装且缓存保留。本轮未启动第二个测试虚拟机，不增加 Linux 计数。签名私钥、密码、凭据和完整私有检查点不入库。现有每半小时续跑计划保留。

[独立审查](evidence/2026-10-11-texworks-independent-review.json) 的完整重跑通过 606 项断言，无仓库阻塞问题；独立核实角色 41 和 root 的 RSA-PSS-SHA256 签名、摘要链、64 个实际目标、37 个公共目录文件、135 份旧冻结文件、9 份原 TeXworks 固定文件、输出与全部 10 张截图。初次审查的两项夹具谓词失败属于审查假设错误：台账原有三个排除的 macwin 测试套件与目录两个内部夹具是不同对象，原台账三项保持不变；最终审查记录保留纠正过程，没有修改台账来迎合错误谓词。审查未重新运行客体或 GUI；实际运行状态由父代理的正式任务、完整检查点和最新客体只读核对证明。[最终核验](evidence/2026-10-11-texworks-final-verification.json) 固定各项收据摘要和该审查范围。
