# Windows 滚动验收：Tiled 1.11.2（2026-10-11）

Tiled 已完成有限 GUI 和独立本地市场生命周期验收，Windows 累计 **34/1000**；12 项未解决候选保持。仅计一款应用，重复代次、合成地图和图块不计应用。当前目录 `candidate-v37`，root 1，targets/snapshot/timestamp 46；36 个目录项、73 个实际签名目标、42 个公共目录文件。目录项数不等于通过应用数。

官方固定版本来自 [mapeditor/tiled v1.11.2](https://github.com/mapeditor/tiled/releases/tag/v1.11.2)，发布于 2025-01-28；不称最新版本。安装包 `Tiled-1.11.2_Windows-10+_x86_64.msi`，22,192,128 字节，下载字节 SHA-256 `6c906099e4bb9a871c13a4f81ecf6554c835a36b2cfffb1dee44e215120a61c7`。API 资产大小一致，但 digest 为 null，摘要为下载后的自行计算值，不是已核验的上游摘要。固定源提交 `517643173e15dbbc22e472ef48c2023e287a1b8c`，源码归档摘要 `19619147e0c6ee457fc6344d264cdf7a8af726413b2f51812deb559b7f5d362c`。MSI 结构检查及托管 msiexec 安装成功；未更改安装处理器。

固定 `src/tiled/mainwindow.cpp` 声明主程序 GPL-2.0-or-later。安装 COPYING、LICENSE.GPL、LICENSE.BSD 与对应固定源码原字节相同，CRLF 标准化次数为零。Qt、插件、示例资源、引擎格式和全部捆绑许可闭包未验收；Authenticode、OpenPGP 与可复现构建未核验。

## 实际功能与代次

正式原代次 `gen-job-1791707208335-4` 与独立市场代次 `gen-job-1791708025849-7` 分别打开小型中文名地图。两份输入各 515 字节：原地图左上角红色图块，市场地图右下角红色图块。通过真实 GUI 擦除红块，在 (1,1)、(2,2) 绘制绿色图块，Save As 到 ASCII 路径的 TMX，使用 JSON 格式导出 `.tmj`。各代次在新正式进程中显式重开两种输出，实际显示正确；输入保持原字节。辅助脚本只创建输入，不写验收输出。

两代次 TMX 输出各 548 字节，SHA-256 `a52b114d30ea35d1ff69eff0b8eafa31b7557e6e170e868e32a9ed3409f988f8`；JSON 输出各 947 字节，SHA-256 `83e98a754d2c4ab49aa4131c1414ad9a492ef688dac55287755521533454d1db`。独立解析核对 16 个 GID、4×4、单图层与相对图像路径。原与市场输出相同是上述编辑的预期结果，原输入摘要不同且保持。

最终五次正式 GUI（原保存、原新进程、市场保存、市场新进程、回滚后新进程）均正常 Quit、退出 0 并停止 wineserver。更早使用上游文件名的代次 `gen-job-1791706281896-1` 也完成两次有限 GUI，完整保留，不能重复计数。21 张编号截图保留，其中 07 是自然锁屏而非 GUI 通过证据；未改变认证、锁屏或不自动息屏配置。

## 文件名修复与失败历史

原定义摘要 `d05f8118371461a8eb054ca2b22cdbad2cf4b5e14ad39ee62e72d1b272fe80f8` 使用上游包含 `+` 的安装文件名。角色 44/candidate-v35 的 TUF 签名、摘要及显式 root 下载通过，但 Store 实际目录激活失败，报 `invalid artifact size or SHA-256`；现有目标名字符校验不接受 `+`，真实大小和摘要并无错误。

保存并读回完整任务、SQLite 和信任检查点，正常停止 Store，保留角色 44 目录与前移的信任缓存，再发布角色 45/candidate-v36，将本地目标改为 `Tiled-1.11.2_Windows-10plus_x86_64.msi`。此时目录启动成功，但更新任务 `9ace0d36-dd89-4e5e-b53e-27df9a961ad9` 失败：目标别名与冻结安装定义的 fileName 不一致。没有创建新 Core 任务或市场安装代次；失败完整 SQLite 行保留。

随后使用新冻结定义统一 installer.fileName 与本地目标别名；新定义摘要 `926343a11d29536e8ef01b5f9cc98eba025225b83689db84d9ae83ba67f13c27`，包字节、大小与摘要仍相同，官方下载 URL 与上游文件名仍保留 `+`。重新安装新正式原代次并重做 GUI，再签名角色 46/candidate-v37，引用独立新收据 `windows-tiled-20261011.alias.acceptance.json`。原 GUI、来源、原收据、角色 44/45、更正收据及其历史待市场阶段标签均保持原字节；后续市场完成单独记录。当前角色 46 保留 73 个目标的实际字节，包括不再由目录引用的旧安装目标与旧收据。

未修改 Core 代码、原生 launcher 环境、工作目录、字体、全局上下文、程序/DLL 或运行时字节。程序 tiled.exe 为 256,440 字节，摘要 `4b58e71f04eb896fca5f1d6c73d1dc3bb5fc4bb929c0d79e01d2576f2b701379`；三个代次安装文件摘要均保留。

其他失败原样记在 [failures](evidence/2026-10-11-tiled-failures.json)：原 helper 错用 arguments；原 JSON 导出被附加 `.tmj`；宿主引号解析失败后仍尝试缺失 helper（未执行客体变更）；停止 Store 后摘要 helper 错把时间字符串当字典；新代次导致 canRollback 状态变化而初次观察误要求全快照相同；未执行的市场草稿 helper 含重复检查点及复制失败标签，执行前另写修正版本；截图计数误包含 tiles.png 而停止后续公共证据与台账写入，后按编号截图区分。不能宣称所有历史依赖门槛都严格遵守。此后每项依赖变更均检查实际断言结果；失败或 false 即停止，保留失败而不补造证明。

## 市场生命周期与恢复

角色 46 目录实际激活成功后，通过 Store IPC 预缓存同版本更新 `66b01664-b14f-4aae-a95f-31e9583ad2f7`，Core 安装 `job-1791708025849-7` 成功创建独立市场代次。市场 GUI 完成后，先保存读回 **266 Core /85 Store 完整行** 的回滚前检查点，再提交 Store IPC 回滚 `e9211c0c-8c42-45ce-a38d-313bea31bf36`。回滚后 **266/86**，原字段精确保留，选择恢复到上述别名正式原代次，早期上游文件名代次保持。回滚后新进程再次打开原 TMX/JSON，正常退出。

随后正常重启 Core 和 Store，两次进程 PID 改变，服务 active，当前客体仍固定 launcher-cwd-v11-r1 Core /managed-msi-v2 Store。重启前后 **267 Core /86 Store 完整历史行** 及所有字段、全部代次记录、正式 Core 配置、信任与防护精确比较通过；受测固定数据摘要保持。不将最近 64 条 IPC 摘要代替完整 SQLite 历史。

私有完整检查点 `tiled-20261011/after-restart.json`，SHA-256 `16b429f18485ec975e15a85c7038b295f8ec88d2c90739957682449ec30d7e68`；完整检查点、凭据与签名私钥不入库。只读核验 helper `L:/project/FOS/.transfer/rolling-current-state-tiled-20261011.py` 必须通过严格 SSH 的 `rolling-guest-20261010.py` 在客体执行。旧 Sigil 257/83、CherryTree 246/81 等仅代表各自历史。

## 验收范围

通过范围仅为上述有限正交单图层、两图块的小地图操作。无限、等轴/六角、多图层、对象、地形/自动映射、动画、world/project、引擎集成、广泛导出插件、大地图、直接中文输入和高级功能未验收。更新/回滚是 Store IPC 验收，不是 GUI 按钮验收；客体联网下载、新机注册、迁移、跨版本升级及无人值守启动未验收。数据核对仅覆盖旧固定验收数据、本轮固定数据与三个 Tiled 代次安装文件，不称遍历全部旧应用文件系统；计划中的 network deny 不作为实际网络隔离证据。

Ubuntu 仍独立记账：Kate .deb/APT、Snap 计算器两款通过；Flatpak Text Editor 两次运行时下载超时且未安装，缓存保留。本轮未同时启动两台 4 GiB 虚拟机。MPC-HC 手动渲染器/时间轴限制、DB Browser DPI 警告及旧回滚前检查点缺口、CherryTree 自动保存输入变化和其余失败/签名历史保持。

关键证据：[publication](evidence/2026-10-11-tiled-publication.json)、[market GUI](evidence/2026-10-11-tiled-market-gui.json)、[runtime recovery](evidence/2026-10-11-tiled-runtime-recovery.json)、[catalogue diagnostic](evidence/2026-10-11-tiled-catalogue-diagnostic.json)。最终独立审查和提交字节复核单独记录，不代替实际客体 GUI 测试。
