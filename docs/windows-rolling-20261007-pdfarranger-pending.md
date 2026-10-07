# Windows 滚动适配：PDF Arranger 1.14.0（安装前阻塞）

本轮未新增验收或上架，累计保持 **23/1000**。新增一个固定来源已核对、MSI 结构检查未通过的候选，阻塞候选由 11 个增至 12 个。没有向测试客体提交安装任务，也没有把下载或检查当作安装成功。

采用 [PDF Arranger 官方 1.14.0 发布](https://github.com/pdfarranger/pdfarranger/releases/tag/1.14.0) 的 `pdfarranger-1.14.0-windows-installer.msi`，63311360 字节，SHA-256 `33ba1c3028cc81eb364b8eed288389fc8a4869c015ce817b9408a3d31e0e90b0`，与官方 GitHub 资产摘要相同。固定源码提交为 `29f306bb6932e28d3001b098f6dad9824ab74100`；主程序源码头明确允许 GPL v3 或更高版本，记录为 GPL-3.0-or-later。组件许可闭包、Authenticode 与可重复构建未验收。详见[来源记录](evidence/2026-10-07-pdfarranger-source.json)。

原始安装包在宿主下载校验后，复制到构建机私有检查目录再次校验。固定的 `msi-tables` 检查器连续两次返回 `DIFAT chain must terminate with 4294967294, not 4294967295`。直接读取原文件确认：512 字节扇区、7 个 DIFAT 扇区，最后链接位于字节偏移 57082876，值为 `0xffffffff`。[Microsoft MS-CFB 第 2.5 节](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/0afa4e43-b18f-432a-9917-4f276eca7a73) 要求末尾链接使用 `0xfffffffe`。这是本次固定输入的结构检查失败；没有证明 Windows 原生安装器会否拒绝该包。

作为对照，已验收 WinDirStat 2.9.0 的原固定 MSI 通过同一检查器。保留了两次失败，独立重复检查结果、扇区链与对照见[诊断记录](evidence/2026-10-07-pdfarranger-msi-blocked.json)。安装包未修改，检查未放宽，产品代码未变更。由于检查发生在客体复制之前，PDF Arranger 尚未注册、安装或启动；GUI 页面重排、旋转、中文路径保存和重开均未验收，未生成签名验收证明。

环境核对确认原有 177 条核心任务均终态，63 条市场完整记录、应用代次与输出、签名信任和不自动息屏设置保持完整。信任根 1、在线角色 33、`candidate-v24` 未改变。只运行原 4 GiB Windows 兼容虚拟机，Ubuntu 虚拟机保持停止；Ubuntu 两项验收与 Flatpak 两次超时未改变，未重复下载。详见[环境记录](evidence/2026-10-07-rolling-environment.json)。

宿主 noVNC 隧道已恢复，HTTP 返回 200，通过现有内嵌浏览器看到原虚拟机的 Plasma 锁屏。登录输入未确认进入桌面；没有取消认证或锁屏，也没有声称此前独立 Edge 的网址识别拒绝已修复。

![原测试桌面仍在锁屏；不是 PDF Arranger GUI 验收](evidence/2026-10-07-pdfarranger/test-desktop-locked.jpg)

证据脚本首次把浏览器返回的 JPEG 字节按 PNG 文件名处理，格式断言失败。已核对原始 JPEG 文件头并更正扩展名，截图字节未转码或编辑；该脚本失败不属于应用安装失败。

后续核对官方 portable ZIP 与现有受管理安装路径是否匹配，或选择下一款固定输入可核对的应用。任何替代输入都需要独立摘要、安装、真实文件工作流和市场生命周期证据。
