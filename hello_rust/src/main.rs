mod array;

use std::time::Instant;

fn main() {
    println!("Hello, world!");
    
    // Create an instance of DataProcessor
    let mut processor = array::DataProcessor {
        data: Vec::new(),
    };
    
    // Measure the time taken to process
    let start = Instant::now();
    processor.process_large_dataset();
    let duration = start.elapsed();
    
    println!("Time taken: {:?}", duration);
} 


