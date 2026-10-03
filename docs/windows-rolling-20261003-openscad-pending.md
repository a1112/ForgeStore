# Windows 滚动适配：OpenSCAD 2021.01 来源通过、图形工具阻塞（2026-10-03）

本轮新增 OpenSCAD 固定安装包与许可核对证据。**累计仍为 20 / 1000；OpenSCAD 未安装、未进行 GUI 验收、未上架，不能计入完成。** 原有 10 项阻塞原样保留；新增一项测试工具阻塞，队列为 11 项。签名目录仍为 candidate-v20，可信根 1、角色版本 29，不创建新签名、不修改信任状态。

[官方固定发布 openscad-2021.01](https://github.com/openscad/openscad/releases/tag/openscad-2021.01) 对应源码 commit `41f58fe57c03457a3a8b4dc541ef5654ec3e8c78`，发布时间为 2021-02-07T00:14:34Z，非预发布、未标记 immutable。未宣称最新或发布不可变。

完整 `OpenSCAD-2021.01-x86-64-Installer.exe` 为 **21,876,824 字节**，与官方 GitHub API 的资产长度一致；SHA-256 为 `fbe1e590e1af2af863f5d92bd263eb4644d0ee29352e3ab566366e87a8f6ed12`，与[官方校验文件](https://files.openscad.org/OpenSCAD-2021.01-x86-64-Installer.exe.sha256)的 `releases/OpenSCAD-2021.01-x86-64-Installer.exe` 行完全一致。先前网页读取工具返回 Internal Error；实际完整下载成功，不把网页读取失败当作校验不存在。GitHub API asset digest 为空，不声称 API digest 匹配。安装器 PE machine 为 0x14c；这不等于已安装载荷的架构，64 位载荷仅由固定发布文件名指示，需安装后检查 launcher。

[固定 src/mainwin.cc](https://raw.githubusercontent.com/openscad/openscad/41f58fe57c03457a3a8b4dc541ef5654ec3e8c78/src/mainwin.cc) 的主程序头明确声明 GPL v2 或更高版本及 CGAL 链接例外；[COPYING](https://raw.githubusercontent.com/openscad/openscad/41f58fe57c03457a3a8b4dc541ef5654ec3e8c78/COPYING) 含 GPL v2 全文和该例外。主许可记录为 **GPL-2.0-or-later with CGAL linking exception**。捆绑库、字体、示例各有条款，完整组件对应源码及再分发审计未完成。

固定 `scripts/installer.nsi` 与 `scripts/installer64.nsi` 的有界检查显示预期安装目录为 Program Files/OpenSCAD、launcher 为 openscad.exe；安装还包含字体、库、示例、语言资源、.scad 文件关联、所有用户开始菜单快捷方式和 HKLM 卸载信息。这是源脚本观察，尚未证明安装器对应构建、实际路径、注册表结果或完整安全性。

旧浏览器控制工具在当前工具集中不可用。使用支持的 Computer Use 枚举后，选择唯一返回的 Microsoft Edge 窗口；首次 get_window_state 即返回：

> Computer Use has been stopped for this turn because it could not determine the current browser URL on Windows with enough confidence to enforce policy.

工具停止后没有发送任何应用输入，没有使用其他路径绕过停止，也没有操作 Codex 窗口。此项为图形测试工具阻塞，不是 OpenSCAD 功能失败。下一步需在独立 Microsoft Edge 浏览器显示现有 noVNC 地址和地址栏，再于新回合重新观察工具状态。无需移除密码或修改认证。

完整安装、实际 GUI 建模修改、保存、渲染、STL 导出、输出检查、新进程重开和签名本地市场更新/回滚/重启均待执行。AuthentiCode、独立 GPG 验证、可复现构建和完整组件再分发审计未验证。安装包、临时下载签名 URL、密码和签名密钥不入库。

可核对长度、摘要、固定许可及安装脚本 pin 见 [来源证据](evidence/2026-10-03-openscad-source.json)。本轮只更新候选证据和台账，已有 20 款通过项、旧队列记录及活动市场指针保持。
