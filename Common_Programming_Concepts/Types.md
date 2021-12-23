# Numbers

## Integer Types in Rust

|Lenght|Signed|Unsigned|
|------|------|--------|
|8-bit|i8|u8|
|16-bit|i16|u16
|32-bit|i32|u32
|64-bit|i64|u64
|128-bit|i128|u128|
|arch|isize|usize|

|Number literals|Example|
|-|-|
|Decimal|98_222|
|Hex|0xff|
|Octal|0o77|
|Binary|0b1111_0000|
|Byte(u8 only)|b'A'|

### To explicity gandle the possibility of overflow

> Wrap ain all modes with the "wrapping_*" methods, such as "wrapping_add"

> Return the None value if there is overflow with the "checked_*" methods

> Return the value and a boolean indicating whether there was overflow with the "overflowing_*" methods

> Saturate at the value's minimum or maximu values with "saturating_*" methods

## Floating-Point Types

|f32|f64|
|---|---|
|32-bits|64-bits|

## Numeric Operations

|Operation|Symbol|||
|---------|------|-|-|
|sum|+|
|subtraction|-|
|multiplication|*|
|division|/|
|remainder|%|

# The Boolean Type

|true|false|
|-|-|

# The Character Type

Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.

|character|string|
|---------|------|
|'a'|"text"|

Rust's char type is **four bytes** in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

# Compound Types

Can group multiple values into one type. Rust has two primitive coumpound types: tuples and arrays.
|Tuple|Array|
|-----|-----|
|Imutable|Mutable|

## The Tuple Type

Tuples have a fixed lenght: once declared, they cannot grow or shrink in size.
Tuples can be created by writing comma-separed list of values inside parentheses. Each position in the tuple has a type, and the types of differente values in tuple don't have to be the same.

    '''let tup:(i8,u8,f32) = (127, 255, 54.5);
    let (x, y, z) = tup; //destructuring'''

The tuple without any values, (), is a speciel type that gas only one value.
The type is called the unit type and the value is called the unit value.
Expressions implicitly return the unit value if they don't return any other value.

## The Array Type

Unlike a tuple, every elementof an array **must have** the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a **fixed length**, like tuples.

> Writing an array's type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number number of elements in the array, like so:

    let a: [i32; 5] = [1, 2, 3, 4, 5];

> Creating an array that contains the same value for each element.

    let a = [3; 5]; // equals to let a = [3, 3, 3, 3, 3];

## The String Type

This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:


    let s = String::from("hello");

> Memory and Allocation

In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

+ The memory must be requested from the memory allocator at runtime.
+ We need a way of returning this memory to the allocator when we’re done with our String.

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

> So what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

+ All the integer types, such as u32.
+ The Boolean type, bool, with values true and false.
+ All the floating point types, such as f64.
+ The character type, char.
+ Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## The  Slice Type

Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method:

> A String slice

    fn main() {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
    }
> String Literals Are Slices

Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:  

    fn main() {
        let s = "Hello, world!";
    }

> String slice as parameter

Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:

    fn first_word(s: &String) -> &str {


A more experienced Rustacean would write the signature shown below instead because it allows us to use the same function on both &String values and &str values.

    fn first_word(s: &str) -> &str {

## Type Self
