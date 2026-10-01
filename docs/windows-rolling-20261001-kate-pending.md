# 2026-10-01 Windows 滚动适配：Kate 待发布

本轮新增 Kate 26.08.1（官方 release-26.08 CI build 12388）的安装和有限 GUI 功能证据。启动路径已修正，中文文件查找替换、另存、重开和两次正常退出均通过。许可声明与参考源码不一致，暂不上架、不签发完成验收；累计仍为 **11/1000**。

安装包来自 [KDE 官方固定构建目录](https://cdn.kde.org/ci-builds/utilities/kate/release-26.08/windows/)，102,660,176 字节；SHA-256 `480aa82b61e299b5636f801366a23377b9ce8ff5f815e1663652bfef32054286` 与官方 `.sha256` 一致，并在构建机和测试机复核。About 实际显示 26.08.1。固定 CI 构建与版本源码标签分开记录，未声称已核对编译提交。

首次安装任务 `job-1790817049484-6` 的安装进程退出 0，但服务随后报 `registry I/O failed: No such file or directory`，世代保持 failed。实盘发现可执行文件位于 `Program Files/Kate/bin/kate.exe`，而初始配方预期 `Kate/bin/kate.exe`。移除未生效的自定义目录参数，改用实际默认路径；重新安装 `job-1790817232316-7` 成功，世代 `gen-job-1790817232316-7` 为 ready。首次失败任务和文件均保留，没有修改服务二进制或把失败世代改为成功。

GUI 从含中文的 UTF-8 输入文件执行 `pending` → `passed` 查找替换，另存此前不存在的 `rolling-kate-output.txt`。输出 56 字节，逐字节等于预期，SHA-256 `77aadcac8b4a0ce2f933139774ba39756de67886f36b08b63d04884f23dc5118`；原始 57 字节输入未变。首次启动 `job-1790817268851-8` 和重开 `job-1790817468597-9` 均通过正常退出 0 收尾；重开后再次复核输出未变。

**发布阻塞：** 安装包 About 打开的许可文本显示 GNU GPL Version 3（29 June 2007）。[v26.08.1 参考源码入口](https://raw.githubusercontent.com/KDE/kate/v26.08.1/apps/kate/main.cpp) 使用 `KAboutLicense::GPL_V2`，[AppStream 参考元数据](https://raw.githubusercontent.com/KDE/kate/v26.08.1/apps/kate/data/org.kde.kate.appdata.xml) 声明 `LGPL-2.1+`。这些是可复核的不同声明；当前没有该 CI 构建与编译提交的证明，不能直接用版本标签的许可替代固定二进制声明。需核清构建/许可对应关系再发布；本报告不判断上游不合规，也不更改原许可。

本轮未签名、未激活新目录、未创建 Kate 市场任务。实时市场仍是 candidate-v11，13 项中 11 款 Windows 应用与 2 个内部 fixture；Kate 不在目录或市场安装清单。38 条市场任务完整记录摘要与上轮最终记录一致，现有安装条目保持一致。TUF timestamp/snapshot/targets 为 20，root 为 1，信任根摘要 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e` 未变；两个服务均 active，Core 任务已全部收尾。

证据：[GUI 待发布记录](evidence/2026-10-01-kate-gui-pending.json)、[来源与许可](evidence/2026-10-01-kate-source.json)、[修正后的配方](evidence/2026-10-01-kate-recipe.json)、[原始服务结果](evidence/2026-10-01-kate-backend.json)。截图：[版本](evidence/2026-10-01-kate-about.jpg)、[许可文本](evidence/2026-10-01-kate-license.jpg)、[编辑保存](evidence/2026-10-01-kate-edit-save.jpg)、[文件重开](evidence/2026-10-01-kate-reopen.jpg)。工程、插件、LSP、终端、调试器、在线下载、新机器注册和 Kate 市场更新/回滚尚未验收。
