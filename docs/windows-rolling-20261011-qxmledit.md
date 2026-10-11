# Windows rolling acceptance: QXmlEdit 0.9.18

2026-10-11：Windows 累计 **36/1000**，12 个未解决候选保留。本轮仅验收以下小型 XML 流程，原 GUI、来源和签名收据中的市场待执行历史标签保持原字节，完成情况另记于 [publication](evidence/2026-10-11-qxmledit-publication.json)。

- 原正式代次 `gen-job-1791711953732-1` 与独立市场代次 `gen-job-1791712872837-4` 分别打开中文名 UTF-8/LF 三子节点 XML，保留中文 title 与 value，通过 Edit Element / Edit Text Node 将 note 的 BEFORE 改为 AFTER，并追加 ASCII ORIGINAL42 / MARKET42。
- 实际 GUI Save As 到 ASCII XML 路径，输出为 UTF-8/CRLF、单空格缩进；原输入/输出 154/167 字节，市场输入/输出 152/163 字节。独立解析核对 project、kind 属性、子节点顺序及全部文字。两份输入原字节保持；输出由应用 GUI 写出，检查助手未写出或规范化输出。
- 两个显式新正式进程重开实际文件、显示中文内容与修改结果；回滚后再以正式原代次重开。五次 GUI 均通过 File/Quit 正常退出 0，随后 wineserver 停止。
- Store IPC 使用已核对缓存提交同版本更新，生成独立市场代次；完整回滚前检查点 280 Core / 89 Store 先保存并读回，再提交精确原代次回滚。回滚后 280/90 保留所有旧行和字段；正常 Core/Store 重启前后 281/90 完整字段一致。GUI 更新/回滚按钮未验收。

固定官方安装包 `QXmlEditInstaller-0.9.18.exe` 为 32537109 字节，SHA-256 `9a0e8f6ad3fce01bb232ea0ac84e87029a81aee2fb9dfbe2152bbcc30a72ba48`；固定源码 `qxmledit-0.9.18-src.tgz` 为 19725693 字节，SHA-256 `6966c286b71239304c5038b419ae67dd2492c235c024186e7dd581fef1dad29f`。两者与官方固定目录单独提供的 SHA-256 文件内容一致，不声称最新版本、PGP/Authenticode 或可复现构建。

固定 main.cpp 声明主程序 LGPL-2.0-or-later，README 声明额外链接例外和组件信息。每代次 55 个安装程序文件分别固定；市场安装实际比较并匹配原代次全部 55 文件，随后固定市场自身全部 55 文件的独立摘要。助手中的大写 Uninstall.exe 跳过条件未匹配实际小写 uninstall.exe，因此未发生跳过；初稿的跳过描述另作失败更正记录。两个安装 COPYING 均与固定源码 COPYING、LGPLV2 资源原字节相同（25372 字节，SHA-256 `2713d3f6196c77a9e14d80f8d6434a2b7daa1275e7acc14ae3f42a74c4195099`），无 CRLF 标准化差异。Qt、QwtPlot3D、Oxygen 图标和完整捆绑组件许可闭包未核验。

首次两个新安装进程显示 LGPLv2 提示，正常 OK 后选择 Expert user、推荐树形显示并关闭欢迎窗口；后续也正常关闭欢迎窗口。此前小字误读成 GPLv3，原始 JPEG 保留，放大核对与固定 qrc/许可源文件证实没有该不一致。未独立提取安装二进制的内嵌资源，不将源码资源引用当可复现构建证明。无人值守启动未验收。

本轮没有 Core/程序修补、全局 context、原生启动器环境/工作目录、字体、DLL 或 Wine runtime 变更。固定安装定义 `6a05d932d5f1c124f846c07668761019ae37f5f072026127d7557d9a169ae76b` 使用原生 NSIS `/S` 与最后的 `/D=C:\QXmlEdit`（实际一个反斜杠，UTF-8 hex `2f443d433a5c51586d6c45646974`）；Core 保持 launcher-cwd-v11-r1，Store 保持 managed-msi-v2。

保留的检查助手失败见 [failures](evidence/2026-10-11-qxmledit-failures.json)：许可截图误读；PIL 不可用后仍尝试读取缺失图像；签名包摘要 entries 错报 37、实际签名目录 38；激活助手误把旧目录 v38 检查替换为 v39，在客体变更前断言停止。只读核对证明旧完整 277/88 状态与检查点一致，再以独立 v2 激活当前目录。宿主引号 SyntaxError 和缺失历史助手文件读取也保留；不声称所有历史门槛始终有效，不改写旧收据或签名历史。

当前 `candidate-v39`，root 1，targets/snapshot/timestamp 48，38 个目录条目、77 个实际签名目标、44 个公共目录文件。旧 root 字节、角色 44/45/46/47 和全部签名历史保留，TUF 信任只正常前移。完整私有最终检查点仅公开标识 `qxmledit-20261011/after-restart.json` 与 SHA-256 `9dba64c5c86fcdb811076fbab996b03264482bbbd15daabe6615df1c1f22410e`；密码、凭据、私钥及完整检查点不入库。

数据保持检查覆盖完整任务/SQLite 历史、全部代次/正式配置/信任/防护记录、已有固定验收数据和本轮两代次固定输入/输出、安装文件、AppData 与注册表摘要；不声称遍历全部旧应用文件系统。不自动息屏配置保持，认证/锁屏设置未修改。只读核对使用 `L:/project/FOS/.transfer/rolling-current-state-qxmledit-20261011.py`，必须经严格 SSH `rolling-guest-20261010.py` 在客体执行；旧固定总数助手只代表历史。

未验收 XSD/schema、XSLT、split/compare、sessions、namespace/属性编辑、base64、图形可视化、大型或损坏 XML、其他编码、直接中文键盘输入及完整高级编辑器功能；联网下载、新机器注册、迁移、跨版本升级未验收。Linux 独立记账：Ubuntu Kate、Snap 计算器维持 2 款，Flatpak Text Editor 下载阻塞及缓存保留，本轮未切换虚拟机或增加 Linux 数量。
