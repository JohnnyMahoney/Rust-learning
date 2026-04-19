// struct - Data structure (like C# class/struct combined)
struct MyStruct {
    field: i32,
}
// enum - Algebraic data type (much more powerful than C# enum)
enum MyEnum {
    Variant1,
    Variant2(i32),              // Can hold data
    Variant3 { x: i32, y: i32 }, // Struct-like variant
}

// trait - Interface definition (like C# interface but more powerful)
trait MyTrait {
    fn method(&self);
    
    // Default implementation (like C# 8+ default interface methods)
    fn default_method(&self) {
        println!("Default implementation");
    }
}

// type - Type alias (like C# using alias) simple - it's just an alias for another type, no new type is created
type UserId = u32;
type Result<T> = std::result::Result<T, MyError>;

// impl - Implementation block (no C# equivalent - methods defined separately)
impl MyStruct {
    fn new() -> MyStruct {
        MyStruct { field: 0 }
    }
}
    
impl MyTrait for MyStruct {
    fn method(&self) {
        println!("Implementation");
    }
}