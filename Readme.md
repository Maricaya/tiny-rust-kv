# a simple kv storage written in rust

1. 设计内存和磁盘
index - 内存 BTree

fio - 磁盘设计
将标准文件操作API 进行简单封装

2. 数据读写流程
写 —— 先写磁盘数据文件，再更新内存索引。
读 —— 去内存中找索引信息，如果没找到，说明不存在；如果找到了，用id找文件。

3. 数据库启动流程
加载数据目录中的文件，打开其文件描述符。
遍历数据文件中的内容，构建内容索引

4. 数据删除流程
根据 bitcask，删除数据也是向数据文件中增加一条记录。类似墓碑值。
删除内存索引。

5. 数据文件逻辑
补全系统默认 IO read、write、close、sync 方法
打开数据文件，从数据文件中读取 LogRecord：
CRC｜TYPE｜KEY SIZE｜VALUE SIZE｜用户存储 KYE｜用户存储 VALUE
