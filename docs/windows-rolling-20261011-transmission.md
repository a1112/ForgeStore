# Transmission 4.0.6 Windows 有限离线流程验收（2026-10-11）

新增1款，累计41/1000；13个未解决候选（含BowPad）及旧台账字段保持。候选、安装、原GUI、独立市场GUI与本地发布分别记账。合成输入与种子不计应用；Ubuntu仍两款独立验收，本轮未切换虚拟机。

固定官方Qt5 x64 MSI为18149376字节，SHA-256 8fe99234539da38be335fc3d999d1364c0c203d5dbc53109519b1df634e0462d；官方源包11908296字节，SHA-256 2a38fe6d8a23991680b691c277a335f8875bdeca2b97c6b26b598bc9c7b0c45f。GitHub API403，没有取得上游公开size/digest；只声称固定HTTPS下载及本地摘要。10份源成员原字节固定。Qt主源码许可为GPL-2.0-only OR GPL-3.0-only，后续许可需Mnemosyne认可，COPYING含OpenSSL链接许可；不无条件称or-later。完整Qt/OpenSSL/curl/DBus/翻译等许可闭包、安装许可比较、源码二进制对应、PGP/Authenticode及可复现构建未核验。MSI ProductVersion4.0.6/ProductName4.0.6(38c164933e)及输出created-by均核对，GUI About版本未观察。

首个ADDLOCAL=CommonLibs,QtClient定义被Core封闭属性白名单拒绝，应用/任务未创建。按固定MSI源码默认feature level省略该属性后正常托管安装，不绕过安装条件、不伪造vcredist注册项。两代次113个应用文件跨代次原字节相同，transmission-qt.exe3017472字节SHA-256 f674da676b678b7a743626093559f72929293d0837c6099d6fdf3cbf8da62259；实际daemon/create命令行程序不存在。无Core/程序、全局context、launcher环境/工作目录、字体/DLL/runtime修复。

原正式代次 gen-job-1791731934014-4，市场代次 gen-job-1791732876165-7，冻结定义 sha256:18f592b3bd360e69bc36e271f51f7e251bce624550cc8eb4ea22f12dfc404087。两份中文名单文件输入各4096字节，GUI CtrlN选择源文件和保存目录，留空tracker、ASCII注释ORIGINAL42/MARKET42，生成种子216/213字节。独立严格bencode完整解析/规范化、UTF8文件名、长度、32768字节分块和唯一20字节SHA1均通过；输出由GUI写入，检查助手没有生成输出。输入原字节保持。两份infohash分别 859625458c0b7f287a661e69c988274227de4a4a、72d16d61d2873a3ff81ab84242cf88ad26b86ab4。新正式进程显式按种子路径重开，取消Start when added、保留Move to trash未勾选，指定已有本地数据目录。实际GUI Verify Local Data后属性Have4KiB100%、Availability100%、Paused与注释/摘要匹配，网络传输未验收。

回滚后原进程显式打开种子，正常提示重复，关闭后读取恢复的原暂停条目，再执行本地校验并查看原注释/摘要/100%属性；未添加第二条记录。五次正式GUI正常CtrlQ退出0并停止wineserver。首次文件共享普通提示和本地会话选择手动完成，不称无人值守启动。原CtrlI未打开属性、一次误选Open Folder和初次未选中行的校验无效，随后正确菜单实际通过。市场保存目录一次点击只聚焦，读新鲜界面再打开；过渡空白/未加载注释画面不计通过。

当前candidate-v44/root1/在线角色53，43目录项为41应用与2内部夹具，87实际签名目标、49公共目录文件。旧root和全部52及以前历史目录保持，MPC-HC角色32原证明与33更正证明继承。Store IPC预缓存同版本更新成功，完整322 Core/99 Store回滚前检查点先保存读回，再提交精确原代次回滚；回滚后322/100全历史旧字段与全部代次保持。第三原GUI退出后323/100，正常Core/Store重启前后全字段精确一致，运行时摘要/正式配置/信任/防护和不自动息屏保持。GUI更新回滚按钮未验收。

第三原GUI改变system.reg、user.reg和一份resume原字节，前两份大小不变，resume690→699字节。前后实际原文、287/20/5行私有差分及当前pins分别固定，原因未确定，不称原字节跨第三GUI保持；当前新pins跨服务重启相同。程序、输入、GUI种子、市场全部pins和旧固定验收数据保持。私有完整CP transmission-20261011/after-restart.json，SHA-256 d853fc032db0c0f12458404d0a547a3d2c87e198c2e2d5c1596c7dc7ed992d94。只读rolling-current-state-transmission-v2-20261011.py必须严格SSH客体执行；旧316/98等助手仅历史。覆盖固定验收数据、本轮113文件和有限配置/注册表，非全应用文件系统遍历，完整SQLite行不以最新64条摘要替代。

原23份来源/GUI/acceptance冻结字节及市场待执行历史阶段标签保持，后续市场完成另记；1091旧immutable保持。市场GUI helper内联Python引号被PowerShell拒绝，无客体变更，保存编译脚本后受控继续。几次猜测历史helper只读路径失败后rg发现真实文件名；未据失败读执行依赖变更。首次当前核对助手继承selectedNomacsGeneration字段名但值为正确Transmission代次，保留原收据，v2仅更正字段标签并新鲜核对通过。所有依赖门槛实际assert，false/缺assert/非终态/检查点失败即停止，不只exit0、不补造历史。

仅两个小型单文件离线流程通过。下载/上传/做种、peer/trackers/DHT/PEX、magnet、私有/多文件/大型或坏种子、远程daemon/RPC、直接中文输入、高级功能、新机注册、迁移、跨版本升级及完整无人值守未验收。认证/锁屏未改，登录密码和画面不入库。私钥、凭据、完整私有CP及临时签名下载查询串不入库。GitHub认证沿用既有阻塞，本轮未重新检查远端/认证、未新发设备码、未尝试推送；累积提交待恢复验证后普通推送。独立审查及真实index/commit字节核对另记。
