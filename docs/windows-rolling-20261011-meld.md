# Meld 3.22.2 Windows 有限鼠标合并流程（2026-10-11）

新增1款，累计42/1000；13个未解决候选（含已安装但未验收的BowPad）及旧台账字段保留。仅有限鼠标比较、单差异块合并、GUI另存和新进程重开通过。键盘编辑、导航与快捷键仍有问题，未修复，非完整差异编辑器验收。合成文件不计应用，Ubuntu仍两款独立验收，本轮未切换虚拟机。

官方GNOME固定MSI28401664字节，SHA-256 e1525657ff0e0d9762619b4930fa758756d5e766967764bd12c2f3914f16ece8，未核上游公开安装包SHA。官方源包676376字节，SHA-256 46a0a713fbcd1b153b377a1e0757c8ce255c9822467658eacfbd89b1e92316ef，与固定官方.sha256sum匹配。8份源码成员及官方SOURCES通知原字节固定；SOURCES说明标签构建并不证明本次二进制与源码对应。主meldapp.py为GPL-2.0-or-later，安装COPYING18092字节与固定源码COPYING原字节相同，cx_freeze许可另外固定。完整GTK/Python/cx_freeze/GtkSourceView/OpenSSL等许可闭包、二进制对应、PGP/Authenticode与可复现构建未核验；不称最新版本。原下载慢但完成后核对使用，另一次curl重试HTTP200在15891894/28401664字节时60秒超时，部分缓存保留且未安装。

既有原生managed MSI默认安装，无Core/程序/global context/launcher environment/workingDirectory/字体包/DLL/runtime修复。6个结构表检查成功，客体实际安装1303文件分别pins；公开表收据仅预览，不冒称全表。Meld.exe67584字节SHA-256 fb9f6246ae5dba651cd961b91f278e8bf8c97a9060e61b62729916af60ac9d2f。两代次1303文件跨代次原字节同。GUI设隔离应用Monospace24偏好，只为可读性，不是键盘修复。

首次正式job-1791735356726-2可显示中文名58/57字节文件，但CtrlEnd/Home/Backspace/Down/CtrlQ等不可靠；ORIGINAL42输入位置错误且未保存。通过caption X、Close without Saving正常退出0，磁盘输入不变、没有输出，仅失败诊断。原截图01/02/03保留，不计通过。字体选择最初CtrlA/输入24无效，取消后用GUI加号实际设置24；noVNC遮挡按钮时先误点键盘图标，读取新鲜状态后正确折叠。GTK日志报告不能加载布局而无法翻译按键。静态GTK3.24.41 gdkkeys-win32.c及只读布局检查仅支持诊断方向；三条猜测DLL路径和注册表文本搜索不是全面runtime清单，不声称定位具体运行时函数/HRESULT或正式/通用修复。

正式原代次 gen-job-1791735260445-1，独立市场代次 gen-job-1791736703487-5，冻结定义 sha256:baf77e4683a7d7a29d10b3f408b2bd894bc7e717e7f3fa0d35c8c3a0dd19d0fb。新的原正式和市场正式流程均通过鼠标单差异块右向左合并，左侧显示AFTER，实际Save Left Pane As到ASCII路径。原输入58/57字节，输出57字节；市场输入56/55，输出55字节；两份输出分别与其右侧输入原字节及SHA精确同，所有输入原字节保持，UTF8/LF和中文内容匹配。两个新正式进程使用输出路径和对应右输入显式参数，实际显示Files identical；截图04/05/06与07/08/09分别证明原与市场流程。回滚后新原进程显式重开显示原AFTER与Files identical，截图10。五次通过GUI正常caption X退出0并停止wineserver；首个失败键盘诊断额外退出0不计通过。未声称GUI About版本实测。

第一次市场安装后启动助手误用arguments字段，Core在新launch任务创建前拒绝。停止后完整核对after-market-install CP、输入与程序，再用独立更正助手argumentOverrides正常提交；未重复安装。第一次当前状态助手实际assert331/102与完整检查点一致，却输出硬编码历史323/100标签，原助手和收据保留，v2改为完整列表长度并新鲜严格SSH核对。所有依赖操作逐步实际assert；false、缺assert、任务非终态或检查点失败即停，不只判断exit0、不补造历史。

当前candidate-v45/root1/在线targets、snapshot、timestamp54，44目录项为42应用和2内部夹具，89实际签名目标、50公共目录文件。旧root、全部角色53及以前目录原字节保持，MPC-HC32原证明与33更正继续继承。Store IPC预缓存同版本更新成功；完整330 Core/101 Store回滚前CP先保存读回，再提交精确原代次回滚，回滚后330/102旧全字段及全部代次保持。第三原GUI后331/102，正常Core/Store重启前后完整全字段精确同，正式配置、信任、防护、运行时摘要与不自动息屏保持。GUI更新/回滚按钮未验收。

第三原GUI后system.reg/user.reg/meld.log原字节变化，前两大小相同，log51262→51708字节，298/20/10行实际差分和前后原文私有保留，原因未确定；不能称原字节跨第三GUI保持。程序、输入输出、市场全部pins和旧固定验收数据不变，当前实际新pins跨重启相同。私有完整CP meld-20261011/after-restart.json SHA-256 9360b609a9a76b5794014d51b923f43ada92959ecb621b91183a3f64d78c8280；只读rolling-current-state-meld-v2-20261011.py必须严格SSH在客体执行。覆盖固定证据、本轮1303程序文件和有限配置/注册表，不遍历全部旧应用文件系统，不用最新64条IPC摘要替代完整SQLite历史。

原26份来源/GUI/acceptance冻结原字节及市场待执行历史阶段保留，后续市场完成另记，1185份旧immutable保持。大文件/损坏文件、其他编码/换行、三路或多差异合并、目录/VCS、正则过滤/撤销/高级、直接中文输入、联网、新机器注册、迁移/跨版本、完整无人值守与完整组件许可闭包未验收。认证锁屏不改，密码/登录画面/凭据/私钥/完整私有CP不入库。GitHub既有认证阻塞，本轮未重新查远端/认证、未发设备码、未推送；累计本地提交待恢复认证后普通推送。独立审查及真实index/commit字节核对另记。
