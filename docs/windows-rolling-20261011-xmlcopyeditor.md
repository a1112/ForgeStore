# Windows rolling acceptance: XML Copy Editor 1.3.1.0

2026-10-11：Windows 累计 **37/1000**，12 个未解决候选完整保留。本轮有限验收和本地市场完成见 [publication](evidence/2026-10-11-xmlcopyeditor-publication.json)。原 GUI、source、acceptance 的冻结字节和“市场待执行”历史标签保持，后续完成另记。

- 正式原代次 `gen-job-1791715752381-1` 和独立市场代次 `gen-job-1791716480475-4` 分别打开中文名三子节点 UTF-8/LF XML。普通 BEFORE→AFTER 替换、ASCII ORIGINAL42/MARKET42 追加，通过实际 F2 良构检查，GUI F12 Save As 到 ASCII 路径。
- 两份输入 156/154 字节且原字节保持；输出为 UTF-8 BOM/LF，169/165 字节。独立解析 document、kind 属性、title/note/value 顺序和文字。助手只写测试输入，不写出或规范化输出；合成 XML 不计独立应用。
- 两个新正式进程先关闭自动恢复的文档，再经真实 Open 对话框显式加载保存文件，显示中文内容和修改结果，并再次通过良构检查。回滚后第三次原代次同流程重开。五次正式 GUI 经 File/Exit（Alt+F、X）正常退出 0，wineserver 正常停止。About 画面版本为 1.3.1.0。
- Store IPC 使用核对缓存提交同版本更新，形成独立市场代次。完整 287 Core / 91 Store 回滚前检查点先保存读回，再精确回滚原代次；回滚后 287/92 全旧字段保留。正常两服务重启前后 288/92 任务、完整 SQLite 行和全部字段精确一致。GUI 更新/回滚按钮未验收。

固定官方安装包 `xmlcopyeditor-1.3.1.0-x64-install.exe` 20046337 字节，SHA-256 `7ba6900595d0ca68b79b2c14a43538d1190bf717228651701d40fc693660caf4`。这是官方 HTTPS 下载后的本地固定摘要，未核对上游安装包摘要。源码 `xmlcopyeditor-1.3.1.0.tar.gz` 5273837 字节，SHA-256 `e871ca97b86acaf177809f6f9a32e34d3e3da6b27c2a1104574a8fb097d07c91`，与官方 DSC 中另一文件名 `xmlcopyeditor_1.3.1.0.orig.tar.gz` 的大小和 SHA-256 完全一致。明确区分别名，DSC OpenPGP 签名未验证；首轮解析到的镜像重定向 URL 未保存。不声称最新版本、Authenticode 或可复现构建。

固定主源码声明 GPL-2.0-or-later。两代次 `copying/xmlcopyeditor/copying.txt` 均与该特定源码成员逐字节相同：744 字节，SHA-256 `a9c68e5082e3184f9a8aa39da77f7af0d862593080941bc4d3179a27ff2c064f`。根 COPYING 只是许可证/致谢目录指针，不能当作完整 GPL 文本。wxWidgets、Xerces/Xalan、PCRE、Aspell、Hunspell、DocBook/DTD 和全部捆绑组件许可闭包未核验。

原生 Inno 参数 `/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /DIR=C:\XMLCopyEditor /TASKS=disable_registry`，DIR 含一个实际反斜杠。两代次只读核实三份 Wine 注册表均无 SourceForge Project 桌面关联任务条目。本轮无 Core/程序修复或全局 context、launcher environment/workingDirectory、字体、DLL、Wine runtime 变更；仅通过应用原生 Ctrl+U 放大文字。每代次 499 个安装文件分别固定。跨代次 498 个原字节相同，唯一差异是 Inno 卸载日志 unins000.dat：各 70133 字节、18 处字节差异；生成变化原因和卸载功能未验收，不宣称两个日志同字节。

保留 [failures](evidence/2026-10-11-xmlcopyeditor-failures.json) 中全部已识别失败：Python DSC TLS 失败后普通 TLS curl 成功；DSC 文件名断言错误后按别名摘要核实；冻结清单字段误读、缺失模板读取；exec 变量复用导致第一次检查点收据库摘要标签错误（独立读回已更正）；无效 GUI 快捷键和 noVNC 遮挡、可访问定位失败；jobs.get 返回结构误读；市场卸载日志不等断言；诊断 JSON 输出格式错误；锁屏上过早 Open 快捷键。失败助手及原收据保留，不宣称所有历史门槛始终有效。

回滚后正常第三次原 GUI 退出使 system.reg 与 user.reg 原字节变化，大小仍为 3423303/102745。严格旧 pins 检查在重启前失败即停，未保存 before-restart、未执行重启；首次只读诊断误判仅一份注册表变化也停止。随后独立核实两份新实际摘要，其余原代次程序/输入/输出/固定配置、市场全部固定 pins 和既有应用 pins 保持。旧原始注册表只有摘要，不能进行原文差分，原因未确定；不声称原注册表跨第三次 GUI 原字节保持。新字节单独私有固定后，以严格新 pins 保存读回检查点，再正常重启，两服务重启前后完全一致，见 [registry change](evidence/2026-10-11-xmlcopyeditor-registry-change.json)。原冻结收据没有改写。

当前 candidate-v40，root 1，在线 targets/snapshot/timestamp 49，39 个目录条目、79 个实际签名目标、45 个公共目录文件。固定根和全部旧目标摘要保留，全部签名历史不改、信任只正常前移。私有完整最终检查点仅公开标识 `xmlcopyeditor-20261011/after-restart.json`，SHA-256 `adb5ac88184f2e5760e2a8d60d71d0ffb3f78b9ba1b492220318b48931214626`，不公开完整内容、私钥、密码或凭据。

核对覆盖完整任务/SQLite 历史、全部代次/正式配置/信任/防护字段、旧固定验收数据、本轮两代次程序文件、输入/输出、AppData/注册表有限固定摘要，不声称遍历全部旧应用文件系统。不自动息屏配置保持，认证/锁屏未修改，采用现有账号正常登录。只读助手 `L:/project/FOS/.transfer/rolling-current-state-xmlcopyeditor-20261011.py` 必须经严格 SSH `rolling-guest-20261010.py` 在客体执行。Core 保持 launcher-cwd-v11-r1，Store 保持 managed-msi-v2，Provider v2 未部署或 GUI 验收。

仅小型良构 XML 流程通过，不等于完整 XML 规范或高级编辑器验收。XSD/DTD/RelaxNG 验证、XSLT/XPath、全局替换、命名空间/属性、拼写、打印、大型/损坏文件、其他编码、直接中文输入、全部高级功能、联网下载、新机器注册、迁移、跨版本升级和无人值守未验收。Ubuntu Kate/Snap 计算器仍为独立 2 款；Flatpak 下载失败缓存保留，本轮未切换或同时运行两 VM。

此前 QXmlEdit 本地提交 `cc3c5626b73fe1c9cd71e0c352e9b83ba40ed0ae` 未推送，设备验证已过期。GitHub 认证仍阻塞普通推送；本轮不重复安装已完成应用、不强推、不输出或保存凭据。后续恢复认证后可普通推送累积提交。
