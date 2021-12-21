# Numbers

## Integer Types in Rust

|Lenght|Signed|Unsigned|
|------|------|--------|
|8-bit|i8|u8|
|16-bit|i16|u16
|32-bit|i32|u32
|64-bit|i64|u64
arch|isize|usize
|128-bit|i128|u128|

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

