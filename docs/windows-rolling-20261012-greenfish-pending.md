# Greenfish Icon Editor Pro 4.5 Windows 未解决候选（2026-10-12）

Windows保持57/1000，Greenfish新增为第16个未解决候选，原15项（含Zim）与57款全部字段保持。一次正式安装gen-job-1791808654012-1；正式启动job-1791808906768-2只显示4.5启动画面及“MDB error: Input/output error”。原生OK确认后程序关闭，真实exit1和wineserver停止。没有编辑主窗口、中文BMP原生Open、图像变换/保存/新进程重开、独立市场或上架。原630B BMP已预置客体但未GUI打开，市场1014B仅宿主准备，均不计应用；original.png不存在。

官方安装gfie_setup_4.5.exe46008466B SHA1cddb81ba7e9e50e3a6ae28e64bdc496fcd8df2fd4abd5ad4eccecf3b5b9987d，官方便携ZIP75501094B SHA7cc9b150b3e09e1d59b162fc5aa1073387c91003f1dd6e75749067ae4af3f533，仅官方HTTPS本地摘要，不称上游公开digest、最新、PGP/Authenticode、源码二进制对应或可复现。便携ZIP只用于许可研究，不是另一安装代次。宿主首次GET读取超时，无完整安装包；错误原文保持，内部已接收片段未单独落盘、长度未知，不称所有网络失败原字节都保存。

外层64KiB探测206/ContentRange/长度/ETag通过，22安装段完整通过。便携37段中34成功，21/22读取超时留196608/65536B部分原文，28 TLS EOF在HTTP前无部分文件；全部旧段/失败独立文件保持。初并行助手全终态失败后，新V2串行只取三个缺口尾段，核206/Range/IfRange同ETag，保留原部分前缀并独立合并，再核所有ZIP CRC及本地摘要；不能据串行成功定位并发根因。两实际下载路线均终态后才传入客体/一次安装。

现有7-Zip无法将安装包打开为归档，首次inspection断言失败先停安装；378B原列表保持，不称包损坏。新V2从实际两处Inno Setup Setup Data(6.4.0.1)标识和PE引导Machine0x14c核类型，原生参数查官方Inno文档；未核完整Inno格式。/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /DIR=C:后单反斜杠GFIE /NOICONS /TASKS=，DIR12UTF8B hex2f4449523d433a5c47464945。定义sha256:be53361f08d6fffd069feff2f9e1592e76fb445d972fb675f3bd1d1203f838d4。

许可关键边界：4.5实际主EULA是自定义免费软件协议，允许使用/原样复制发布分发、禁止修改/衍生软件并要求随软件包含EULA；不能沿用旧3.6 GPL、不能称4.5主程序开源，未提供主源码。1315B BOM/CRLF license.txt SHA fccbf60a25ba2e7f4ec39ac0a019fe388c74a92393a86650f4c05125712cbb57与正式安装同字节；比较对象是官方同版本便携EULA，不是源码。readme19709B原文保持。完整Qt/MSVC/图形压缩/组件/图标/翻译许可闭包未核；只有本地原样安装/诊断，公共证据没有安装包或程序二进制。

实际171程序文件严格pins；便携186不是安装计数。gfie.exe78336B SHA6c0692c2b93134f2166112e1a9f1b78450405aa9750b0d5f8a3db53030651706是DLL入口，不当整个编辑器大小；libgfiedll.dll16148992B SHA2dff47f26fe4e9745697cf4553215a148090feaec1b352955930af596d1e1c75。一GUI失败及服务重启前后171全同字节。无程序修改/字体包/全局DLL或runtime修复，无Core/定义/launcher环境或workingDirectory改动；未尝试禁用通知、删除缓存或掩盖错误。

GUI后首次只读发现AppData/Local/gfie/shaders.lmdb8192B与shaders.lmdb-lock262208B，不能证明GUI前不存在或具体创建时间，不能证明数据库语义或具体GPU/LMDB/Wine函数、HRESULT根因。三个registry实测新pins与实际选定缓存私有原文保存；安装后旧registry只有摘要，无法补造原文或真实前后diff。旧新摘要差异见later-observations，不称旧registry跨GUI字节保持。job前缀枚举只找到Core任务JSON，不能当应用stderr日志；进程结束前未保存真实运行stdout/stderr，因此没有运行时具体函数trace。

初只读观察不CP/restart，固定实际新pins后才保存读回454Core/134完整StoreSQLite全部字段失败CP，再正常Core/Store重启；七类全字段精确同，所有171程序、输入/无输出、选定配置与旧57及Zim固定数据保持。失败job错误/时间/事件全字段保持，无隔离operation，未recover或VM重启。完整私有CP greenfish-20261012/after-restart.json SHA2dd6f2c7e8ed010ac9ecdd73de3395babe8b38c0fbd2bb5850fab8822dd047c0不入库。

目录仍candidate-v63/root1/role72、59条=57应用+2内部夹具、123实际签名目标与68公共JSON；Greenfish/Zim/CopyQ均不在活动目录，本轮没有TUF签名/市场/目录变化。旧root和所有签名/失败历史保持。Corelauncher-cwd-v11-r1/Storemanaged-msi-v2摘要保持，Provider v2未部署。只核固定验收数据、本轮171程序、3registry和选定gfie/MDB位置，非全应用FS；最近64IPC不代替完整SQLite，core查询可写既有scratch，不称FS零写。

自然锁屏后已有授权账号新鲜字段逐字符一次正常登录，密码/登录画面不入库，认证/锁屏/不自动息屏设置未改。一张JPEG只是MDB启动失败诊断。宿主缺失助手猜读、错误帮助路径、web二进制UnsupportedContentType和截断输出不当已运行检查；原helper/失败原文保持，旧Scribus私有canonical失败raw丢失限制不补造恢复。没有About、实际图像编辑、格式转换、中文输入、完整图标/光标/图层/动画/高级图形/打印/大坏文件、网络/新机注册/迁移跨版本/无人值守或GUI市场按钮验收。

Ubuntu Kate/APT和Snap计算器两款独立记账，Flatpak两次runtime超时未安装缓存保持；未运行Ubuntu VM，两4GiB VM未同时运行。下一轮读当前Greenfish pending证据/台账及最终freeze，不重复安装；可受控定位MDB或另选官方固定许可候选。rolling-current-state-greenfish-pending-20261012.py须通过rolling-guest-20261010.py严格SSH发送stdin客体执行，不要求host helper客体落盘；旧Zim452/134助手只历史。独立审查和真实Git审计另记，不代替GUI/私有CP重演。
