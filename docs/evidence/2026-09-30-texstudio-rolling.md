# Windows 滚动适配：TeXstudio 4.9.5

隔离 ForgeOS v12 虚拟机（Wine 11.14）已完成 TeXstudio 的官方安装包校验、真实 GUI 文档编辑保存、签名验收和本地 ForgeStore 上架。独立 Windows 应用累计 **6/1000**。这是本地市场候选目录，尚未发布到远程公共市场。

## 输入、修复与功能证据

- [TeXstudio 4.9.5 官方发布页](https://github.com/texstudio-org/texstudio/releases/tag/4.9.5)的 `Texstudio-4.9.5-win-qt6-signed.exe` 为 154,728,568 字节，SHA-256 `618c633e1ad6d9aba90ff8c4498d265b2cbfb6d02173d4577ca8ba3c989cc1e4`，与项目内固定配方一致。安装器为 PE32+ x86-64，安装后的 `texstudio.exe` SHA-256 为 `d037bba7ceec90ce2e3493181c7f6c19907baf25e3522a42979c4d070e962340`。
- 首次作业 `job-1790702224230-8` 的进程退出码为 0，但安装完成检查失败。虚拟机中的真实目录是 `C:\Program Files\texstudio`，原 Mac-Win 配方写成 `C:\Program Files\TeXstudio`；Linux 文件系统区分大小写。修正启动器路径后，重新安装作业 `job-1790702457484-9` 成功，GUI 启动作业 `job-1790702501596-10` 正常退出。配方修复在 Mac-Win 仓库中。
- 通过 TeXstudio 的 GUI 文件选择器打开 `rolling-texstudio.tex`，在编辑器内输入 ` gui ok` 并保存。落盘文件 87 字节，SHA-256 `2a2bfb50d9f81b7c70fbbb83466f88cc90b49b096331c9b29ad1056f3fb06cd3`；见[编辑截图](2026-09-30-texstudio-gui.jpg)和[签名验收收据](windows-texstudio-20260930.acceptance.json)。
- [项目官网](https://www.texstudio.org/)称 GPL v2，[4.9.5 主程序源码头](https://github.com/texstudio-org/texstudio/blob/4.9.5/src/texstudio.cpp)明确 GPL v2 或更新版本；同一标签的 [COPYING](https://github.com/texstudio-org/texstudio/blob/4.9.5/COPYING) 包含 GPL v3 文本。市场展示主程序为 GPL-2.0-or-later，并提醒捆绑组件仍适用各自告知。

## 签名市场与回退

[`candidate-v6`](../../catalogue/candidate-v6/tuf/metadata/15.targets.json) 的 TUF targets/snapshot/timestamp 版本为 15，沿用可信根 SHA-256 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。目录 target SHA-256 `f6bb66d5c9753e4f43e798cc4e65354409e00b5bc107581dd390964f57fbb2aa`，本轮验收收据 target SHA-256 `5bd0dd7dbc567102d0b72af5ddee154738612a56cef7be886fe3e623bf89a6bf`。签名私钥和安装包未进入 Git 仓库。

虚拟机加载新目录后，[已安装页](2026-09-30-texstudio-market.jpg)和[详情页](2026-09-30-texstudio-market-detail.jpg)显示 TeXstudio 4.9.5、许可证、官方来源、`tested` 状态及签名验收引用。利用再次校验的安装包填充商店本地缓存后，同版本更新作业 `06232e0d-cfc6-403b-8efd-55930dcb9cc8` 成功，回退作业 `22966b50-2238-4499-b449-5c71eaf7e029` 成功，原文档 SHA-256 保持一致。[最终机器记录](2026-09-30-texstudio-publication.json)显示六款 Windows 应用均已安装、两个服务运行且商店作业全部为终态。

`cargo test -p forge-store-core --test candidate_catalog --locked` 通过，验证 v6 签名目录与收据以及旧版本回退保护。虚拟机未安装 TeX 发行版，因此编译/PDF 输出未测试；未验证在线下载安装、跨版本升级、整机重启或远程公共市场分发。持续队列见[进度账本](../windows-1000-progress.json)。
