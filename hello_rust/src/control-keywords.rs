fn get_value() -> i32 {
    return 42;
}

fn find_value() -> Option<i32> {
    loop {
        let value = get_next();
        if value < 0 { continue; }
        if value > 100 { break None; }      // Break with value
        if value == 42 { break Some(value); } // Break with success
    }
}

fn main() {
    
    //infinite loop example
    loop {
        if condition {
            break; // Exit the loop when the condition is met
        }
    }

    while condition {
        // Loop body   
    }

    for item in collection {
        // Loop body
    }
    
}