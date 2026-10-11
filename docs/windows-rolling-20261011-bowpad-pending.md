BowPad 2.9.6 Windows 滚动诊断（未通过）

本轮安装成功，但功能区缺失，替换和另存为入口无响应，尚未完成编辑、保存、新进程重开。Windows 仍为 40/1000，本项加入未解决候选（13 项），未签名验收或上架。

官方固定 MSI `BowPad64-2.9.6.msi` 为 3,637,248 字节，SHA-256 `1cce569a0eda6552b7b8f49251be2ceecc2ed3ae44fd5a0397757976f6ec46c3`，仅核对本地 HTTPS 下载摘要，没有取得上游公开 digest。MSI ProductVersion 为 2.9.3128，File 表主程序版本为 2.9.6.3128；不把 MSI 三段值当作 GUI 版本实测。固定 tag 源包为 22,442,533 字节，SHA-256 `ef442b1c54da34c99266e3aec4353df29146974866b75d8695e34099430a5420`；四个成员与关联提交 `bfcb30c6fc44eaaa331bb8db37df5c00e2ae716e` 的固定 raw 文件原字节一致。主程序源码声明 GPL-3.0-or-later；完整组件许可闭包、安装许可比较、Authenticode、二进制源码对应及可复现构建未核验。

使用现有 managed MSI 正常安装，任务 `job-1791729824065-1` 成功，冻结代次 `gen-job-1791729824065-1` 就绪。应用目录四文件分别固定，主 exe 5,683,360 字节，SHA-256 `25b9f668887403c846f1013300993cef63fd5f1a8e14ef2040b02175dad7f8a2`。MSI 含 SparsePackage 注册/注销自定义动作和 shell DLL，其功能未验收。

正式默认启动 `job-1791729893959-2` 打开 63 字节小型中文名 UTF-8 BOM/CRLF 文本。第一截图是初始过渡，最大化后截图显示正文；这些不证明完成编辑工作流。功能区缺失，Ctrl+H、Ctrl+Shift+S 和延长按键没有打开预期对话框。输入原字节保持，预期输出文件缺失。通过标题栏 X 正常退出 0 和 wineserver 停止，不把正常退出计为 GUI 验收。

请求诊断 `job-1791730049410-3` 的 WINEDEBUG 请求被固定运行时绑定覆盖，实际仍是 -all；预期日志环境断言失败即停止，另作只读更正记录后正常关闭。然后使用私有上下文副本和原固定 CLI 的 prepared-launch，仅将副本运行时绑定 WINEDEBUG 改为 -all,+uiribbon；单独临时单元保留 NoNewPrivileges、ProtectSystem=strict、PrivateTmp=false，沿用既有诊断所需 /tmp 可写设置和原网络约束。正式上下文、定义、Core/Store、程序、字体、DLL、runtime 均未修改。该副本进程也缺失功能区，正常关闭退出 0；初始化日志断言失败，实际完整日志只有三个生命周期事件，没有 Ribbon trace。因此不能声称已实际捕获某 HRESULT、函数调用或正式修复。

固定 Wine 11.14 的 uiribbon.c 中 Initialize/LoadUI 等为 E_NOTIMPL stub；固定 BowPad MainWindow.cpp 在 CreateRibbon 失败时会在命令初始化前返回，而 HandleCreate 未检查 Initialize 返回值。这支持功能区初始化缺口的假设，但不能替代当前二进制实际调用证明。后续修复应先补齐可观察的 Ribbon 初始化证据，再评估原生实现或正式可验证的应用修复；不能用窗口出现、诊断副本或 shell 包装器代替通过。

安装前完整 313 Core/98 Store 检查点保存读回；当前 316/98，原全部任务字段、市场完整 SQLite 行、全部既有代次、正式配置、信任、防护和固定验收数据一致。新增两项启动和安装均正常终态，隔离诊断也已停止。私有当前完整检查点 after-diagnostic.json SHA-256 `e0e564046e02d6a81d9e7d882423e764658be4b815fcaa11b569fa272efba9d8`。固定本轮程序、输入、配置及注册表摘要；范围不等于遍历全部应用文件系统，不用最新 64 条 IPC 摘要替代完整 SQLite 历史。本轮没有 Store 生命周期、服务重启、签名或目录更改的验收声明。

当前市场 candidate-v43/root 1/在线角色 52 保持；40 个应用、2 个既有夹具、全部签名历史和 nomacs 等既有有限验收保持。认证和锁屏未改，不自动息屏配置不变。Ubuntu 两项通过和 Flatpak 下载阻塞独立保留，没有切换或同时运行两个测试 VM。

保留 API 限流、无效请求日志、日志断言及辅助历史文件读取失败。仅 .gitattributes 和台账可更新；既有冻结证据原字节保持。GitHub 登录阻塞未恢复，本轮不重复发设备码或声明推送。详细记录见 source、installation、gui-diagnostic、failures、preservation 和 current-state JSON。
