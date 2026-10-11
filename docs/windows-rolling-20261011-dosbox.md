# Windows 滚动适配：DOSBox 0.74-3（2026-10-11）

本轮新增一款有限验收应用，Windows 累计 **29/1000**，12 项既有阻塞保留。candidate-v30 共 31 条目，其中两个夹具不计数；本轮两个合成 COM 程序也不计入独立应用。root 1、在线角色 39。Ubuntu 两款独立记账，本轮未切换虚拟机。

## 固定来源与许可

[官方固定安装包](https://sourceforge.net/projects/dosbox/files/dosbox/0.74-3/DOSBox0.74-3-win32-installer.exe/download)为 1,493,703 字节，SHA-256 `a9e270217d12867c2609d9423d1b2ed83fcf7cd08aeeee3ab09a4afdf9e9418e`；[固定源码](https://sourceforge.net/projects/dosbox/files/dosbox/0.74-3/dosbox-0.74-3.tar.gz/download)为 1,326,339 字节，SHA-256 `c0d13dd7ed2ed363b68de615475781e891cd582e8162b5c3669137502222260a`，均与官方 SourceForge SHA-256 和实际下载字节相符。源码 configure.ac 为 0.74-3，src/dosbox.cpp 明确主程序 GPL-2.0-or-later。安装 COPYING.txt 18,332 字节，仅 CRLF→LF 标准化后与固定 COPYING 17,992 字节相同，原字节不同。完整 SDL、SDL_net、zmbv 及其他捆绑组件许可闭包未核验。归档检查器不存在，未验证安装器归档清单；实际固定 PE 与 NullsoftInst 标头核对，标准 managed NSIS 安装成功 exit 0，没有手工解包替代安装。Authenticode 和可复现构建未核验。

独立审查发现原来源把 JSON 转义误当作两个实际反斜杠。实际 `/D` 参数仅一个反斜杠、12 个 UTF-8 字节，十六进制 `2f443d433a5c444f53426f78`，definition 和 launchPlan 一致，安装到 drive_c/DOSBox。原来源、原收据及截图保持冻结原字节；另增更正来源和收据，市场引用更正收据。实际 recipe 和安装未改，定义摘要 `sha256:e4eaa0079ab076affeb0a957457e810e15a6aaf40a32ca14cfd1d4bf93e4ec7d` 保持。

## 实际 GUI 功能

原安装 `job-1791688839198-1`，代次 `gen-job-1791688839198-1`。DOSBox 窗口实际执行 `mount c c:\ForgeQA\dos`、切换 C、dir、COPY、verify.com、type result.txt、exit。原夹具输入 1515（21 与 21），181 字节 COM 通过 DOS int21 读取输入，在仿真 CPU 内相加、AAM 转十进制、显示 42，并创建和写入 RESULT.TXT。GUI COPY 另生成两字节 COPIED.BIN。结果 32 字节，SHA-256 `c88bfb8f2a4d706b49081bf8faf9c5a9c1548fe246f13a95adf61f989e1cf01b`。新进程重新挂载并显式 type 原结果，未重跑程序或通过脚本重造输出。正常退出 exit 0，输入与 COM 摘要不变。只读助手核对实际输出精确字节；独立审查对实际 COM 指令解释，确认静态文本为 00 而结果在运行时计算为 42。

Store IPC 预缓存同版本更新 `52feed5e-8e37-40e3-99d7-55fedc0e2b18` 创建独立市场代次 `gen-job-1791689893592-4`，输入 2802（40 与 2），重新计算全部地址和长度的 179 字节 COM，重复相同 GUI 文件流程。结果 30 字节，SHA-256 `c296561f457eda82c42685bb9c5a40003158c64223f8000360da1d8d42a1f65f`。新进程重挂载并显式重读，输入和输出字节核对一致。原与市场各一次小型合成程序是功能测试输入，不增加独立应用数量。原代次两次启动未观察到启动警告，仅表示观察结果，不声称完整无人值守启动验收。

## 市场生命周期与恢复

tuftool 0.17 明确信任旧 root，验证目录、更正收据和安装包，无 unsafe 或过期绕过。角色 39 继承所有既有非目录目标摘要，原 root 字节及角色 32 MPC 原证明、33 更正证明和全部目录保留。市场 GUI 显示版本、主程序许可、tested、证明引用、有限范围、已安装行与回退控件；更新与回滚实际通过 Store IPC，未声称 GUI 按钮功能已验收。

完整回滚前检查点保存并读回成功后才提交回滚 `b75b1a56-59de-4607-9084-076b69000fa4`。218 Core/74 Store 回滚前与 218/75 回滚后旧字段精确比较，恢复原代次，两个代次 ready；原输出再经新 GUI 进程重挂载并显式 type 后正常退出。所有任务终态后保存检查点，再正常重启 Core/Store，219 条 Core 与 75 条 Store 完整字段、市场快照、信任、配置、代次和输入输出逐项不变，known time 单调推进。不自动息屏配置保留，认证与锁屏设置未修改。旧 DB Browser 缺失回滚前检查点的记录保留，不将本轮成功用于补写历史。

回滚后检查助手首次错误地要求 IPC 快照包含全部旧任务摘要，因快照只保留最近 64 条，新增回滚摘要使最旧一条离开窗口而失败。失败后停止后续 GUI 与重启，使用完整 SQLite 行核对确认被移出的记录仍保留。修正助手为比较全部旧行、共同摘要字段及当前 64 条最高 sequence 的摘要 ID 后复核通过，再保存回滚后检查点并继续。无产品代码变更，无历史记录删除或缺失回滚前检查点；失败原文及原因保留在证据中。

私有完整检查点 `dosbox-20261011/after-restart.json`、九阶段完整归档不入库，只读助手 `L:/project/FOS/.transfer/rolling-current-state-dosbox-20261011.py`。旧 212/73 等固定总数助手仅代表历史。客体仍是 managed-msi-v10 Core 和 managed-msi-v2 Store，源码 Provider v2 未部署或 GUI 验收。签名私钥、密码、凭据、安装包、私有完整检查点均不入库。

## 界限与证据

仅两个很小的 ASCII 路径夹具和所述 DOS 文件/计算流程通过。中文路径与输入、其他 DOS 编码、DOSBox 全屏模式、音频、手柄、图形游戏、CD 镜像、联网、硬件兼容、大程序、完整模拟器、客体联网下载、新机器注册、配置迁移、跨版本升级均未验收。

- [更正后 GUI 收据](evidence/windows-dosbox-20261011-corrected.acceptance.json)
- [更正后来源](evidence/2026-10-11-dosbox-source-corrected.json)
- [冻结原收据](evidence/windows-dosbox-20261011.acceptance.json)
- [市场发布](evidence/2026-10-11-dosbox-publication.json)
- [失败与更正](evidence/2026-10-11-dosbox-failures.json)
- [最终验证](evidence/2026-10-11-dosbox-final-verification.json)
- [独立审查](evidence/2026-10-11-dosbox-review.json)
- [市场详情](evidence/2026-10-11-dosbox/store-details.jpg)
