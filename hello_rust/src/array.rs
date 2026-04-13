pub struct DataProcessor {
    pub data: Vec<i32>,
}

impl DataProcessor {

    pub fn process_large_dataset(&mut self) {
        for i in 0..10_000_00  {
            self.data.push(i * 2);
            println!("Processing item: {}", i);
        }
    }   
}

//Runtime: Consistent
// Memory Usage: exact allocation
//Predictability: High 