# NSIS 暂存检查补充（2026-10-12）

首次真实暂存的 Git 空白门槛发现固定上游源码和两个同字节许可文件的原有尾空白，以及 makenssi.cpp 的末尾空行，立即停止，未提交。随后只读全索引复核 1421 份冻结文件均与原磁盘及第一冻结清单原字节一致；1277 份旧 immutable、17 份原 NSIS source/GUI/receipt、第一审查、原报告和角色 55/56 保持。

仅为实际报告的六个具体路径增加 -blank-at-eol 属性，其中 makenssi.cpp 另加 -blank-at-eof，保留上游原字节。未改客体、程序、信任或 GUI 数据。第一失败清单、第一审查和原报告保持历史阶段；v2 增量独立审查及全 index/commit 复核另记。下一轮采用 nsis-all-frozen-files-v2-20261012.json，第一清单只代表失败暂存阶段。
