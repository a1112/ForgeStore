# ForgeStore bytes 安全更新与滚动环境复核

bytes 固定版本从 1.10.1 升至 1.11.1，修复 [GHSA-434x-w66g-qw3r / CVE-2026-25541](https://github.com/tokio-rs/bytes/security/advisories/GHSA-434x-w66g-qw3r)。依赖与锁文件仅改变这一包。上游修复 unique split owner 的容量加法溢出。

新增 release 回归先在 1.10.1 失败，再在 1.11.1 通过；测试只捕获 reserve 的容量溢出，不进行潜在越界写入。Linux release 全量测试 45 通过、0 失败、2 个既有环境依赖测试忽略。服务构建成功，新二进制 SHA-256 为 `857d88c90cbc4384885fa19da0ff25fd9579fb8e241214ed640b8f99072a1f75`。

新服务已在测试虚拟机运行，并经过一次有持久化前后快照的受控重启。15 项市场目录、15 项安装记录、42 条完整 SQLite 任务均保持一致。CompatForge 任务全部终态；原配置、服务防护、现有 catalogue-v13 与已发布信任根保持一致。旧服务二进制保留。

首次部署后的复合等值断言失败，其内存前后快照未落盘，因此无法证明最初失败的具体比较项。之后持久化复现显示，缓存 root/targets 的原始字节摘要改变，但解析后的完整 JSON（含 signed 与 signatures）相等，且逐项等于现有公开签名元数据；这说明缓存表示可能随重新序列化变化。此处没有宣称缓存字节保持不变，也没有重置 TUF 状态。信任根版本 1，timestamp/snapshot/targets 版本 22。复现时另一次读取服务命名空间外元数据路径失败，改为核对既有只读绑定源目录后通过。

初次离线构建缺缓存的失败也予以保留；使用专用 Cargo 缓存 `cargo fetch --locked` 后按锁文件离线测试和构建。没有修改签名密钥、密码或账户配置。

本次安全更新不新增应用计数：累计仍为 13/1000，阻塞 11 项。统一服务端仍处于已完成设计阶段。详细记录见 [安全更新证据](evidence/2026-10-03-bytes-security-update.json)。
