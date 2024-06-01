use std::path::PathBuf;

#[derive(Clone)]
pub struct Options {
    // 数据库目录
    pub dir_path: PathBuf,
    // 数据库文件大小
    pub data_peth_size: u64,

    // 是否每次都写持久化
    pub sync_writes: bool,

    // 索引类型
    pub index_type: IndexType,
}

#[derive(Clone)]
pub enum IndexType {
    // BTree 索引
    BTree,
    // 跳表索引
    SkipList,
}
