# Windows 滚动适配：Audacity 3.7.9（2026-10-03）

本轮完成 Audacity **3.7.9 维护分支**的原始安装、真实音频文件工作流、签名本地市场激活、缓存更新、更新代次重新保存/重开、原始代次回滚/重开及 Core/Store 重启。累计完成 **17 / 1000 款独立 Windows 应用**；签名目录 v17 有 19 个条目，其中两项内部夹具不计入完成。Audacity 4.0.0 安装成功但首次启动向导无法推进，两次 GUI 作业均明确取消并保留，未计入通过。

[固定官方发布](https://github.com/audacity/audacity/releases/tag/Audacity-3.7.9)于 2026-09-01 发布。`audacity-win-3.7.9-64bit.exe` 为 19,943,880 字节，SHA-256 `e3096847ac4270d304e9b112d153642a72b66b42e13f83a32f06eecfb4ce7e48`，与 GitHub 官方资产 digest 一致。实际包为 Inno Setup，经过 Core 支持的托管 PE 安装路径，以 `/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /SP- /DIR=C:\Audacity379 /LOG` 安装；未用手工解包代替安装。

[固定标签 LICENSE.txt](https://raw.githubusercontent.com/audacity/audacity/Audacity-3.7.9/LICENSE.txt)明确应用采用 GPL v3；单独源文件、文档与组件仍适用各自许可。安装后的根 LICENSE.txt 与固定源码文件字节一致（73,540 字节，SHA-256 `1580ffb4a0c6bbb716324c645682964120eae418b9a1c51842cda140343cb139`）。市场保守标为 GPL-3.0-only；未宣称完成全部组件对应源码审计、AuthentiCode 检验或可复现构建。

原始安装 `job-1790987450702-4` 成功退出 0，代次 `gen-job-1790987450702-4`。GUI 作业 `job-1790987612870-5` 在空的 ForgeQA 目录下生成 440 Hz、幅度 0.25、3 秒正弦波，用界面保存 AUP3、导出 WAV；播放时观察到进度线、电平表和 44100 Hz 状态。正常退出后，独立作业 `job-1790988541262-6` 通过文件选择器显式打开原项目并正常退出。未注入输出样本。

| 文件 | 字节数 | SHA-256 |
| --- | ---: | --- |
| audacity-original.aup3 | 851968 | `82ef153d720a1d880167eea0b80a39c06c9844df808b288308ce11a4921b911a` |
| audacity-original.wav | 264644 | `500091313aeeadcf3fbbe09fbebafc1fd3490ac0936dc730ff06e939ed65e1c2` |
| audacity-market.aup3 | 655360 | `0e721d7ccd15f5aece9e5729a75b420e1459810aee8d63c317f60344d9661158` |
| audacity-market.wav | 176444 | `ccff45af0a94ee6deb3e47211f3875b8b52e7b8161084646fbdfc48b3517ccf0` |

两份 AUP3 的 SQLite `integrity_check` 均为 ok，包含 project/sampleblocks 与一个实际采样块。原 WAV 为 44.1 kHz、单声道、16 位 PCM、132300 帧；正向过零频率 440 Hz，基频投影幅度 0.249999992。更新 WAV 同格式、88200 帧，频率 880 Hz、幅度 0.499999996。导出峰值略高于输入幅度的浮点/量化误差已保留在数值证据中，未改写样本。

签名目录 v17 从上一代信任根用 tuftool 0.17 `clone --metadata-only` 验证成功；root 为 1，timestamp/snapshot/targets 版本均为 26，到期 2026-10-29。所有旧安装包/验收目标 pin 保持不变。原始验收收据 SHA-256 `b5bdd0e6f7a37746cfa0075723d9764a32e1b18ed3c926ae04df1c318c802cd6` 冻结，生命周期证据另行记录。公开目录仅含 JSON。

市场缓存更新 `d30d1186-a502-4b8e-bc0b-88418306831c` 成功，新安装代次 `gen-job-1790988944734-7`。GUI `job-1790988982364-8` 在空目录独立生成 880 Hz、幅度 0.5、2 秒音频，保存项目和导出 WAV；`job-1790989210147-9` 新进程显式重开更新项目。两者正常退出 0。原文件没有复制到更新代次。

市场回滚 `9e4bc8fc-b882-401f-949d-859e20ec148c` 成功。现有最新其他 ready 代次规则选回原始 3.7.9（比保留的 4.0.0 代次更新），无需 Core 额外切换。`job-1790989322270-10` 显式重开原 AUP3 并正常退出 0。四份项目/音频在更新重开、回滚及服务重启后字节不变。

Core 和 Store 重启后，127 个 Core 终态作业、51 条 Store 完整数据库记录、所有代次/选择、已安装列表及目录均保持一致；本轮之前的 117 个 Core 作业及 49 条完整市场记录也逐条未变。51 条市场记录规范 JSON SHA-256 为 `118037ef801d57c07da9959c4d1735407186ce637d45f0d7b73a8901d185fc3c`。原 Core 配置字节及 Store hardening 保持。运行二进制仍为 managed-MSI-v10 与 Store managed-MSI-v2，校验哈希见发布证据。

21 个公开目录源文件字节与已核验传输清单完全一致；TUF 缓存完整解析内容和签名在重启前后相同，不对缓存序列化字节作断言。latestKnownTime 从 `2026-10-03T00:55:28.577155753Z` 单调推进到 `2026-10-03T01:04:50.864587232Z`；信任根 SHA-256 始终为 `345053af51944c6ad56e4d5ac79c65be0b31b515befcc160fe5d226c6a027d0e`。未重置、降级或绕过信任。

Audacity 4.0.0 的原始/软件渲染向导问题仍未定位根因：日志记录 DirectComposition 不支持及 QML 属性/绑定警告，但 Return/Escape 被主线程处理，不能据此声称整个进程死锁。3.7.9 欢迎/升级通知 HTML 超链接竖排问题也保留；关闭非约束通知、欢迎 OK 后的原生音频编辑、生成、保存、导出和打开均经过实测。未点击云服务、MuseHub、升级安装或接受新协议。

保留本轮失败：GitHub 未认证 API 限流、构建机 HTTPS 连接重置、noVNC 完整路径输入产生无效字符警告、首次误选 Silence 后取消、两次未执行的 JavaScript 坐标脚本错误，以及 Core 重启套接字尚未就绪时验收脚本过早读取。最后一项通过读取原有重启前快照、确认 Core 已恢复、等待就绪并继续 Store 重启完成；没有重复 Core 重启、重写历史或改动信任状态。测试桌面自动锁定后使用现有授权凭据自行登录，未移除密码或改动认证设置。

范围限于所列音调/文件工作流和播放指示器。未独立听到物理音频输出；录音与设备输入、多轨编辑、效果/插件、云功能、其他格式、客户机在线下载、全新机器配方注册、跨版本升级和远端安装包再分发均未验证。下一轮继续固定来源候选，不能将同一应用版本计作多个完成项。

证据入口：[源与许可](evidence/2026-10-03-audacity-source.json)、[配方](evidence/2026-10-03-audacity-recipe.json)、[原始验收](evidence/windows-audacity-20261003.acceptance.json)、[后端数据](evidence/2026-10-03-audacity-backend.json)、[市场生命周期](evidence/2026-10-03-audacity-publication.json)、[失败与限制](evidence/2026-10-03-audacity-failures.json)、[累计台账](windows-1000-progress.json)。签名私钥、密码及安装包不入库。
