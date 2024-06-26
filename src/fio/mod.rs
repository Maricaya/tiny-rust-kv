pub mod file_io;

use crate::errors::Result;
use crate::fio::file_io::FileIO;
use crate::options::IOType;
use std::path::PathBuf;

// 抽象 IO 管理接口，可以接入不同的 IO 类型，目前支持标准文件 IO
pub trait IOManager: Sync + Send {
    // read 从文件的给定位置读取对应的数据
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;
    // write 写入字节数组到文件中
    fn write(&self, buf: &[u8]) -> Result<usize>;
    // sync 持久化数据
    fn sync(&self) -> Result<()>;
}

// 根据文件名称初始化 IOManager

/// 根据文件名称初始化 IOManager
pub fn new_io_manager(file_name: PathBuf, io_type: IOType) -> Box<dyn IOManager> {
    match io_type {
        IOType::StandardFIO => Box::new(FileIO::new(file_name).unwrap()),
        // IOType::MemoryMap => Box::new(MMapIO::new(file_name).unwrap()),
    }
}
