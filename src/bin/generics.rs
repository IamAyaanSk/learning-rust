// Generics are like generics in typescript
fn main() {
    let my_vec = vec![1, 4, 9, 2, 3];
    let largest_num = check_largest(&my_vec);

    println!("{}", largest_num);

    let float_points = Point { x: 20.0, y: 10.2 };

    float_points.only_for_float_point();
    let new_mixup = float_points.mixup(Point {
        x: 20,
        y: String::from("Hello"),
    });

    println!("x : {} , y : {}", new_mixup.x, new_mixup.y)
}

// Generics in functions
fn check_largest<T: PartialOrd + Copy>(vec: &Vec<T>) -> T {
    let mut largest = vec[0]; // Since if Copy tait is not there for type, we will move ownership of the element which would cause error
    for el in vec {
        if *el > largest {
            largest = *el;
        }
    }

    largest
}

// Generics in struct
struct Point<T, U> {
    x: T,
    y: U,
}

// We can also pass the hardcoded type instead of generics and that method will be only avialble to the instance following that type
impl Point<f64, f64> {
    fn only_for_float_point(&self) {
        println!("Hello floats");
    }
}

// We can use generic too
impl<T, U> Point<T, U> {
    fn mixup<A, B>(self, other_point: Point<A, B>) -> Point<T, B> {
        Point {
            x: self.x,
            y: other_point.y,
        }
    }
}

// We can use generics on enums too -> Option and Result use them
enum _Result<T, E> {
    Ok(T),
    Err(E),
}

// Generics have no runtime performance cost.
// Rust uses monomorphization: at compile time, it generates
// specialized versions of generic code for each concrete type used.
