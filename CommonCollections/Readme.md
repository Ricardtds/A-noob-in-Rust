# [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html#common-collections)

Rust’s standard library includes a number of very _useful data structures_ called **collections**. Most other data types represent one specific value, but **collections can contain multiple values**. Unlike the built-in array and tuple types, the data these collections point to is **stored on the heap**, which means the **amount of data does not need to be known at compile time** and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. In this chapter, we’ll discuss three collections that are used very often in Rust programs:

1. A vector allows you to store a variable number of values next to each other.
2. A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
3. A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

To learn about the other kinds of collections provided by the standard library, see the documentation.

We’ll discuss how to create and update vectors, strings, and hash maps, as well as what makes each special.

## [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html#storing-lists-of-values-with-vectors)

To create a new, empty vector, we can call the Vec::new function.

    let v: Vec<i32> = Vec::new();
>In more realistic code, Rust can often infer the type of value you want to store once you insert values, so you rarely need to do this type annotation.

It’s more common to create a Vec<T> that has initial values, and Rust provides the vec! macro for convenience. The macro will create a new vector that holds the values you give it.

## [Updating a Vector](https://doc.rust-lang.org/book/ch08-01-vectors.html#updating-a-vector)

To create a vector and then add elements to it, we can use the push method

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

Note that we need to make it **mutable** using the `mut` keyword.

## [Dropping a Vector Drops Its Elements](https://doc.rust-lang.org/book/ch08-01-vectors.html#dropping-a-vector-drops-its-elements)

Like any other `struct`, a vector is freed when it goes out of scope.
    
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

> When the vector gets dropped, all of its contents are also dropped, meaning those integers it holds will be cleaned up.

## [Reading Elements of Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors)


Below shows both methods of accessing a value in a vector, either with **indexing syntax** or the **get method**.

    let v = vec![1, 2, 3, 4, 5];

    let third: & i32= &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

The two ways to get the third element are by using `&` and `[]`, which gives us a reference, or by using the `get` method with the index passed as an argument, which gives us an `Option<&T>`.

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

When we run this code, the first `[]` method will cause the program to panic because it references a nonexistent element. This method is best used **when you want your program to crash if there’s an attempt to access an element past the end of the vector**.

When the `get` method is passed an index that is outside the vector, it **returns None without panicking**. Your code will then have logic to handle having either `Some(&element)` or `None`.

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {}", first);

_ERROR: cannot borrow `v` as mutable because it is also borrowed as immutable_

This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

## [Iterating over the Values in a Vector](https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector)

If we want to access each element in a vector in turn, we can iterate through all of the elements rather than use indices to access one at a time. The code below shows how to use a `for` loop to get immutable references to each element in a vector of `i32` values and print them.

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The `for` loop in Listing below will add 50 to each element.

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

> Note that to change the value that the mutable reference refers to, we have to use the dereference operator (`*`) to get to the value in `i` before we can use the `+=` operator.