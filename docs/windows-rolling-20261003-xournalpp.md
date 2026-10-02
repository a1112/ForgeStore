# Windows 滚动适配：Xournal++ 1.3.8

记录日期：2026-10-03。独立 Windows 应用累计 **13/1000**，明确阻塞候选仍为 **11**。当前本地市场为 `candidate-v13`，TUF timestamp/snapshot/targets **22**、root **1**，目录 15 项，其中 2 项为内部测试 fixture，不计入 Windows 完成数。

本轮自行使用已有授权凭据解锁原 Linux 测试会话，空闲锁屏后重复登录。没有移除密码，也没有修改认证、锁屏、SSH 或安全限制。外层 Hyper-V 网络地址变化后恢复原回环隧道；既有 QEMU 进程、40 GiB 覆盖盘和原实例保持不变。原服务上下文未修改。详见 [会话恢复证据](evidence/2026-10-03-rolling-session-recovery.json)。

## 来源、许可与安装修复

使用官方 [v1.3.8 固定发布](https://github.com/xournalpp/xournalpp/releases/tag/v1.3.8)，发布于 2026-09-26。安装包 `xournalpp-1.3.8-windows-setup-AMD64.exe` 为 **96,924,916 字节**，SHA-256：

```text
0524a354a2c2c680e9c4b3817a3fc9284b1e60760bac1a74c562701a4e355637
```

本机、构建机、来宾均核对该摘要，并与 GitHub release asset digest 一致。固定标签源码提交 `938bdb8de4d32f38f48dd7f6544886722157a848` 作为来源参考；未验证可复现构建。

许可采用 `GPL-2.0-or-later; bundled component notices apply`。固定源码的 AboutDialog.cpp 明确写出 “GNU GPLv2 or later”，安装后关于窗口显示同一许可声明和版本 1.3.8；固定 LICENSE 是 GPL v2 模板，不单独用模板推断 or-later。引用文件摘要见 [来源证据](evidence/2026-10-03-xournalpp-source.json)。未把安装包加入 Git。

首次安装任务 `job-1790963571997-7` 的安装进程退出 0，但配方指定的 `C:\Xournalpp` 没有生成，后置启动器校验失败，任务及代均保留为 failed。真实文件位于 `Program Files/Xournal++/bin/xournalpp.exe`，与固定 NSIS MultiUser 默认目录相符。配方改为仅 `/S`，启动器使用真实 bin 可执行文件；重装任务 `job-1790963642854-8` 成功。没有更改服务代码、Wine 运行时或保护条件。详见 [失败及修复记录](evidence/2026-10-03-xournalpp-failures.json) 和 [最终配方](evidence/2026-10-03-xournalpp-recipe.json)。

## 真实功能验收

原始受控 XOPP 输入只含两行 ASCII/中文文本，没有笔画。通过 GUI 绘制两条笔画、增加两个 ASCII 文本区域、另存为原生 XOPP、导出单页 PDF，查看关于窗口，正常退出，再打开保存文件并正常退出。原始输入摘要保持不变。

noVNC 面板改变尺寸后曾误触 LaTeX 工具，其缺少外部 LaTeX 的提示被取消，该功能不计通过。键入时实际产生文件名 `rrolling-xournalpp-output.xopp` 和两个文本区域 `gui pa` / `ssed`，记录保留实际名称及内容，没有在后台修饰输出。这些操作观察不作为应用缺陷断言。

| 文件 | 字节数 | 验证结果 |
| --- | ---: | --- |
| `rolling-xournalpp-input.xopp` | 330 | 原始输入未变，0 条笔画、2 个文本元素 |
| `rrolling-xournalpp-output.xopp` | 1,998 | GUI 保存，2 条笔画、4 个文本元素；正常退出后重开可见 |
| `rrolling-xournalpp-output_annotated.pdf` | 62,626 | 一页 A4 PDF；提取到原 ASCII/中文及新增文本；渲染检查可见中文和两条笔画 |

各文件及解压 XML 摘要见 [后台证据](evidence/2026-10-03-xournalpp-backend.json)。PDF 文本和页数见 [PDF 验证](evidence/2026-10-03-xournalpp-pdf-validation.json)。原始 [保存截图](evidence/2026-10-03-xournalpp-saved.jpg)、[重开截图](evidence/2026-10-03-xournalpp-reopened.jpg)、[关于截图](evidence/2026-10-03-xournalpp-about.jpg) 与 [PDF 渲染页](evidence/2026-10-03-xournalpp-pdf-render.png) 已保存。

## 签名及本地市场生命周期

验收收据为 [windows-xournalpp-20261003.acceptance.json](evidence/windows-xournalpp-20261003.acceptance.json)。`candidate-v13` 保留旧条目及旧验收目标原始字节，追加本轮条目；共有 11 个签名验收收据、17 个公开 TUF 文件。私钥仍在源码库外。使用 tuftool 0.17 从上一代 trusted-root 执行 `clone --metadata-only`，签名验证退出 0。

可信根字节及 SHA-256 保持：

```text
345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e
```

初次目录传输的模板替换误将指纹文件改为不存在的 `known_hosts_v13_final`，严格 SSH 校验拒绝传输；随后依赖的 stage 因缺少归档而失败，未生成来宾目录或改变信任状态。保留构建机归档并重新核对摘要后，使用原 `known_hosts_v12_final` 成功传输，没有跳过校验、重设指纹或降低信任版本。该工具失败记录在 [发布证据](evidence/2026-10-03-xournalpp-publication.json)。

| 阶段 | 任务 / 安装代 | 观察结果 |
| --- | --- | --- |
| 初次 GUI 会话 | `job-1790963701772-9` | 绘制、文本、原生保存、PDF 导出、正常退出成功 |
| 初次重开 | `job-1790964608177-10` | 原输出重开、正常退出成功 |
| 市场同版本更新 | `483d7d7b-8cac-44dc-b44a-bae5ebf20087` | succeeded，新代 `gen-job-1790964893638-11` |
| 更新代真实操作 | `job-1790964935622-12` | 先写入无笔画受控输入；GUI 绘制两条新笔画，保存和导出 PDF，正常退出 |
| 更新代重开 | `job-1790965113251-13` | 保存文档重开、正常退出成功 |
| 市场回滚 | `e0b62944-b809-486a-89f7-d7d0fec5c3c0` | succeeded，精确恢复 `gen-job-1790963642854-8` |
| 回滚代重开 | `job-1790965177044-14` | 原有输出摘要未变，GUI 重开、正常退出成功 |
| 服务重启 | 42 条完整 SQLite 任务 | 目录、安装状态、42 条任务重启前后相同；扣除本轮两条任务后原 40 条记录摘要不变 |

更新代的 XOPP 为 1,813 字节，含 2 条笔画与原 2 个文本元素；PDF 为 58,631 字节，单页并保留中文。更新代 [重开截图](evidence/2026-10-03-xournalpp-market-update-reopened.jpg)、[PDF 渲染页](evidence/2026-10-03-xournalpp-market-pdf-render.png) 和 [回滚重开截图](evidence/2026-10-03-xournalpp-market-rollback-reopened.jpg) 已保存。

最终所有 Core 任务均为终态；首次失败代仍为 failed，两次成功安装代仍为 ready，当前选择原成功代。服务重启后持久化的 TUF 为 root 1，其余三个角色均为 22。42 条完整市场任务的规范 JSON 摘要：

```text
77c42f18cd57fcc45d4d699c3bf0d6ecb7c9ef583c08d3f6250d555634de883f
```

## 范围与下一轮

计入完成的是本轮一个独立应用。11 个既有阻塞条目及失败记录保留。统一服务端文档仅同步现有数量与目录版本，仍是设计，没有声明服务端已实现或部署。

本轮验收覆盖笔画、ASCII 文本插入、现有中文显示及保存、原生文档重开、单页 PDF 导出、经验证缓存的同版本市场更新与安装代回滚。中文键盘输入、数位板/压感、音频、LaTeX、插件、复杂 PDF 编辑、来宾在线下载、新机器配方注册、跨版本升级及公开远程分发未验收。来宾原时钟显示 2026-10-02，原始任务时间戳保留，记录日期按本次客户端日期 2026-10-03；没有修改系统时钟。
