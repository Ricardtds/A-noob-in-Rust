# [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html#generic-types-traits-and-lifetimes)

Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics. Generics are abstract stand-ins for concrete types or other properties. When we’re writing code, we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

...

Then you’ll learn how to use traits to define behavior in a generic way. You can combine traits with generic types to constrain a generic type to only those types that have a particular behavior, as opposed to just any type.

Finally, we’ll discuss lifetimes, a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to borrow values in many situations while still enabling the compiler to check that the references are valid.

## [Removing Duplication by Extracting a Function](https://doc.rust-lang.org/book/ch10-00-generics.html#removing-duplication-by-extracting-a-function)

1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.

For example, say we had two functions: one that finds the largest item in a slice of `i32` values and one that finds the largest item in a slice of `char` values. How would we eliminate that duplication? Let’s find out!

# [Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html#generic-data-types)

We can use generics to create definitions for items like **function signatures** or **structs**, which we can then use with many different concrete data types. Let’s first look at how to define _functions_, _structs_, _enums_, and _methods_ using generics. Then we’ll discuss how generics affect code performance.

## [In Function Definitions](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions)

When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

Combining both functions into one we get:

    fn largest<T>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

_Note that this code won’t compile yet_

## [In Struct Definitions](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions)

We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax. Listing below shows how to define a `Point<T>` struct to hold `x` and `y` coordinate values of any type.

    struct Point<T> {
        x: T,
        y: T,
    }

    fn main() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }

To define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters. For example, in Listing 10-8, we can change the definition of `Point` to be generic over types `T` and `U` where `x` is of type `T` and `y` is of type `U`.

    struct Point<T, U> {
        x: T,
        y: U,
    }

    fn main() {
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }

## [In Enum Definitions](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)

As we did with structs, we can define enums to hold generic data types in their variants. Let’s take another look at the `Option<T>` enum that the standard library provides, which we used in Chapter 6:

    enum Option<T> {
        Some(T),
        None,
    }

This definition **should now make more sense to you**. As you can see, `Option<T>` is an enum that is generic over type `T` and has two variants: Some, which holds one value of type `T`, and a `None` variant that doesn’t hold any value.

Enums can use multiple generic types as well. The definition of the `Result` enum that we used in Chapter 9 is one example:

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

The `Result` enum is generic over two types, `T` and `E`, and has two variants: Ok, which holds a value of type `T`, and Err, which holds a value of type `E`.

## [In Method Definitions](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions)

We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions, too. Listing below shows the `Point<T>` struct we defined in Listing before with a method named `x` implemented on it.

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }

We could, for example, implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type. In Listing below we use the concrete type `f32`, meaning we don’t declare any types after `impl`.

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

This code means the type `Point<f32>` will have a method named `distance_from_origin` and other instances of `Point<T>` where `T` is not of type `f32` **will not have this method defined**. The method measures how far our point is from the point at coordinates (0.0, 0.0) and uses mathematical operations that are available only for floating point types.

...

Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures. For example, Listing below defines the method `mixup` on the `Point<T, U>` struct. The method takes another `Point` as a parameter, which might have different types from the `self` `Point` we’re calling `mixup` on. The method creates a new `Point` instance with the `x` value from the `self` `Point` (of type `T`) and the `y` value from the passed-in `Point` (of type `W`).

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

The purpose of this example is to demonstrate a situation in which some generic parameters are declared with `impl` and some are declared with the method definition. Here, the generic parameters `T` and `U` are declared after `impl`, because they go with the struct definition. The generic parameters `V` and `W` are declared after `fn mixup`, because they’re only relevant to the method.

## [Performance of Code Using Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)

You might be wondering whether there is a runtime cost when you’re using generic type parameters. **The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.**

    let integer = Some(5);
    let float = Some(5.0);

When Rust compiles this code, it performs monomorphization...

The monomorphized version of the code looks like the following. The generic `Option<T>` is replaced with the specific definitions created by the compiler:

    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    fn main() {
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }
_Filename: src/main.rs_

> Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.

# [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior)

A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

## [Defining a Trait](https://doc.rust-lang.org/book/ch10-02-traits.html#defining-a-trait)

 Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

 We want to make a media aggregator library crate named `aggregator` that can display summaries of data that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we need a summary from each type, and we’ll request that summary by calling a `summarize` method on an instance. Listing below shows the definition of a public `Summary` trait that expresses this behavior.

    pub trait Summary {
        fn summarize(&self) -> String;
    }
_Filename: src/lib.rs_

## [Implementing a Trait on a Type](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
_Filename: src/lib.rs_

Now that the library has implemented the `Summary` trait on `NewsArticle` and `Tweet`, users of the crate can call the trait methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods. The only difference is that the trait has to be brought into scope as well as the types to get the additional trait methods. Here’s an example of how a binary crate could use our `aggregator` library crate:

    use aggregator::{Summary, Tweet};

    fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

## [Default Implementations](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations)

Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
_Filename: src/lib.rs_

To use a default implementation to summarize instances of `NewsArticle` instead of defining a custom implementation, we specify an empty `impl` block with `impl Summary for NewsArticle {}`.

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

## [Traits as Parameters](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters)

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait. In the body of `notify`, we can call any methods on `item` that come from the `Summary` trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`. Code that calls the function with any other type, such as a `String` or an `i32`, won’t compile because those types don’t implement `Summary`.

## [Trait Bound Syntax](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax)

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound; it looks like this:

    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    
This longer form is equivalent to the example in the previous section but is more verbose. We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.

The `impl Trait` syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases. For example, we can have two parameters that implement Summary. Using the impl Trait syntax looks like this:

    pub fn notify(item1: &impl Summary, item2: &impl Summary) {

If we wanted this function to allow `item1` and `item2` to have different types, using impl Trait would be appropriate (as long as both types implement `Summary`). If we wanted to force both parameters to have the same type, that’s only possible to express using a trait bound, like this:

    pub fn notify<T: Summary>(item1: &T, item2: &T) {

## [Specifying Multiple Trait Bounds with the + Syntax](https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax)


We can also specify more than one trait bound. Say we wanted `notify` to use display formatting on `item` as well as the `summarize` method: we specify in the `notify` definition that `item` must implement both `Display` and `Summary`. We can do so using the + syntax:

    pub fn notify(item: &(impl Summary + Display)) {

The + syntax is also valid with trait bounds on generic types:

    pub fn notify<T: Summary + Display>(item: &T) {

## [Clearer Trait Bounds with where Clauses](https://doc.rust-lang.org/book/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses)

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature. So instead of writing this:

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

we can use a where clause, like this:

    fn some_function<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone,
            U: Clone + Debug
    {

## [Returning Types that Implement Traits](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits)

We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait, as shown here:

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
