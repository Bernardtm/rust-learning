mod authentication;

fn main() {
    let mut user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    user.set_password("even-more-secret");
}

/*
Place a semicolon after mod authentication instead of a code block. 
As files grow in size, this technique lets you move modules to new files automatically. 
The compiler loads the module contents from another file that's named the same as the module.

When the content loads, the module tree remains the same. The code will also work without requiring any changes, even though the definitions exist in different files.

*/