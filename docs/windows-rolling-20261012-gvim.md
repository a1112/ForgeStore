# gVim 9.2.0000 Windows 有限文本编辑验收（2026-10-12）

新增1款，累计57/1000，14未解决候选保持，CopyQ/BowPad不计完成。合成文本和undo文件不另计应用。Ubuntu Kate/APT、Snap计算器两款独立记账，Flatpak两次runtime超时未安装、缓存保持；本轮未切换VM，两4GiB VM未同时运行。

原正式gen-job-1791799154641-1、独立市场gen-job-1791800485389-4，定义sha256:7da009ffd7341dc7f262a1a1cb99af376c015ca6e87437573273eec3e9372cd5。仅两个中文名UTF8BOM/CRLF小文本原生File/Open，默认字体中文实际可读；Edit/FindAndReplace普通STATUS BEFORE→STATUS AFTER、MatchCase原默认勾选/WholeWord原默认未勾，ReplaceAll一处。原生Escape/G/o编辑ASCII ORIGINAL42/MARKET42后Escape，File/SaveAs到ASCII original.txt/market.txt。两个新正式进程显式File/Open保存文件，回滚后第三原同样显式重开。五正式File/Exit退出0+wineserver停止；CtrlQ没有验收。

中文输入76/74B原字节保持，输出87/83B，SHA3be5bc054468ef7aa7db026168a3c7dcb804b7f5fe0e7bbc0d6e39345ef32f6d / 92dbe12e22a1bc36380e7e396adb0abe7d91c082d10802326d6f7973c060e9cb。独立从实际不可变输入重构普通替换及ASCII追加，完整原字节匹配；utf-8-sig中文/四行完整文本、四CRLF和BOM核对。输出跨新进程/回滚/重启字节保持。不当完整Vim/编码规范、直接中文键盘输入或高级编辑验收。原Help/Version实测VIM9.2(2026Feb14)、compiledFeb14 2026 12:17:29、MSWindows64bit GUI/consolewithOLE support、HugeversionwithGUI；About未测，市场Version未单测，显示feature flags不当函数验收。菜单自动回显是应用Vim语句，未向OS命令行或Shell输入命令。未验收Vimscript/Vim9、宏/插件、多文档/project/diff、广泛modal keys、terminal/shell/OLE、Python/Lua/Perl/Ruby接口、其他编码、大坏文件、打印、联网下载、新机注册/迁移/跨版本、无人值守和GUI市场更新回滚按钮；不称后台联网不存在。

官方固定https://www.vim.org/downloads/gvim_9.2.0000_x64.exe完整HTTPS200，11654015B SHA243ed9ed2951c26d27801afbb4cc5fa92ed6395f8f388c6d432b62012c9f3684，Content-Length同实际。固定tag v9.2.0000关联commit e7e21018fc0b60c153c8e668f696d95e574cc5a4，官方codeload源19594383B SHAc68e850f25816fce7995493ae004d8f8256e5c993edb3984efc17db723fd3c51，完整HTTPS200，Content-Length缺失，只有实际本地长度/摘要。未核上游公开digest、PGP/Authenticode、源二进制对应、可复现构建或最新。GitHub installer release tag API初404在全部下载前停止，原失败保留，随后用官方Vim网站实际链接，不当认证失败、包损坏或最新版本声明。

9源码成员固定main.c/gui_w32.c/fileio.c/version.c、LICENSE/README、runtime/doc/uganda.txt、nsis/gvim.nsi/README。主自定义Vim许可SPDX Vim，不无条件称GPL：未修改复制需保留通知，修改分发有I/II条件；GPL2-or-later是II2e针对GPL变化的分支。本轮不改程序/源码。安装vim92/LICENSE.txt5021B SHA0b3f1f330cb1b179bb17c7c687d4cec601e0aa3462bc7f890ad4c3888d37d720与固定源LICENSE逐字节同。完整Lua/Perl/Python/Ruby/Gettext/libsodium/runtime/icons/docs等捆绑闭包未核。

原生NSIS /S和最后/D=C:后一个反斜杠Vim，9UTF8B hex2f443d433a5c56696d。静态NSIS与源码RequestExecutionLevel highest/default optional components保持，不当真实Windows UAC/admin、无关联/shortcut或完整安装行为。每代次2311程序文件跨代次全部原字节同，vim92/gvim.exe206848B SHA2ee6df23158e5fc836df737917e54db0d9b128a587478000cb7489ea40e6067b，是DLL配套入口，不把stub大小当全部编辑器。GUI无新增程序文件；原生生成精确.original.txt.un~/.market.txt.un~持久undo文件纳入各代次应用配置pins，私有原文保持，不当GUIUndo功能或完整undo格式验收。prefix-user _viminfo和三个默认Desktop快捷方式纳入有限配置；selected home Vim配置本轮未发现，不当全配置/全文件系统排除。未调字体或装字体包，无Core/程序/globalcontext/definition/launcher参数、environment、workingDirectory、DLL/runtime修复。

StoreIPC预缓存同版本更新与精确原回滚成功；回滚前447Core/133完整Store CP先保存读回，回滚后447/134旧全部字段/代次保持，第三原GUI后448/134正常两服务重启七类全字段精确同。candidate-v63/root1/targets snapshot timestamp72，59目录项57应用2内部夹具、123实际签名目标68公共JSON文件。旧121中120非catalogue目标精确保持，仅catalogue新增gVim，CopyQ不在活动目录，role71/root/全部签名历史保持。完整私有CP gvim-20261012/after-restart.json SHAda2286d23b50fdca7c25d1e48d56956f1f4b0d4944d7d83c4282017d447cdbc8。当前rolling-current-state-gvim-20261012.py经严格SSH rolling-guest-20261010.py发送stdin核对，不要求host helper客体落盘，不用最近64IPC摘要替代完整SQLite。

第三原GUI system.reg3428367B/user.reg102586B大小同字节变，_viminfo1453→1878B，实际前后私有原文与287/20/41行diff保存，原因/键值等价未定。先只读实际观察没有CP/重启，再严格固定新实际pins、CP保存读回与正常重启；新pins跨restart同。其余程序、两输入/两输出/undo、市场全部pins和56旧固定验收数据保持。核对仅本轮2311程序、固定输入输出、3registry、selected Vim-named prefix-user配置及精确undo，不全应用FS；core()可写已有请求scratch，不称FS零写。认证/锁屏/不自动息屏设置未改。

范围与失败：fre:ac1.1.7官方安装/源下载仅研究，最初NSIS静态假设被实际proSetup PE内嵌ZIP断言挡住，无正式任务/安装/GUI/新pending计数；未证实其原生静默目录参数，改选本轮gVim，原包/源码/失败保持。gVim verify-builder V1 bytes literal编译SyntaxError在library write/guest执行前停止，V2仅raw literal修正后编译并严格实际字节核对；不称应用/Core缺陷。web工具二进制UnsupportedContentType不是包损坏。多次自然锁屏后新鲜字段逐字符、一次提交正常授权登录，密码/登录画面不入库，认证设置未改。

catalogue host传输成功一次，其完整stdout留工具历史、未另保存exact host收据文件；没有重传或伪造原收据。独立read-only transfer-readback实际archive1812480B SHAc68a519f1610dddfae1cb78f51b147b6341dfe8b04909c095c0466fb8c1e7cf7在stage/activation前核对并另记，不当原传输exact stdout重演。原19 source/gui/acceptance与market pending标签冻结不改，完成另记；旧3136immutable精确保留，旧Scribus一份私有canonical失败raw丢失限制保持，不伪造恢复，不称所有历史门槛全有效。8实际JPEG中03是Version诊断，01/05中文输入、02/06真实SaveAs、04/07新正式Open、08回滚原Open，无登录图。

独立FINAL审查、Git真实预检/index/commit/普通推送另记，未替代客体GUI/私有完整CP独立重演。私钥/密码/凭据/登录画面/私有完整CP不入库。
