struct MyStruct {
    my_value: String,
    testing: u32,
}

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct {
            my_value: "test".to_string(),
            testing: 0
        }
    }
}
