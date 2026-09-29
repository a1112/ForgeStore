# Windows 滚动适配：WinMerge 2.16.58.2

隔离 ForgeOS v12 虚拟机（Wine 11.14）已完成 WinMerge 官方安装包校验、真实 GUI 文件比较与合并保存、签名验收，以及本地 ForgeStore 上架、更新和回退。独立 Windows 应用累计 **7/1000**。这是本地市场候选目录，未发布到远程公共市场。

## 安装与功能证据

- [WinMerge 官方发布页](https://github.com/WinMerge/winmerge/releases/tag/v2.16.58.2)的 `WinMerge-2.16.58.2-x64-PerUser-Setup.exe` 为 17,184,616 字节，发布资产 SHA-256 与实测一致：`6ac1e3c0028c39e49962dacd662ab553989d7e817a5cdf96ebbbc1fb345bcff4`。安装作业 `job-1790704920941-2` 成功；已安装 `WinMergeU.exe` SHA-256 为 `a14132ee6a395da2b3e15c4166ff261dd30cb57bf8669b81aa8896e06f14503e`。
- 用 GUI 启动 `job-1790705089309-3` 同时打开两份 UTF-8 文本。WinMerge 显示“1 Difference Found”，右侧原始 SHA-256 为 `0bfdf7b0e37887ad33f48ce0de4ed2ece88cc031f9035e9aed62763b3181751a`。按[官方快捷键说明](https://manual.winmerge.org/en/Shortcut_keys.html)执行 `Alt+Right`，在 GUI 中将左侧差异合并到右侧并保存；两份落盘文件的 SHA-256 均为 `e1c9b784c6f6f85ee0574a687960658c7513c55ebbe93341fd370b16897d25ce`。关闭 GUI 后启动作业退出码为 0。[比较与合并截图](2026-09-30-winmerge-gui.jpg)和[签名验收收据](windows-winmerge-20260930.acceptance.json)保留了证据。
- 用户级 Inno 安装器已退出，但同一 Wine 测试瓶中留下的 `C:\windows\explorer.exe` 让作业等待 `wineserver -w`。核对进程所属瓶后终止该 Explorer 进程，安装及后续同版本更新作业均以退出码 0 完成。该行为仍需产品级处理，不能把长时间等待当作安装失败或手动结束无关进程。
- [项目源码许可说明](https://winmerge.org/source-code/?lang=en)声明主程序按 GPL 第 2 版或后续版本发布；目录展示 `GPL-2.0-or-later`，并提示捆绑组件另有告知。

## 签名市场与回退

[`candidate-v7`](../../catalogue/candidate-v7/tuf/metadata/16.targets.json) 的 TUF targets/snapshot/timestamp 版本为 16，沿用可信根 SHA-256 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。目录 target SHA-256 `2fc75af146de0e18d8f2581025fa05fac67352c52c9714a9cf00e21afa4e0f93`，WinMerge 验收收据 target SHA-256 `a511a8261a331c547ae3a17d1cb229c3c24df39c4252f2d0d98059d9e18928a4`。签名私钥和安装包未进入 Git。

虚拟机加载新目录后，[市场列表](2026-09-30-winmerge-market.jpg)和[详情页](2026-09-30-winmerge-market-detail.jpg)显示 WinMerge 2.16.58.2、许可证、官方来源、`tested` 状态及签名验收引用。商店从已校验的本地缓存执行同版本更新作业 `8180e7f3-4f63-46dd-a2b5-7d841c3765d0` 和回退作业 `337745b1-c4e4-4832-af10-0f4ca3d1c70e`，两者均成功。回退后选中原安装代，GUI 合并文件的 SHA-256 保持一致。服务重启后安装状态和作业记录保持一致；[最终机器记录](2026-09-30-winmerge-publication.json)显示 7 款 Windows 应用均已安装，商店与兼容层服务运行，作业均为终态。

`cargo test -p forge-store-core --test candidate_catalog --locked` 通过，验证 v7 签名目录、验收收据及旧版本回退保护。WinMerge 的 CompatForge 应用定义目前注册在这台验收虚拟机；全新机器的配方注册与商店安装尚未验证。也未验证虚拟机内直接在线下载、跨版本升级或远程公共市场分发。持续队列见[进度账本](../windows-1000-progress.json)。
