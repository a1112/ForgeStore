# Windows 滚动适配：DB Browser for SQLite 3.13.1（2026-10-11）

本轮新增一款有限验收应用，Windows 累计 **28/1000**；既有 12 项阻塞保留。candidate-v29 本地市场 30 条目，其中两个夹具不计数。root 1、在线角色 38。Ubuntu 两款独立记账，本轮未切换虚拟机。

## 固定来源与许可

[官方固定 release](https://github.com/sqlitebrowser/sqlitebrowser/releases/tag/v3.13.1) MSI 为 19,857,408 字节，SHA-256 `d023d54b3a5db10c7e896089bb3dbe6e7f4bc4eaa9bbecb34ca414be5970f688`，与官方 release 正文摘要和客体实际字节一致；API asset digest 为空，不作该字段匹配声明。固定源码 commit `5d84f53f85570a31767548cded89c96de6ff70ae`，ZIP 5,396,873 字节，摘要 `d4f77fb61e848b55b3da582d8984a35bbb7dcea8ca77a42acf33f2dcbb3b41d9`。

固定根 LICENSE 核实主程序 `MPL-2.0 OR GPL-3.0-or-later`；GPL、MPL、MIT 和插件许可分别阅读。安装版 `licenses/LICENSE` 为 607 字节，固定根为 598 字节，仅九处 CRLF→LF 标准化后同文，原字节不同。插件许可注明项目特许图标为 CC BY-SA 4.0，其他再分发者可能需 CC BY-NC-SA 4.0；完整组件及图标再分发闭包未验收。本地条目引用官方 URL，不将主程序许可当作整个捆绑包的许可闭包。捆绑 SQLCipher 未测试、未单独计数。Authenticode 与可复现构建未核验。

同一固定 MSI 检查器完整读取六张安装表成功，标准 managed MSI install 成功 exit 0，没有手工解包替代安装。GUI About 显示 3.13.1、build 5d84f53、x86_64、Qt 5.15.2、SQLite 3.46.1。

## 实际 GUI 验收

原安装 `job-1791686327258-1`，代次 `gen-job-1791686327258-1`。原生文件选择器打开中文名输入数据库，经 File / Save Database As 另存到 ASCII 输出路径，再通过 Execute SQL 执行 UPDATE、INSERT SELECT、SELECT，点击 Write Changes。原输出 8192 字节，SHA-256 `4244e2645b4696f5a6770acd740d56e1873c4a34b12f15973540b1d946c0aa46`；原输入摘要未变。新进程显式重开，Browse Data 显示原中文记录、更新记录和新插入记录共三条。只读 SQLite 核对精确记录、表结构及 `integrity_check=ok`，没有通过脚本写入 GUI 输出。

Store IPC 预缓存同版本更新任务 `9a5aad8b-efef-4f34-8593-b9761360b81a` 创建独立市场代次 `gen-job-1791687174079-4`，用独立中文名夹具重复同流程。市场输出 8192 字节，SHA-256 `74f8bf8fb82cc0e34df5ac05c65616609b7779ffe78ba78ca6d4ade43027b04b`；新进程重开显示市场中文记录、passed/3 和 market/42，共三条，完整性通过。输入同样未改。

每个测试进程启动时出现 `EnableNonClientDpiScaling() failed (120, Call not implemented)`，手动正常确认后完成有限流程。该警告未修复，无人值守启动未验收。SQL 输入助手把括号误输为数字，执行前已观察并用 GUI 改为 INSERT SELECT，错误 SQL 未执行；该记录与许可断言、错误许可路径均保留在失败证据。

## 市场、回滚与恢复

原 GUI 证明、来源和截图在市场执行前冻结，其待执行历史标签保留；后续完成单独记录。tuftool 0.17 明确信任旧 root 验证目录、证明及 MSI，无 unsafe 或过期绕过参数。root 字节不变，旧非目录目标摘要继承，角色 32 原 MPC 证明、33 更正证明及历代目录未改。

精确原代次回滚任务 `7726c98f-5542-4bb1-bfdd-822424463a9f` 成功，两个代次 ready；GUI 再显式重开原输出后正常退出。更新、回滚使用 Store IPC，未声称 GUI 按钮功能已验收。市场 GUI 实际显示版本、主程序许可、tested、证明引用、启动 DPI 提示及已安装行和回退控件。

全部任务终态后正常重启 Core/Store：212 条 Core 和 73 条 Store 完整字段、市场快照、签名信任、所有配置和代次保持，known time 单调推进，旧 205/71 字段逐项保留。既有应用输出、本轮两代次输入/输出/启动器摘要和不自动息屏配置核对通过。账号认证及锁屏设置未修改。客体仍使用已固定 managed-msi-v10 Core、managed-msi-v2 Store；源码 Provider v2 未部署或 GUI 验收。

私有完整检查点 `dbbrowser-20261011/after-restart.json` 不入库，只读核对助手 `L:/project/FOS/.transfer/rolling-current-state-dbbrowser-20261011.py`；旧固定总数助手仅代表历史。签名密钥、密码、凭据、安装包与私有完整检查点不入库。

## 验收界限与证据

仅两份 8192 字节 SQLite 夹具、中文输入文件名和内容、ASCII 输出路径、上述 SQL 与保存重开流程。直接中文键盘输入、大数据库、其他表结构、导入导出、加密、远程连接、插件、高级编辑、完整数据库编辑器、客体联网下载、新机器注册、配置迁移、跨版本升级均未验收。

- [冻结 GUI 证明](evidence/windows-dbbrowser-20261011.acceptance.json)
- [固定来源](evidence/2026-10-11-dbbrowser-source.json)
- [后续市场发布](evidence/2026-10-11-dbbrowser-publication.json)
- [失败记录](evidence/2026-10-11-dbbrowser-failures.json)
- [最终验证](evidence/2026-10-11-dbbrowser-final-verification.json)
- [独立审查](evidence/2026-10-11-dbbrowser-review.json)
- [市场详情截图](evidence/2026-10-11-dbbrowser/store-details.jpg)

回滚前检查点助手因全局函数别名复用递归而失败，顺序工具调用未在失败后停止，已提交回滚成功。未补造 `before-rollback.json`；修复为独立别名后直接核对回滚后的 211/73 状态，与激活后 208/71 旧完整字段比较一致，更新任务和安装后观察一致。未验证 211/72 完整回滚前后检查点精确对比，该证据缺口明确保留；正常重启前后 212/73 完整检查点精确比较已通过。
