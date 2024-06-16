mod btree;

use crate::data::log_record::LogRecordPos;
use crate::errors::Result;
use crate::options::{IndexType, IteratorOptions};
use bytes::Bytes;

// indexer 抽象索引接口，后续如果想要接入其他的数据结构，则直接实现这个接口即可
pub trait Indexer: Sync + Send {
    // Put  向索引中存储 key 对应的数据位置信息
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool;
    // get 根据 key 取出对应的索引位置信息
    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos>;
    // delete 根据 key 删除对应的索引位置信息
    fn delete(&self, key: Vec<u8>) -> bool;
    // 获取索引存储中所有的 key
    fn list_keys(&self) -> Result<Vec<Bytes>>;
    // 返回索引迭代器
    fn iterator(&self, option: IteratorOptions) -> Box<dyn IndexIterator>;
}

// 根据类型打开内存索引
pub fn new_indexer(index_type: IndexType) -> impl Indexer {
    match index_type {
        IndexType::BTree => btree::BTree::new(),
        IndexType::SkipList => todo!(),
    }
}

/// 抽象索引迭代器
pub trait IndexIterator: Sync + Send {
    /// Rewind 重新回到迭代器的起点，即第一个数据
    fn rewind(&mut self);

    /// Seek 根据传入的 key 查找到第一个大于（或小于）等于的目标 key，根据从这个 key 开始遍历
    fn seek(&mut self, key: Vec<u8>);

    /// Next 跳转到下一个 key，返回 None 则说明迭代完毕
    fn next(&mut self) -> Option<(&Vec<u8>, &LogRecordPos)>;
}
