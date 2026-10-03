# Windows 滚动适配：JPEGView 1.3.46（2026-10-03）

本轮完成 JPEGView **1.3.46** 固定来源/许可核对、托管 MSI 安装、中文路径图片打开、GUI 顺/逆时针 90 度旋转、PNG 保存与显式新进程重开，以及签名本地市场更新、回滚和 Core/Store 重启。累计 **20 / 1000 款独立 Windows 应用**。目录 v20 有 22 项，两项内部夹具不计数；10 项既有阻塞继续保留。

[官方发布 v1.3.46](https://github.com/sylikc/jpegview/releases/tag/v1.3.46)发布于 2023-10-07T07:35:29Z，非预发布、未标记 immutable，源码 commit `af075dbd1918554ea4491b42646f03286749a0e6`。固定 `JPEGView64_en-us_1.3.46.msi` 为 5,165,056 字节，SHA-256 `3108a85b5d408fa17027b25454f0ee7e7b450e680dbc1dab68d26d9099e18bf2`，完整包与官方发布正文的正确文件名/校验行一致；GitHub API asset digest 为空，没有冒称 API digest 匹配。未宣称最新版本或发布不可变。

[固定 LICENSE.txt](https://raw.githubusercontent.com/sylikc/jpegview/af075dbd1918554ea4491b42646f03286749a0e6/LICENSE.txt)与安装器 License.rtf 均将主程序标为 **GPL-2.0-or-later**；COPYING.txt 完整 GPL v2 文本也按固定 commit 核对。各图像库仍有自己的条款，完整组件对应源码/再分发审计未完成。安装 LICENSE.txt 为 893 字节、SHA-256 `63c5c94f0a6a3b122f677275ac89ee9165e895e0cd1d4ec01147c43a22f28319`，仅 CRLF→LF 后 875 字节、SHA-256 `3834a17ceecdf637633fedab1eb95a49fd12c8524a1c24b1edd131c289d3f4ef`，匹配固定源文件；原始字节不相同。GUI About 实际显示 JPEGView 1.3.46.0（64-bit）及 GPL 许可说明。

只读检查 MSI 六表：Property 19 行、Directory 6、Component 58、File 54、Media 1、CustomAction 11。ProductVersion 和 EXE 表版本均为 1.3.46.0，APPLICATIONFOLDER 指 ProgramFiles64Folder/JPEGView，一个内嵌 cab1.cab。此检查有界，不代表完整 custom-action 安全审计。原始安装 `job-1790994464257-1` 通过正常 managed msiexec `/i /qn /norestart REBOOT=ReallySuppress`、空额外属性成功退出 0，代次 `gen-job-1790994464257-1` ready，没有手工解包替代安装。Unix Wine 退出 0 已验证，完整 Win32 MSI 退出码未单独捕获。launcher 为 `Program Files/JPEGView/JPEGView.exe`，2,947,584 字节，SHA-256 `59cf443bd42a6643319898c1f30a6a1355b56281fa6aa5c9c4472fbb81bc8d0f`。

原始输入由测试台准备，C:\ForgeQA\input\彩色.bmp 为 320×200、24-bit 未压缩 BMP、192,054 字节，SHA-256 `c64eba6205f507b0d573a34f995809d64f2742dbf57540adce2714b48bdf454f`。四象限依次红/绿/蓝/黄，左上有非对称白色 L；output 在 GUI 前为空。`job-1790994542617-2` 经实际文件选择器打开中文图片，观察图案，右键 Transform image > Rotate +90 顺时针旋转，Ctrl+S Save processed image，在保存选择器选 PNG，另存 jvoriginal.png，查看 About 并正常退出 0。保存 processed image 后标题仍是输入 BMP；PNG 成功由实际磁盘输出和后续新进程重开证明，没有将标题当保存证据。

输出 PNG 每个 chunk CRC、IHDR、IDAT 解压及 PNG 行过滤均验证。200×320、8-bit RGB、非交错；64,000 个 RGB 像素与独立计算的顺时针 90 度旋转逐一完全相等，differentPixels=0、maximumChannelDelta=0，解码 RGB SHA-256 `49b58b154f8a05d1e91fc98982ee6cb39d7a0e349f85c43dd88d15cc954146d3`。原始 BMP 保持。新进程 `job-1790994740097-3` 用选择器显式打开 PNG，观察标题、旋转象限与白 L，再正常退出 0；PNG 字节保持。

| GUI 输出 | 尺寸 | 字节数 | SHA-256 |
| --- | --- | ---: | --- |
| jvoriginal.png | 200×320 | 1380 | `d14b53820306315071c47b3b6733d5993f07f886b0e30b10a21cf11c7ae42c3b` |
| jvmarket.png | 240×360 | 1486 | `e823d92e739ec05f4fe987562d3c79aa8ea21f7aa515ca4601bb8f9c917f313b` |

签名目录 v20 保留 root 1，timestamp/snapshot/targets 均为 29，到期 2026-10-29。tuftool 0.17 使用上一代可信根 `clone --metadata-only` 返回 0；24 个公开 JSON 逐字节核对、18 份收据及旧目标 pin 保持。原始收据冻结 SHA-256 `33204cd925622a25af3bd1aa61c41e3f50abcd59acf6da3d42ae1eace5d3006b`，后续生命周期另记。激活时 55 条完整 Store 记录和旧已安装项不变。

市场 verified-cache 更新 `e75721e4-8522-4b6c-ba96-b00b5a81d520` 成功，Core MSI 作业 `job-1790995058054-4` 生成新代次 `gen-job-1790995058054-4`。新输入 C:\ForgeQA\input\市场彩色.bmp 独立准备：360×240、259,254 字节、SHA-256 `84e2023c5a5c11c5d9274b75d71b26b9e10bf740823eb5f7656494377588cad8`，洋红/青/橙/绿四象限、白 T 和右下深色方标。不同尺寸/颜色/标记，output 为空，未复制原输出。GUI `job-1790995113718-5` 显式打开新输入，使用实际菜单列出的 Up 快捷键 Rotate -90 逆时针旋转，另存 PNG 并退出 0。输出 240×360，86,400 个 RGB 像素与预期逆时针旋转全部相等，解码 RGB SHA-256 `874e3143c9a6ab8e80f029dcdb3c9c583d89ce527d0e8e36a33c4a43641164ca`。`job-1790995292704-6` 新进程显式打开更新 PNG，真实图案可见，正常退出 0；原图/原输出继续保持。

市场回滚 `1c60796d-fa8a-408a-8abe-56cc3c99edad` 成功，现有 newest-other-ready 策略选回原始代次，两个 ready 代次均保留，无额外 Core 切换。`job-1790995345219-7` 新进程显式打开原 jvoriginal.png，观察原始顺时针图案与白 L，截图后正常退出 0。

Core/Store 重启后，**149 个完整终态 Core 作业、57 条完整 Store 数据库记录、全部代次与选择、市场目录/已安装列表、配置及服务约束一致**；本轮前 142 个 Core 作业和 55 条市场记录逐条不变。57 条规范 JSON 市场记录 SHA-256 `0e2123be61dbf8b72d5bef06c3e98a8789e076d387ab78b7afc6b3733100a3e9`。两套 BMP/PNG 与安装许可字节保持，运行仍为 Core managed-MSI-v10 / Store managed-MSI-v2；Store NoNewPrivileges=yes、ProtectSystem=full、PrivateTmp=yes 保持。

根 SHA-256 始终为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。24 个目录源文件逐字节保持，缓存解析后完整文档/签名保持，不对缓存序列化字节作断言。latestKnownTime 从 `2026-10-03T02:37:25.463999263Z` 单调推进至 `2026-10-03T02:43:42.44391294Z`。服务 readiness 本次均首轮通过，没有信任重置、降级或绕过。

测试台问题保留：短 C 盘名称第一次点击落空；两次不存在的 helper 路径与一次不存在的 tuftool sign help 子命令；noVNC 直接 Windows 路径输入错误，改用可见目录导航；上传脚本首次误在 Windows 主机运行，以及随后 stage/activation 对缺失源的提前停止，均发生于目录/drop-in/TUF 修改前。改为 builder 上传、逐步检查退出状态后成功。现有会话自动锁定后自行使用授权登录，没有移除密码或改变认证。八张截图来自实际完整 GUI；密码、签名密钥和安装包不入库。

本轮没有新增 Core/Store 或应用行为修补。验收限于受控 BMP/PNG 旋转文件工作流、缓存同版本更新及代次回滚。裁剪、任意角度、色彩调整、其他格式/EXIF/ICC/RAW/PSD/动画/批量/幻灯片/打印、文件关联/注册表结果、客户机在线下载、全新机器注册、跨版本升级、远端安装包再分发、AuthentiCode、可复现构建和完整组件/custom-action 审计均未验证。

证据入口：[固定来源与许可](evidence/2026-10-03-jpegview-source.json)、[MSI 六表](evidence/2026-10-03-jpegview-msi-inspection.json)、[配方](evidence/2026-10-03-jpegview-recipe.json)、[原始收据](evidence/windows-jpegview-20261003.acceptance.json)、[后端与像素验证](evidence/2026-10-03-jpegview-backend.json)、[市场生命周期](evidence/2026-10-03-jpegview-publication.json)、[测试台失败记录](evidence/2026-10-03-jpegview-failures.json)、[累计台账](windows-1000-progress.json)。继续选择来源、许可、固定版本与完整安装包可核对的未完成独立应用。
