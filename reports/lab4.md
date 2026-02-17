### 实现功能

1. 为 `trait File` 添加了 `get_stat` 函数，用于获取文件信息，并为 `OSInode` 实现该函数
2. 为 `Inode` 实现了添加和删除目录项的函数，`link` 和 `unlink` 函数，以及获取底层 disk inode 元信息的函数
3. 为 `EasyFileSystem` 添加了 `dealloc_inode` 函数，用于删除 inode 和其管理的所有数据块

### 问答作业

root inode 记录了文件系统中的文件名和 disk inode id 的映射关系。若 root inode 损坏，内核将无法根据文件名访问文件系统中的对应文件。