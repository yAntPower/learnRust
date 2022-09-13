use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
#[derive(Debug)]

struct Ant{
    name:String,
    email:String,
    addr:String,
    isactive:bool,
    count:i32,
}
impl Ant{
    fn clc(&self)->i32{
        self.count * self.count
    } 
}
impl Ant{
    fn clc2(&self,senc:&Ant,xishu:i32)->i32{
        self.count * senc.count * xishu
    } 
    fn clc3(a:i32)->bool{
        a>10
    }
}

fn build_ant(name1:&String,email1:&String)->(Ant,Ant){
    let straddr = &String::from("深圳");
     let ant1 = Ant{
        name:name1.to_string(),
        count : 2,
        isactive:true,
        addr : straddr.to_string(),
        email:email1.to_string(),
    };
    let ant2 = Ant{
        name:name1.to_string(),
        email:email1.to_string(),
        addr:straddr.to_string(),
        ..ant1
    };
    return (ant1,ant2);
}
fn kk()->i32{
    let (ant,ant1) = build_ant(&String::from("billchoy"),&String::from("156@@"));
    println!("{:?}",ant);
    println!("{:#?}",ant1);
    println!("clc:{}",ant.clc());
    println!("clc2:{}",ant.clc2(&ant1,3));
    println!("clc3:{}",Ant::clc3(19));
    
   let _addr = ant.addr;
   let _addr = ant.email;
   let _addr = ant.name;
   let _addr = ant.isactive;

   println!("I am Init()");
    let x = 2;
    if x != 2{
        println!("x!=2");
    }
    let str = String::from("ant,power");

    let strnew = option_str(&str);
    println!("{}",strnew);

    x+1
}
fn option_str(s:&String)->&str{
    let bytes = s.as_bytes();
    for (i,&arr) in bytes.iter().enumerate(){
        if arr == b','{
            return &s[..i];
        }
    }
    return &s[..];
}
fn main() {
    test_enum();
    println!("I am main()");
    println!("init和{}",kk()+1);
    let randnumb = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("请输入数字：");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(n) => {
                println!("{n} bytes read");
                println!("{guess}");
            }
            Err(error) => println!("error: {error}"),
        }
        println!("输入的内容：{}",guess);
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(errmsg)=>{
                println!("转化失败{}",errmsg);
                continue;
            }
        };

        match guess.cmp(&randnumb){
            Ordering::Less=>println!("太小了"),
            Ordering::Greater=>println!("太高了"),
            Ordering::Equal=>{
                println!("你赢了");
                break;
            }
       }
    }
}
enum Test{
    IPv4,
    IPv6(String),
}

fn test_enum(){
    let _v4 = Test::IPv4;
    Test::IPv6(String::from("s: &str"));
    let num:i32 = 5;
    let num2:Option<i32> = Some(5);
    //let num3 = num2.expect("msg: &str");
    let num3 = num2.unwrap();
    let _num1 = Some(num);
    let ruz = num + num3;
    println!("{}",ruz);
    let mut v = vec![
        1,
        2,3,4,5
    ];
    v.push(6);
   for i in &mut v{
        *i +=1
   }
   for i in v{
    println!("vvvvvv:{}",i);
   }   
   let mut s1 = String::from("foo");
   let s2 = "bar";
   s1.push_str(s2);
   println!("s2 is {}", s2);
   let mut map = HashMap::new();
   let s = String::from("a");
   map.insert(s,10);
   let s1 = String::from("a");
   for (key,vul) in &map{
    println!("map:key{},vul{}", key,vul);
   }
   map.get(&s1);
}
fn _study_enum(st :Test){
    let _aaa = Test::IPv6(String::from("s: &str"));
   
    match st{
        Test::IPv4=>{
            println!("I am ipv4");
            1;
        },
        Test::IPv6(_string)=>{
            println!("I am ipv6");
            1;
        },
    };
    let t = 1u8;
    match t{
        1=>println!("sfd"),
        oetha=>oeth(oetha),
    }
    let aaaaaa = aaaaa::Rng{a:String::from("a")};
    println!("{:#?}",aaaaaa);
}
fn oeth(u:u8){
    let config_max = Some(3u8);
    if let Some(max) = config_max{
        println!("{}",max);
    }
}
mod aaaaa{
    #[derive(Debug)]
    pub struct Rng{
       pub a:String,
    }
}
