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
        None => println!("There is no third element"),
    }

    // for loop for a vector
    for i in &v {
        println!("{}", i)
    }

    // mutating a mut vector using an iterator
    println!("Now mutating a vector with an iterator");
    let mut v = vec![100, 32, 53];
    for i in &mut v {
        *i += 50;
        println!("{}", i)
    }

    // using an enum to store different types in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(6.7),
        SpreadsheetCell::Text(String::from("Hello world")),
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

    // hash maps
    use std::collections::HashMap;
    // creating a new hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);

    // using iterator tuples to construct hashmaps from vectors with zip function
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scoresHash: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    let name = String::from("Blue");
    println!("The blue's score is {:?}", scoresHash.get(&name));

    // checking if an hashmap has a key else update it
    scoresHash.entry(String::from("Yellow")).or_insert(300);
    scoresHash.entry(String::from("White")).or_insert(40);
    println!("{:#?}", scoresHash);

    // using an hashmap for a word counter
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
