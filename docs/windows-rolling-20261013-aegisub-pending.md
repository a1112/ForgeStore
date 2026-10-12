# Aegisub 3.2.2 Windows 首次启动失败（2026-10-13）

Windows 仍为 61/1000，新增第 17 个未解决候选。61 个 accepted 与旧 16 个 pending 全部字段保持。Aegisub 只有一次正式原安装和一次正式首次启动，未进入字幕编辑器；中文名 693 B 原始 ASS 输入已预置但未 GUI Open，市场 693 B 输入只在宿主准备。没有编辑、保存、新进程重开、独立市场安装或上架；合成 ASS 不计应用。

官方固定 v3.2.2 安装包 20891980 B，SHA256 19ae304c586f5d59171c44e89037006dda2c511c866fa9e453038eaadc58afcd；source 5252052 B，SHA256 c55e33945b82d8513c02ea6e782f0d72c726adcd3707e95b8c0022f6151e6885。两完整 HTTPS 200 GET 与 Content-Length 通过，安装包和源包的 MD5 分别匹配官方页面 3cb95e30c73b7a227a4590b0cd986ddb 与 d80e852c34811add358c06d77f5cd40d；MD5 不作强密码学真实性证明。仅本地 SHA256，未核上游 SHA/API digest、Authenticode/PGP、源二进制对应、可复现或最新。2264 个普通源成员中 21 个原字节保存，包和程序二进制不公开，完整响应与签名 redirect query 仅私有保存。

源码主 BSD3，文件级 ISC/MIT/MPL 例外保持。README 明确 Windows 二进制通常因依赖为 GPL，不能称整个 Windows 包 BSD3。安装 license.txt 与固定源 packages/win_installer/license.txt 1510 B 的全部原字节摘要相同，4739752067a1c832c14b7d520016c1ad65b978b867172409f27609a7c184aa21；完整 wxWidgets/Boost/MSVC/Lua/FFMS2/FFmpeg/字体/字典/图标/翻译许可闭包未核。固定原 fragment_setupbase.iss 有非 UTF8 字节 0xA9；严格解码失败保留，后续转义诊断不作原文替换或完整编码判定。

实际安装包有两处 Inno Setup Data 5.5.0(u)，PE bootstrap 为 0x14c；实际安装主 aegisub64.exe 独立 PE Machine 为 0x8664，20004352 B，SHA256 c3aa3cba4c7f5d21a17af4e8a00c38cbcb14c8f41620b3e254d2fb4a4d006565，不当整个应用大小。正式原生 Inno /VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /DIR=C:后单反斜杠Aegisub /NOICONS /LANG=en /TASKS=checkforupdates，DIR 为 15 UTF8 B，hex 2f4449523d433a5c41656769737562。原生更新任务保留；静态源 privileges/关联代码不证明真实 Windows UAC 或完整安装行为。

正式安装 job-1791838596481-1 成功，代次 gen-job-1791838596481-1，定义 sha256:66d8459d74cc3adccd2e17d1dd040a46cdc65ceb26dc4b06ff98a5e3726166e1。首次正式 job-1791838699015-2 原生 serious problem，完整实际 Wine 错误报告 10770 B，SHA256 dddb9513492b571dbd57826f38c371abbe5ef81f67956f8a5fcb9bc8d8945181。首行明确 msvcr120.dll 的 ?_NewCollection@_AsyncTaskCollection@details@Concurrency@@SAPEAV123@PEAV_CancellationTokenState@23@@Z 未实现，backtrace 含 kernelbase/msvcr120/aegisub64。这是实际错误详情，不是 wrapper stderr，也未完成应用源码调用链或完整根因定位。原生 Show Details/Save As，通过 C 盘/AegisubTests 树保存 aegisub-crash.txt，正常 Close 后 exit 1、wineserver 停止。两 JPEG 仅启动失败/错误详情，不含登录图。没有 GUI 编辑器、About、中文显示或任何字幕功能验收。

87 个安装程序文件跨失败与正常服务重启原字节保持；原输入 SHA256 91660e44920cb68340ee3e4367b9c694f15762927318c9730f61511c77c98058 保持，original.ass/original.srt 不存在。没有 Core/程序/globalcontext/正式定义/launcher 参数、environment/workingDirectory、新字体、DLL 或 runtime 修复。剪辑/时间轴/音视频/听音或设备/ASS SRT 导出/复杂格式/大坏文件/直接中文键盘/完整路径/打印/联网/迁移/无人值守均未验收，不称无后台联网。

system.reg 从 3931739 B 变为 3925198 B，user.reg 从 102820 B 变为 104213 B；安装后旧 registry 只有 pins，没有旧 raw，不能补造前后差分或声称旧字节保持/键值等价。先只读观察保存实际新 registry、输入/错误报告及选定 Aegisub hotkey/log/dump 私有原文，后以实际新 pins 保存读回完整失败 CP。选定 Roaming/Aegisub 配置只是失败后首次发现，不证明 GUI 前不存在或创建时间；Local/Aegisub 本轮没发现不作全 FS 排除。

484 Core 全任务、142 完整 Store SQLite 全字段的终态 CP 保存读回后，正常 Core/Store 各重启一次；七类状态全字段、新 pins、失败任务错误/时间/事件、全部旧历史/数据/程序/信任/正式配置/防护精确保持。当前 candidate-v67/root1/role76，63 条=61 应用+2 夹具、131 实际签名目标/72 公共 JSON；Aegisub 不在活动目录，本轮无 TUF 签名或激活、recover、VM 重启。当前完整私有 CP aegisub-20261013/after-final-restart.json SHA256 76142d7031bf05ad9b1fe79c7e98f088eb886ca80addea6dfe94bdb1411669af，不入库。当前只读 rolling-current-state-aegisub-pending-20261013.py 必须经 rolling-guest-20261010.py 严格 SSH 发送 stdin 执行，不要求 host helper 客体落盘；旧 Fityk482/142 helper 仅历史，不再强求旧数量。查询可写既有 scratch，最近64 IPC 不替代全 SQLite，核固定数据/87程序/三registry/selected 配置，不是全应用 FS。

原 URL quoted href 断言在任一包 GET 前停止；V2 按实际无引号 href 解析并复用原首页。缺失猜读助手、7z 不存在路径测试、过渡截图和原失败工具历史保持，不当完成检查或补造 raw。旧 Scribus 一份私有 canonicalV1 raw 丢失限制保持。Ubuntu KateAPT/Snap计算器两款独立记账，Flatpak 两次 runtime 下载超时未安装缓存保持，本轮未运行 Ubuntu VM。

公共写入前旧冻结3792证据原字节保持；完整repo5033tracked与1241范围外index/commit单独核对，不称全部工作区字节相同。Git预检、独立FINAL审查与真实stage/index/commit/普通push另记，独立review不代替客体GUI或私有完整CP重演。下一轮可受控定位 Aegisub 或其他未解决问题，或另一官方固定来源候选；不得重复安装、窗口/exit0/夹具加完成、删除缓存或替换全局DLL掩盖错误。
