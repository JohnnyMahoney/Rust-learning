
pub struct Counter {
    value: i32,
}  

impl Counter {
    pub fn new() -> Counter {
        Counter{value: 0}
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

}


fn main() {

    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("{}", counter.get_value());


    let name = "John"; //defaut $str (string slice)
    let count = 42; //default int i32
    let price = 23.33; //default float f64

    let count: u32 = 42;
let price: f32 = 29.99;
}