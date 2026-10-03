# Windows 滚动适配：Qalculate! 5.12.0 与托管 MSI 修复

2026-10-03，原 ForgeOS v12 隔离虚拟机、Wine 11.14。本轮新增 **1 款，累计 16/1000**。Qt、GTK、命令行同属一个应用，只计一次。11 个原候选继续保留，其中 Audacity 待逐应用复测。统一服务端仍为设计稿。

## 来源与许可

官方固定 [v5.12.0 x64 MSI](https://github.com/Qalculate/libqalculate/releases/download/v5.12.0/qalculate-5.12.0-x64.msi)，70,448,128 字节，SHA-256 `f677d8c3c63c7757e6efc7bee7d4ccded725437194dce94485fc3fb713692e25`。该摘要来自官方 HTTPS 资产的实际下载，未声称与上游公布校验值一致，也未验证 Authenticode 或可复现构建。

固定 libqalculate 5.12.0 源包 Calculator.cc 与 Qt v5.12.0 src/main.cpp 支持 GPL-2.0-or-later；安装根 COPYING 为 GPL v2 文本，与固定源文件有格式及地址差异，不能宣称字节或仅换行相同。记录了 99 份附带组件声明的摘要，组件适用各自许可；没有完成全部组件源码重建审计。详见 [来源](evidence/2026-10-03-qalculate-source.json)和[原代签名验收凭据](evidence/windows-qalculate-20261003.acceptance.json)。公开目录仅含 JSON，不包含安装器和签名私钥。

## 托管 MSI 与市场修复

Core 的新路径将 MSI 与 256 MiB PE 检查分开：MSI 上限 2 GiB，流式校验长度/摘要，检查 Installer CFB 类型和有界 Template Summary 架构。CFB 检查不等于完整 MSI 数据库或自定义动作审计。包和固定 Wine msiexec 工具通过持有的目录/文件句柄存储、复核和执行，保留代租约、取消、截止时间、输出限制和进程树清理。根进程退出 0 后继续等待 Wine 空闲；预期启动器校验通过才进入 ready。原 EXE 和历史 runtime binding 序列化不变。

只允许封闭 msiexec 安装行为，禁止原始参数、transform、响应文件、远程包及重启。修正原属性值中的字面引号；Wine 11.14 实测空格目录覆盖会把外层 argv 引号留在属性名中，因此目录覆盖暂限无空格 ASCII C: 路径。MSI 默认 `Program Files` 目的地正常。v9 的严格规则曾导致旧失败代不可读，启动失败后恢复 v8；v10 仅放宽 Failed/Cancelled 历史元数据读取。新注册、安装请求、ready 和回滚仍严格拒绝空格覆盖，旧记录摘要没有重写。

ForgeStore 首次 update 因缓存别名仅允许 EXE 后缀而失败；现允许 MSI，其他文件名、路径、SHA、大小、下载规则和 **1 GiB Store 上限保持不变**，bytes 仍为 1.11.1。JASP 包约 1.30 GB，仍超过该市场上限，不因 Core MSI 实现就计为通过。

Linux v10 的 debug/domain/guest-artifact/inspect/orchestrator/process/service 回归、五个受影响 crate 的 Clippy `--all-targets -D warnings`、13 项 Python 合同检查及 release 构建通过；process 单测 114 项通过。Windows v10 domain/service 合同通过，既有其他 Windows 回归保留；Windows 测试不等于 Windows 客体 MSI 安装。Store Windows 全包测试与 Linux release 全包测试/构建通过。测试使用 Windows Rust 1.94、Linux Rust 1.98，未验证声明的 1.78 最低版本。相关真实红灯和失败构建保存，见[回归与部署证据](evidence/2026-10-03-managed-msi-regression.json)。

运行 Core v10 摘要 `feaad5a5010fbf1506f2e4771bb7677345ccbba757b5d4aa97b65be2b502ce0a`，源码归档 `c85ce25943e2c086badcf9275d488f91eb56be6c99500bcc6cf992f9cfae867a`；Store MSI v2 摘要 `152e111f252393b2721d14f100df9bd1a9918fcf34f411e4ab3de5b1376c369f`，源码归档 `b8f43b8fb61568ac84158c9229d4ba556bc97cc2d880f82c23d881bf66f57004`。归档先于本报告等文档更新及 Store 测试的 rustfmt 排版；生产实现代码没有随后变更。旧二进制、drop-in、原配置和失败记录保留。原授权密码用于登录，认证设置没有修改，密码没有入库。

## 原代真实 GUI 验收

默认安装 `job-1790982218603-1` 成功，原代 `gen-job-1790982218603-1` ready，启动器 `Program Files/Qalculate/qalculate-qt.exe` 摘要 `f69e2a26d134421763442d7b721a5df5f5a005e19a495afe8679efd9425e6167`。这里原代安装/GUI 用的是 Core v7，签名凭据保持该事实，未改写为 v10。

GUI 计算 `12345 × 6789 + 10 = 83810215`，转换 `100 cm to m = 1 m`，从保存变量对话框关闭 Temporary，持久保存 `qaforge`。正常退出后新进程求值 `qaforge = 83810215`。应用原生 `definitions/variables.xml` 为 148 字节，SHA-256 `c631e98dd302ac0fe44a0ec6f6462f095525dbfd07f87f50d5fc4ff269c8aa63`；XML 使用 `r:qaforge` 名称。两次 GUI 正常退出 0，文件字节不变。没有通过显式文件选择器做导入/导出。见[后端证据](evidence/2026-10-03-qalculate-backend.json)及原代截图。

## 已签名本地市场发布与生命周期

candidate-v16 激活，可信根仍为版本 1、SHA-256 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`；timestamp/snapshot/targets 版本 25。旧目标 pin 保留，本地 tuftool 0.17 从既有根验签通过。20 份公开 JSON 含 14 份验收凭据和 18 条目录项：16 款 Windows 应用、2 条内部 fixture。签名原代凭据 SHA-256 `24433e70d6f6cba2d606f66caafb71108caa3c531a9ee0dd22c2701f0c8d0f66` 保持不变，后续生命周期另记。

第一次市场 update `fe65ba4c-3ced-48c1-86f5-61351702912a` 失败记录保留，尚未创建 Core 安装作业。修复后的 update `69fb3e12-f66e-4da9-b957-615cb3603198` 成功，新代 `gen-job-1790984639032-1` ready。新代 GUI 前变量文件不存在，没有复制原代变量作为输入。GUI 作业 `job-1790984656448-2` 计算 `12345 × 6789 + 11 = 83810216`，保存持久 `qamarket`，正常退出。新进程 `job-1790985017931-3` 求值相同，正常退出。149 字节 XML 摘要 `903db8442b369d8fdd120aca3c1734904eabc30a280fc986f73b455c3d55a74c` 不变。

市场 rollback `67d00543-824e-47aa-a003-a9eba2f7cc12` 成功。现有市场规则选择“最新其他 ready 代”，故实际选中无空格覆盖 canary `gen-job-1790983535431-1`，并非最初原代。新进程 `job-1790985085403-4` 计算结果 83810215，正常退出；此 canary 此前只有安装证据，本次才增加有限 GUI 算术证据。随后通过 Core `applications.rollback` 明确恢复原代，新进程 `job-1790985138059-5` 读取 `qaforge = 83810215` 并正常退出，原 XML 摘要不变。三个 ready 代及五个失败代全部保留，不把每代重复计数。

受控重启空闲 Core 和 Store 后：117 条 Core 作业、49 条市场全字段记录、所有代/选择、安装状态和目录均保持不变；本轮前 112 条 Core 作业、46 条市场全字段记录逐项一致。46 条旧市场记录规范化摘要仍为 `e137eb4fe2c64c1f2b84ef4594ab3380a6ec409d8ee919ba70f43768fe3c4238`。两个变量文件及 20 份公开签名目录文件字节不变，原 Core 配置摘要不变，Store 原保护设置保持。Core 原 unit 没有这组 Store 保护指令，不能把 Store 的保护宣称为 Core 已启用。

TUF 缓存 root.json/targets.json 重启后序列化字节不同，但完整 JSON 内容及签名逐项等于部署前快照；可信时间从 `2026-10-02T23:42:49.448771883Z` 单调增至 `2026-10-02T23:54:33.507350537Z`。没有回退、重置或跳过信任验证。初次原始缓存哈希断言失败、错误保护指令断言和 consistent-snapshot 路径断言均保存。详见[发布证据](evidence/2026-10-03-qalculate-publication.json)与[后续失败记录](evidence/2026-10-03-qalculate-lifecycle-failures.json)。

## 范围和下一轮

验收覆盖 Qt 算术、单位转换和应用自身持久变量文件保存/新进程加载。CSV、显式文件选择器、矩阵、复杂函数、绘图、GTK/qalc 前端等未验收。市场验证的是已核验缓存的同版本更新和安装代回滚；客体在线下载、新机 recipe 注册、跨版本升级及远程公开安装器分发未验证。公开 Git 推送发布代码和证据，不能等同于远程市场上线。

五次错误引号/空格安装完整保留，退出 67/103 是 Unix Wine 状态，未映射为完整 Win32 MSI 或 NTSTATUS。GUI 修饰键输入和 XML 名称误读也保留，[原失败证据](evidence/2026-10-03-qalculate-failures.json)不能由后续成功覆盖。Store 第一次 Linux 归档误规范化签名 JSON 导致校验失败，v2 保留原始签名文件字节后全套通过，没有放宽信任。

下一轮优先 Audacity 4.0.0 固定官方 MSI，逐应用核对许可和 recipe，做真实音频保存/重新打开及市场生命周期；其余阻塞候选继续保留。托管 MSI 基础能力通过一个 canary 不等于所有 MSI 已适配，也不等于 1000 款目标已完成。
