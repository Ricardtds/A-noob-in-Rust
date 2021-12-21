# Functions

Rust code uses snake case as the conventional style for **function** and **variable names**. In snake case, **all** letters are *lowercase* and *underscores* separate words.

Rust doesn't care where you define your functionsm only that they're defined somewhere.

## Parameters

in function signatures, you **must** declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean.

## Function Bodies Contain Statements and Expressions

Statments are instructions that perfome some action and **do not** return a value. Expressions evaluate to a resulting value. Let's look at some examples.

    Creating a variable and assigning a value to it with the let keyword is a statement. In Listing 3-1, let y = 6; is a statement.

Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}, is an expression, for exampl

    fn main() {
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {}", y);
    }     
>Note the x + 1 line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. **If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.**

## Functions with Return Values

Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->)