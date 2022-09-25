use std::time::Instant;
use humansize::{format_size, DECIMAL};
use std::path::PathBuf;

//同步代码用来对比异步
pub fn start(input:&String) {
    let sync = Instant::now();
    let result = calc_dir_size_sync(PathBuf::from(input.trim()), (0, 0)).unwrap();
    let size = format_size(result.0, DECIMAL);
    println!("目录：{}总大小:{}\n共有{}个文件", input, size, result.1);
    let after = sync.elapsed();
    println!("同步用时:{:?}", after);
    println!("=======================");
}
fn calc_dir_size_sync(pathbuf: PathBuf, mut num: (u64, u64)) -> std::io::Result<(u64, u64)> {
    for dirs in std::fs::read_dir(&pathbuf)? {
        let dir = dirs?;
        if dir.path().is_dir() {
            num = calc_dir_size_sync(dir.path(), num)?;
        } else {
            num.1 += 1;
            num.0 += std::fs::symlink_metadata(dir.path())?.len();
        }
    }
    Ok(num)
}