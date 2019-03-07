fn main() {
    let mut a_string = String::from("Hello, world!");
    let word = first_word_second_attempt(&a_string[..]);

    println!("{}", word); //try using this with the string after compiling it
}

fn first_word(a_string: &String) -> usize {
    let bytes = a_string.as_bytes();    //convert string to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {      //iterates over the array of bytes, returns each element in a collection. Enumerate wraps the result of iter and returns it as tuple.
        if item == b' ' {                               //  i is index in tuple, &item is a byte in the tuple
            return i;                                   // searching for byte representing the space, index is returned
        }
    }

    a_string.len()              // if not space is found, length of the string is returned
}

fn first_word_second_attempt(a_string: &str) -> &str {
    let bytes = a_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &a_string[0..i]
        }
    }

    &a_string[..]
}
