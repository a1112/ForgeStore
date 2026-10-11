# CherryTree 1.7.2.1：字体后端适配与有限 XML 笔记验收

2026-10-11，CherryTree 完成正式原代次和独立市场代次的小型两节点 XML 笔记打开、编辑、另存、新进程重开，以及预缓存同版本更新、精确回滚和 Core/Store 正常重启。累计 **32/1000** 款独立 Windows 应用，12 项未解决候选独立保留。安装包版本为 1.7.2.1，GUI 显示 1.7.2。验收仅覆盖本报告的有限流程。

## 固定来源与安装

官方 [v1.7.2 发布页](https://github.com/giuspen/cherrytree/releases/tag/v1.7.2) 的 `cherrytree_1.7.2.1_win64_setup.exe` 为 96,911,680 字节，SHA-256 `81048bf7b326516dace5a7078794d7bbe05ec44d5d7be9492f304b8347a4f898`；实际下载与 GitHub 官方资产摘要一致。固定源码 `cherrytree_1.7.2.tar.xz` 为 4,904,756 字节，SHA-256 `848c0478f8aa4ec2e932ed00ee6c6618f21b197bbd7fea1bafdd8456468d7d45`，固定提交 [76bbccce9443aa68b45b369b70998df872cac7d0](https://github.com/giuspen/cherrytree/tree/76bbccce9443aa68b45b369b70998df872cac7d0)。主程序源码声明 GPL-3.0-or-later。安装许可 731 字节与源码 716 字节经 15 处 CRLF 转为 LF 后相同，原字节不同；完整 GTK/Pango/Cairo/图标等捆绑许可闭包、Authenticode/OpenPGP 和可重复构建未核验。

实际固定 `scripts/cherrytree.iss` 用于核对 Inno 参数：`/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /TASKS= /DIR=C:\CherryTree`。最后一个参数含一个实际反斜杠，18 个 UTF-8 字节，hex `2f4449523d433a5c43686572727954726565`。JSON 显示转义不代表实际两个反斜杠。`/TASKS=` 关闭可选桌面图标；隔离 prefix 内的 `.ctb/.ctx/.ctd/.ctz` 文件关联仍按安装器注册。没有声称取消全部文件关联。

初次安装 `job-1791699558291-1` 成功，代次 `gen-job-1791699558291-1` 就绪。原定义摘要 `ab6a6b6d6bd131f0ac060785b0527d48b842674dbf8efac997df4d093a947f38`。安装主程序 `CherryTree/ucrt64/bin/cherrytree.exe` 为 8,585,000 字节，SHA-256 `a4e3a57a7c2fbcd96542866b17cc08f0b634fbd463c760e27fb92b5c10d131e6`。启动器未设置工作目录，使用既有默认行为。[固定来源](evidence/2026-10-11-cherrytree-source.json) 保留原阶段和原字节。

## 显示问题与应用级修复

默认后端的两个正式启动 `job-1791699776366-2`、`job-1791700274808-3` 均正常退出 0，但 GTK 菜单、树和正文的文字被裁切。原生 Windows 保存对话框可读。初次程序创建并保存了诊断笔记，随后新进程重开仍异常；这些只作为失败与诊断，不能因退出 0 计入 GUI 验收。截图 01–08 原样保留。

单变量诊断任务 `job-1791700325789-4` 仅在该次请求增加 `PANGOCAIRO_BACKEND=fc`，菜单和两节点正文实际恢复可读。[GNOME 固定 Pango 源码](https://raw.githubusercontent.com/GNOME/pango/1.58.2/pango/pangocairo-fontmap.c) 支持该环境变量选择 fontconfig 后端，Windows 默认选择 win32；捆绑 DLL 核对也显示相应后端。本轮没有确定 Wine/GDI/Pango/Cairo 内具体失效函数，不声称修复底层通用缺陷。

正式修复使用现有原生 launcher 的 `environment={"PANGOCAIRO_BACKEND":"fc"}`，重新安装 `job-1791700398780-5`，形成正式原代次 `gen-job-1791700398780-5`。新冻结定义摘要 `3b3cbb7cfec767ac3fe32c783528fd196ab066c89ff67300065c4543f6ee0521`。正式 GUI 请求只提供既有 DISPLAY/XAUTHORITY，fc 来自冻结启动器。程序、DLL、字体及运行时摘要不变；没有修改 Core 代码、全局上下文或安装器环境。初始失败代次与全部任务字段保留。

## 正式 GUI 与独立市场代次

原代次从中文名 `C:\ForgeQA\input\中文笔记.ctd` 打开 694 字节的两节点 XML 输入，根节点普通替换 BEFORE→AFTER，子节点进行 ASCII 正文编辑，再 GUI Save As 到 `C:\ForgeQA\output\formal-edited.ctd`。输出 711 字节，SHA-256 `d24ef00be53cdeea5a7efc294045010f33849bba938f0255c11b1f441ef0330a`。原输入保持原字节。新正式进程显式重开该输出，实际核对根、子节点层级和正文，再正常退出。[原 GUI 证明](evidence/2026-10-11-cherrytree-gui.json) 和 [签名收据](evidence/windows-cherrytree-20261011.acceptance.json) 在独立市场待执行阶段冻结，不追改完成标签。

原子节点初次 Ctrl+End 追加与原行合并，随后换行位置也错误；截图 14、15 保留。最终通过 GUI Select All、Backspace 和完整 ASCII 正文重输修正，实际保存、XML 字节核对、新进程和回滚重开均为三行正确正文。验收表述为有限 ASCII 正文编辑，不声称该快捷键追加流程通过。

Store IPC 预缓存同版本更新 `dd830034-b0c9-443a-bd6e-b8e035ff8305` 成功，Core 安装 `job-1791701246780-8` 形成独立市场代次 `gen-job-1791701246780-8`。定义、程序和运行时摘要与正式原代次一致，GUI 前输出目录为空。市场代次打开另一中文名 `市场笔记.ctd`，初始输入 694 字节，根普通替换，子正文编辑，GUI Save As 到 ASCII 输出路径 `market-edited.ctd`。实际输出 709 字节，SHA-256 `1e94e00428f21e8dbefc7fcb1b92eaee08d2876a00f0d03c9c1bc5b259e4dce6`，新正式进程显式重开并核对两节点。

市场输入在编辑期间被应用自动保存为 693 字节，SHA-256 `c1dffa219231a46c8d6f6f92cc9a77da344ae9222b76f433265d28ac9642beac`。该输入根变为 AFTER，子节点仍为 VALUE=40，没有追加内容；应用产生的 `.市场笔记.ctd~` 保留初始 694 字节和原 SHA-256 `062abf2c29ad2a690f820921a00548cb94add2cbe7ceb3454921bf31f4f180e1`。**不声称市场输入未变**，没有重置输入或重写历史。保存、重开和层级正文验证通过与该输入变化分别记录。[原保存阶段历史](evidence/2026-10-11-cherrytree-market-save-history.json)、[后续市场 GUI](evidence/2026-10-11-cherrytree-market-gui.json)、[实际市场输出](evidence/2026-10-11-cherrytree/market-edited.ctd)、[自动保存输入](evidence/2026-10-11-cherrytree/market-input-autosaved.ctd) 与 [原输入备份](evidence/2026-10-11-cherrytree/market-input-original-backup.ctd) 保留实际字节。

独立审查还发现，私有后续助手的 `initialInputPreservedInAppBackup.file` 从初始夹具元数据复制，仍写作 `input/市场笔记.ctd`；实际备份读取和固定摘要均使用 `input/.市场笔记.ctd~`。另加 [备份路径更正](evidence/2026-10-11-cherrytree-backup-path-correction.json)，在市场、失败、恢复和发布记录明确实际路径，原私有助手收据不改。694 字节的初始备份与 693 字节的当前输入是不同文件。

## 签名市场、回滚与重启

`candidate-v33` 保留 root 1，在线 targets/snapshot/timestamp 角色 42。34 条目录项包含 32 款独立 Windows 应用和 2 个内部夹具；66 个实际签名目标包含新安装包及原正式 GUI 收据，除目录本体增加 CherryTree 外旧目标字节保持，38 个公共目录文件核对通过。tuftool 0.17 使用明确的既有 root 正常验证并下载目录、安装包、收据，没有关闭签名或有效期检查。旧 root SHA-256 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e` 保持，全部历史签名目录及 MPC-HC 原证明/更正证明保留。[激活证明](evidence/2026-10-11-cherrytree-activation.json) 保留激活时的待市场验收标签，后续完成另记。

回滚前完整 **245 Core/80 Store** 检查点保存并读回后，Store IPC 精确回滚 `6c185cff-320b-43d1-a6c9-537524ffb37f`，恢复正式原代次 `gen-job-1791700398780-5`。回滚后 **245/81** 旧全字段与全部代次记录保留。新正式 GUI `job-1791701592397-11` 显式重开原输出，再核对根、子节点并正常退出。原、市场、回滚共五次正式 GUI 均正常 Quit 退出 0，并有 wineserver 停止事件。更新与回滚使用 Store IPC，不声称 GUI 按钮功能验收。

Core、Store 顺序正常重启，重启前后均为 **246 条 Core 任务/81 条完整 Store SQLite 行**，完整字段、代次记录、正式配置、信任文档、防护和受测固定数据精确核对通过；信任时间不倒退。检查涵盖既有固定验收文件、本轮原/市场输入输出、原市场输入备份、程序、许可、配置及 Wine 注册表摘要，不声称遍历全部应用文件系统。

私有完整检查点 `cherrytree-20261011/after-restart.json`，SHA-256 `e94e61d09e8671fe80eb5fb5a05be17e9c34ad154b11cb680c8c2c214eceeea7`，完整内容不入库。当前只读助手 `L:/project/FOS/.transfer/rolling-current-state-cherrytree-20261011.py` 必须通过既有严格 SSH 主机密钥校验的 `rolling-guest-20261010.py` 在客体执行。旧 TeXworks 235/79、Artha 228/77 等固定总数只代表历史。[恢复记录](evidence/2026-10-11-cherrytree-runtime-recovery.json)、[当前状态](evidence/2026-10-11-cherrytree-current-state.json) 与 [发布记录](evidence/2026-10-11-cherrytree-publication.json) 区分实际阶段。

## 失败、检查缺口和边界

市场保存助手显示 `inputUnchanged=false`，但缺少断言，仍提交了新进程重开 `job-1791701496984-10`。识别后停止后续变更，正常退出已经运行的重开进程，只读核验输入、精确原备份、完整任务和实际 XML；另存更正证明并固定实际新字节，然后先保存读回完整检查点再回滚。原 false 收据保留。**不声称此前所有依赖门槛均生效**；后续变更以更正后的实际数据核验为前提。这一缺口分别写入失败、市场、恢复、发布与台账。

初次市场启动准备读取 registry 出现 I/O ENOENT，立即停止依赖操作；只读检查确认服务 active、安装任务成功及完整历史，之后受控重试通过。具体原因未确定，没有重置 registry、额外重启服务或声称通用竞态已修复。GUI Save As 的错误 JS 转义路径未提交；随后正斜杠路径被原生对话框拒绝，正常确认后用实际反斜杠输入并保存，截图 21、22 保留。来源 API 403、字段错误、只读路径发现和签名助手准备时的过早读取另有 [失败记录](evidence/2026-10-11-cherrytree-failures.json)，没有补造检查点。

SQLite/加密笔记、附件、图像、代码框、表格、导入导出、链接、广泛格式/搜索、大型笔记、高级功能及直接中文键盘输入未验收。客体联网下载、新机器注册、配置迁移、跨版本升级及完整捆绑许可闭包未核验。测试助手只生成输入、读取实际输出，不代写 GUI 输出。

Core 保持 `compatforge-cli-launcher-cwd-v11-r1`，SHA-256 `4dcf174ea80934e2d87fa47fbe17fee3fb308e00d7ff820c2e8c4aa06f4f8818`；Store 保持 managed-msi-v2，旧 Core v10 保留，Provider v2 未部署或 GUI 验收。认证、锁屏配置和不自动息屏设置保持。Ubuntu 独立维持 Kate、Snap 计算器两项已验收，Flatpak Text Editor 两次下载超时、未安装、缓存保留；本轮未启动第二个虚拟机。签名私钥、密码、凭据与完整私有检查点不入库，每半小时当前聊天续跑计划保留。
