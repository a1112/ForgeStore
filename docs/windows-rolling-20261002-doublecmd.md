# 2026-10-02 Windows 滚动适配：Double Commander

Double Commander 1.2.9 已完成受控安装、真实 GUI 文件操作、签名验收、本地市场更新、回滚和服务重启验证。累计 **12/1000 款独立 Windows 应用**；另有 11 个阻塞候选，保留全部失败记录。目录 candidate-v12 含 14 项，其中 2 个内部 fixture 不计数。

安装包取自 [官方固定 v1.2.9 发布](https://github.com/doublecmd/doublecmd/releases/tag/v1.2.9)，`doublecmd-1.2.9.x86_64-win64.exe` 为 10,639,174 字节，SHA-256 `3824250b280d1eaefe6422690bb6a0fc652aa8006545c05a63f9d2e1cababfd4` 与 GitHub asset digest 一致；宿主、构建机和测试机均复核。About 显示 1.2.9 gamma / GNU GPL 2，安装内 `doc/COPYING.txt` 与 [版本源码许可](https://raw.githubusercontent.com/doublecmd/doublecmd/v1.2.9/doc/COPYING.txt) 字节一致，摘要 `8177f97513213526df2cf6184d8ff986c675afb514d4e68a404010521b880643`。参考源码标签提交已固定，但未声称可重现编译或安装包提交来源已逐一验证；组件许可继续适用。目录仅引用官方安装包，不把安装包或私钥发布到仓库。

安装任务 `job-1790949239508-1` 成功，世代 `gen-job-1790949239508-1` 为 ready。配方使用现有经验证的 `wineAppearance=classic`，Inno `/DIR=C:\DoubleCommander` 与实际启动文件 `DoubleCommander/doublecmd.exe` 对应。Wine 11.14、Noto CJK 与启动文件摘要固定，无新增服务代码修改。

GUI 左右面板按[官方命令行接口](https://doublecmd.github.io/doc/en/commandline.html)分别打开受控 source/dest。通过 F5 复制 ASCII 与中文文件到空目录，F2 把 `rolling.txt` 副本改名为 `renamed.txt`，F3 查看中文文件内容。48 字节 ASCII 文件摘要为 `02bb7420c4aa158730135ee92c43a8db24b0868af6ca8a3a873d4ea3b008636a`，51 字节中文文件摘要为 `b8d4b3dddba42f9f8c9bbb8c2736c97525e29c8b17888d74c55b415d0a9066d7`。源文件未变、两个输出逐字节一致。首次启动 `job-1790949294915-2` 和重开 `job-1790949406776-3` 均正常退出 0；重开后中文内容再次可读，输出再核对未变。

签名候选使用原可信根，tuftool 0.17 从 candidate-v11 root 执行 metadata-only clone 验证成功。candidate-v12 的 timestamp/snapshot/targets 为 **21**，root 为 **1**，可信根摘要仍为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。全部 10 份签名验收目标和目录目标的实际字节、长度与目标元数据核对；未降级或清空现有 TUF 状态。

市场缓存用同一固定安装包核对后填充。更新任务 `1386fe5f-e405-4ab1-903f-af9646d99dec` 成功，产生新世代 `gen-job-1790949638292-4`；在新世代重新执行 GUI 复制、重命名和中文查看，输出与原始固定数据一致，启动任务 `job-1790949684698-5` 正常退出 0。回滚任务 `ccba7bf6-5313-43a5-96f5-1f57f29874d3` 成功，精确恢复原验收世代，GUI 重开原中文输出并正常退出（`job-1790949778999-6`）。两个世代的定义、运行时和启动文件摘要一致，均 ready。

服务重启后目录、安装清单和全部 **40 条**市场任务逐条保持一致；记录摘要 `41c1d39b7f5ae0650bbf75775e769d9dea99de1530766428ac9ebfa28679c407`。发布前的 38 条任务与上轮摘要一致；本轮只新增 update/rollback 两条。CompatForge 任务全部终结，两个服务 active，TUF 21 已持久化。这里验收的是同版本本地缓存更新及世代回滚；在线下载、新机器配方注册、跨版本升级、公共远程市场发布尚未验收。压缩、搜索、插件、FTP、网络共享和提权文件操作也未验收。

本轮还恢复了宿主重启后的测试链路：从 Hyper-V NIC MAC/邻居表定位新宿主地址并保留主机密钥验证；使用原 qcow2、20 GiB 只读基盘和固件恢复 40 GiB 测试盘。旧启动器要求覆盖盘与基盘等大，因此拒绝此前已审批的扩容；恢复包装器只接受该固定实例、基盘摘要、两层格式与精确 40 GiB 容量，并保留原私有路径、摘要、固件、通过 QMP 验证基盘只读 和完整启动参数核对。noVNC 与 SSH 使用原环回端口。第一次 user transient unit 因最后 SSH 会话结束被停止，第二次被旧容量校验拦截，均未启动新测试；最终 system transient unit 的进程仍以 fosdev 运行。它们没有设置开机启用，后续宿主重启仍需恢复。登录后原桌面与服务恢复，未更改凭据、自动登录或安全设置；密码和签名密钥未入库。

证据：[来源](evidence/2026-10-02-doublecmd-source.json)、[配方](evidence/2026-10-02-doublecmd-recipe.json)、[安装与 GUI 后台数据](evidence/2026-10-02-doublecmd-backend.json)、[签名验收](evidence/windows-doublecmd-20261002.acceptance.json)、[市场发布](evidence/2026-10-02-doublecmd-publication.json)、[虚拟机恢复及失败记录](evidence/2026-10-02-rolling-vm-recovery.json)。[GUI 复制重命名](evidence/2026-10-02-doublecmd-copy-rename.jpg)、[中文查看](evidence/2026-10-02-doublecmd-view-cjk.jpg)、[版本许可](evidence/2026-10-02-doublecmd-about.jpg)、[重开](evidence/2026-10-02-doublecmd-reopen-view.jpg)、[市场新世代](evidence/2026-10-02-doublecmd-market-view-cjk.jpg)、[市场回滚](evidence/2026-10-02-doublecmd-market-rollback-view.jpg)。服务端设计稿同步当前计数和信任版本，仍处于设计状态。
