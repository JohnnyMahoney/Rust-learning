//Immutable reference example(like C# 'in' keyword)
fn read_only(data: &Vec<i32>) {
    println!("Read-only data: {:?}", data);
}
//Mutable reference example(like C# 'ref' keyword)
fn modify(data: &mut Vec<i32>) {
    data.push(42);
    println!("Modified data: {:?}", data);
}

fn main() {
    let mut my_data = vec![1, 2, 3];
    read_only(&my_data); // Pass an immutable reference
    modify(&mut my_data); // Pass a mutable reference



    let data = vec![1, 2, 3];
     let closure = move || {
    println!("{:?}", data); // data is moved into closure
};
}