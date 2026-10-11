# Artha 1.0.3.0：安装完成，托管工作目录阻塞

2026-10-11，本轮累计仍为 **29/1000**。Artha 是新增独立候选，安装成功，但正式托管启动失败；两次指定正确工作目录的受控诊断不构成托管 GUI 验收或市场发布。台账保留原有 12 个阻塞候选，并新增 Artha，阻塞候选为 13 个。

## 固定来源、许可和安装

官方 [下载页](https://artha.sourceforge.net/wiki/index.php/Download) 指向 [1.0.3 发行目录](https://sourceforge.net/projects/artha/files/artha/1.0.3/)。固定 Windows 安装包 `artha_1.0.3.0.exe` 为 12,711,596 字节，SHA-256 `db633951c6d8603062d8594c1b69c59188827875f0803069f0336ce9fcab2dc2`；源码 `artha-1.0.3.tar.gz` 为 441,630 字节，SHA-256 `e383d1e20147a7e278ab1c89e5b5fb5fd7f515f5ef57c0c0aae2449eebf50239`。官方 SourceForge 页面摘要与实际 HTTPS 下载字节匹配。

安装包是 PE32/x86 Inno Setup。固定 `src/gui.c` 声明主程序 GPL-2.0-or-later，`COPYING` 为 GPL v2。安装后的 `Artha/COPYING`、`Artha/bin/gui.glade` 分别与固定源码对应成员逐字节相同。实际安装的 WordNet `data.noun` 前 29 行包含 Princeton 许可及保留通知、免责声明和不背书要求。本轮没有核验完整 GTK/GLib/Cairo/字体/enchant 字典及其他捆绑组件的许可闭包，也没有核验 Authenticode 或可重复构建。

安装任务 `job-1791691244756-1` 成功退出 0，代次 `gen-job-1791691244756-1` 为 ready，启动器 `Artha/bin/artha.exe` 的固定摘要为 `a033000e7b069d823706a0acd5ae185dadd58866151d1d9d8e9406e2dc8c7f2f`。实际 `/DIR=C:\Artha` 参数包含一个反斜杠，为 13 个 UTF-8 字节，hex `2f4449523d433a5c4172746861`。安装助手的 Python 源码产生 `\A` 转义警告，实际参数字节已核对，recipe 未改。

[来源收据](evidence/2026-10-11-artha-source.json) 保留下载核验时 `installed=false` 的原字节；后续实际安装另记在 [安装收据](evidence/2026-10-11-artha-installed-pending.json)，没有覆盖早期阶段。

## 正式启动失败与受控诊断

正式启动任务 `job-1791691304343-2` 显示 `Error loading GUI from .\gui.glade! aborting...`，正常确认错误对话框后退出 3。托管计划的工作目录是代次根目录，资源实际位于 `prefix/drive_c/Artha/bin`。固定 `configure.ac` 的 Windows `APP_DIR` 为 `.`，`src/gui.c` 使用该目录构建 GUI 资源路径；安装资源摘要匹配，问题是工作目录而非资源缺失。

当前启动器定义只有 executable、arguments、environment 等字段，没有按启动器设置工作目录的能力。没有修改正式服务的全局运行时目录，没有复制资源到代次根目录，也没有更换程序、绕过启动器摘要或使用 shell 包装器。

诊断使用原 `compatforge-cli-managed-msi-v10 prepared-launch` 和同一固定程序，在一份私有上下文副本中仅将运行时工作目录指向 `Artha/bin`。首个临时单元因 `ProtectSystem=strict` 下 `/tmp` 只读而在 Wine 初始化清理时报 `FontPreparation; ServerExit`，退出 1；实际写入探针复现 EROFS/errno 30。仅重试诊断单元增加 `ReadWritePaths=/tmp` 后启动成功。正式服务的防护和上下文保持原样，失败单元和日志保留。

两次诊断进程的实际 GUI 操作为：

1. 查询 `happy`，观察形容词解释及同义词；查询 `computer`，观察两个名词义项及同义词。
2. 用查询框菜单 Save History，通过 GUI Save As 导出到 ASCII 路径 `C:\ForgeQA\output\original-history.txt`，观察成功对话框；About 显示 Artha 1.0.3。
3. 使用工具栏 Quit 正常退出，运行时记录 wineserver 停止与进程退出 0。
4. 独立新诊断进程从托盘展开，历史下拉框保留 `computer` 和 `happy`；选择 `computer` 后实际重新查询，再用 Quit 正常退出 0。

导出文件和内部历史均为 17 字节 `happy\r\ncomputer\r\n`，SHA-256 `2d6ab91c7baecb5e3e34065f13e2ae764c013752d1fd02f1da37a4328bc0d68a`。导出由应用 GUI 写入，核验助手只读。截图及实际输出见 [诊断收据](evidence/2026-10-11-artha-gui-diagnostic.json) 和 [输出](evidence/2026-10-11-artha/diagnostic-history.bin)。

第二次正常退出时，Artha 将配置注释后的一个空行删除，180 字节变为 179 字节。初次完整核对因错误要求配置原字节完全相同而停止，未生成该次最终检查点或执行后续变更。实际重新读取确认所有配置节和键值相同，仅该空行不同；初次收据保留，最终实际字节单独固定。私有归档核验还出现手工成员计数 20/19 的错误，已在同一份归档字节上更正。提交前独立审查发现来源 JSON 的 61 个 CRLF 会被默认 Git 过滤器标准化，已增加仅作用于该来源文件的 `-text` 规则；原收据未改，过滤后 blob 与原始 blob 相同。全部失败和修正见 [失败记录](evidence/2026-10-11-artha-failures.json)。

这些结果仅证明两个英文查词、ASCII 历史导出及诊断新进程历史恢复。正式托管启动仍未修复；中文路径和输入、其他关系/拼写/热键/通知/高级功能、独立市场代次、更新、回滚、服务重启、客体联网下载、新机器注册、迁移及跨版本升级未验收。未创建签名验收目标，未上架，未增加完成数量。

## 状态保留和后续

本轮私有完整检查点依次为 before 219/75、after-install 220/75、after-gui-failure 221/75、after-diagnostic-first 221/75、after-diagnostic 221/75。旧 Core 任务及完整 Store SQLite 行的所有字段保留；正式配置、全部代次、市场快照、信任文档及服务防护在首次 GUI 失败后至最终诊断完全一致。全部任务及诊断进程已终结，既有输出和不自动息屏配置摘要通过只读核验。

当前仍为 **candidate-v30 / root 1 / 在线角色 39**，没有签名或市场变更。私有最终检查点为 `artha-20261011/after-diagnostic.json`；当前只读助手为 `L:/project/FOS/.transfer/rolling-current-state-artha-20261011.py`，219/75 及更早固定总数助手只代表历史。[环境核验](evidence/2026-10-11-artha-environment-verification.json) 记录实际状态。私钥、密码、凭据、完整私有检查点和原程序包没有入库。

下一步需要在 Core 托管启动链路提供按启动器设置的、限制在所选代次内的工作目录，验证路径越界和符号链接拒绝、旧定义默认行为及启动器摘要保持；随后重新通过正式任务执行相同功能流程，再验证独立市场代次和生命周期。不能把诊断副本配置直接当作正式修复，不能改变全局运行时目录或重写已有失败任务和代次历史。

已只读复核独立 Ubuntu 台账和实施计划：Kate 与 Snap 计算器仍为 2 项 Linux 上架验收，Flatpak Text Editor 未安装且保留两次下载超时。没有启动第二个 4 GiB 虚拟机，Linux 不计入 Windows 数量。现有每半小时续跑计划继续保留。
