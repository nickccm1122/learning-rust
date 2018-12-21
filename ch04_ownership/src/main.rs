fn main() {
    let s1 = string::from("hello world");

    let len = calculate_length(&s1);

    println!("the length of {} is {}", s1, len);

    let word = first_word(&s1);

    println!("the space in position {}", word);

    let sliced_first_word = first_word_sliced(&s1);
}

// references allow to refer to some value without taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}// nothing will happen, as the func does not have the ownership of s1

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_sliced(s: &String) -> &str {
   let index = first_word(&s);

   &s[0..index]
}