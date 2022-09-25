use std::fs::Metadata;
use std::path::PathBuf;
use std::time::Instant;
use tokio::fs;
use futures::{stream::FuturesUnordered, TryStreamExt};
use humansize::{format_size, DECIMAL};


pub async fn run(input:&String) {
    let sync = Instant::now();
    let result_size = calc_dir_size(PathBuf::from(input.trim()))
        .await
        .unwrap();
    let size = format_size(result_size.0, DECIMAL);
    println!(
        "目录：{}总大小:{}\n共有{}个文件",
        input, size, result_size.1
    );
    let after = sync.elapsed();
    println!("异步非递归用时:{:?}", after);
    println!("=======================");
}
async fn calc_dir_size(pathbuf: PathBuf) -> std::io::Result<(u64, u64)> {
    let mut size = 0u64;
    let mut filecount = 0u64;
    let mut futures = FuturesUnordered::new();
    futures.push(get_metadata(pathbuf));
    while let Some((path, meta)) = futures.try_next().await? {
        if path.is_dir() {
            let mut dirs = fs::read_dir(path).await?;

            while let Some(dir) = dirs.next_entry().await? {
                futures.push(get_metadata(dir.path()));
            }
            //============= for 循环不行  =============
            //和 _calc_dir_size2()方法一样只能读取一个文件
            // for dir in dirs.next_entry().await? {
            //     futures.push(get_metadata(dir.path()));
            // }
            //================ end ===================
        } else {
            filecount += 1;
            size += meta.len();
        }
    }
    Ok((size, filecount))
}

async fn get_metadata(path: PathBuf) -> std::io::Result<(PathBuf, Metadata)> {
    let meta = fs::symlink_metadata(&path).await?;
    Ok((path, meta))
}























//只能读取输入目录下的一个文件  todo("待查原因")!
async fn _calc_dir_size2(pathbuf: PathBuf) -> std::io::Result<(u64, u64)> {
    let mut size = 0u64;
    let mut filecount = 0u64;
    let mut futures = FuturesUnordered::new();
    futures.push(get_metadata(pathbuf));
    while let Some((path, meta)) = futures.try_next().await? {
        if path.is_dir() {
            for dirs in fs::read_dir(&path).await?.next_entry().await? {
                futures.push(get_metadata(dirs.path()));
            }
        } else {
            filecount += 1;
            size += meta.len();
        }
    }
    Ok((size, filecount))
}
