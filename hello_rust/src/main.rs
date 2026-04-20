
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
}