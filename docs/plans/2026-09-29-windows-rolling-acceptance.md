# Windows 应用滚动适配执行计划

**Goal:** 按安装、测试、修复、记录、市场核验闭环推进首批 Windows 应用。

**Architecture:** 使用现有 ForgeStore 签名目录、CompatForge 用户服务和隔离的 v12 验收虚拟机。运行时问题归 CompatForge，目录和商店问题归 ForgeStore。每轮固定应用版本及安装包摘要，保留失败证据，只将实际通过的范围写入记录。

**Tech Stack:** Rust、Python 标准库、TUF、Linux/QEMU、Wine、Qt/QML。

## 本轮范围与步骤

1. 核对已有签名目录、v12 虚拟机、应用安装状态与安装包 SHA-256。
2. 通过真实 ForgeStore socket 为 7-Zip 26.01、Notepad++ 8.9.6.2、SumatraPDF 3.6.1 执行缺失安装，保存请求及终态；已有安装先核对版本，避免覆盖用户数据。
3. 启动真实应用，核对窗口、中文文件操作、桌面入口和服务重启后的持久化；将自动化与人工 GUI 证据分开。
4. 对发现的问题先复现，再加回归测试、修复和复测。重点核对目录生成器是否无证据地继承历史 tested 标签。
5. 在 `docs/evidence/` 保存本轮逐应用结果、失败原因及后续队列，核验市场显示与签名目录。已有签名元数据不可原地篡改；需要变更时使用新单调版本签名。

## 验收命令

- `python -m unittest discover -s tests -p 'test_*.py'`
- 修改 Rust 时运行 `cargo test --workspace --locked`、`cargo fmt --all --check`。
- 在真实 Linux 测试 VM 中通过私有 socket 查询目录/作业/安装状态，检查实际进程和窗口。

## 发布边界

当前市场是本地预置的签名 ForgeStore 目录，不是公网市场。记录应分别说明已在目录中、安装成功、功能验证与本轮是否新增上架。下一批应用在版本、来源、配方、安装包和实际测试证据齐备后进入签名目录。
