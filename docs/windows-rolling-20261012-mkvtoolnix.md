# MKVToolNix 94.0 Windows 有限音频封装验收（2026-10-12）

新增1款，累计46/1000；13项未解决候选保留。日期采用亚洲/上海，客体UTC时间不改。Ubuntu两款独立验收不变，本轮未切换虚拟机。合成WAV/Matroska不另计应用。

官方固定安装包32100808字节SHA-256 5c8fb973f181a51ff37b9e6c392f8cbc0b00c1dea2accbfc33a339b09d29d5f2，源码11708096字节SHA-256 babbcff2362c9dd00b2e79336eff83fad177603a51a458ef1fa421b27fbc4703，均匹配官方固定.sha256文件；不称最新或PGP/Authenticode验证。14份源码/许可/安装脚本成员原字节固定。README声明主程序GPLv2；确切or-later/SPDX范围未核，市场显示“GPL-2.0 (upstream declaration; or-later scope unverified)”。安装doc/COPYING.txt18092字节与固定源码COPYING原字节相同。全部Qt/EBML/Matroska/codec/icons/translations捆绑许可闭包、源码二进制对应和可复现构建未核。

正式原gen-job-1791746776538-1及独立市场gen-job-1791747680633-4，定义sha256:847865d340049e05734564e03465ce0d9dd97b380f1514f0af26799ec01d9fa0；209程序文件跨代次全部同字节。GUI主exe52001320字节SHA-2567a6907169c1259cffc1dcae80bfd93cfc14911a68c199ce7d9ef2b83ebc0965d，真实窗口版本v94.0('Initiate')64-bit。原生NSIS /S与/D=C:后一个反斜杠再MKVToolNix，16UTF8字节hex2f443d433a5c4d4b56546f6f6c4e6978。无Core/程序/global context/launcher环境/工作目录/字体包/DLL/runtime修复。

两个中文名1644字节WAV输入，各800个8000Hz单声道16位PCM样本、1600PCM字节（名义0.1秒）。原公式(i*42)%20000-10000，市场(i*40+2)%20000-10000。真实GUI打开，Output设置ASCII标题ORIGINAL42/MARKET42和ASCII目标，Start multiplexing完成100%无警告/错误，输出7400/7398字节.mka。两个新正式进程显式重开输出显示Matroska/PCM，并通过Info tool实际读取标题、音轨和容器结构。回滚后原新进程再次显式加载原输出。五次GUI Exit/CtrlQ及正常Close settings确认退出0+wineserver停止。三次重开退出时关闭未执行封装设置，不写新输出；原输入和输出字节重核保持。没有播放/音频听感验收。

独立有限EBML解析标题、codec A_PCM/INT/LIT、8000Hz/mono/16bit及全部1600PCM字节，各与对应WAV精确同。实际时间戳尺度124999ns、Duration800，时长0.0999992秒；按一个采样间隔0.000125秒容差核对。SamplingFrequency/Channels缺省按官方Matroska规范8000/1，不把缺失当数据损坏；第一块fixed lacing两个640字节帧，第二块320字节unlaced，全部拼接相同。不称完整Matroska规范、广泛视频/音轨/编码、字幕附件章节/header editor/extract/复杂媒体/大坏文件/高级功能验收。直接中文键盘输入、客体联网下载、新机注册、迁移/跨版本、GUI市场更新回滚按钮和完整无人值守未验收。两个新代次首发实际出现Online check for updates通知，正常Close；不称离线或没有后台联网。

candidate-v50/root1/在线targets snapshot timestamp59，48目录项=46应用+2内部夹具，98实际签名目标、55公共文件。旧root/所有旧非catalogue目标pins及签名历史保持。Store IPC预缓存同版本更新和精确原代次回滚通过，回滚前完整359Core/109Store CP先保存读回，回滚后359/110旧所有字段/代次保持。第三原GUI后360/110正常Core/Store重启前后所有字段精确同，PID改变，固定exe、防护和不自动息屏配置保持。

第三原GUI改变system.reg/user.reg原字节、大小不变，321/20行实际私有差分和前后原文保留，原因未定。旧收据不改、不称旧注册表原字节跨第三GUI保持；当前新pins跨restart同，其余程序/输入输出/市场及既有固定验收数据保持。完整私有CP mkvtoolnix-20261012/after-restart.json SHA-2567a02adeb960770af02a57823ae521e43c2ddd79d18adb516cd69be8cf494900b；只读rolling-current-state-mkvtoolnix-20261012.py必须经严格SSH在客体执行。核对仅固定验收数据、本轮209程序文件和有限config/registry，非全部应用文件系统；完整SQLite历史不以最新64IPC摘要替代。

失败历史保留：初次source助手预期main.cpp而实际入口mkvtoolnix_gui.cpp，停止后基于已有固定归档受控补齐，不重下载/改旧成员。初始HTTP最终redirect未保存，不能重构其原观察。安装observer输出两条JSON导致宿主单对象quiet读取失败，实际安装/CP成功；只读354/108和209pins确认后继续，无重复安装。首独立解析器过严要求0.1秒失败，v2本地准备因忽略8000/1默认失败，v3因未支持fixed lacing失败；各原代码/原收据保持，新v4有限解析通过后才新正式重开。原新进程CtrlQ后Close modified settings待确认，终态门槛失败停止，没有before-market CP/签名变更；正常确认退出后受控重试通过。所有后续实际assert，false/缺assert/非终态/CP失败即停、不补造历史。35份原source/gui/acceptance及市场待执行阶段标签原字节冻结，后续完成单记；1631旧immutable保持。

认证/锁屏配置未改，不自动息屏保持。私钥、密码、凭据、登录截图和完整私有CP不入库。独立审查、真实Git index/commit字节核对及普通推送另记，不以审查替代客体GUI或私有CP独立重演。
