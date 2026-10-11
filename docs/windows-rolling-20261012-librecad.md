# LibreCAD 2.2.1.5 Windows 有限 ASCII 绘图验收（2026-10-12）

新增1款，累计45/1000；13项未解决候选保持。日期采用亚洲/上海，实际客体UTC时间不改。Ubuntu两款独立验收不变，本轮没有切换虚拟机。合成DXF不另计应用。

官方固定MSVC Windows x64安装包54350692字节、SHA-256 738912fea40aca934689444af9c5cd281572417983cc081c204fe6c779f6941d，匹配官方GitHub API size/digest。标签v2.2.1.5关联commit 7ebab007d9eb4c68609388b835a2487648f0877b；源码归档20201974字节、SHA-256 e13ca18a943933978c3474f845d3fd7cb15c521a49cada8721c08f95dfe7aa37仅本地固定摘要，不称上游源包摘要验证或最新版本。17份源码/许可/NSIS成员原字节固定。main.cpp和LICENSE主程序GPL-2.0-only，不称or-later；官方发布LICENSE4035字节摘要匹配API且与固定源码同字节。1613安装文件没有COPYING/LICENSE/gpl-2命名文件，未提取嵌入许可或称安装许可与源码比较。全部Qt/MSVC/plugins/字体捆绑许可闭包、Authenticode/PGP、源码二进制对应和可复现构建未核验。

正式原代次gen-job-1791743660835-1和独立市场gen-job-1791744585038-5，冻结定义sha256:9467c02e582ecc7f68c612bb840f23e207cb64f8b8cec65c44c065c53ceb7c46。全部1613程序文件跨代次原字节相同；LibreCAD.exe7681536字节SHA-2569b5ec58c40c9e88b2fc110678f03c5b071191cd8dd79b3c68d6620b269c03773。原生NSIS /S和/D=C:后一个反斜杠再LibreCAD，实际14 UTF8字节、hex 2f443d433a5c4c69627265434144。本轮无Core/程序、全局context、launcher环境/workingDirectory、字体包/DLL/runtime修复。

原中文名385字节图形在真实GUI导入失败；同一原字节复制为ASCII名字后显示。首个job-1791743774031-2只作路径诊断，正常退出0不当验收，截图01/02保留为失败/诊断。中文路径导入未修复；正式与市场验收均限定ASCII路径。首次启动手动选择Millimeter/English GUI/English command，不称无人值守。

原385字节与市场384字节小型单层DXF各有四条矩形LINE和一个圆。使用应用原生GUI命令输入line、0,0和40,20/30,18新增一条对角线，实际GUI Save As输出19270/19269字节。两个新正式进程显式Open输出、View/AutoZoom显示全部图形；独立解析group pairs/EOF及ENTITIES坐标，输出各五条线和一个圆，圆心20,10/15,9、半径4/3。输入及输出保存字节保持，不声称完整DXF规范。市场首发Open后活动文档仍为unnamed document2，Drawings菜单明确选中已导入market-ascii.dxf后才绘图；空白活动窗口不当加载通过。AutoZoom使文档标星，后续File/Quit选择Discard，仅视图变更不写回输出，实际字节重新核对。

五次正式GUI File/Quit退出0与wineserver停止；CtrlQ最初无效，终态断言失败即停，正常File/Quit后才提交新正式任务，不称快捷键退出验收。回滚后的原代次新进程显式重开并显示原输出。块/其他图层、尺寸文字填充、复杂编辑、广泛DXF/DWG/导入导出、打印、大型损坏图形、插件、高级CAD、直接中文输入、客体联网、新机注册、迁移/跨版本及完整无人值守未验收。

candidate-v49/root1/targets snapshot timestamp58，47目录项=45应用+2内部夹具、96实际签名目标、54公共文件。全部旧target pins/root原字节和签名目录历史保持。Store IPC预缓存同版本更新cfbb5fac-f3c9-44f1-bc7a-47608ed1b314成功，回滚前完整352 Core/107 Store检查点先保存读回，回滚9f9ebbe3-156d-42c6-9804-7cb07aa43dc3成功后352/108旧全部字段与全部代次保持、精确恢复原代次。最后原GUI退出后353/108，正常Core/Store重启前后全字段精确一致，PID改变且固定exe摘要保持；GUI市场更新回滚按钮未验收。

第三原GUI改变system.reg/user.reg原字节、大小保持，前后原文及287/20行私有差分保存，原因未确定。原收据不改、不称旧原字节跨第三GUI保持；当前实际新pins跨重启同，其余固定程序/输入输出/市场/旧固定验收数据保持。完整私有检查点librecad-20261012/after-restart.json SHA-256 4c9c044722d0b5cfbcff537d82a05b0e48d31ca3c670ed9cd8bc45e911cf49ff；当前rolling-current-state-librecad-20261012.py必须经严格SSH在客体执行。仅固定验收数据、本轮1613程序文件及有限配置/registry覆盖，不称遍历全部旧应用文件系统。完整SQLite历史不以最新64 IPC摘要替代。

保留observer准备错误、fixture tuple/JSON array写后读回失败及原收据、中文导入错误和CtrlQ终态失败。后续逐步实际assert，false/缺assert/非终态/CP失败即停。认证/锁屏未改，不自动息屏配置保持。密码/登录画面、私钥、凭据和完整私有CP不入库。1523旧immutable、34份本轮原source/gui/acceptance与市场待执行阶段标签原字节保持，市场完成另记。独立审查、真实Git index/commit字节核对及普通推送结果后续单记。
