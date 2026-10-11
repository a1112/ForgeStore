# Windows rolling acceptance: RedNotebook 2.43

2026-10-11：Windows 累计 **38/1000**，12 个未解决候选保留。有限本地市场验收见 [publication](evidence/2026-10-11-rednotebook-publication.json)。原 source、GUI、acceptance 32 份冻结文件及其市场待执行历史标签保持原字节，后续完成另记。

正式原代次 `gen-job-1791719938509-4` 和独立市场代次 `gen-job-1791721534514-8` 打开两个中文名小型单日日记，普通 BEFORE→AFTER 替换、ASCII ORIGINAL42/MARKET42 追加、GUI Save As 到 ASCII 文件夹、GUI 当前日期纯文本导出及显式新正式进程重开。原/市场不可变输入各 58 字节；最终保存 YAML 为 79/77 字节，纯文本导出 89/87 字节，SafeLoader 独立解析正文和导出文字完全匹配。活动编辑副本按应用行为改变，不声称其原字节不变。五次通过的正式 GUI 均通过 Journal/Quit 正常退出 0，wineserver 正常停止。合成日记不计应用。

初始默认文字裁切虽退出 0 不计验收。单变量请求层 `PANGOCAIRO_BACKEND=fc` 诊断可读后，使用现有原生 launcher.environment 固定新正式定义；两个正式代次均使用该字段。没有 Core/程序、全局 context、工作目录、字体包、DLL 或 Wine runtime 改动，不声称定位具体 Wine/Pango/Cairo 缺陷函数或通用修复。只在隔离应用配置准备 Sans 24 字体偏好；失败字体对话框没有验证此设置。GTK 内多种 Ctrl/导航快捷键未产生预期效果，改用真实鼠标选词/选行操作并验证实际输出，未修复这些快捷键问题。中文名路径打开/保存时日志出现 cp1252 UnicodeEncodeError，有限 GUI 和磁盘数据有效，但日志问题未修复，不声称无人值守或完整中文路径支持。

固定官方安装包 rednotebook-2.43.exe 为 25489461 字节，SHA-256 `9b153c8a88db1743689672a488daa96cce817d97db18524db63e0e2c8248c43a`，与官方固定版本 API 的大小和 digest 匹配。源码固定提交 `5df78689682a587089795f212873877f99c84747`；2291556 字节归档 SHA-256 `36c30a40bdfe506835e4f0130b00370f774e1484d9521a7d67ceb7ca1d79d894` 仅本地固定摘要，未核实上游源码归档摘要。固定 journal.py/storage.py 声明主源码 GPL-2.0-or-later；README 因 LGPLv3+ 捆绑组件声明组合发行物 GPLv3+，市场按该组合声明展示。完整实际捆绑许可闭包未核验，两代次未找到独立主程序 COPYING/LICENSE 命名文件，不声称安装主许可与源码同字节或二进制许可提取。Authenticode、签名验证和可复现构建未验收。

原生 Inno 参数 `/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /DIR=C:\RedNotebook /TASKS=`，DIR 实际一个反斜杠。两个正式代次分别固定 5951 个安装文件，5950 跨代次原字节一致；唯一差异 unins000.dat 各 1346985 字节，单独固定，原因与卸载功能未验收。GUI About 操作未实际打开 About，不声称 About 版本观察成功。

Store IPC 预缓存同版本更新，形成独立市场代次。完整 298 Core / 93 Store 回滚前检查点先保存读回，之后精确恢复上述正式原代次；最早失败默认代次保持且未选作正式原代次。回滚后 298/94 全旧字段保留，新 GUI 显式重开原保存日记。重启前严格旧 pins 发现 system.reg 变化即停，首次只读诊断仅允许注册表也因日志变化停止；确认仅原 system.reg、user.reg 和 rednotebook.log 三份原字节变化，其他程序和固定数据以及市场全部 pins 保持。旧原文仅摘要无法差分，原因未确定，不声称三份文件跨第三 GUI 字节保持。新原文私有保存读回后按严格新 pins 正常两服务重启，299/94 完整任务和 SQLite 全字段、全部代次/配置/信任/防护以及受测固定数据重启前后精确一致。

完整失败见 [原失败](evidence/2026-10-11-rednotebook-failures.json) 与 [生命周期失败](evidence/2026-10-11-rednotebook-lifecycle-failures.json)。错误首次编辑、额外尾空行、无效快捷键/字体操作、导出向导 Back 误点、错误函数调用、库摘要标签和只读助手数字替换均保留。失败后依赖变更停止，实际修正另记，不补造检查点或重写旧收据。

当前 candidate-v41，root 1、在线 targets/snapshot/timestamp 50，40 个目录条目（38 应用和 2 内部夹具）、81 实际签名目标和 46 公共文件。所有旧签名目录与根字节保留。私有最终完整检查点 `rednotebook-20261011/after-restart.json` SHA-256 `6089d456d6010970f5b20927743a281a6885807e5bc2f9542d9bb7938cf92bb8`；完整内容、私钥、密码和凭据不入库。只读核对使用 `.transfer/rolling-current-state-rednotebook-v3-20261011.py`，须经严格 SSH rolling-guest-20261010.py 在客体执行，旧助手仅代表历史。

核对覆盖完整历史字段、全部代次/配置/信任/防护、既有固定验收数据及本轮固定文件，不声称遍历全部旧应用文件系统。不自动息屏设置保持，认证锁屏配置未改。高级标签/分类/搜索/预览/拼写/模板、HTML/LaTeX/PDF/打印、大型或多日期日记、其他编码、直接中文输入、联网下载、新机注册、迁移、跨版本、无人值守及 GUI 更新/回滚按钮未验收。Ubuntu 两款独立记账，Flatpak 未安装且保留缓存；本轮未切换 VM。

GitHub 现有认证仍阻塞普通推送，之前设备代码已过期。本轮不重发验证，不强推；恢复认证后普通推送本地累积提交。
