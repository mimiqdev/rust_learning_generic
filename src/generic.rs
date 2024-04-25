#![allow(unused)]

// restrict the types valid for T to only those that implement PartialOrd
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic in struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic in method definition
// We need to declare T just after impl
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This will make Point<f32> have distance_from_origin()
// but other Point<T> will not
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic in Enum
enum Option<T> {
    Some(T),
    None,
}

// Multiple Generics in Enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn generic() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 1.0 };
}
