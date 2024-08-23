/*
A generic data type is a type that's defined in terms of other, partially unknown types. 
We've been using many generic data types since the beginning of this course, for example:

The Option<T> enum is generic over the type T, which is the value contained by its Some variant.
The Result<T, E> is generic over both its success and failure type, contained by its Ok and Err variants, respectively.
The vector type Vec<T>, the array type [T; n], and the hash map HashMap<K, V> are generic over the types they contain.
When we use generic types, we can specify the operation we want without many concerns about the inner types held by the defined type.

To implement a new generic type, we must declare the name of the type parameter inside angle brackets just after the name of the struct. 
Then we can use the generic type in the struct definition where we would otherwise specify concrete data types.

*/
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point { x: "high", y: "low" };
}
/*
The preceding code defines a Point<T> struct. This struct holds any type (T) of x and y values.

Even though T can assume any concrete type, x and y must be of that same type, because they were defined as being of the same type. 
If we try to create an instance of a Point<T> that has values of different types, as in the following snippet, our code won't compile.
*/
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 25, y: true };
}
// error[E0308]: mismatched types
/*
The error message says that the expected type for the y field was an integer. 
But because we defined y to have the same type as x, the compiler complained of a type mismatch error.

As seen in the following example, we can use multiple generic type parameters. 
In this case, we show a Point<T, U> generic over two types so that x and y can be values of different types.
*/
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_and_boolean = Point { x: 5, y: false };
    let float_and_string = Point { x: 1.0, y: "hey" };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 10, y: 30 };
    let both_boolean = Point { x: true, y: true };
}
/*
All of the preceding Point types have different concrete types. In order:

Point<integer, bool>
Point<f64, &'static str>
Point<integer, f64>
Point<integer, integer>
Point<bool, bool>
So you can't really mix those values directly with each other until you've implemented that kind of interaction in your code.

In the next unit, we're going to learn about traits and discover how generic types can be useful in our code. 
We can use them to write generic functions that will operate on objects of different but related types.
*/
/*
A trait is a common interface that a group of types can implement. The Rust standard library has many useful traits, such as:

io::Read for values that can read bytes from a source.
io::Write for values that can write out bytes.
Debug for values that can be printed in the console using the "{:?}" format specifier.
Clone for values that can be explicitly duplicated in memory.
ToString for values that can be converted to a String.
Default for types that have a sensible default value, like zero for numbers, empty for vectors, and “” for String.
Iterator for types that can produce a sequence of values.
Each trait definition is a collection of methods defined for an unknown type, usually representing a capability or behavior that its implementors can do.

To represent the concept of "having a two-dimensional area," we can define the following trait:
*/
trait Area {
    fn area(&self) -> f64;
}
/* 
Here, we declare a trait by using the trait keyword and then the trait's name, which is Area in this case.

Inside the braces, we declare the method signatures that describe the behaviors of the types that implement this trait, which in this case is the function signature fn area(&self) -> f64.
The compiler will then check that each type implementing this trait must provide its own custom behavior for the body of the method.

Now, let's create some new types that will implement our Area trait:
*/
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
/*
To implement a trait for a type, we use the keywords impl Trait for Type, where Trait is the name of the trait being implemented and Type is the name of the implementor struct or the enum.

Within the impl block, we put the method signatures that the trait definition has required, 
filling the method body with the specific behavior that we want the methods of the trait to have for the particular type.

When a type implements a given trait, it's promising to uphold its contract. 
After implementing the trait, we can call the methods on instances of Circle and Rectangle in the same way we call regular methods, like this:
*/
let circle = Circle { radius: 5.0 };
let rectangle = Rectangle {
    width: 10.0,
    height: 20.0,
};

println!("Circle area: {}", circle.area());
println!("Rectangle area: {}", rectangle.area());
/*
You might have noticed that our custom types are a little difficult to use in practice. 
This simple Point struct can't be compared to other Point instances or displayed in the terminal. 
Because of this difficulty, we might want to use the derive attribute to allow new items to automatically be generated for the struct.

Downside of generic types
Take a look at the following code example:
*/
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!

}
/*
The preceding code will fail for three reasons. See this output:
error[E0277]: `Point` doesn't implement `std::fmt::Display`
--> src/main.rs:10:20
 |
10 |     println!("{}", p1);
 |                    ^^ `Point` cannot be formatted with the default formatter
 |
 = help: the trait `std::fmt::Display` is not implemented for `Point`
 = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
 = note: required by `std::fmt::Display::fmt`
 = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `Point` doesn't implement `Debug`
--> src/main.rs:11:22
 |
11 |     println!("{:?}", p1);
 |                      ^^ `Point` cannot be formatted using `{:?}`
 |
 = help: the trait `Debug` is not implemented for `Point`
 = note: add `#[derive(Debug)]` or manually implement `Debug`
 = note: required by `std::fmt::Debug::fmt`
 = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `==` cannot be applied to type `Point`
--> src/main.rs:13:11
 |
13 |     if p1 == p2 {
 |        -- ^^ -- Point
 |        |
 |        Point
 |
 = note: an implementation of `std::cmp::PartialEq` might be missing for `Point`

error: aborting due to 3 previous errors#+end_example
*/

/*

This code fails to compile because our Point type doesn't implement the following traits:

The Debug trait, which allows a type to be formatted by using the {:?} format specifier, is used in a programmer-facing, debugging context.
The Display trait, which allows a type to be formatted by using the {} format specifier, is similar to Debug. But Display is better suited for user-facing output.
The PartialEq trait, which allows implementors to be compared for equality.
Use derive
Luckily, the Debug and PartialEq traits can be automatically implemented for us by the Rust compiler by using the #[derive(Trait)] attribute, if each of its fields implements the trait:
*/
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
/*
Our code will still fail to compile because Rust's standard library doesn't provide automatic implementation for the Display trait, because it's meant for end users. 
But if we comment out that line, our code now produces this output:
 not equal!
Point { x: 1, y: 2 }
Nevertheless, we can implement the Display trait for our type by ourselves:
*/

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}