use std::{env, fs, process};

struct Inputargs {
    query: String,
    filename: String,
    case_sensitive: bool,
}
impl Inputargs {
    fn new(mut args: env::Args) -> Result<Inputargs, &'static str> {
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => {
                return Err("无法获取第一个参数");
            }
        };
        let filename = match args.next() {
            Some(f) => f,
            None => {
                return Err("无法获取第二个参数");
            }
        };
        let issensitive = match args.next() {
            Some(f) => {
                f.cmp(&String::from("true")).is_eq()
            },
            None => {
                println!("未找到第三个参数，按照不区分大小写处理");
                false
            }
        };
        // let issensitive;
        // issensitive = match env::var("CASE_INSENSITIVE"){
        //     Ok(_) => true,
        //     Err(e) => {println!("错误代码:{}",  e); false},
        // };
        // issensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("Searching for: {}", query);
        println!("In file: {}", filename);
        println!("是否区分大小写 {}", issensitive);
        Ok(Inputargs {
            filename,
            query,
            case_sensitive: issensitive,
        })
    }
}
pub fn run() {
    let args = match Inputargs::new(env::args()){
        Ok(args)=>args,
        Err(err)=>{
            println!("读取命令行参数失败：{} \n程序退出!",err);
            process::exit(1);
        }
    };
    let file_content = openfile(&args.filename);
    let result = find(&file_content, &args.query, args.case_sensitive);
    println!("找到的数据{:?}", result);
    println!("共找到{}行数据", result.len());
}

fn openfile(filename: &String) -> String {
    let file_content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            println!("打开文件{:?}失败，原因{}", filename, err);
            process::exit(1)
        }
    };
    file_content
}

fn find<'a>(file_content: &'a String, query: &String, issensitive: bool) -> Vec<&'a str> {
    let result;
    if issensitive {
        result=  file_content.lines().filter(|line| line.contains(query)).collect()
    }else{
        result=  file_content.lines().filter(|line| {
            line.to_lowercase().contains(&query.to_lowercase())
        }).collect()
    }
    result
}

