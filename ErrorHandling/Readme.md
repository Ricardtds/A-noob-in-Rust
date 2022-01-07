# [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html#error-handling)

Rust has a number of features for handling situations in which something goes wrong. In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile. This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your code to production!

> Rust groups errors into two major categories:

+ recoverable
+ unrecoverable

For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Rust doesn’t have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error. 

# [Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)

When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

Read about [Unwinding the Stack or Aborting in Response to a Panic](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic)

How to Panic? See the example below!

    fn main() {
        panic!("crash and burn");
    }

## [Using a panic! Backtrace](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#using-a-panic-backtrace)

Let’s look at another example to see what it’s like when a `panic!` call comes from a library because of a bug in our code instead of from our code calling the macro directly.

    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }
    
A backtrace is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above the lines mentioning your files are code that your code called; the lines below are code that called your code. These lines might include core Rust code, standard library code, or crates that you’re using. Let’s try getting a backtrace by setting the RUST_BACKTRACE environment variable to any value except 0.

Debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag, as we have here.

# [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)

Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

If we give `f` a type annotation that we know is not the return type of the function and then try to compile the code, the compiler will tell us that the types don’t match. The error message will then tell us what the type of `f` is. Let’s try it! We know that the return type of `File::open` isn’t of type `u32`, so let’s change the let `f` statement to this:

    use std::fs::File;

    fn main() {
        let f: u32 = File::open("hello.txt");
    }

Attempting to compile now gives us a output.

This tells us the return type of the `File::open` function is a `Result<T, E>`. The generic parameter `T` has been filled in here with the type of the success value, `std::fs::File`, which is a file handle. The type of `E` used in the error value is `std::io::Error`.

We need to add to the code to take different actions depending on the value `File::open`

    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

## [Matching on Different Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors)

What we want to do instead is take different actions for different failure reasons: if `File::open` failed because the file doesn’t exist, 
+ we want to create the file and return the handle to the new file.
+ If `File::open` failed for any other reason—for example, because we didn’t have permission to open the file—we still want the code to `panic!`

>

    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }

> A more seasoned Rustacean might write this code below instead

    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }


## [Shortcuts for Panic on Error: unwrap and expect](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error-unwrap-and-expect)

Using `match` works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The `Result<T, E>` type has many helper methods defined on it to do various tasks. One of those methods, called `unwrap`, is a shortcut method that is implemented just like the `match` expression.

+ If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us.

>

    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").unwrap();
    }


Another method, `expect`, which is similar to `unwrap`, lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier. The syntax of `expect` looks like this:

    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }

## [Propagating Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors)

When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.


    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

## [A Shortcut for Propagating Errors: the ? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)

    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

The `?` placed after a `Result` value is defined to work in almost the same way as the `match` expressions we defined to handle the `Result` values in Listing 9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?`

    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

> Speaking of different ways to write this function, Listing below shows that there’s a way to make this even shorter.

    use std::fs;
    use std::io;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

Reading a file into a string is a fairly common operation, so Rust provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it. Of course, using `fs::read_to_string` **doesn’t give us the opportunity to explain all the error handling**, so we did it the longer way first.

## [The ? Operator Can Be Used in Functions That Return Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#the--operator-can-be-used-in-functions-that-return-result)

The `?` operator can be used in functions that have a return type of `Result`, because it is defined to work in the same way as the `match` expression we defined before. The part of the `match` that requires a return type of `Result` is `return Err(e)`, so the return type of the function has to be a `Result` to be compatible with this `return`.

The `main` function is special, and there are restrictions on what its return type must be. One valid return type for main is `()`, and conveniently, another valid return type is `Result<T, E>`, as shown here:

    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }

> The `Box<dyn Error> `type is called a trait object.

# [To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic)

So how do you decide when you should call `panic!` and when you should return `Result?` When code panics, there’s no way to recover. You could call `panic!` for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision on behalf of the code calling your code that a situation is unrecoverable.

## [Examples, Prototype Code, and Tests](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#examples-prototype-code-and-tests)

> Read doc

## [Cases in Which You Have More Information Than the Compiler](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler)

It would also be appropriate to call `unwrap` when you have some other logic that ensures the `Result` will have an `Ok` value, but the logic isn’t something the compiler understands. You’ll still have a `Result` value that you need to handle: whatever operation you’re calling still has the possibility of failing in general, even though it’s logically impossible in your particular situation. If you can ensure by manually inspecting the code that you’ll never have an `Err` variant, it’s perfectly acceptable to call `unwrap`. Here’s an example:

    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1".parse().unwrap();

## [Guidelines for Error Handling](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling)

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

+ The bad state is not something that’s expected to happen occasionally.
+ Your code after this point needs to rely on not being in this bad state.
+ There’s not a good way to encode this information in the types you use.

However, when failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call.

## [Creating Custom Types for Validation](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation)

> Example of the the guess game

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

Check the documentation for a better explanation!

# [Summary](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#summary)

Rust’s error handling features are designed to help you write more robust code. The `panic!` macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. The `Result` enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use `Result` to tell code that calls your code that it needs to handle potential success or failure as well. Using `panic!` and `Result` in the appropriate situations will make your code more reliable in the face of inevitable problems.

Now that you’ve seen useful ways that the standard library uses generics with the `Option` and `Result` enums, we’ll talk about how generics work and how you can use them in your code.