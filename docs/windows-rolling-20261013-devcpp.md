# Orwell Dev-C++ 5.11 No Compiler Windows 有限 ASCII 编辑验收（2026-10-13）

新增 1 款，Windows 62/1000；旧 61 个 accepted 与 17 个 pending 全部字段保持。只验收两个小型 ASCII 源文件的读取、单词替换、注释追加、原生 Save As、新正式进程显式 Open，以及预缓存 Store IPC 同版本更新/精确原代次回滚和服务重启。No Compiler 版不带编译器；未编译、链接、运行、调试或验收 project/完整 IDE。合成 C++ 输入输出不另计应用。Ubuntu KateAPT/Snap计算器两款独立记账，Flatpak 两次 runtime 超时未安装缓存保持，本轮未运行 Ubuntu VM。

官方固定 Dev-Cpp 5.11 No Compiler Setup.exe 2148605 B，SHA256 9dc1703385e46f8bd2fc081a78842e089c589969f3bbc582d82f741eaf1b0711；SOURCE 5.11.7z 2091225 B，SHA256 95275fc0d6bd9cca70cd0a613875d64a02431e07954fbdadac009ba32bfab0ec。两个完整 HTTPS 200 GET 的 Content-Length 和实际长度一致，摘要匹配官方固定 SourceForge 文件下载页的显示值；该上游观察是 web 工具文本，不是保存的原 HTML。所有下载终态后才一次原安装、一次独立市场安装，不重复 GET。未核 Authenticode/PGP、源码二进制对应、可复现构建或最新。精确响应/redirect query 私有保存，安装包与程序二进制不入公共仓库。

7-Zip 实测安装包 Type Nsis/SubType NSIS-2、bootstrap PE 0x14c；源码 975 个唯一归档成员不称全为普通文件，9 个固定源成员原字节保存，完整源归档 test exit0/EverythingIsOk。Source/main.pas 和 Version.pas 明确 GPL2-or-later，实际安装 COPYING.txt 与固定源全部 18581 B 原字节相同，SHA256 cea462ee95efc9ce3bfca23a9596f443713e65f41a14c4b59aec88efe272e14f。完整 Delphi VCL/SynEdit/AStyle/tools/icons/translations 许可闭包未核。实际 devcpp.exe 为 x86/PE 0x14c，2795008 B，SHA256 5e490f306cace14810c9fdabf7e30a2c2e1941bedd5e00c54b24e208926527e1，不当整个应用大小。

正式 NSIS 参数 /S 最后 /D=C:后一个反斜杠DevCpp，DIR 12 UTF8 B、hex 2f443d433a5c446576437070，没有 /NCRC。固定官方源码的默认 finish run、关联、快捷方式与 privileges 只是静态上下文，不证明真实 Windows UAC 或完整安装行为。原正式 gen-job-1791840525058-1，原定义 sha256:576e0660960410b5df7afb36d142f7a032af0927f857630c26734ff9d1842d15；首次配置保持默认 English/Courier/Classic Plus/New Look，没有新字体、全局 DLL/runtime/launcher 参数或 workingDirectory 修复。

原中文名原始代码.cpp 在公共原生对话框列表可见，但打开后异常 Cannot open file C:\DevCppTests\????.cpp. Invalid name，未读取到编辑器，未修。真实 Show Report 详情及两张诊断图保存；CtrlA/CtrlC 尝试没有实际完整 clipboard/raw 读回，不能称完整报告已保存。没有 Send Anonymous Error Report。正常 Continue/File Exit0 仅诊断，排除验收。新正式进程通过同一不可变输入的精确 ASCII 名副本 original-input.cpp 才完成读取，不称中文路径/Core 归一化修复。

原输入 52 B，SHA256 def416e7983314bcb2f214ff07ead0c01fcf0883a38570bea890bea0385e69d8；市场 48 B，SHA256 47a008d31f1b46dc0e8266198392b201d3f7c6a187b60f3f6e3082e976d15dcd，市场中文名只预置，实际 GUI 打开 market-input.cpp。两输入 ASCII/无 BOM/两 CRLF 且全字节保持。原生单处 STATUS BEFORE→STATUS AFTER、追加 // ORIGINAL42 或 // MARKET42，再 File/Save As 到 original.cpp 66 B、market.cpp 60 B/各三 CRLF。输出 SHA256 分别 05b19ae04c697bba208f340673bbc368dd0f80a0dda4dea8f43c315686666e18、9b0e03e5e932d63ffe978b5d06baf874b57f364f1735a6f7c4040c71f420b8cc。宿主从实际不可变输入独立重构全部输出字节和三行文字一致。原 CUA uppercase typeText 曾显示 lowercase，未保存的 buffer 用实际 Shift 字母键修正；不称所有键盘方法修复。CtrlShiftS 实际是 Save All，验收使用原生 File/Save As。保存后旧窗口 caption 仍输入文件名，实际 tab、原字节和新正式进程显式 Open 才证明输出。

两代次的新正式进程初始为空，CtrlO/native 文件目录显式选择保存输出；精确原代次回滚后第三个原正式进程同样显式 Open original.cpp。五个验收进程均正常 File/Exit0 且 wineserver 停止，退出0不是单独验收依据。Version5.11/April27 2015 原错误详情和窗口标题实际观察，About 与市场错误详情未单测。广泛 Unicode、中文正文/直接中文键盘、其他编码/大坏文件、复杂编辑、AStyle/packages/tools、打印、编译链接运行调试、网络/新机注册/迁移跨版本/无人值守/GUI Store 按钮未验收，不称无背景联网。

原 candidate-v68/root1/role77 根据有限原 GUI 预览，不能称完成。tuftool 的 file:// 验证把空格编码 %20 后按字面路径读取失败，已有签名、原文件和失败保留；同一签名和目标随后经原 root、临时宿主回环 HTTP 只读验证通过，服务器停止，没有放松签名或过期检查、没有重新签 role77。真实一次传入/stage 后首次 Store 激活拒绝目录，日志 Catalog(Invalid("invalid artifact size or SHA-256"))、按原策略自动重启循环；当时没有 Store update 或市场安装任务。静态契约 Artifact.validate/valid_target_name 只准 ASCII 字母数字与 -._/，原官方 target 名有空格，错误文本虽通用，不能称包损坏或程序运行失败。

首次诊断保存五份实际私有服务 raw 后因 market socket unavailable 停止，第二只读诊断复用这些 raw。实际 488 全 Core 任务、142 全 SQLite 行及代次/配置/信任字段读取保存为六类 offline partial，明确不是七类完整 live CP；保存读回后正常 stop Store，Core 保持 active。更高 candidate-v69/role78 撤回 DevCpp 预览，全部原 61 应用+2夹具目录原字节恢复，保留 role77 的全部签名/目标与失败。正常 start Store 后 full market 对象、488/142 全字段、代次/配置/防护匹配原预览前状态，七类完整 CP 保存读回，才进入文件名别名操作。没有回退 root/role、删除 trustcache、reset failed、放松 guard 或强杀安装。

只通过正式 applications.upsert 将活动定义 installer.fileName 改为 Dev-Cpp-5.11-No-Compiler-Setup.exe；别名全部 2148605 B 与官方缓存原包同，官方 URL/参数/sha/version/launcher/runtime 保持，原 ready 代次不可变定义保持官方含空格原名，没有原重装。更高 candidate-v70/root1/role79 别名预览验证、传入、激活通过，全部 role78 的 133 个非 catalogue 目标保持，仅 catalogue 加 DevCpp，另加别名 binary 与别名 receipt；136 实际签名目标/75 公共 JSON、64 条=62 应用+2夹具。公共目标无程序或安装包 binary。role76/77/78 和所有签名/信任历史保持；两个原 frozen marketPending 标签保留，真实完成另记。

正式 Store IPC 预缓存 update 建立独立 gen-job-1791843045130-5，市场定义 sha256:9cce6383a52d9da3fd69ac6fdcfeaeaae899f6511ffcb335de5d7d87c0d86335；仅 installer.fileName/定义摘要与原不同，launcherDigests/runtime 相同，两个代次各 192 程序全部原字节相同，原定义和全部历史不变。市场两正式 GUI 后 491/143 完整 CP 先保存读回，唯一 IPC rollback 恢复精确原代次，491/144 所有旧字段/两代次保持。第三原 GUI 后活动定义仍 ASCII 别名，原被选代次的不可变定义仍官方原名，分别核对，不混称全部定义相同。

第三原 system.reg 3427072 B、user.reg 101602 B 同大小字节变；实际前后私有 raw、287/20 行 diff 保存，原因/键值等价未定。先只读观察，不保存 CP/重启，再固定实际新 pins 保存读回 492/144 完整 CP，正常 Core/Store 各重启一次，七类全字段、活动别名、新 pins、两代次程序/输入输出、旧 61+17 固定数据保持。当前私有 devcpp-20261013/after-final-restart.json SHA256 d6daa236117ba3a7fe1ccbb44e1b6289a025254a1d8e2a0522021fa2efd384d3，不入库。只读 rolling-current-state-devcpp-20261013.py 必须经 rolling-guest-20261010.py 严格 SSH stdin 客体执行，不要求 host helper 客体落盘。范围只 192 程序/固定输入输出/三 registry/selected Dev-Cpp Roaming/Local 命名配置，非全应用 FS；最近64 IPC 不代替完整 SQLite，Core 查询可写既有 scratch。

最初 archive V1 先 strip 再 split 导致空 ValueError，在 source listing/安装前停；V2 用错误 LF block 分割返回0成员，passed 布尔无效，没有用于正式安装；V3 误期望 Source/Main.pas，真实 Source/main.pas，断言在写V3报告/安装前停，V4 复用原 CRLF raw、精确路径与975成员检查通过，原 helpers/失败全部保持。猜读缺失助手、无效 literal rg glob、截断和过渡画面不当已完成检查。旧 Scribus 一份私有 canonicalV1 raw 丢失限制不补造；认证/锁屏/电源不改，所有正常登录都新鲜稳定字段逐字符一次提交，9 张 JPEG 的 01/02 为路径诊断，其余为实际有限流程，不含登录图、凭据或私有完整 CP。

Git 前旧3830 evidence 与完整 repo5071 tracked 分开，3828旧immutable与原22DevCpp冻结保持，1241范围外 index/commit 与父HEAD实际原字节单独核对，不称全部工作区字节相同。精确属性 raw/filter/relative-no-index、独立明确 FINAL 审查、真实首次 stage/index/commit/普通 push 另记；独立审查不代替客体 GUI 或私有完整 CP 重演。
