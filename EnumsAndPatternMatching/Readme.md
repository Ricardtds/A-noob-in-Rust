# Defining an Enum

Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate, because _enum values can only be one of its variants_. Both version four and version six addresses are _still fundamentally IP addresses_, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6. These are the variants of the enum:

    enum IpAddrKind {
        V4,
        V6,
    }

_IpAddrKind is now a **custom data type** that we can use elsewhere in our code._

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

And we can call this function with either variant:

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

> We attach data to each variant of the enum directly, so there is no need for an extra struct.

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

> A better way

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

+ The name of each enum variant that we define also becomes a function that constructs an instance of the enum.

# The match Control Flow Operator

+ Match allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
+ Patterns can be made up of literal values, variable names, wildcards, and many other things.

>

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

## Patterns that Bind to Values

+ Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

+ We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it,

>

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

## Matching with Option`<T>`

+ Instead of comparing coins, we’ll compare the variants of Option `<T>`, but the way that the match expression works remains the same.

> Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

### Matches are Exhaustive

+ If we don't handle the None case, the code will cause a bug. Luckily, it’s a bug Rust knows how to catch. If we try to compile the code, we’ll get this error:

>

    $ cargo run
    Compiling enums v0.1.0 (file:///projects/enums)
    error[E0004]: non-exhaustive patterns: `None` not covered
    --> src/main.rs:3:15
        |
    3   |         match x {
        |               ^ pattern `None` not covered
        |
        = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
        = note: the matched value is of type `Option<i32>`

    For more information about this error, try `rustc --explain E0004`.
    error: could not compile `enums` due to previous error

- Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.


## Catch-all Paterns and the _ Placeholder

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

+ The other matches all others possible values, in this case we put the catch all last because the patterns area evaluated in **order**.

+ _Rust will warn us if we add arms after a catch-all because those later arms would never match!_

> Rust also has a pattern we can use when we don’t want to use the value in the catch-all pattern: _, which is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

>

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}

Note that the we aren't using the 'other' anymore, instead we are using the '_'. This is because we aren't going to use that value again. _W    e’re explicitly ignoring all other values_

>If we change the rules of the game one more time, so that nothing else happens on your turn if you roll anything other than a 3 or a 7, we can express that by using the unit value (the empty tuple type)

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

> Check documentation [Catch-all Patterns and the _ Placeholder](https://doc.rust-lang.org/book/ch06-02-match.html#catch-all-patterns-and-the-_-placeholder) for a better explanation.

# [Concise Control Flow with if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)

The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest. Consider the program in Listing 6-6 that matches on an Option<u8> value in the config_max variable but only wants to execute code if the value is the Some variant.

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

+ The syntax if let takes a pattern and an expression separated by an equal sign. It works the same way as a match
+ __PS:__ The code in the if let block isn’t run if the value doesn’t match the pattern.
+ Using if let means less typing, less indentation, and less boilerplate code.

> We can include an else with an if let

 The block of code that goes with the else is the same as the block of code that would go with the _

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

