# SynWrite 6.41.2780 Windows 有限文本编辑验证（2026-10-14）

本轮有限验证全部通过，发布后累计72/1000；已提交基线仍为71/1000，18项未解决候选保持。仅单处文本编辑、原生Save As、三个新正式空编辑器显式Open，以及Store IPC独立同版本更新和精确原代次回滚。完整编辑器功能未验收。

原32B中文名输入保存original.txt42B，市场32B中文名输入保存market.txt40B，UTF8 BOM和两CRLF保持。中文第一行不变，第二行分别为STATUS AFTER ORIGINAL42或MARKET42。宿主和客体均从不可变输入独立重构全部原生输出字节。三个新正式进程实际从空Untitled经File/Open在记住的SynWriteTests目录双击保存文本；五次正常File/Exit有完整exited0/success和wine-server-stop-requested，并另核ps无wine。退出0单独不作验收。

完整市场重开562/163 CP读回后，唯一Store rollback恢复gen-job-1791935437260-1；562/164旧全部字段、两immutable代次和两程序/选定数据保持。第三原Open及正常退出后563/164 CP读回，再Core和Store最终各正常restart一次；七类全部字段精确同、PID实际改变、二进制摘要/固定数据保持。当前完整私有CP after-final-normal-restart.json SHAa0d272f6cc224e4da48c5117c05a53bad4e42a2578819ed653e40284909b3e62，仅摘要公开。唯一当前只读rolling-current-state-synwrite-final-20261014.py经严格SSH stdin，不创建job/CP/restart；旧running助手与所有旧数仅历史。

每代次368初始程序367跨同，唯一unins000.dat27036B不同，实际两raw私有固定，原因/卸载未核。每代次4个原生INI及12selected固定。GUI实际registry前后raw/diff保存，原因/键值等价未立，不称旧registry跨GUI同字节或全FS相等。原34证据、原3JPEG和signed marketPending receipt逐raw冻结，完成另记。本轮market/reopen/第三原画面只有CUA工具历史，未落盘新JPEG。

用户另明确要求不要自动锁屏；在第三原进程正常退出及完整CP后通过KDE原生系统设置关闭闲置和待机恢复自动锁屏。用户kscreenlockerrc原实际不存在，现53B配置Autolock=false、LockOnResume=false、Timeout=0，应用后和最终服务重启后独立读回保持。账号、密码、认证旧配置不变；手动锁屏快捷键仍显示。旧不改锁屏文字仅冻结历史，本项由用户新要求授权，不是应用兼容修复。两次用户正常登录后仅继续唯一既有进程，无agent秘密输入/认证submit/登录图保存。

官方EXE/ZIP缓存及24selected源保持；部分源码/binary bundle不当纯源，EControl约30%闭源，SourceForge MPL1.1 metadata和组件MPL2/LGPL仅各自范围，main与完整捆绑许可闭包未核。Setup6.40.2770旧于binary6.41.2780；源二进制对应/可复现/PGP/Authenticode/上游digest/latest未核。Inno5.5.7、PE14c和DIR16单反斜杠仅有限安装证据，不当完整UAC/关联/shortcut/卸载行为。无Core/程序/immutabledefinition/launcher envcwd/字体/globalDLL/runtime修复或recover。

首次市场完整诊断保存后，recent64旧ID KeyError在CP/fixture/GUI前停；独立全SQLite旧162行和Core旧559任务全部字段保持，旧ID只是滑出recent64。原部署选定源码list_recent(64)/ORDER BY sequence DESC LIMIT及reverse仅静态上下文，V2严格实际window shift、新成功job以及旧ID仍fullSQLite同字段后才市场GUI，不称数据丢失/应用失败或唯一运行时根因。所有失败助手/exact可用stderr/私有raw和旧签名/root/trust/cache历史保持。

candidate-v81/root1/role90，74entries72apps2fixtures、156targets85publicJSON，153旧noncatalogue目标/root原raw同。仅一次签名/原root验证、一次严格archive传入/stage/preview；不重签重传/放松guard/resettrust。公共不含安装包/程序binary、完整私有CP、私钥/凭据/密码/登录图/exactredirectquery。完整编辑器、About、直接中文键盘、插件/FTP/Python/广泛编码/大坏输入/打印/联网背景排除/迁移/无人值守/GUI市场按钮未验收。Ubuntu独立旧账本/cache保持，无VM切换或两4GiBVM并发。独立FINAL和Git byte/filter/index/commit/普通push另记；本证明冻结时尚待最终发布，不代替全私有CP或GUI重演。
