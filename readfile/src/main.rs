use std::fs;
use std::io;
use std::path::Path;

fn readfile(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for result_dir in fs::read_dir(dir)? {
            let direntry = result_dir?;
            let path = direntry.path();
            println!("当前路径：{}",path.to_str().unwrap());
            if path.is_dir() {
                println!("发现文件夹开始递归！");
                readfile(&path)?;
            } else {
                let filename = path.file_name().unwrap().to_str().unwrap();
                match fs::read_to_string(&path) {
                    Ok(context) => {
                        println!("文件{}的内容为{}", filename, context);
                    }
                    Err(e) => {
                        println!("路径：{},错误:{}",path.to_str().unwrap(),e);
                        continue;
                    }
                }
            }
        }
    }
    Ok(())
}
fn main() {
    if let Err(err) = readfile(Path::new("./files/")) {
        println!("{}", err);
    } else {
        println!("over");
    }
}
