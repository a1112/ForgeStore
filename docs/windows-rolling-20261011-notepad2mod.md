# Windows 滚动适配：Notepad2-mod 4.2.25.998（2026-10-11）

本轮新增 1 款有限验收应用，Windows 累计 **27/1000**，既有 12 项阻塞候选保留。candidate-v28 本地市场含 29 条目，其中 2 个测试夹具不计数；信任 root 1，targets/snapshot/timestamp 37。Ubuntu 两款验收独立记账，本轮没有切换虚拟机或增加 Linux 数量。

## 固定来源和许可

安装包来自 [官方固定 release](https://github.com/XhmikosR/notepad2-mod/releases/tag/4.2.25.998)，源码锁定 commit `62db550916cefcb5f21dbb27c8c8edd11695b23c`。安装器 1,449,064 字节，SHA-256 `ebd0bfd738ba9f07598369d09c91241807c94fe63f6629a0a9c66f2d66d3aed9`；固定源码 1,575,854 字节，SHA-256 `204519afe34b6b8f3db8037b82ede3f40dc7e75601e0bcd6e4f60464445c320a`。摘要是实际 HTTPS 字节计算值，未验证上游发布摘要、Authenticode 或可复现构建。

固定根目录 License.txt 的三条 BSD 条件和免责声明已核对，主程序标注 BSD-3-Clause。两代次已安装 License.txt 均与该根成员逐字节相同。distrib/license.txt 的年份/排版不同，Scintilla 许可已单独阅读；完整捆绑组件许可闭包未核验。实际 About 显示 `Notepad2-mod (64-bit) 4.2.25 r998 (62db550)`。Inno 安装器为 x86 包装，客体选择并安装 x64 主程序，两者不混淆。

安装参数采用 `/TASKS=` 取消默认可选任务，并固定独立瓶内目录 `C:\Notepad2mod`。安装日志成功，两代次系统注册表均没有 Notepad IFEO 替换节；现有账号认证和锁屏设置没有修改。

## 实际 GUI 工作流

原安装任务 `job-1791684570539-1` 与原代次 `gen-job-1791684570539-1` 安装成功。GUI 通过原生文件选择器打开 `中文验收.txt`，保留中文内容、UTF-8 BOM 和 CRLF，将 `before` 普通替换为 `after`，追加 ASCII `result=42`，经 File/Save As 保存到 ASCII 输出路径 `original-edited.txt`，正常关闭后在新进程显式重开。原输入 98 字节未修改；GUI 输出 108 字节，SHA-256 `1dcfa2db8acf2056c58a023e040ffabf91e54fbf31447de773fda47eb62b6023`。

Store IPC 提交预缓存同版本更新，市场任务 `ed6fd5bc-bfaf-4f8f-a3c4-b023fec7e292` 成功，创建独立代次 `gen-job-1791685142410-4`。该代次从独立 `市场验收.txt` 夹具执行 `pending` → `passed` 普通替换、ASCII 追加、GUI Save As 到 `market-edited.txt` 和新进程显式重开。输入 108 字节未修改；输出 118 字节，SHA-256 `c55caac4d58b224d7a683b942777916af940b1c0a4be1d16caaaf8ea03dff710`。两份输出都来自编辑器 GUI 保存，核对实际文件精确预期字节。

Store IPC 精确回滚任务 `e453301f-5b40-40c0-9a2f-bdec0ba62093` 成功重新选择原代次，两个代次均 ready；随后 GUI 再次显式重开原输出并正常退出。更新/回滚按钮仅观察到存在，未将 IPC 操作当作按钮功能验收。

## 签名、本地市场与恢复

原 GUI 证明与来源在市场工作流前冻结；其市场待执行历史标签保持原字节，后续发布单独记录。tuftool 0.17 使用明确旧信任根验证目录、证明和安装包，未使用 unsafe 或过期绕过选项。root 字节保持，所有旧 targets（目录自身除外）摘要继承，MPC-HC 角色 32 原证明和角色 33 更正证明及历代目录保留。

任务全部终态后正常重启 Core 与 Store，205 条核心任务及 71 条市场任务逐字段相同，市场完整快照、配置摘要、全部应用代次、签名信任文档保持；known time 单调推进。AkelPad、LazPaint、MPC-HC、Frhed 原输出和本轮双代次输入/输出/启动器摘要均核对。不自动息屏配置保持，原 Arch/Windows 实例运行，Ubuntu 实例保持关闭。客体 Core/Store 仍为此前固定 managed-msi-v10/v2；源码 Provider v2 未部署或 GUI 验收。

市场 GUI 实际显示 4.2.25.998、BSD-3-Clause、tested、签名证明引用及“高级编辑未验收”范围；已安装列表显示 Notepad2-mod 和回退控件。当前私有完整检查点为 `notepad2mod-20261011/after-restart.json`，完整内容不入库；新只读核对助手为 `L:/project/FOS/.transfer/rolling-current-state-notepad2mod-20261011.py`。旧固定总数脚本只代表历史。

## 界限与失败记录

本轮只验收两份小型中文名 UTF-8 BOM/CRLF 输入、普通替换、ASCII 追加、ASCII 输出路径和保存重开。直接中文键盘输入、大型文件、其他编码、语法功能、正则、插件、高级编辑、完整编辑器能力、客体联网下载、新机器注册、配置迁移、跨版本升级未验收。

匿名 GitHub API 限额导致 403，固定官方 release/codeload 下载成功；许可日志观察断言、生成助手缺失换行、浏览器控件焦点及异步终态检查问题保留在失败记录。没有确认产品缺陷，也没有修改产品代码、取消安装或重复安装。签名私钥、密码、凭据及私有完整检查点未加入仓库。

## 证据

- [冻结 GUI 证明](evidence/windows-notepad2mod-20261011.acceptance.json)
- [固定来源](evidence/2026-10-11-notepad2mod-source.json)
- [市场后续发布](evidence/2026-10-11-notepad2mod-publication.json)
- [失败记录](evidence/2026-10-11-notepad2mod-failures.json)
- [最终验证](evidence/2026-10-11-notepad2mod-final-verification.json)
- [独立审查](evidence/2026-10-11-notepad2mod-review.json)
- [市场详情截图](evidence/2026-10-11-notepad2mod/store-details.jpg)
