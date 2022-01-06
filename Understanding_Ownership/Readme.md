# What is Ownership?
|Stack|Heap|
|-|-|
|fixed size|unknown size|
||return a pointer|
|fast|slow|

>Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

+ Each value in Rust has a variable that’s called its owner.
+ There can only be one owner at a time.
+ When the owner goes out of scope, the value will be dropped.

# References and Borrowing

Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

## Mutable References

    fn main() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

First, we had to change s to be mut. Then we had to create a mutable reference with &mut s where we call the change function, and update the function signature to accept a mutable reference with some_string: &mut String. This makes it very clear that the change function will mutate the value it borrows.

But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. 