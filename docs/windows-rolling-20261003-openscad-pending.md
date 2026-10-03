# Windows 滚动适配：OpenSCAD 2021.01 已安装、GUI 工具阻塞（2026-10-03）

本轮完成 OpenSCAD 固定安装包与许可核对，以及正常托管 NSIS 安装。**累计仍为 20 / 1000；OpenSCAD 已安装，未进行 GUI 验收、未上架，不能计入完成。** 原有 10 项阻塞原样保留；新增一项测试工具阻塞，队列为 11 项。签名目录仍为 candidate-v20，可信根 1、角色版本 29，不创建新签名、不修改信任状态。

[官方固定发布 openscad-2021.01](https://github.com/openscad/openscad/releases/tag/openscad-2021.01) 对应源码 commit `41f58fe57c03457a3a8b4dc541ef5654ec3e8c78`，发布时间为 2021-02-07T00:14:34Z，非预发布、未标记 immutable。未宣称最新或发布不可变。

完整 `OpenSCAD-2021.01-x86-64-Installer.exe` 为 **21,876,824 字节**，与官方 GitHub API 的资产长度一致；SHA-256 为 `fbe1e590e1af2af863f5d92bd263eb4644d0ee29352e3ab566366e87a8f6ed12`，与[官方校验文件](https://files.openscad.org/OpenSCAD-2021.01-x86-64-Installer.exe.sha256)的 `releases/OpenSCAD-2021.01-x86-64-Installer.exe` 行完全一致。先前网页读取工具返回 Internal Error；实际完整下载成功，不把网页读取失败当作校验不存在。GitHub API asset digest 为空，不声称 API digest 匹配。安装器 PE machine 为 0x14c；实际安装 launcher 的 PE machine 为 0x8664，分别是 32 位安装引导与 64 位应用程序。

[固定 src/mainwin.cc](https://raw.githubusercontent.com/openscad/openscad/41f58fe57c03457a3a8b4dc541ef5654ec3e8c78/src/mainwin.cc) 的主程序头明确声明 GPL v2 或更高版本及 CGAL 链接例外；[COPYING](https://raw.githubusercontent.com/openscad/openscad/41f58fe57c03457a3a8b4dc541ef5654ec3e8c78/COPYING) 含 GPL v2 全文和该例外。主许可记录为 **GPL-2.0-or-later with CGAL linking exception**。捆绑库、字体、示例各有条款，完整组件对应源码及再分发审计未完成。

固定 `scripts/installer.nsi` 与 `scripts/installer64.nsi` 的有界检查显示预期安装目录为 Program Files/OpenSCAD、launcher 为 openscad.exe；脚本还包含字体、库、示例、语言资源、.scad 文件关联、所有用户开始菜单快捷方式和 HKLM 卸载信息。安装后程序路径确已核对；其余为源脚本观察，尚未证明安装器对应构建、快捷方式/注册表结果或完整安全性。

旧浏览器控制工具在当前工具集中不可用。使用支持的 Computer Use 枚举后，选择唯一返回的 Microsoft Edge 窗口；首次 get_window_state 即返回：

> Computer Use has been stopped for this turn because it could not determine the current browser URL on Windows with enough confidence to enforce policy.

工具停止后没有发送任何应用输入，没有使用其他路径绕过停止，也没有操作 Codex 窗口。后续回合枚举已返回标题包含 noVNC 的独立 Edge 窗口，重新 get_window_state 仍因同一网址识别问题停止。本项为图形测试工具阻塞，不是 OpenSCAD 功能失败；打开 noVNC 已不足以解决，需恢复工具的浏览器网址识别能力。无需移除密码或修改认证。

安装已通过；实际 GUI 建模修改、保存、渲染、STL 导出、输出检查、新进程重开和签名本地市场更新/回滚/重启均待执行。AuthentiCode、独立 GPG 验证、可复现构建和完整组件再分发审计未验证。安装包、临时下载签名 URL、密码和签名密钥不入库。

可核对长度、摘要、固定许可及安装脚本 pin 见 [初始来源证据](evidence/2026-10-03-openscad-source.json)，其中 installed=false 是安装前观察，保留不改。最新状态见 [安装待验收证据](evidence/2026-10-03-openscad-installed-pending.json)；已有 20 款通过项、旧队列记录及活动市场指针保持。

后续用户说明 Windows 主机已重启，要求恢复环境。只读检查发现 Hyper-V 虚拟机、原 QEMU run 和 noVNC 服务仍 active；Default Switch 中地址已变化，旧 SSH 地址超时，本地 6096 监听不存在。按虚拟机 MAC 获取当前地址，保留严格 SSH 主机密钥校验，恢复隐藏的 localhost 转发；noVNC 页面返回 HTTP 200 且内容正确。未重启存活的虚拟机或 Core/Store，未修改密码。149 个完整 Core 终态作业、57 条 Store 记录、市场、完整 TUF 文档及签名、配置摘要、全部应用代次和服务约束与 JPEGView 最终快照一致；24 个签名目录源文件与输入/输出逐字节保持。桌面显示、登录和 GUI 功能尚未重新观察；工具的本回合停止仍需于新回合解决。见 [重启后连接恢复证据](evidence/2026-10-03-host-restart-recovery.json)。

随后在新回合执行正常 managed NSIS 安装，仅使用 [NSIS 官方文档](https://nsis.sourceforge.io/Docs/Chapter3.html)规定的 `/S` 静默参数，没有使用关闭 CRC 的参数，没有手工解包替代安装。作业 `job-1791003419276-1` 正常结束，Unix Wine 退出 0；代次 `gen-job-1791003419276-1` ready 并被选中。程序实际为 `Program Files/OpenSCAD/openscad.exe`，43,975,680 字节，SHA-256 `c6b741fc0907e4d571b8a91097a1a5e1e426f5c422385c77b788ebca76e369d0`，与代次 main launcher pin 一致。应用目录共 183 个文件。

文件版本信息只做 VS_FIXEDFILEINFO 签名候选扫描，得到零版本和 DLL 类型；不是完整 PE resource-tree 解析，不把该候选当有效安装版本。实际 About 版本和许可仍未观察。文件名检索到安装的 examples/COPYING-CC0.txt 与 fonts/Liberation-2.00.1/LICENSE，长度/摘要已记录；未观察到安装的主程序 COPYING 文件，不推断主许可已在安装中完整保留。固定源码主许可声明仍可核对，GUI notice 待验收。

安装后 Core 作业为 **150 条终态**，原 149 条完整记录逐条不变；57 条完整 Store 记录及市场 snapshot 不变。完整 TUF 信任文档/签名、24 个签名源文件、所有原 Core 配置摘要保持。只准备了中文路径 `C:\ForgeQA\input\模型.scad`：UTF-8、104 字节，含 `cube([18,12,7]);`，SHA-256 `6f123b4af0ddef990e3c0bfcb6a8b5e98ae444c9d15a2ede5eafc229a29e583e`；output 为空，没有生成 GUI 输出，没有启动应用窗口。真实模型编辑、保存、渲染和导出均未验证。首次查找不存在 recipes/schemas 目录的只读错误也保留在安装记录中。
