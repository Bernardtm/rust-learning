/*
Validate references by using lifetimes

The use of references presents a problem. 
The item a reference is referring to doesn't keep track of all of its references. 
This behavior can lead to an issue: when the item is dropped and its resources are freed, how can we be sure that there are no references that point to the now freed, and invalid, memory?

Languages like C and C++ often have a problem where a pointer points to an item that's already been freed. 
This problem is known as a "dangling pointer". 
Fortunately, Rust eliminates this problem. It guarantees that all references always refer to valid items. 
But, how does it do it?

Rust's answer to this question is lifetimes. 
They allow Rust to ensure memory safety without the performance costs of garbage collection.

Consider the following snippet, which tries to use a reference whose value has gone out of scope:

*/
fn main() {
    let x;
    {
        let y = 42;
        x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
}
// The preceding code fails to compile with the following error message:
//    error[E0597]: `y` does not live long enough

/*
This error occurs because a value was dropped while it was still borrowed. 
In this case, y is dropped at the end of the inner scope, but x borrows it until the println call. 
Because x is still valid for the outer scope (because its scope is larger), we say that it "lives longer."


Here's the same code snippet with drawings around each variable lifetime. We gave each lifetime a name:

'a is the lifetime annotation for our value x.
'b is the lifetime annotation for our value y.
*/
fn main() {
    let x;                // ---------+-- 'a
    {                     //          |
        let y = 42;       // -+-- 'b  |
        x = &y;           //  |       |
    }                     // -+       |
    println!("x: {}", x); //          |
}

/*
Here we can see that the inner 'b lifetime block is shorter than the outer 'a block.

The Rust compiler can verify if the borrows are valid by using the borrow checker. 
The borrow checker compares the two lifetimes at compile time. 
In this scenario, x has a lifetime of 'a but it refers to a value with a lifetime of 'b. 
The reference subject (y at lifetime 'b) is a shorter time than the reference (x at lifetime 'a) so the program doesn't compile.
*/

/*
Annotating lifetimes in functions
As with types, lifetime durations are inferred by the Rust compiler.

There may be multiple lifetimes. 
When that occurs, annotate the lifetimes to help the compiler understand which lifetime it will use to ensure the references are valid at runtime.

For example, consider a function that takes two strings as its input parameters and returns the longest of them:
*/
fn main() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);
}

fn longest_word(x: &String, y: &String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// The preceding code fails to compile with an informative error message:
//     error[E0106]: missing lifetime specifier

/*
The help text says Rust can't tell whether the reference that's being returned refers to x or y. 
Neither can we. So, to help identify what the reference is, annotate the return type with a generic parameter to represent the lifetime.

It's possible that lifetimes could be different whenever the function is called. 
We don't know the concrete lifetimes of the references that will be passed to our longest_word function, and we can't determine if the reference that will be returned will always be a valid one.

The borrow checker can't determine if the reference will be a valid one either. 
It doesn't know how the input parameters' lifetime relates to the return value's lifetime. 
This ambiguity is why we need to annotate the lifetimes manually.

Luckily, the compiler gave us a hint on how to fix this error. 
We can add generic lifetime parameters to our function signature. 
These parameters define the relationship between the references so the borrow checker can complete its analysis:
*/
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
Make sure to declare generic lifetime parameters inside angle brackets, and add the declaration between the parameter list and the function name.

In the signature, the return value and all the parameter references must have the same lifetime. 
As such, use the same lifetime name, for example 'a. 
Then, add the name to each reference in the function signature.

There's nothing special about the name 'a in this case. 
It would be just as fine to use any other word, such as 'response or 'program. 
The important thing to keep in mind is that all parameters and the returned value will live at least as long as the lifetime associated with each of them.

Let's experiment with this sample code and change some values and lifetimes of the references passed in to the longest_word function to see how it behaves. 
The compiler would also reject the following snippet, but can you guess why?
*/
fn main() {
    let magic1 = String::from("abracadabra!");
    let result;
    {
        let magic2 = String::from("shazam!");
        result = longest_word(&magic1, &magic2);
    }
    println!("The longest magic word is {}", result);
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// If you guessed that this code is broken, you're right. This time, we see the following error:
// error[E0597]: `magic2` does not live long enough
/*
This error shows that the compiler expected the lifetime of magic2 to be the same as the lifetime of the returned value and of the x input argument. 
Rust expected this behavior because we annotated the lifetimes of the function parameters and return value by using the same lifetime name: 'a.

If we inspected the code, as humans, we would see that magic1 is longer than magic2.
 We would see that the result contains a reference to magic1, which will live long enough to be valid. 
 However, Rust can't run that code at compile time. 
 It will consider both the &magic1 and &magic2 references to be possible return values and will emit the error that we saw earlier.

The reference's lifetime that the longest_word function returns matches the smaller of the references' lifetimes that are passed in. 
As such, the code possibly includes an invalid reference and the borrow checker will disallow it.
*/

/*
Annotating lifetimes in types
Whenever a struct or enum holds a reference in one of its fields, we must annotate that type definition with the lifetime of each reference that it carries along with it.

For example, consider the following example code. We have a text string (which owns its contents) and a Highlight tuple struct. 
The struct has one field that holds a string slice. The slice is a borrowed value from another part of our program.
*/
#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}
/*
We place the name of the generic lifetime parameter inside angle brackets after the name of the struct. 
This placement is so we can use the lifetime parameter in the body of the struct definition. 
This instance of Highlight can't live longer than the reference in its field because of the declaration.

In the preceding code, we annotated our struct with a lifetime called 'document. 
This annotation is a reminder that the Highlight struct can't outlive the source of the &str that it borrows, a supposed document.

The main function here creates two instances of the Highlight struct. 
Each instance holds a reference to a segment of the String value owned by the variable text:

fox references the segment between the 4th and 19th characters of the text string.
dog references the segment between the 35th and 43rd characters of the text string.
Also, Highlight goes out of scope before text goes out of scope, which means that the Highlight instance is valid.
*/