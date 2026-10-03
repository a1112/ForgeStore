# ForgeOS 测试系统启动恢复（2026-10-03）

按用户“重启下系统”的要求，恢复当前 noVNC 所对应的隔离 ForgeOS 应用测试系统。外层 Hyper-V 虚拟机仍在运行，内层 QEMU 已不存在，SSH 2254 拒绝连接；日志显示旧启动脚本要求 40 GiB overlay，因此拒绝已按现有 [扩容记录](evidence/2026-10-03-rolling-capacity.json)批准到 128 GiB 的原磁盘。

旧私有启动脚本 SHA-256 为 `8b7172764afd6c4b1c29ba19cd124710ce59402cb97c7cc5eefea513fc43b909`。只替换两处旧容量断言/日志数值并更新所引用的扩容证据日期；原磁盘身份、20 GiB 基础镜像 pin、完整 backing chain 与原 QEMU 命令检查均保留。修正脚本 SHA-256 为 `3294b2f02c6a7e3d06cbb45ad418e234b9543675684a29091cde01ef05e9fe25`。原启动脚本入口同步修正，旧字节另作私有 0600 备份；磁盘没有重建或再次扩容。

新启动服务 `forge-windows-rolling-vm-20261003-capacity128.service` active，原 instance `d37397c7-dcee-4d83-b6d2-3b7628de1cba` 的新 run 为 `run-2a528af69f134ebd99513b682b6e5412`。只读 QMP 检查 status=running、KVM enabled，base/base-file block 节点仍只读；原 localhost SSH 转发保持。noVNC 服务 active，本地 6096 隐藏 SSH 转发恢复，页面 HTTP 200 且含正确 noVNC 内容。这些是服务可用性证据，没有冒称真实桌面画面已经观察。

系统启动后，Core 用户服务因桌面用户会话尚未启动而 inactive，首次 snapshot 出现 socket ENOENT；Store active。启动既有 Core/Store 用户服务，并保留使用现有密钥认证的后台用户会话以维持用户服务。没有改变密码、登录配置或 linger 配置。两服务随后 active，客体 boot ID 为 `3d9454c0-7f84-4df4-aad3-9525b5a89648`。

验收证据证明：**150 条完整终态 Core 作业、57 条完整 Store 记录、22 项市场目录、全部原应用代次与 OpenSCAD 安装代次保持**。完整市场 snapshot、TUF 信任文档及签名、原配置摘要、服务约束保持；24 个签名目录源文件、JPEGView 两套输入/输出和 OpenSCAD 程序/中文模型输入 pin 保持，OpenSCAD output 仍为空。latestKnownTime 从 `2026-10-03T02:43:42.44391294Z` 单调推进到 `2026-10-03T10:54:36.186001913Z`，可信根 SHA-256 仍为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。

客体停机时未能取得新的重启前快照，因此上述保持声明明确对照既有 JPEGView 最终快照，加上 OpenSCAD 成功安装作业及代次记录；不冒称取得本次即时前后快照。首次连接拒绝、旧容量检查失败、首次 Core socket 不存在均保留，不清除原失败历史。

累计仍为 **20 / 1000**，11 项阻塞候选不变。OpenSCAD 仅已安装，GUI 版本/许可、建模、保存、渲染、STL 导出、新进程重开及市场生命周期均待验收；GUI 工具网址识别故障未由本次启动操作验证解决。Windows 主机和外层构建虚拟机没有重启，密码、安装包及签名密钥不入库。

[完整启动与保持证据](evidence/2026-10-03-system-boot-capacity128.json)

只读审查发现首次恢复 helper 只核对 JPEGView 两份输出 PNG，遗漏两份输入 BMP；已另行实查原始及市场输入的长度与 SHA-256，分别为 192,054 字节 / `c64eba6205f507b0d573a34f995809d64f2742dbf57540adce2714b48bdf454f` 与 259,254 字节 / `84e2023c5a5c11c5d9274b75d71b26b9e10bf740823eb5f7656494377588cad8`，均匹配既有夹具 pin。完整结果与补充 helper 摘要记录在恢复证据中，原遗漏保留为审查修正记录。
