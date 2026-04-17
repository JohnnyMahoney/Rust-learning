//Immutable reference example(like C# 'in' keyword)
fn read_only(data: &Vec<i32>) {
    println!("Read-only data: {:?}", data);
}


fn modify(data: &mut Vec<i32>) {
    data.push(42);
    println!("Modified data: {:?}", data);
}
