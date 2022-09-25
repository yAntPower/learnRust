mod my_async;
mod my_sync;
mod my_async_recursive;
use std::io;
#[tokio::main]
async fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("输入错误");
    //异步  release（calc_dir_size） 非递归
    // 用时:2.947691154s
    // result_size:(20740376603, 67700)
    // 目录：/Users/xx/project
    // 总大小:20.74 GB
    // 共有67700个文件
    my_async::run(&input).await;


    //异步  release（calc_dir_size_recursive）递归
    // 用时:1.105025943s
    // result_size:(20740376603, 67700)
    // 目录：/Users/xx/project
    // 总大小:20.74 GB
    // 共有67700个文件
    my_async_recursive::run_recursive(&input).await;


    //同步 release
    // 目录：/Users/xx/project
    // 总大小:20.74 GB
    // 共有67700个文件
    // 用时:1.145774996s
    my_sync::start(&input);
}

