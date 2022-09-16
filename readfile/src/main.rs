use std::fs;
use std::io;
use std::path::Path;

fn readfile(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for result_dir in fs::read_dir(dir)? {
            let direntry = result_dir?;
            let path = direntry.path();
            if path.is_dir() {
                readfile(&path)?;
            } else {
                if let Some(name) = path.file_name() {
                    if let Some(filename) = name.to_str() {
                        match fs::read_to_string(&path) {
                            Ok(context) => {
                                println!("文件{}的内容为{}", filename, context);
                            }
                            Err(_) => {
                                continue;
                            }
                        }
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
