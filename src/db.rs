use crate::data::data_file::DataFile;
use crate::data::log_record::{LogRecord, LogRecordPods, LogRecordType};
use crate::errors::Errors;
use crate::errors::Result;
use crate::index;
use crate::options::Options;
use bytes::Bytes;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

// bitcask 存储引擎实例结构体
pub struct Engine {
    options: Arc<Options>,
    active_file: Arc<RwLock<DataFile>>, // 当前活跃数据文件
    older_files: Arc<RwLock<HashMap<u32, DataFile>>>, // 旧到数据文件
    index: Box<dyn index::Indexer>,     // 数据内存索引
}

impl Engine {
    // 存储key/value 数据key不能为空
    pub fn put(&self, key: Bytes, value: Bytes) -> Result<()> {
        //  判断key的有效性
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }
        // 构造 LogRecord 结构体
        let mut record = LogRecord {
            key: key.to_vec(),
            value: value.to_vec(),
            rec_type: LogRecordType::NORMAL,
        };
        // 追加写到活跃数据文件中
        let log_record_pos = self.append_log_record(&mut record)?;
        // 更新内存索引
        let ok = self.index.put(key.to_vec(), log_record_pos);
        if (!ok) {
            return Err(Errors::IndexUpdateFailed);
        }
        Ok(())
    }

    // 根据 key 获取对应的数据
    pub fn get(&self, key: Bytes) -> Result<Bytes> {
        //   判断key的有效性
        if key.is_empty() {
            return Err(Errors::KeyIsEmpty);
        }
        // 从内存索引中获取key对应的数据信息
        let pos = self.index.get(key.to_vec());
        // 如果 key 不存在，则直接返回
        if pos.is_none() {
            return Err(Errors::KeyNotFound);
        }

        // 从对应的数据文件中获取对应的 LogRecord
        let log_record_pos = pos.unwarp();
        let active_file = self.active_file.read();
        let older_files = self.older_files.read();
        let log_record = match active_file.get_file_id() == pos.unwrap().file_id {
            true => active_file.read_log_record(log_record_pos.offset)?,
            false => {
                let data_file = older_files.get(&log_record_pos.file_id);
                if data_file.is_none() {
                    // 找不到对应的数据文件，返回错误
                    return Err(Errors::DataFileNotFound);
                }
                data_file.unwrap().read_log_record(log_record_pos.offset)?;
            }
        };

        // 判断 LogRecord 的类型
        if log_record.rec_type == LogRecordType::DELETED {
            return Err(Errors::KeyNotFound);
        }

        // 返回对应的 value 信息
        Ok(log_record.value.into())
    }

    // 追加写数据到当前到活跃文件
    fn append_log_record(&self, log_record: &mut LogRecord) -> Result<LogRecordPods> {
        let dir_path = self.options.dir_path.clone();

        // 输入数据进行编码
        let enc_record = log_record.encode();
        let record_len = enc_record.len() as u64;

        // 获取当前活跃文件
        let mut active_file = self.active_file.write();

        // 判断当前活跃文件是否到达了阈值
        if active_file.get_write_off() + record_len > self.options.data_peth_size {
            // 对当前活跃文件 持久化
            active_file.sync()?; //todo ???

            let current_fid = active_file.get_file_id();

            // 旧的数据文件，存储到map中
            let mut order_files = self.older_files.write();
            let old_file = DataFile::new(dir_path.clone(), current_fid)?;
            order_files.insert(current_fid, old_file);

            // 打开新的数据文件
            let new_file = DataFile::new(dir_path.clone(), current_fid + 1)?;
            *active_file = new_file;
        }
        // 追加数据到当前活跃文件中
        let write_off = active_file.get_write_off();
        active_file.write(&enc_record)?;

        // 根据配置项决定是否持久化
        if self.options.sync_writes {
            active_file.sync()?;
        }

        // 构造数据索引信息

        Ok(LogRecordPods {
            file_id: active_file.get_file_id(),
            offset: write_off,
        })
    }
}
