# Windows 滚动适配：LazPaint 7.3（已安装，GUI 待验收）

本轮增加一个已安装候选，累计验收仍为 **23/1000**。LazPaint 尚未启动图形界面，未进行绘画、保存、导出或重开，也未进入本地签名市场。原有 12 个阻塞候选和失败记录保持不变，待执行 GUI 候选为 1 个。

采用 [LazPaint 官方 v7.3 发布](https://github.com/bgrabitmap/lazpaint/releases/tag/v7.3) 的 `lazpaint7.3_setup_win32_win64.exe`，14153628 字节，固定 SHA-256 `938fd2694247369f28fa219a8cf4b8c8a468c8691697e8c0519191ba2ecd3459`。摘要来自官方 HTTPS 下载字节的本地计算，客体接收后独立核对；该历史 GitHub Windows 资产没有公布摘要，不能声称已匹配官方 SHA-256。

源码固定到 `a11930b418c7d9050886bd7d321fe78acfcd7ea6`，About 源码具有 `GPL-3.0-only` SPDX 声明，COPYING 为 GPL v3。固定 Inno Setup 脚本声明版本 7.3，并在静默安装时跳过应用启动。完整组件许可闭包、Authenticode、源码到二进制的可重复构建均未验收。见[来源记录](evidence/2026-10-08-lazpaint-source.json)及[配方](evidence/2026-10-08-lazpaint-recipe.json)。

通过 CompatForge 实际提交安装任务 `job-1791421778900-1`，终态 `succeeded`、退出码 0，选中就绪代次 `gen-job-1791421778900-1`。安装目录为瓶内 `C:\LazPaint`，117 个文件；主程序为 AMD64，11631104 字节，SHA-256 `a59b76180412579bef367e1f678680067912a155b16c00649c86f9e6746592d6`，与管理代次中的 launcher 摘要相同。安装的 readme 也声明 GPLv3。见[实际安装与文件核对](evidence/2026-10-08-lazpaint-installed-pending.json)。

完整检查点确认新增核心任务只有这一条安装，总计 178 条且全部终态；原 177 条任务的完整字段、63 条市场完整记录、全部先前配置和代次、MPC-HC 两代 PNG、签名信任及不自动息屏设置保持完整，信任时间单调。原 root 1、在线角色 33 与 `candidate-v24` 未改变。检查点保存在测试客体私有目录，后续只读维护脚本以该完整状态为基线。见[运行时保留证明](evidence/2026-10-08-lazpaint-runtime-preservation.json)。

原虚拟机 noVNC 隧道恢复后仍显示 Plasma 锁屏，与已报告状态相同；此前手动解锁请求仍待回复，本轮没有再次尝试输入凭据或修改认证。锁屏观察不属于 LazPaint 验收。后续需实际 GUI 加载和编辑图像、中文文件名保存、PNG 导出、新进程重开，以及签名市场更新、回退与持久化核验。

Ubuntu 仍独立累计两项验收，Flatpak 缓存及两次超时保留，本轮未启动 Ubuntu 虚拟机或重复其下载。外层仍只运行原 4 GiB Windows 兼容虚拟机，无交换空间；产品代码、签名历史和现有市场未修改。
