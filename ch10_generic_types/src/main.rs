#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let numbers = vec![23, 34, 54, 23, 12, 65, 34];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_str(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // exp the struct lifetime
    let novel = String::from("Call me Nick. and I am a boy");

    let first_sentence = novel.split('.').next().expect("Could not find a .");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("important excerpt {:?}", _i);

    let infered_str = infer_lifetime("Nick");
    println!("string: {}", infered_str);
}

fn infer_lifetime(s: &str) -> &str {
    &s[0..2]
}

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest_number = list[0];
    for &number in list.iter() {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
