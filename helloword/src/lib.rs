enum Coin{
    One,
    Five,
    Ten,
    Twenty,
}
fn match_test(c:Coin)->u8{
    match c{
        Coin::One=>1,
        Coin::Five=>5,
        Coin::Twenty=>20,
        _ =>10,
    }
}
#[test]
fn run(){
    let c = Coin::Ten;
    let a = match_test(c);
    eprintln!("{}",a);
}

