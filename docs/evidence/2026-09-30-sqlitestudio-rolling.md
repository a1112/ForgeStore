# Windows 滚动适配：SQLiteStudio 3.4.17

本轮在隔离 ForgeOS v12 虚拟机（Wine 11.14）完成 SQLiteStudio 的安装、GUI 数据库读写、修复、签名验收和本地 ForgeStore 上架。1000 款独立 Windows 应用目标当前为 **4/1000**；其余三款是 7-Zip、Notepad++、SumatraPDF。该计数只覆盖固定版本和安装包摘要的本地验收，不代表公共远程市场发布或跨版本升级。

## 固定输入与 GUI 功能

- 官方 [3.4.17 发行页](https://github.com/pawelsalawa/letos/releases/tag/3.4.17)提供的 x64 安装器 `SQLiteStudio-3.4.17-windows-x64-installer.exe`，大小 35,103,114 字节，SHA-256 `5018ea571c2a3416944267d387cb75eea99d46ebd010f6aa5c35df1f7690c894`；源码许可证为 GPL-3.0，带 OpenSSL 链接例外。
- CompatForge 安装作业 `job-1790697900088-1` 成功。GUI 启动后在 SQLiteStudio 中新建 SQLite 3 数据库 `qa.db`，通过 SQL 编辑器执行 `create table t(x);insert into t values('ok');select * from t;`。结果表显示一行 `ok`；独立读取实际数据库文件也得到同一行，`PRAGMA integrity_check` 为 `ok`。数据库 SHA-256 为 `69a8c84a9a26b36193eead21d477fe76fc2f80f2dae6d9489a984be6b769589d`。见 [GUI 截图](2026-09-29-sqlitestudio-gui.jpg)和[签名验收收据](windows-sqlitestudio-20260929.acceptance.json)。
- 安装后 SQLiteStudio 经正常 GUI 关闭。商店同版本更新生成新安装代，之后商店回退作业 `dd7b6087-0e28-4aa3-b365-62c94afcb6da` 成功；原数据库再次校验仍为一行 `ok`、完整性 `ok`。该结果不构成跨版本升级测试。

## 故障与修复

1. 测试宿主的应用盘满导致虚拟机暂停。将专用动态 VHDX 的虚拟容量从 96 GiB 增至 128 GiB，并在线扩容 ext4 后恢复 QEMU；原始镜像、既有安装和历史任务均保留。
2. 图形安装器在未收到会话 `DISPLAY`、`XAUTHORITY` 时退出。手工向 CompatForge 安装任务传递两个受限覆盖项后继续，随后修复 ForgeStore 的 CompatForge 请求构造，使商店从自己的正常会话环境传递本地显示与授权文件路径。
3. SQLiteStudio 的 MinGW 导入库 `libstdc++-6.dll` 被 PE 检查器错误拒绝。CompatForge 允许导入库名中的 `+`，同时新增拒绝 `../evil.dll` 的回归测试。部署的 CompatForge CLI SHA-256 为 `201dffa0e69c712b1548f72227d80fab59ec5c27a0051ab69b2af1a46a6bbbb5`。
4. 商店首个在线更新作业 `418c43b5-4dd7-4975-a8a5-bcc1638bb1d1` 因响应体解码错误失败，不能据此认定具体网络原因。将同一官方安装包按已核验 SHA-256 放入用户缓存后，重试作业 `155eb5f9-bacc-45b2-8a8e-bfa8ebe3f077` 暴露第 2 项会话环境缺陷。使用修复后的商店服务再重试，`52ce07ae-12b1-45fc-a611-5eb2a3d6b0e2` 成功；在线下载链路仍待单独复测。新商店服务 SHA-256 为 `9845c7b2df83bf7f6a061423fe7f69151c4b22444f84b027a1f4b24c9507fd3e`。
5. 目录准备工具原本只识别首批三款应用，并将 Windows 安装包限制为 32 MiB。现支持配方中提供的新应用显示信息，并将上限与核心解析器的 1 GiB 工件上限对齐。35 MiB 新应用回归用例先失败后通过；新工件仍默认 `unknown`，不能从摘要校验继承 `tested`。

## 签名与本地市场

新目录为 [`candidate-v4`](../../catalogue/candidate-v4/tuf/metadata/13.targets.json)，TUF targets/snapshot/timestamp 版本 13；可信根未更换，SHA-256 仍为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。目录 target SHA-256 为 `ad84864f5e5269468308436cc997e5c87d54a18d05937e017b0290aafe7f6fde`，SQLiteStudio 验收收据作为独立签名 target 的 SHA-256 为 `79ccc06e1d68f0c987b0f2571baf7895d2739cea47010ecb955eeca6c8dee038`。旧三款的验收收据仍在版本 13 的签名目标中。私钥只位于 Git 仓库外的私有签名目录，未复制进 VM 或 Git。

隔离 VM 的用户服务只读映射新 metadata 和 targets；服务重启前后的已安装应用和历史任务保持一致。ForgeStore 的[已安装页](2026-09-29-sqlitestudio-market.jpg)显示 SQLiteStudio 3.4.17，[详情页](2026-09-29-sqlitestudio-market-detail.jpg)显示许可证、官方来源、`tested` 和对应收据。最终[机器记录](2026-09-30-sqlitestudio-publication.json)核对签名目录共六项（四款 Windows 应用、两项内部原生测试夹具）、四款 Windows 应用均已安装、所有商店作业终止，ForgeStore 与 CompatForge 用户服务正常运行。

## 验证范围和后续规模约束

ForgeStore `cargo test --workspace --locked` 通过；Python 单测 4 通过、1 项因缺少 Linux ForgeOS 包工具跳过；`cargo fmt --all --check` 与 `git diff --check` 通过。CompatForge PE 检查器单测在 Windows 为 15 项通过，在隔离 Linux 构建源为 17 项通过。Linux 端修复后的商店服务后端测试 11 项通过、1 项需特定镜像环境而跳过。仅执行了同版本安装代更新与回退，没有跨版本升级、整机重启或公共市场分发验证。

当前 ForgeStore 源码的单目录 512 KiB 限制、商店 IPC 响应 1 MiB 限制和 UI 的 1 MiB 响应限制预计会在累计 1000 款前成为瓶颈；后续需要扩容或分页，并做 1000 项的端到端测试。持续队列和计数见[进度账本](../windows-1000-progress.json)。
