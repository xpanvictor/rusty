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
}