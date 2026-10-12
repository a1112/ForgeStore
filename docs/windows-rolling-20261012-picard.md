# MusicBrainz Picard 2.13.3 Windows 有限 WAV 标签验收（2026-10-12）

新增1款，累计56/1000，14未解决候选保持，CopyQ/BowPad不计完成。仅有限标签读写，不是完整音乐识别/匹配验收。合成WAV不另计应用。Ubuntu Kate/APT、Snap计算器两款独立记账，Flatpak两次runtime超时未安装、缓存保持。本轮未切换VM，两4GiB VM没有同时运行。

原正式gen-job-1791794047570-1、独立市场gen-job-1791796001088-4，冻结定义sha256:66cff6713fc3350bcea28a7b777afc59e8496b0e70e6be318ad06b0ed3956da9。各中文名不可变输入先由原生AddFiles/列表双击打开，实际标题BEFORE、Artist FORGE BEFORE、时长0:01及中文完整路径可见，再仅Remove GUI列表项目、不删除文件。另打开同字节预置ASCII original-work.wav/market-work.wav，原生Title/Artist编辑为ORIGINAL AFTER/ORIGINAL42、MARKET AFTER/MARKET42，工具栏Save和正常File Save Warning OK保存工作副本。没有SaveAs或不可变中文输入编辑的声明。两新正式进程正常New User OK后显式CtrlO从列表打开保存工作副本，回滚后第三原进程同样显式打开。五正式CtrlQ正常退出0+wineserver停止。

原输入16102B，8000Hz mono16bit8000帧；市场64100B，16000Hz stereo16bit16000帧，各一秒，输入原字节保持。输出17190/65182B，SHA a901eefadf71247b54f070cc05a0bfe054380e3ba05517fcf7f2f64f1f135851 / 45230ea73ede7153c162571f2f8e5b0ce6422b346b826392deb73ecf49169411。独立有限RIFF分块/INFO INAM IART与ID3v2.4 TIT2 TPE1文本帧/1024B零padding核对，消费全部字节；stdlib wave另核rate/channel/frames/bitdepth，16000/64000B PCM分别与原输入精确同。输出跨新进程/回滚/服务重启原字节保持。不等于完整RIFF/ID3规格或实际听音/播放/音频设备验收。没有MusicBrainz匹配、CD、AcoustID指纹、封面、插件脚本、重命名搬移/batch、多值/广泛格式/大坏输入、中文键盘标签输入、客体联网下载、新机注册/迁移/跨版本/无人值守或GUI市场按钮验收。

原About实际2.13.3/Python3.8.10/PyQt5.15.11/Qt5.15.2/Mutagen1.47.0，其他组件只显示不当完整清单，市场About未独立实测；444程序原字节全同。New User Show Again保持勾选，Save Dont Show Again保持未选。两个代次出现3.0.2更新提示，正常Cancel，不称最新、无人值守或纯离线。GUI显示Listening on port8000；原代次只读ss确实127.0.0.1:8000 wineserver，市场没有另测ss。market-final继承port8000布尔仅此原观察范围，不能证明没有后台联网或完整RPC/browser集成。

官方固定安装32331256B SHA cfe7fb0957f9bd561dfd97b4685cbd26adeb6d9485d25bbbdf39d86c552e1fec；源6089477B SHA63fdb3aeb2e7ab41c73aa44083030c22a6a0e955b932960c05669998e2e21fcc，均与官方1536B SHA256SUMS原文一致。宿主完整GET930.619/178.157秒最终200且长度/摘要完整；同时受控外层路线3源/16安装有界206段逐段ContentRange/长度/IfRange ETag核对后合并，51.857/542.326秒成功。两路线原包全同，均在正式安装前终态，不把慢传输当失败、重复下载当重复安装或未运行项通过。所有段及实际收据保留。

11固定源码成员：10来自官方release tar，installer/picard-setup.nsi.in来自关联commit8a062c9a7c103e1abed485e62f75bdc332753311的官方blob API，11111B SHA5a72b66014c32ff8d1a637167e529f963a1f7fabe4b90b05cca03823439aed9d、Git blob9f4ee4609860e6d8a10fab6e141494a7eaf9b8a3与新鲜完整tree独立对应。初只筛.nsi/.nsh漏掉.nsi.in，release tar本身未含该template，不称原归档安装脚本或源码二进制对应。主固定源码GPL2-or-later；安装无主COPYING/LICENSE命名文件，有Markdown BSD3/importlib_metadata Apache2组件通知，不当主许可比较或Qt/Python/Mutagen/chromaprint/discid/OpenSSL/icons/translations完整闭包。GPG/Authenticode/源码二进制对应/可复现构建/最新未核。

原生NSIS /S与最后/D=C:后单反斜杠Picard，12UTF8B hex2f443d433a5c506963617264。template RequestExecutionLevel admin和默认language/shortcut sections保留，不当真实Windows UAC/admin或完整安装行为、无关联/快捷方式证明。各444安装文件跨代次逐字节全同，picard.exe3162088B SHA9b28e22d1625949aba549b82464eaf261b75d828d9a94f432ab5bd1597159e5d，没有GUI新增程序文件。无Core/程序/globalcontext/definition/launcher参数或environment/workingDirectory/字体包/DLL/runtime修复。

StoreIPC预缓存同版本更新及精确原代次回滚通过。回滚前440Core/131完整Store CP先保存读回，回滚后440/132旧全部字段/代次保持。第三原GUI后441/132正常两服务重启前后七类全字段精确同。candidate-v62/root1/targets snapshot timestamp71，58目录项56应用2内部夹具、121实际签名目标67公共JSON文件；119旧目标中118非catalogue原字节精确保持，只catalogue新增Picard，CopyQ仍不在活动目录，role70、root和全部签名历史保留。完整私有CP picard-20261012/after-restart.json SHA86bd2fad9e3a3bf296f144007c72d0be5c2e5f5833762c3c89e67e64c48c9d90。只读rolling-current-state-picard-20261012.py严格SSH rolling-guest-20261010.py发送stdin执行，不要求host助手客体落盘。最近64IPC摘要不替代完整SQLite。

第三原GUI system.reg3899149B/user.reg108512B/Picard.ini11021B大小同而字节变，实际前后私有原文和321/64/13行diff保存，原因/键值等价未确定，不称旧字节跨第三GUI保持。先只读观察、不写CP/重启，固定实际新pins后才CP保存读回及正常重启，当前新pins稳定。程序、不可变输入、ASCII输出、市场所有pins和55旧固定验收数据保持。只遍历本轮444程序、固定数据、3registry和selected Picard/MusicBrainz prefix-user配置，非所有应用FS；core()查询可写已有scratch，不称FS零写。认证/锁屏/不息屏设置未改。

实际失败和诊断范围：web二进制工具UnsupportedContentType不是安装包HTTP损坏；原manifest/native官方blob读取另成功。缺AGENTS/猜读缺历史助手与宽rg截断保持，后来focused读取，不当全部输出已读。catalogue-builder V1漏hashlib，在四份host助手写入后、任何guest transfer/stage/activation/update/market GUI前NameError停止；原失败助手和四部分原文pins保持，V2补import并逐字节核既存四文件而不重写，之后各动作只执行一次。早市场CtrlO被实际更新弹框挡住，Cancel后正常；加载过渡点击Clusters，fresh观察后选真实文件，编辑/保存前恢复，不称应用缺陷。既有授权正常登录，一次bulk输入只一字符未提交，清空后逐字符正常一次提交；空闲noVNC旧时钟锁屏帧Return后显示仍运行应用，不当新登录或绕过认证。凭据/登录画面不入库。原25 source/gui/acceptance和market pending历史标签不改，完整有限完成另记；不称历史全部门槛有效。不重写旧Scribus失败raw丢失限制或全部更早证据。

14实际JPEG中01/02/03/05/08/09/11为warning/update/About诊断，04/10中文输入读取，06/12实际保存，07/13新正式Open，14回滚原Open。有限实测及独立核对不替代完整音乐识别客户端验收。独立FINAL review、Git字节预检/index/commit/普通推送另记，不替代客体GUI/私有CP独立重演；私钥/密码/凭据/登录图/私有完整CP不入库。
