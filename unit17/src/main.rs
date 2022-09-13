use test_unit17::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {}
}
fn main() {
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    r.split_at_mut(3);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
        ],
    };
    screen.run();

    let o = (1, 1, 1, 1, 1, 1, 2, 3);
    match o {
        (i, .., j, k) => {
            println!("i:{},j:{},k:{}", i, j, k);
        }
        _ => (),
    }
    let s = Ste::Id { id: 4 };
    match s {
        Ste::Id { id: iii @ 1..=4 } => {
            println!("{}", iii);
        }
        _=>(),
    }
}
enum Ste {
    Id { id: i32 },
}
