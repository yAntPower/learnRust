pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for com in self.components.iter() {
            com.draw();
        }
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
    }
}
