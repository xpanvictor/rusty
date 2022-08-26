fn main() {
    let mut word = String::from("Hello my beautiful abode");
    println!("The word initially is {}", word);
    edit_word(&mut word);
    println!("The new word is {}", word);
    // Now let's split the word
    let first_word = split_and_return_first_word(&word);
    println!("First word: {}", first_word);
    // clone the first word and clear word
    let first_word_cloned = String::from(first_word);
    word.clear();
    println!("After word is cleared, can we use the cloned first_word?: {}", first_word_cloned);
    // test slices with arrays
    let nums = [1, 2, 3, 4, 5];
    let num_slice = &nums[1..3];
    assert_eq!(num_slice, &[2, 3]);
}

fn edit_word (word: &mut String) {
    word.push_str(", world")
}

fn split_and_return_first_word(word: &str) -> &str {
    let bytes = word.as_bytes();
    // iterator
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &word[..i];
        }
    }
    // return the word's length
    &word[..]
}
