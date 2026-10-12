# Doxygen 1.13.2 Windows 有限 ASCII 文档生成验收（2026-10-12）

新增1款，累计50/1000，达到首批50款有限验收里程碑；14未解决候选及CopyQ/BowPad原失败保持，合成C++源码、生成文档、配置不另计应用。Ubuntu Kate/Snap计算器两款独立记账，Flatpak未安装，本轮不切换VM。

正式原gen-job-1791763366912-1、独立市场gen-job-1791764559927-5，冻结定义sha256:5087cf5514e52ae9ffedca5d10389eb0571bc1f270205fa82e2501f694894b51。仅小型ASCII路径Doxyfile/C++单文件：GUI项目名ORIGINAL/MARKET BEFORE→AFTER、原生SaveAs到ASCII路径、真实Run生成HTML/XML、两个新正式进程显式Open保存配置并再生成、回滚后原配置重开并再生成。原源码309B/配置405B，市场300B/397B原字节保持；保存配置131496/131488B，每代次62文档文件，新进程与第三原GUI再生成后全部原字节同。独立XML/HTML解析单int函数original_add/market_add、left/right参数与中文brief，XML2722/2691B、HTML5250/5217B。原/市场首次完整日志3776/3758B和回滚新日志3676B无warning/error；两次新进程重开日志仅GUI观察、未另存完整日志，不把原首次日志当新日志。五次正式CtrlQ正常0+wineserver停止；About实测Doxygen1.13.2、Qt5.15.2。没有执行夹具C++函数或浏览器渲染HTML验收。

最早job-1791763478502-2及截图01/02只有失败/单变量对照：中文名配置标题出现而实际项目与路径仍默认，405B同字节ASCII别名实际字段正确。中文配置路径未修，固定config_doxyw.l的fopen(toLocal8Bit)仅原因假设，没有运行时函数/HRESULT定位；不改Core/程序/globalcontext/launcher环境/workingDirectory/字体/DLL/runtime，也没有shell包装器或私有副本代替正式流程。该诊断正常Quit0不算通过，不重复安装；正式流程另用新进程与ASCII配置。

官方固定Inno安装58172541B SHA08261faf23a9bd4a337cb9d25e9b339d06c788ddadb03283519fa217db58f1bb，source8512178B SHA3a25e3386c26ea5494c784e946327225debfbc5dbfa8b13549010a315aace66d，官方API大小匹配且digest=null；只本地HTTPS摘要，不称上游公开摘要核验、最新、PGP/Authenticode、源码二进制对应或可复现构建。最初10源码成员固定，另2成员用于路径/选项诊断。主GPLv2文本和未指定版本源码声明，确切or-later范围未核。各2078安装文件独立pins，2077跨代次同字节，唯一system/unins000.dat各284941B不同，原因/卸载未验收；GUI exe15472128B SHA31414f218c1cd5d220fe8012007cfbf3f1e7d2a1d42fa6efe330e211a1996b45，backend22010368B SHA162744d3f3f1210f73980b52a8582bff475ca470078c2bdeeb2e4a53f841cb07。无LICENSE/COPYING命名安装文件，不称安装许可比较或完整Qt/MSVC/工具/docs/templates/icons许可闭包。原生Inno /VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /DIR=C:后单反斜杠Doxygen /NOICONS /TASKS=，DIR15UTF8B hex2f4449523d433a5c446f787967656e；不声称广泛关联/背景联网不存在或无人值守。

Store IPC预缓存同版本更新与精确原代次回滚通过，GUI市场按钮未验收。回滚前397Core/119Store完整CP先保存读回，回滚后397/120全部旧字段与代次记录保持；最后原GUI后398/120正常两服务重启前后7类全字段精确一致。最新完整私有CP doxygen-20261012/after-restart.json SHA24cd00bace9578a67439e76fdd9c327a2d2e6d7f013c2c04991f08fd43844eaf。当前只读rolling-current-state-doxygen-20261012.py通过严格SSH发送stdin到客体，host助手不要求客体落盘；查询可写已有请求scratch，不能称文件系统零写。

第三原GUI system.reg3424286B同大小而字节变，user.reg103202→103139B，真实前后原文和287/34行差分私有固定，原因未定，不称旧字节/键值保持。实际新pins跨restart同；原/市场各2078程序、固定输入/保存配置/生成文件/旧日志和既有固定验收数据保持。只核固定证据和本轮文件/有限registry，不遍历全部旧应用文件系统；完整SQLite历史不用最近64IPC摘要替代。

全部失败保留：三份旧helper猜测读取失败无依赖客体变更；初Poedit来源研究未安装/计数，poedit-start-current-state文件名只是实际MediaInfo390/118基线；首次配置选项子串误用双引号而源码单引号，XML独立解析确认全部选项；中文配置默认字段失败与ASCII单变量对照。市场安装后首helper错误期望根unins000.dat，实际system/unins000.dat断言立即停止，没有夹具/市场GUI；只读395/119全字段与2077相同/原代次全部pins核验，诊断CP保存读回后v2仅改实际路径并增加该CP门槛，无重复安装。原helper/收据/94source-GUI-acceptance冻结字节和market待执行标签保持，后续完成单记。

candidate-v56/root1/在线roles65，52目录项50应用2内部夹具，109实际签名目标/61公共文件；旧107目标除catalogue新增一项外106非目录目标精确保持，全部旧role/root/签名历史不改，CopyQ不在活动目录。14候选与所有既有有限边界/失败按原报告保持。复杂项目/其他语言/图表Graphviz/PlantUML/LaTeX/PDF/其他输出/高级文档、大坏输入、完整HTML/XML规格、直接中文输入、客体联网下载、新机注册/迁移/跨版本/无人值守均未验收。

2241旧immutable与94原本轮冻结保持；独立审查、真实index/commit审计和普通推送另记，不代替客体GUI/私有完整CP重演。私钥/密码/凭据/登录画面/私有完整CP不入库。
