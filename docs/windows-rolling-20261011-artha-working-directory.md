# Artha 1.0.3.0：原生托管工作目录修复与有限验收

2026-10-11，Artha 完成正式托管功能测试、独立市场代次、预缓存同版本更新、精确原代次回滚及 Core/Store 正常重启，累计 **30/1000**。当前未解决候选为 12 项；原 Artha 阻塞记录完整移入台账 `resolvedCandidateHistory`，原失败任务、代次和诊断证据保留。诊断副本没有计入正式验收。

## 原因与修复

原正式任务 `job-1791691304343-2` 从代次根目录启动，因固定 Windows `APP_DIR=.` 无法读取 `Artha/bin/gui.glade` 而退出 3。资源实际存在且摘要正确。[原阻塞报告](windows-rolling-20261011-artha-pending.md) 和全部早期收据保持原字节。

Core 新增可选启动器字段 `workingDirectory`，相对于所选冻结代次的 `prefix/drive_c` 解析。Artha 设置为 `Artha/bin`。目录必须存在且是普通目录，拒绝绝对路径、父级/点/空组件、Windows 前缀和反斜杠、ASCII 控制字符、越界及任一祖先的符号链接或 reparse 标记。安装完成与每次启动均检查；启动准备和授权使用同一份任务内上下文。没有更改全局运行时工作目录、复制资源、更换程序或加入 shell 包装器。

未设置字段的旧定义保持原序列化、摘要及默认启动行为；安装器不应用启动器工作目录。所选代次使用安装时冻结定义，后续注册表变更不改变旧代次。程序和运行时摘要检查仍执行。已有边界不能原子阻止同一用户在检查与启动之间恶意重命名目录；Windows junction 实际执行未测试。

修复提交为 [CompatForge 93650d1](https://github.com/a1112/CompatForge/commit/93650d1404df1c546eb689344e921c95bd041fad)，分支 `fix/managed-launcher-working-directory`，基于固定 `cfc9b6c`，已普通推送。旧 MSI 分支远端领先，故另建隔离分支，没有强制推送或部署 Provider v2。源码归档 SHA-256 为 `91f786cd32e8bd445edbfad383ee47fb682c54be02459daf779627f88e89c1da`。客体部署 `compatforge-cli-launcher-cwd-v11-r1`，6,942,792 字节，SHA-256 `4dcf174ea80934e2d87fa47fbe17fee3fb308e00d7ff820c2e8c4aa06f4f8818`；旧 Core v10 保留，Store 仍为 managed-msi-v2。

验证包括 Linux 三个相关 crate 的 266 项主测试通过、3 项忽略；日志中的一个子进程重复运行不再计数。真实目录/内部链接/外部链接/祖先链接、旧定义默认行为、冻结定义选择、安装参数隔离、程序被替换后的摘要拒绝均覆盖。Windows 服务 85 项单元与 2 项集成通过、2 项忽略；非 Linux 既有 dead-code 警告保留。ECMAScript schema 24 项、Linux clippy `-D warnings`、格式与发布构建通过。部署后实际 Core 对 13 项非法注册定义拒绝且完整状态保持。独立代码审查无阻塞问题。[修复与验证收据](evidence/2026-10-11-artha-core-working-directory.json) 保留当时 GUI 待执行的阶段标签，后续完成另记。

## 固定来源与正式 GUI

官方 [下载页](https://artha.sourceforge.net/wiki/index.php/Download) 对应固定 `artha_1.0.3.0.exe`，12,711,596 字节，SHA-256 `db633951c6d8603062d8594c1b69c59188827875f0803069f0336ce9fcab2dc2`。固定源码核实主程序 GPL-2.0-or-later，安装 COPYING 与源码逐字节相同；WordNet 数据的 Princeton 通知已阅读。完整 GTK/GLib/Cairo/字体等捆绑许可闭包、Authenticode 和可重复构建未核验。市场明确显示主程序许可及该限制。[来源](evidence/2026-10-11-artha-source.json) 的 `installed=false` 历史标签不改写。

新正式安装任务 `job-1791694338197-1` 成功，代次 `gen-job-1791694338197-1` 就绪。实际 `/DIR=C:\Artha` 参数含一个反斜杠，为 13 个 UTF-8 字节，hex `2f4449523d433a5c4172746861`。冻结定义摘要 `3af1481bccc88f3fb32e2c4aa371bfc7f40394ed2f47629931b3c3383704f8d7`，程序摘要 `a033000e7b069d823706a0acd5ae185dadd58866151d1d9d8e9406e2dc8c7f2f` 与原失败代次相同。

实际正式 GUI 流程为：

1. 托管启动 `job-1791694510665-2`，查 `happy` 的四个形容词义项及 `computer` 的两个名词义项；About 显示 1.0.3。
2. 使用查询框 Save History，经 GUI Save As 写入 `C:\ForgeQA\output\managed-history.txt`；文件和内部历史均为 17 字节 `happy\r\ncomputer\r\n`，SHA-256 `2d6ab91c7baecb5e3e34065f13e2ae764c013752d1fd02f1da37a4328bc0d68a`。
3. Quit 正常退出 0，并有 wineserver 停止事件。新正式任务 `job-1791695018150-3` 展开托盘，历史保留两词；选择 `computer` 后实际再次查词，再正常 Quit。

输出及内部历史在 GUI 前不存在，核验助手只读结果。原 GUI 收据和签名验收收据固定第二次退出时的状态，保留市场待执行标签；完成后的证据另存。[正式 GUI](evidence/2026-10-11-artha-cwd-gui.json)、[17 字节输出](evidence/2026-10-11-artha-cwd/original-history.bin) 和 [签名收据](evidence/windows-artha-20261011-cwd.acceptance.json) 可核对。

## 独立市场、回滚与重启

`candidate-v31` 使用原 root 1，在线角色 targets/snapshot/timestamp 为 40，32 条目录项中有 30 款独立 Windows 应用及 2 个内部夹具。旧 root 原字节和旧签名目录保留；新目录的 62 个目标包含原 60 个目标及 Artha 安装包/收据，除目录本体替换外旧目标固定摘要不变。tuftool 0.17 使用明确的既有 root 正常验证并下载目录、收据及安装包，没有关闭签名或有效期检查。

实际 Store 激活记录确认 Artha 的版本、许可、有限范围摘要、tested 状态和证据引用；该观察来自 Store IPC。[激活收据](evidence/2026-10-11-artha-cwd-activation.json) 保留独立市场代次待执行的原阶段。

Store IPC 更新任务 `d82c17bc-6808-4f0e-a67c-9ea2f40f52b6` 使用核验后的本地缓存，Core 安装 `job-1791695212085-4` 成功，独立代次为 `gen-job-1791695212085-4`。没有验证客体联网下载，也没有把 IPC 提交称为 GUI 按钮验收。

市场代次实际 GUI 查 `friendly`（五个义项、两个词性）和 `machine`（八个义项、两个词性），Save History 写入 ASCII 输出路径。结果为 19 字节 `friendly\r\nmachine\r\n`，SHA-256 `9f19ae753198770dd38586cb8086fa809a80cbde6cfac1da2424b54cfaba82f0`。两个独立正式进程分别完成导出与历史恢复、再次查 `machine`，均正常退出 0。[19 字节输出](evidence/2026-10-11-artha-cwd/market-history.bin) 与实际事件、15 张截图见 [市场 GUI](evidence/2026-10-11-artha-cwd-market-gui.json)。

回滚前完整 227 Core/76 Store 检查点先保存并读回；Store IPC 回滚 `7f4bf2b0-815a-45fe-a8be-b777eace160a` 成功，精确恢复修复后的原代次 `gen-job-1791694338197-1`，并非最早的诊断代次。回滚后 227/77 旧全字段及三个 Artha 代次记录保留，再通过新正式任务 `job-1791695429346-7` 验证原历史与 `computer` 查词、正常退出 0。

实际重启前后均为 **228 Core/77 Store**，全部 Core 任务和完整 Store SQLite 行字段、代次记录、正式配置摘要、信任文档、服务防护精确一致；受测既有输入/输出、数据库内容和完整性、程序/资源/许可摘要以及不自动息屏配置均核对。这里的文件数据核对针对既有固定证据及本轮固定文件，不声称逐字节遍历全部应用文件系统。最新完整私有检查点 `artha-cwd-repair-20261011/after-restart.json`，SHA-256 `12d0a7c7359e0a2e1ec25a32df9b8d8097967e40cdf5968e49627d5aa91548da`；仅摘要公开，完整内容不入库。[重启与恢复](evidence/2026-10-11-artha-cwd-runtime-recovery.json)、[当前只读核对](evidence/2026-10-11-artha-cwd-current-state.json) 和 [发布](evidence/2026-10-11-artha-cwd-publication.json) 记录实际结果。新只读助手为 `L:/project/FOS/.transfer/rolling-current-state-artha-cwd-20261011.py`，旧固定总数助手只代表历史。

## 失败、配置变化与边界

保留本轮缓存构建/红测试标签、早读安装结果、错误 CLI 输出解析、远端分支领先及临时登录锁定等六项早期问题的固定收据。随后三项失败另记：首次输出核验误读 job 嵌套字段，停止后按实际顶层 events 修复；目录暂存助手全局替换角色号误改归档摘要，哈希断言在提取/服务变更前停止，固定完整摘要后重试；重启前误要求配置与第二次退出的原字节相同，检查失败后停止，没有补造检查点。

Artha 正常退出会改变注释后一个空行。原代次首次 180 字节、第二次 179 字节、回滚后第三次又为 180 字节；市场代次首次 180、第二次 179。两个实际配置摘要分别为 `d2d2a6e739f31d6b3b5d246a397c312f15f28b18771d49242eaf5517a2b190fb` 和 `4bb101fd60c1f59b123c5f3e544c87e845276a3416557a023136742081c30290`。键值相同，原字节不同，所有原收据保留。实际第三次退出字节单独固定后，先保存并读回新的重启前检查点，再顺序正常重启两项服务并核对。全部失败与修复见 [最终失败记录](evidence/2026-10-11-artha-cwd-final-failures.json)。

仅四个英文查词、两个小型 ASCII 历史文件及上述有限生命周期验收。中文路径/输入、其他高级关系/拼写/热键/通知、完整捆绑许可闭包、客体联网、新机器注册、配置迁移与跨版本升级未验收。既有 MPC-HC 受限渲染/时间轴问题和 DB Browser 的历史回滚前检查点缺口不受本轮结论覆盖；全部签名历史保留。

测试前通过已有授权账号正常登录，认证与锁屏设置未修改。完成验收后 05:19 UTC 观察到正常空闲锁屏；不自动息屏配置保持。Ubuntu 仍为 Kate 与 Snap 计算器两项独立 Linux 验收，Flatpak Text Editor 两次下载超时、未安装且缓存保留。本轮未启动第二个 4 GiB 虚拟机，Linux 不增加 Windows 计数。现有每半小时续跑计划保留。

[最终独立审查](evidence/2026-10-11-artha-cwd-independent-review.json) 的 460 项断言通过，核实签名/摘要链、62 个实际目标、36 个公共目录文件、69 份旧冻结文件、台账、任务事件、输出及全部 15 张截图。Core 归档的全部编译代码和七个修复文件与提交逐字节一致；一份旧 `docs/evidence/linux-applications/2026-09-29-shared-desktop-service.json` 在提交中为 5,931 字节 CRLF、归档为 5,806 字节 LF，标准化后同文。归档原字节保留，不声称 501 个归档成员与 Git 树全部原字节一致。审查没有重新运行客体或 GUI；父代理实际只读核对和重启收据证明运行状态。[最终核验](evidence/2026-10-11-artha-cwd-final-verification.json) 另固定各项收据摘要及该审查边界。
