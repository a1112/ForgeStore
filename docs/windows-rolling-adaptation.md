# Windows 应用滚动适配流程

本流程面向 ForgeOS 上通过 CompatForge 运行、通过 ForgeStore 分发的 Windows 应用。当前本地市场已验收 7-Zip、Notepad++、SumatraPDF、SQLiteStudio、VLC，共 5/1000 款。执行记录见 [2026-09-29 第一轮](evidence/2026-09-29-windows-rolling.md)、[SQLiteStudio 接续轮](evidence/2026-09-30-sqlitestudio-rolling.md)和 [VLC 接续轮](evidence/2026-09-30-vlc-rolling.md)；持续计数见[进度账本](windows-1000-progress.json)。

| 阶段 | 要保存的事实 | 进入下一阶段的条件 |
|---|---|---|
| 固定输入 | 应用 ID、版本、官方来源、许可、架构、文件名、大小、SHA-256、运行时版本 | 配方匹配精确安装包，不能用浮动 URL 替代固定摘要 |
| 安装 | 商店请求、作业 ID、终态、安装代和桌面入口 | 独立校验下载包；失败及缓存恢复分别记录 |
| 测试 | 窗口、实际文件操作、中文路径、重启和回退结果 | 不把安装成功、窗口存在或单元测试当作功能验收 |
| 修复 | 最小复现、失败测试、修改范围、复测 | 将问题交给所属仓库；保留原失败证据 |
| 记录 | 每应用、每版本的结果与限制，原始证据及摘要 | 可重现并可追溯到准确输入与测试环境 |
| 上架 | 审查后的兼容状态、证据引用、新签名元数据版本 | 签名、回退保护和真实市场安装/显示验证通过 |

`scripts/prepare_candidate_catalog.py` 仅生成待审查目录，Windows 应用默认 `unknown`。审核人在外部签名输入目录中补充经核对的 `compatibility.status/evidence`，再使用 `scripts/sign_candidate_catalog.py` 生成新的版本。测试通过只适用于记录中的应用、版本、安装包摘要和运行时组合。

使用已有私钥与可信根的正常签名流程，密钥始终保留在仓库外。仅把公开签名元数据、目录与验收记录纳入仓库。上架到本地预置目录和发布远程公共市场应分开记录。

当前 `sign_candidate_catalog.py` 只自动复制 catalogue target。验收收据须先作为独立 target 纳入签名输入，再将已签名的原始收据字节复制到公开 `targets/`，文件名使用 `<targets 元数据中的 SHA-256>.<收据文件名>`；逐项核对元数据中的长度与摘要，并通过 TUF 客户端读取验证。不要重新格式化或转换已签名文件的行尾。

每轮允许应用分别通过或阻塞；阻塞项保持原状态及原因，下一轮从缺失证据继续，不抹去失败作业、不复用其他版本的测试日期。
