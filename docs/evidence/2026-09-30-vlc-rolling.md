# Windows 滚动适配：VLC media player 3.0.23

隔离 ForgeOS v12 虚拟机（Wine 11.14）已完成 VLC 的官方安装包校验、WoW64 安装、真实 GUI 视频播放、签名验收及本地 ForgeStore 上架。独立 Windows 应用累计 **5/1000**。这是本地市场候选目录，不代表远程公共市场已发布。

## 固定输入与功能证据

- [VideoLAN 3.0.23 x64 官方目录](https://downloads.videolan.org/videolan/vlc/3.0.23/win64/)中的 `vlc-3.0.23-win64.exe` 大小 45,948,080 字节，SHA-256 `20ad191348684b470ddc4e05204316f3d8e39655f412b3e392a0eef97639daaf`；[官方校验文件](https://downloads.videolan.org/videolan/vlc/3.0.23/win64/vlc-3.0.23-win64.exe.sha256)一致。安装器是 PE32 NSIS 引导程序，安装后的 `vlc.exe` 为 PE32+ x86-64。VideoLAN [许可证说明](https://images.videolan.org/press/lgpl-modules.html)指出播放器为 GPL-2.0-or-later；捆绑组件遵循各自许可告知。
- CompatForge 安装作业 `job-1790700543879-5` 成功，GUI 启动作业 `job-1790700619275-6` 在正常关闭后成功。安装的播放器摘要为 `bfa5740f028a8f310ca01c779bbc5f2f1b435c41539c54de592f715bb0c10389`。
- 用系统 FFmpeg 自制 640×360 MPEG-4 视频与 440 Hz MP3 音频的 20.041667 秒 AVI 文件，大小 4,442,000 字节，SHA-256 `1bf2987304d2827b75a44a1ca02170d40ad03b2eace9212bc24ef22c6d7af8ac`。通过 VLC 的 GUI 文件选择器打开后，画面正常显示并由约 2 秒推进到 18 秒；见[首帧截图](2026-09-30-vlc-gui.jpg)和[后续进度截图](2026-09-30-vlc-gui-progress.jpg)。未独立采集音频输出，因此只将可见的视频播放确认为已验证功能。
- [验收收据](windows-vlc-20260930.acceptance.json)记录精确安装包、作业、媒体样本和截图摘要。

## 签名市场与更新回退

[`candidate-v5`](../../catalogue/candidate-v5/tuf/metadata/14.targets.json) 的 TUF targets/snapshot/timestamp 版本为 14，沿用可信根 SHA-256 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。目录 target SHA-256 `2631cc05a8dd0d2e8212b2c6276bb49f481d0ff38c6cbb8d97c7bdc04438a736`；VLC 验收收据作为独立签名 target，SHA-256 `27755317c7192d300389d19adb9087f3f5c642a67633f3d3baafea49d0d6f710`。旧四款的收据仍在签名目录中，私钥始终保留在 Git 仓库外。

隔离虚拟机加载新目录后，前四款的安装状态与历史商店作业保持不变。商店[已安装页](2026-09-30-vlc-market.jpg)显示 VLC 3.0.23，[详情页](2026-09-30-vlc-market-detail.jpg)显示许可证、版本、来源、`tested` 与签名收据引用。[最终机器记录](2026-09-30-vlc-publication.json)显示目录共七项，其中五款 Windows 应用均已安装，商店与 CompatForge 服务正常运行，全部作业为终态。

商店首次在线更新作业 `a7500443-fa6c-4eae-a2fa-30b0932a7f1d` 失败，报错 `artifact source redirected or returned HTTP error`。官方 `get.videolan.org` 当前将安装包跳转到外部镜像，而 ForgeStore 的固定来源跳转策略仅允许同一主机，故 **在线下载未通过**。没有放宽来源校验；将从官方目录取得且再次核验摘要的同一安装包放入用户缓存后，商店重试作业 `d8c077b7-fcf4-45ca-b61d-82f17977b835` 成功，回退作业 `b4aa3032-d633-427d-b8f4-de15ce0b731f` 成功，原样本文件 SHA-256 保持一致。后续需评审 VideoLAN 镜像跳转的受限来源策略，并单独复测在线下载。

`cargo test -p forge-store-core --test candidate_catalog --locked` 通过，验证了 v5 的签名收据、目录和回退保护。未进行跨版本升级、整机重启或远程公共市场分发验证。持续队列见[进度账本](../windows-1000-progress.json)。
