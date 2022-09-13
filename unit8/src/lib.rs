use rand::Rng;
use std::collections::HashMap;

pub fn run() {
    test_vec();
    test_str();
    test_map();

    test_generics();
}

fn test_generics() {
    let v_num = vec![1, 3, 2, 0, 5, 69, 8, 9];
    let v_str = vec!['b', 'c', 'h', 'a'];
    generics(&v_num);
    generics(&v_str);
    
    let p = Point { x: 5, y: 10 };

    println!("p.x = {},p.y = {}", p.x(),p.y);
}
struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}
fn generics<T>(list: &[T]) -> T {
    todo!();
}
fn test_vec() {
    //-----------生成一个集合----------
    //用作随机长度
    let randnumb = rand::thread_rng().gen_range(20..=30);
    let mut vec = Vec::new();
    for _ in 0..=randnumb {
        let num = rand::thread_rng().gen_range(1..=100);
        vec.push(num);
    }
    // println!("随机生成的集合{:?}", vec);
    //-----------生成一个集合 end----------

    // 排序（冒泡）
    for i in 0..(vec.len()) {
        for j in 0..(vec.len() - 1 - i) {
            if &vec[j] > &vec[j + 1] {
                let tmpnum = vec[j];
                let tmpnum2 = vec[j + 1];
                vec[j] = tmpnum2;
                vec[j + 1] = tmpnum;
            }
        }
    }
    //寻找中位数
    let medium_index = vec.len() / 2;
    if medium_index > vec.len() {
        println!("数组长度异常。{}", medium_index);
        return;
    }
    match vec.get(medium_index) {
        Some(num) => {
            println!(
                "中位数为：{},中位数索引为：{},数组长度为：{}",
                num,
                medium_index,
                vec.len()
            );
        }
        None => {
            println!("无法获取中位数！{:?}", &vec);
        }
    }
    //借用hashmap寻找重复出现次数最多的数字
    let mut map = HashMap::new();
    for i in vec.iter().enumerate() {
        let count = map.entry(*i.1).or_insert(0);
        *count += 1;
        // println!("i:{},Key:{},vul:{}", i.0, i.1, count);
    }
    //找到众数
    let mut map2 = HashMap::new();
    let mut maxnum = 0;
    for (_, v) in &map {
        if maxnum < *v {
            maxnum = *v;
        }
    }
    for (k, v) in &map {
        if maxnum == *v {
            map2.insert(k, v);
        }
    }
    // println!("排序后的 集合{:?}", &vec);
    // println!("每个数字出现的次数{:?}", &map);
    println!("出现最多次的数字是{:?}", &map2);
}

fn test_str() {
    //创建元辅音库
    let mut zm_map = HashMap::new();
    let zm = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    let mut isok = true;
    let yuan = String::from("Y"); //元音
    let fu = String::from("F"); //辅音
    for s in zm.split_whitespace() {
        if isok {
            zm_map.insert(s.to_string(), &yuan);
            isok = false;
        } else {
            zm_map.insert(s.to_string(), &fu);
            isok = true;
        }
    }
    //生成一段单词 Increase your APR by redeeming NFTs to your account. HOPR
    let vec = vec![
        "Increase",
        "your",
        "APR",
        "by",
        "redeeming",
        "NFTs",
        "to",
        "your",
        "account",
        "HOPR",
    ];
    //用于存储修改后的字符串
    let mut new_vec = Vec::new();
    //判断每个单词的首字母是属于元音还是辅音
    for v in vec {
        if v.len() > 0 {
            //获取首字母
            let c = match v.chars().nth(0) {
                Some(ch) => ch.to_string(),
                None => {
                    println!("获取字符串中此{}元素的首字母失败", v);
                    continue;
                }
            };
            //根据首字母获取对应的元、辅音
            let value = match zm_map.get(&c.to_lowercase()) {
                Some(vul) => vul.to_uppercase(),
                None => {
                    println!("未找到该字符{}对应的元辅音", v);
                    continue;
                }
            };
            //判断元、辅音然后按要求处理
            let mut tmp_str = v.to_string();
            //比较两个字符串是否相同
            if value.cmp(&fu).is_eq() {
                //辅音
                let first_c = tmp_str.remove(0);
                tmp_str.push('-');
                tmp_str.push(first_c);
                tmp_str.push_str("ay");
            } else if value.cmp(&yuan).is_eq() {
                tmp_str.push_str("-hay");
            }
            new_vec.push(tmp_str);
        }
    }
    println!("转换后的单词列表：{:?}", new_vec);
}

fn test_map() {
    let mut bm_map = HashMap::new();
    let mut peploe_vec = Vec::new();
    peploe_vec.push("Alice");
    peploe_vec.push("张三");
    peploe_vec.push("李四");
    peploe_vec.push("王五");
    peploe_vec.push("赵六");
    bm_map.insert("研发部", peploe_vec);
}
