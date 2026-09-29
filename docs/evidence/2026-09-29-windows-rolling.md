# Windows 滚动适配：第一轮验收与本地市场发布

日期：2026-09-29。状态：首批三款应用的安装、生命周期、真实 GUI 文件操作、记录及本地签名市场目录更新完成。

## 环境与市场范围

使用既有隔离测试实例 `/srv/forge-apps-fast/lab/apps-v12-final`，保留原始镜像和用户 VM。ForgeStore 基线提交为 `22e745c`，运行服务对应 v12 收据中的二进制 SHA-256 `67fd7d618bb90939aa4d38bc98c446e1295b1430233e2e2d0fc9466a5cc9265f`；CompatForge CLI 为 `b0fb6755ec12d552a3846197281e982ccdbc761fffef0b3dfd30030e0f23e332`，所选 Wine 版本为 11.14。

三款应用原本已列入本地签名 `candidate-v2` 目录。本轮发布 `candidate-v3`，TUF targets/snapshot/timestamp 从 11 升至 12，将兼容证据更新为本轮签名验收收据。已在隔离 VM 的实际 ForgeStore 服务启用并通过 GUI 核验；没有发布公网市场，也没有替换默认镜像或增加新的应用种类。目录生成器修复已保存于源码，运行时二进制保持原版本。

初次锁屏期间的历史记录：[2026-09-29-windows-rolling.json](2026-09-29-windows-rolling.json)，其中未完成标记描述当时状态。当前结果见[签名验收收据](windows-rolling-20260929.acceptance.json)和[发布与最终核验记录](2026-09-29-windows-publication.json)。后者包含实际市场目录、安装状态、发布后作业及 GUI 正常退出状态。

## 本轮结果

| 应用 | 版本 | 安装 | 同版本新安装代 / 回退 | 启动探测 | GUI 文件功能 |
|---|---|---|---|---|---|
| 7-Zip | 26.01 | 保留原安装，重新校验缓存摘要 | 两项成功 | `7zfm.exe` 窗口及中文 ZIP 标题 | 通过 F5 解压；成员和实际文件 SHA-256 相同 |
| Notepad++ | 8.9.6.2 | 首次下载失败；验证缓存后商店重试成功 | 两项成功 | `notepad++.exe` 窗口及中文 TXT 标题 | GUI 输入标记并 Ctrl+S；磁盘内容完全匹配 |
| SumatraPDF | 3.6.1 | 商店下载、校验、安装成功 | 两项成功 | `sumatrapdf.exe` 窗口及中文 PDF 标题 | PDF 正文可见；Ctrl+S 另存副本摘要相同 |

六个 update/rollback 作业均为 `succeeded`。这里的 update 使用相同固定版本产生新安装代，不能据此声称不同版本间升级通过。重启 `forge-store.service` 后，安装状态及完整商店作业列表与重启前一致；没有执行本轮整机重启测试。

通过现有 `compatforge-cli desktop-launch APP main -- FILE` 顺序启动三个应用，均观察到匹配的 WM_CLASS、进程 ID 与中文文件标题。锁屏期间的三个启动探测作业通过 `jobs.cancel` 清理。用户解锁后另外启动三个 GUI 验收作业，真实执行文件操作并通过 Alt+F4 正常退出，均最终 `succeeded`。没有关闭其他用户任务。当前配置限制一个并行运行作业，因此逐应用执行；窗口属性与后续实际文件功能证据分开记账。

GUI 文件结果：Notepad++ 保存标记 `rolling-gui-save-20260929` 后 SHA-256 为 `1df10f59b71d455e24dd0607bd4ba398071aa2c7eb492e44ac7b2cb58f90d190`；7-Zip 解压出的 `资料/你好 100%.txt` 与 ZIP 成员均为 `e9de5fe7ed76059e3d3ac2c04126d13833691369c47fc587766a2217f68dae0c`；SumatraPDF 的 `rolling-copy.pdf` 与原文件均为 `7295797c972d8f64a1b2f562862d232ee877443f7c6526cd2547f6151db81499`。以上文件在发布及商店服务重启后再次核对一致。

Notepad++ 测试文件为 UTF-8-SIG（含 BOM）、LF 换行；其摘要覆盖实际文件字节，收据中的 `text` 为去除 BOM 后的解码文本。签名验收收据本身保留生成时的 CRLF 字节，Git 属性禁止行尾归一化；不要重新格式化已签名文件。

## 已发现与处理的问题

1. **目录生成器继承旧测试结论。** `prepare_candidate_catalog.py` 原先对任意安装包固定写入 `tested` 和 `2026-09-28-real-vm-gui`。新增未执行的 `99.0-unverified` 安装包回归用例，先观察测试失败，再将默认值修复为 `unknown` / `null`。后续签名前必须针对精确应用版本和安装包摘要人工审查真实证据，不能凭下载校验直接提升兼容状态。
2. **Notepad++ 在线下载失败。** 作业 `1a6b92de-6d21-4a8c-a4c7-372fb4582cee` 返回 GitHub HTTP request 错误，不能据此确定是 DNS、TLS 或其他网络原因。使用已有构建机缓存，先校验 6,898,288 字节及 SHA-256 `7c243203265ce8fdac76c839bf744ae35dcf620760eb97c2ea279af498560e45`，放入测试用户缓存后经商店 `retry` 产生 `65010d6b-0778-4183-a15e-55a7dc440294` 并成功。失败作业保留。该操作证明校验缓存的恢复安装路径，未修复或验证 GitHub 在线下载链路。

## 源码验证

`python -m unittest discover -s tests -p 'test_*.py' -v`：4 项，3 项通过，1 项因需要 Linux ForgeOS package tool 跳过。新增目录误标用例已确认修复前失败、修复后通过。

`cargo test --workspace --locked`：35 项通过、1 项在线下载测试默认忽略。新增 candidate-v3 验证在目录缺失时先失败，签名发布后通过；验证新目录及签名收据、逐应用版本/大小/摘要绑定，并拒绝回退 candidate-v2。`cargo fmt --all --check` 与 `git diff --check` 通过。`cargo clippy --workspace --all-targets --locked -- -D warnings` 未通过：未修改的 `trusted_catalogue.rs` 中 `TrustError::Tuf(tough::error::Error)` 导致两个 `result_large_err` 告警（至少 224 字节）；没有压制告警或声称 Clippy 通过。本轮未修改 Rust 产品实现或重新构建部署二进制。

## 签名与启用

- 原可信根 SHA-256 保持 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。
- 目录摘要：`6075952ba5f984f729c77e5435157d02701e564f73b2cbf8134d16089e4acac6`。
- 签名验收收据摘要：`31ea9f25cbf61539bebc75d49208f10d7cda73f77fad58f595e76138c98f1238`。收据作为独立 TUF target，三款应用的 evidence 分别引用对应 ID。
- 用户服务 drop-in `~/.config/systemd/user/forge-store.service.d/40-windows-rolling-catalogue.conf` 只将新目录的 metadata 和 targets 只读映射给实际商店服务。可信根仍来自镜像原路径，签名密钥未传入 VM；原始镜像和全局 `/usr` 内容未修改。
- 启用时原安装列表与全部历史作业保持一致。启用后的 7-Zip update `8274ca12-bba4-4969-8e78-e883a944cccf`、rollback `0fde2818-f847-4f6b-92f7-11f0e1b7fbdb` 再次成功；随后重启商店服务，新目录继续生效。
- [市场详情](rolling-market-v3-details.jpg)可见本轮证据引用；[已安装列表](rolling-market-v3-installed.jpg)可见三款准确版本。

恢复时保留已验收的 candidate-v3 签名文件及映射。已经接受版本 12 的 TUF 状态不会接受版本 11；若后续发布出错，应使用更高版本号修复，不能删除信任状态强行降级。将本目录纳入新默认镜像或发布公共远程市场属于后续发布范围。

## 下一轮队列

1. 独立复测 Notepad++ 官方 GitHub 在线下载，定位网络失败原因；本轮缓存恢复结论不替代在线链路验收。
2. 修复既有 `TrustError` 的严格 Clippy 告警，并分别验证 Windows/Linux 构建。
3. 有新版本或新应用时固定配方、安装包及运行时，重复完整闭环；另行补充跨版本升级和整机重启验收。

接续脚本暂存于工作区 `.transfer/rolling-*.py`；测试 VM 内 `/home/forge/forge-rolling-20260929/` 保留完整 before/after、重试、生命周期、启动与输入文件。公开机器记录已去除 SSH 凭据和无关用户内容。
