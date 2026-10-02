# Windows 滚动适配：Inkscape 1.4.4

日期：2026-10-03。环境：原 ForgeOS v12 隔离虚拟机，Wine 11.14。本轮新增一款独立应用，累计 **15/1000**；已有 11 个阻塞候选保留。统一服务端仍为设计稿。

## 来源、许可与安装

官方固定 Windows x64 EXE `inkscape-1.4.4_2026-05-05_dcaf3e7-x64.signed.exe`，118,507,928 字节，SHA-256 `fa21d25e56dbfe41e9b014a4f37689edda4bb41d5a3597fe2787dff6c10e651f`，与官方下载站公布的校验值一致。官方 tag `INKSCAPE_1_4_4` 对应提交 `dcaf3e7d9e6724cd18d6bd2b4f3d4f12ab691871`。该提交 COPYING 和 share/doc/LICENSE 支持完整二进制 GPL-3.0-or-later；附带组件声明适用。安装后许可与固定源码按换行解码后的文本一致，原始文件摘要不同。文件名中的 signed 不等于已验签：本轮未验证 Authenticode 或可复现构建。

安装作业 `job-1790971324958-23` 正常退出 0，原代 `gen-job-1790971324958-23` ready。NSIS 静默参数和启动路径见 [Recipe](evidence/2026-10-03-inkscape-recipe.json)，启动器摘要 `afef2f49862a87da96348cc80f07a4ee55ca159f86ea64753aa163044e2a1d92`。原授权登录继续使用，认证设置未修改；密码和签名私钥不入库。

## 原代真实 GUI 验收

从新空白文档绘制红色矩形、蓝色椭圆并输入单行 `Forge 15`，保存 verified.svg，导出全页 96 DPI PNG。SVG 恰有 rect、ellipse、text 三个对象；verified.svg 2028 字节，摘要 `356e7651bddd2a55e3b8a9cc22cee251045c5b77ff5235f36a0463360b3f3bc5`。PNG 794×1123，16,939 字节，摘要 `54eb0299584dda306b50982172770b8a19f7235b6fb64cf7b8fcda484ced80d8`；红色像素 101,814、蓝色像素 119,944，背景透明。

GUI 作业 24 关闭两份文档后正常退出；新进程 25 通过原生文件选择器明确载入 verified.svg，显示三个对象，正常退出，四份已记录文件字节保持不变。[后端证据](evidence/2026-10-03-inkscape-backend.json)、[签名验收](evidence/windows-inkscape-20261003.acceptance.json)同时绑定真实输出、截图和运行环境。

## 本地市场更新、回滚与持久化

candidate-v15 验签通过并激活：可信根仍为版本 1、摘要 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`；其他角色版本 24。公开目录只有 19 份 JSON，含 13 份验收凭据和 17 条目录项；15 款 Windows 应用计数，两条内部市场 fixture 不计。旧签名目标摘要保持不变。安装器和密钥位于仓库外。

市场 update `b8ea0739-b8c0-40ad-90a6-93e94b4205a3` 成功，新代 `gen-job-1790973033474-26` ready。受控复制 verified.svg 作为输入，明确不算新代 GUI 创建。新代 GUI 作业 27 显式打开此副本，新增绿色矩形，另存 updated.svg 并导出 updated.png；两个输出在操作前不存在。新 SVG 四个对象，前三对象属性和文字不变。逐像素比较：31,275 个变化像素全部为原透明区域上的绿色新增图形，原红蓝图形和其余 RGBA 像素不变。新 PNG 16,989 字节，摘要 `7b7ecef8c4d7b5e4dfdd906faa3c307dbe583e12377fe0cfc8d77ce507253b41`。

新进程 28 经文件选择器显式载入 updated.svg，显示四对象并正常退出。rollback `8e6f1cd3-f69d-47ce-ba9a-7683ba77f137` 成功恢复原代；新进程 29 显式打开原 verified.svg，显示原三个对象并正常退出。原代四文件、新代三文件摘要全部不变，两代保留且 ready。

受控重启 ForgeStore 后完整 snapshot 与 46 条 SQLite 作业全字段记录相等；既有 44 条记录摘要 `40a263e0bf505814fbe62b7ba3330c197d8cbe6318d7854ef59ea94c5f6f227b` 保持不变。运行中 Store 与 Core 二进制分别核对 `/proc/MainPID/exe`；bytes 1.11.1 修复、现有沙箱配置和原 Core 上下文保留。TUF 缓存完整解析文档及签名等于 v15 源元数据；不声称重新序列化缓存的原始字节不变。详见 [发布证据](evidence/2026-10-03-inkscape-publication.json)。

## 失败记录、范围与下一步

初次组合工具操作未画出矩形；微小文字曾误判为空，但 SVG 证明原文存在。字号栏焦点误操作插入 20/换行，文本拖动创建第二对象，误读菜单项把诊断文档保存。全部保留 vector.svg 和截图，另建空白文档完成正确流程。导出选择器默认 SVG 引发覆盖提示，已取消；一次菜单行误读导出 JPEG，保留该文件，随后选中真实 PNG 格式并核对文件魔数与像素。默认主机 Python 缺少 Pillow，切换已有依赖运行时验证。源文件路径猜测 404 和下载后摘要 API 不兼容也保留。详情见 [失败证据](evidence/2026-10-03-inkscape-failures.json)。本轮没有修改应用或兼容引擎代码，快捷键修饰行为根因未定位。

验收限于所述图形、ASCII 文本、SVG 保存/显式载入及 PNG 导出。高级路径、滤镜、扩展、打印、绘图板、中文输入和其他导出格式未验收；市场仅验证缓存同版本更新及安装代回滚，未验证客体在线下载、新机器注册、跨版本升级或远程公开分发。

发布后客体剩余 1,011,273,728 字节（约 0.94 GiB），下一次新安装前需扩展现有隔离虚拟机容量。保留基础镜像、两代应用、失败记录、市场数据库和 TUF 信任状态。

后续容量修复已完成：专用 Hyper-V 动态 VHDX/宿主 ext4 从 128 GiB 增至 256 GiB，原客体 overlay 从 40 GiB 增至 128 GiB，并在线扩展原 ext4 根分区。客体可用 94,008,922,112 字节（约 87.6 GiB）。启动分区、根分区身份和文件系统 UUID 保留；基础镜像持续只读，size/mtime/inode 未变（未以这些属性声称完整字节哈希相同）。修复后重新只读核对市场 snapshot/46 条全字段作业、历史 44 条、两个应用代/七文件、TUF 信任、服务二进制及保护配置，全部保持上述状态。详情见 [容量证据](evidence/2026-10-03-rolling-capacity.json)。
