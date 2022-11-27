fn main() {
    // one way of declaring vectors by instantiating without values
    // let v: Vec<i32> = Vec::new();
    // another way by using the vec macro and just putting the values
    let mut v = vec![1, 2, 3, 4];
    // we use push to add elements to vectors; NOTE vectors r stored on heap unlike arrays
    v.push(5);
    v.push(6);
    // accessing the item
    let third: &i32 = &v[2];
    println!("The element at index 2 is {}", third);

    match v.get(3) {
        Some(elem) => println!("Element at index 4 is {}", elem),
        None => println!("There is no third element")
    }

    // for loop for a vector
    for i in &v {
        println!("{}", i)
    }

    // mutating a mut vector using an iterator
    println!("Now mutating a vector with an iterator");
    let mut v = vec![100, 32, 53];
    for i in &mut v{
        *i += 50;
        println!("{}", i)
    }

    // using an enum to store different types in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(6.7),
        SpreadsheetCell::Text(String::from("Hello world"))
    ];

    println!("{:?}", row);

    // Strings, let's go
    let data = "initial content";
    // make a string out of data
    let s = data.to_string();
    
    let mut foo_bar = String::from("foo");
    foo_bar.push_str("bar");
    println!("{}", foo_bar);
    // while push_str takes a string literal, push takes just a char

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

}