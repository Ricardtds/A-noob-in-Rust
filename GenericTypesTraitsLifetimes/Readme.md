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