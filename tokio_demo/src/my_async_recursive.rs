use futures::future::{BoxFuture, FutureExt};
use humansize::{format_size, DECIMAL};
use std::path::PathBuf;
use std::time::Instant;
use tokio::fs;

static mut NUM: (u64, u64) = (0, 0);
pub async fn run_recursive(input: &String) {
    let sync = Instant::now();
    calc_dir_size_recursive(PathBuf::from(input.trim())).await;
    let num_tmp;
    unsafe {
        num_tmp = NUM;
    }
    let size = format_size(num_tmp.0, DECIMAL);
    println!("目录：{}总大小:{}\n共有{}个文件", input, size, num_tmp.1);
    let after = sync.elapsed();
    println!("异步递归用时:{:?}", after);
    println!("=======================");
}
fn calc_dir_size_recursive(pathbuf: PathBuf) -> BoxFuture<'static, ()> {
    async {
        let mut dirs = fs::read_dir(pathbuf).await.unwrap();
        while let Some(entry) = dirs.next_entry().await.unwrap() {
            if entry.path().is_dir() {
                calc_dir_size_recursive(entry.path()).await;
            } else {
                unsafe {
                    NUM.1 += 1;
                    NUM.0 += fs::symlink_metadata(entry.path()).await.unwrap().len();
                }
            }
        }
    }
    .boxed()
}
