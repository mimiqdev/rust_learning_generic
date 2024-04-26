pub fn lifetime() {
    // Below example won't compile as lifetime error
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // } // x drop here
    // println!("r: {}", r);

    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    //
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// From compiler
// this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `x` or `y`
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime in struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn lifetime_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
