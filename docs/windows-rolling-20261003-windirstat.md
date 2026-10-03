# Windows 滚动适配：WinDirStat 2.9.0（2026-10-03）

本轮完成 WinDirStat **2.9.0** 的固定来源核对、托管 MSI 安装、普通权限受控扫描、GUI CSV 保存与新进程显式加载，以及签名本地市场更新、回滚和 Core/Store 重启。累计完成 **18 / 1000 款独立 Windows 应用**。目录 v18 有 20 个条目，其中两项内部夹具不计数；10 项既有阻塞候选继续保留。

[官方固定发布](https://github.com/windirstat/windirstat/releases/tag/release/v2.9.0)发布时间为 2026-09-26T20:32:48Z，GitHub 标记 immutable、非预发布；标签对应 commit `9bf36ccd282d7e5059faff78896e677b54d50727`。官方资产 `WinDirStat-x64.msi` 在本地改用带版本的 basename `WinDirStat-2.9.0-x64.msi`，字节不变：2,433,024 字节，SHA-256 `0a7d214586bb7c7fd030b8f2295bc7caa7f88a0cec094b196c803e2dba0d134d`。与 GitHub 官方资产 digest 及同版本 `WinDirStat-Hashes.txt` 的 SHA256/正确文件名行均一致。校验文件原始 CRCRLF 字节保留，未使用 SHA1/MD5 作为完整性依据。

[固定 README](https://raw.githubusercontent.com/windirstat/windirstat/9bf36ccd282d7e5059faff78896e677b54d50727/README.md)明确应用采用 GPL v2 且不能升级为后续 GPL 版本，市场标为 **GPL-2.0-only**。固定 LICENSE.md、应用 resource license.txt 与 MSI license.rtf 均核对并记录字节 pin，应用 GUI About/License 标签也实际查看。部分源码和 CC-BY-3.0 图标仍有各自条款；未宣称完成所有组件对应源码审计、AuthentiCode 或可复现构建。

只读检查 MSI 的 Property、Directory、Component、File、Media、CustomAction 六表：ProductVersion 2.9.0，唯一 EXE 的表版本 2.9.0.2158，默认路径 ProgramFiles64Folder/WinDirStat，一份内嵌 CAB。原始安装 `job-1790990708319-1` 经支持的 managed msiexec `/i`、无界面、抑制重启及空目录属性配置成功退出 0，代次 `gen-job-1790990708319-1` ready。未手工解包替代安装。launcher 为 `Program Files/WinDirStat/WinDirStat.exe`，2,763,952 字节，SHA-256 `fe67a2377a176ede389816e8ca667d5588e6032403ae1a2ac913124739d71447`。六表检查有界，不代表完整 custom-action 安全审计。

原始输入仅由测试台准备，output 在 GUI 前为空。输入为 a.bin 1024 字节、nested/b.bin 2048 字节、nested/deeper/c.txt 38 字节、nested/中文.txt 19 字节，共四文件、两级子目录、逻辑 3129 字节。GUI `job-1790990760531-2` 拒绝提升权限并取消 extended scanning，选择 C:\ForgeQA\input，观察文件树、中文名称与 treemap，使用 File 菜单保存 wdoriginal.csv，正常退出 0。新进程 `job-1790991316284-3` 取消自动扫描选择，通过 File > Load Results From CSV/JSON 显式加载该文件，观察嵌套树、treemap 与 About/License，正常退出 0。

CSV 七个数据行全部按文件名核对：root Files=4、Folders=2、Logical Size=3129；nested Files=3、Folders=1、Logical Size=2105；每个实际文件逻辑大小匹配。输入和 CSV 在重新打开后字节不变。CSV Physical Size 是 Wine 环境观察值（总 16384），没有断言等同原生 NTFS 分配。原始加载后的界面显示逻辑大小比例，首次扫描显示物理大小比例；更新代次加载后则保留物理大小比例，因此不推断所有 CSV 导入会自动改变大小视图。

| GUI 输出 | 字节数 | SHA-256 |
| --- | ---: | --- |
| wdoriginal.csv | 808 | `8b6983f157c31a8ccdab2fe32eb89ccd2d9ff764ebec14cd55ac8d60d238992c` |
| wdmarket.csv | 914 | `a93dcca68b428e938ba676db638425b1e0e22d28c6537574f052c65d64e3ce37` |

签名目录 v18 使用原 root 1，timestamp/snapshot/targets 版本均为 27，到期 2026-10-29。tuftool 0.17 从上一代可信根 `clone --metadata-only` 返回 0；22 个公开 JSON 文件通过字节清单，16 份验收收据和所有旧目标 pin 保持。原始收据 SHA-256 `492bbeb7755b324c46de1391973067f55601db4f513894b03d626d790ad8e573` 冻结，后续生命周期单独记录。

市场缓存更新 `a356dc53-8595-4e52-adb2-96cb0270302a` 成功，Core 托管 MSI 作业 `job-1790991641587-4` 生成 `gen-job-1790991641587-4`。新代次独立准备输入，并增加 market-extra.bin（4096 字节），未复制原 CSV/output。GUI `job-1790991668002-5` 普通权限扫描五文件、两子目录、逻辑 7225 字节，保存 wdmarket.csv 并正常退出 0；`job-1790991831692-6` 新进程显式加载更新 CSV 并正常退出 0。八个 CSV 数据行逐一核对，输入与两个 CSV 的字节 pin 均保持。

市场回滚 `6b3c3eb5-7b8f-4ca4-883e-cb608e242393` 成功，现有最新其他 ready 规则选回原始代次，无额外 Core 切换。GUI `job-1790991921942-7` 显式加载原始四文件 CSV 并正常退出 0。两个 ready 代次和原始/更新输出继续保留。

Core 和 Store 重启后，**134 个 Core 完整终态作业、53 条 Store 完整数据库记录、全部代次与选择、已安装列表、市场目录、配置和服务约束保持一致**；本轮前 127 个 Core 作业和 51 条完整市场记录逐条不变。53 条市场记录规范 JSON SHA-256 为 `23fef2aad73353f0d3948505413202a579b62dfd4f86769a4863eb6d5a4f2606`。服务仍使用 Core managed-MSI-v10、Store managed-MSI-v2；Store NoNewPrivileges=yes、ProtectSystem=full、PrivateTmp=yes 保持。

信任根 SHA-256 始终为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。公开源文件全部字节不变，TUF 缓存解析后的完整内容与签名在重启前后相同；不对缓存序列化字节作断言。latestKnownTime 从 `2026-10-03T01:39:44.322201652Z` 单调推进至 `2026-10-03T01:46:54.697451618Z`。没有信任重置、降级或验证绕过。

保留的测试脚本问题：大小写不同的官方 SHA256 文本最初被错误比较；模板替换误改市场缓存期待字节数，断言在缓存写入/入队前停止；异步市场更新未结束时首次 launch probe 提前停止；noVNC 工具栏遮住导航控件；Ctrl+L/Ctrl+S 不匹配实际 Ctrl+Alt 快捷键，改用可见 File 菜单。以上未注入扫描结果、未误计完成。测试桌面自行使用现有授权登录，未移除密码或改变认证。

本轮没有新增应用或 Core/Store 行为修补；支持的 MSI 默认路径和原生 GUI 工作流已可运行。验收限于所列普通权限受控扫描和 CSV 工作流、缓存同版本更新及代次回滚；全盘、提权、扩展扫描、重复文件、清理/删除、JSON、原生 NTFS 物理分配正确性、客户机在线下载、全新机器注册、跨版本升级和远端安装包再分发均未验证。

证据入口：[固定来源与许可](evidence/2026-10-03-windirstat-source.json)、[MSI 六表](evidence/2026-10-03-windirstat-msi-inspection.json)、[配方](evidence/2026-10-03-windirstat-recipe.json)、[原始收据](evidence/windows-windirstat-20261003.acceptance.json)、[原始后端验证](evidence/2026-10-03-windirstat-backend.json)、[市场生命周期](evidence/2026-10-03-windirstat-publication.json)、[失败记录](evidence/2026-10-03-windirstat-failures.json)、[累计台账](windows-1000-progress.json)。安装包、签名私钥和密码不入库。
