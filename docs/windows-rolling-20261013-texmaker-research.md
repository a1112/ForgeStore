# Texmaker 5.1.4 Windows 来源研究与 MSI 门槛阻塞（2026-10-13）

Windows 保持 61/1000、16 个未解决候选；本轮 Texmaker 仅来源和静态包研究，没有正式定义、任务、安装、GUI、市场或新增计数。既有 61 accepted 与 16 pending 全部字段保持。当前运行仍为 Fityk 后的 candidate-v67/root1/role76、482 Core 全任务及 142 完整 Store SQLite 全字段；当前完整私有 CP 沿用 Fityk after-final-restart，摘要 59b9c7c18d93f7ab85980338c41f8651c43be339045c5d373528f4572cb4b957，不入库。

官方固定下载页实际链接给出 Texmaker_5.1.4_Win_x64.msi 与 texmaker-5.1.4.tar.bz2。两个完整 HTTPS 200 GET 与 Content-Length 读回通过，两路线终态后才检查。安装包 85053867 B，SHA256 8f992ea4236b4094d76bc9a225d849119b2e9aaf36da630d15ec45d1b9c18668，MD5 f48f1377275788553a0be356aa70b921 匹配官方 890 B md5sum.txt 原文。MD5 匹配不等于强密码学真实性证明。源码 15277027 B，SHA256 320504d62b71b591c06b7d0b1202ac750a3112fe2ca328e30580055eb308fea6，仅完整实际本地摘要；官方当前 md5 文件未给这份旧源码摘要。3899 个源码普通成员中 14 个选定原文保存。

固定 texmaker.cpp 主声明为 GPL-2.0-or-later；根 license.txt 开头声明 GPL version 2，紧随正文却是 GNU Library GPL version 2，utilities/license.txt 同样混合，utilities/license_html.txt 则是普通 GPL2 正文。该实际文本差异保持，没有将混合正文解释为整个 Windows 包许可结论。完整 Qt/PDFium/Hunspell/codecs/字典/图标/翻译等捆绑许可闭包、安装主许可比较、Authenticode/PGP、源二进制对应、可复现构建及最新版本均未核。包与程序二进制不公开。

宿主 Windows Installer API 只读核实 ProductName=Texmaker 5.1.4 (64-bit)、ProductVersion=5.1.4.0，235 条 File 行；Template Summary 实际为 Intel;1033。主组件 Attributes=256 只是 MSI 静态属性，不代替主程序实际 PE 架构测量。严格传入外层固定 MSI 检查器（SHA256 d5d8f21631f3d8c531392c478badf9c550a9d484da2f4e087fe742ac936b50eb）并请求 x86_64，真实退出 1，stdout 0 B，stderr 为 MSI Template Summary architecture differs。该实际架构门槛失败后停止任何 applications.upsert/jobs.submit。未把声明改成 x86、未放松 guard、未做另一架构重试或安装；也不据拒绝声称官方 MSI 损坏或真实 Windows 无法安装。主程序 PE 架构尚未独立测量。

初 URL 选择器错误要求原相对源链接含前导斜杠，在源包/安装包/checksum GET 前 assert 停止；唯一 V2 使用实际末级路径选择并核既存首页字节，没有重复下载已成功页面。一次账本 counts KeyError、缺失猜读助手、宽泛搜索输出截断、外层不存在 /usr/bin/7z 的读取断言以及后续宿主许可控件查询退出 1/0 B 均保留原工具历史和范围限制，不把它们当完成检查或补造 exact raw。外层真实只读工具 inventory 未发现 7z/7za/7zz/bsdtar/cabextract，没有安装新工具或解包运行程序。严格 MSI 检查器原 stdout/stderr 与报告私有保存并核摘要。

只读当前状态检查通过原严格 SSH 发送 rolling-current-state-fityk-20261013.py stdin 执行，核全历史/信任/固定数据及选定 Fityk 程序与配置，非全应用 FS；Core 查询可写已有 scratch。没有客体应用/服务/VM 重启、recover、TUF 激活或信任变更。Ubuntu KateAPT/Snap 计算器两款独立记账，Flatpak 两次 runtime 超时未安装缓存保持，本轮未运行 Ubuntu VM。

首次公共写入前核 3769 旧证据工作区/index/HEAD 原字节；完整 repo 为 5010 tracked，1241 范围外 index 等于父 HEAD，不称范围外或全工作区全部原字节相等。当前旧冻结 Fityk 文件不可重写；本轮新的 Git 预检、独立 FINAL 审查及真实 index/commit/push 审计另记。下一轮选择其他官方固定来源候选或受控定位已有问题；保持已安装候选和全部失败/TUF/root/签名/信任历史。
