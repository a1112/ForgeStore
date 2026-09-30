# Windows 滚动适配：PortableApps Platform 待许可确认

2026-09-30 继续首批 1000 款滚动适配。累计已验收并激活本地签名市场的独立应用仍为 **7/1000**。本轮没有新增通过项，也没有修改 TUF 市场目录。

## 安装包与当前作业

[官方 30.4.1 发布说明](https://portableapps.com/news/2026-05-30--portableapps.com-platform-30.4.1-released)可核对版本。安装包来自项目官方 SourceForge 分发路径：`https://downloads.sourceforge.net/project/portableapps/PortableApps.com%20Platform/PortableApps.com_Platform_Setup_30.4.1.paf.exe`。重定向至 SourceForge 的 `master.dl.sourceforge.net` 镜像，文件为 7,424,056 字节，SHA-256 为 `f7ad3bb79472222a807b054cb7092c1cefcd3bdcd86d35a51244723c8df54562`，与 Mac-Win 固定配方一致。传入验收虚拟机后再次校验同一摘要。

已安装的 7-Zip 命令行可列出此 NSIS 安装包，包含 `PortableApps/PortableApps.com/PortableAppsPlatform.exe`、`Other/Source/License.txt` 和 `DirectoryWatchCopyrightAndLicense.txt`。[官方下载页](https://portableapps.com/download)说明主项目采用 GPL，并捆绑 MIT、CC 等组件；不能将这些信息简化为所有资源均具有同一种许可。

CompatForge 注册了此虚拟机专用的临时应用定义，安装作业为 `job-1790729328158-5`，安装代为 `gen-job-1790729328158-5`。安装参数 `/S` 和 `/D=C:\` 未抑制交互界面。通过 GUI 选择 English 并进入下一页后，安装器停在 **License Agreement / I Agree**。该页面说明 GPL/LGPL/BSD 等组件许可，并对商标、外观和图片的使用作出限制。

截至记录时，用户尚未针对该按钮确认，代理未点击 I Agree。作业仍在等待界面操作，不能视为安装成功或重复提交安装。[许可页截图](2026-09-30-portableapps-license.jpg)保留了现场；收到当前操作确认后应重新检查作业状态与窗口，再继续安装、真实功能验证和签名发布。预期启动路径尚未通过落盘验证。

## 连续测试环境与后续来源

本机至验收 VM 的 SSH 桌面转发已失效，恢复了仅监听 `127.0.0.1:6096`、连接构建机 `127.0.0.1:6093` 的现有通道，随后重新连接并解锁。CompatForge 与 ForgeStore 用户服务均为 active。

虚拟机根盘为 20 GiB，`/dev/vda2` 为 ext4，安装前约剩 1.17 GiB；RAM 为 3,902 MiB、无 swap。构建机承载 VM 的文件系统约剩 25 GiB，构建缓存文件系统约剩 9.6 GiB。扩容预检发现 guest 中没有 sudo/growpart，尚未执行磁盘或分区修改。后续大安装包及同版本市场更新需要先解决容量，不能无限提交新安装代。QMP 的 os 块状态仍保留 `nospace` 标记，但随后 `query-status` 返回 `status: running`、`running: true`；不能仅凭该块标记判断出现新故障。

[LTspice 官方页](https://www.analog.com/en/resources/design-tools-and-calculators/ltspice-simulator.html)当前列出 Windows x64 26.1.1。其配方使用的移动地址 `https://ltspice.analog.com/software/LTspice64.msi` 已下载核对：187,368,960 字节，Last-Modified 为 `Mon, 14 Sep 2026 21:50:42 GMT`，SHA-256 `249ebde3c84e01f4ce5b5ff78c6c7588f049bac85ae1635f968cfdbe4f4ecd03`。它与旧配方摘要 `485dabd2d7d8293de733a399719f6538efda4a54b48b181a14e07271186984d3` 不同，因此未传入 guest 或安装，也未继承旧 tested 信息。安装包内部版本、许可及新配方仍需审查。

Firefox 的首次运行确认仍保留为独立待办。以上候选均不计入已完成数。


Heartbeat capacity follow-up: job `job-1790729328158-5` was cancelled to release the sole global service slot. No I Agree acceptance occurred. Artifacts and prior license evidence retained; retry requires the pending user decision.
