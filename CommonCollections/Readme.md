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

## [Using an Enum to Store Multiple Types](https://doc.rust-lang.org/book/ch08-01-vectors.html#using-an-enum-to-store-multiple-types)

We said that vectors can only store values that are the same type. Fortunately, the variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum!

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

Rust needs to know what types will be in the vector at compile time so **it knows exactly how much memory on the heap will be needed to store each element**. A secondary advantage is that we can be explicit about what types are allowed in this vector.

_When you’re writing a program, if you don’t know the exhaustive set of types the program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object, which we’ll cover in Chapter 17._

Now that we’ve discussed some of the most common ways to use vectors, be sure to review the [API documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html) for all the many useful methods defined on `Vec<T>` by the standard library.

## [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)

It’s useful to discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

## [What Is a String?](https://doc.rust-lang.org/book/ch08-02-strings.html#what-is-a-string)

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

> The String type, which is provided by Rust’s standard library rather than coded into the core language, is a **growable**, **mutable**, **owned**, **UTF-8 encoded** string type.

Rust’s standard library also includes a number of other string types, such as `OsString`, `OsStr`, `CString`, and `CStr`.

## [Creating a New String](https://doc.rust-lang.org/book/ch08-02-strings.html#creating-a-new-string)

Starting with the `new` function to create a string.

    let mut s = String::new();

This line creates a new empty string called `s`, which we can then load data into. Often, we’ll have **some initial data** that we want to start the string with. For that, we use the `to_string` method, which is available on any type that **implements** the `Display` trait, as string literals do.

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string()

> We can also use the function `String::from` to create a `String` from a string literal.

    let s = String::from("initial contents");

## [Updating a String](https://doc.rust-lang.org/book/ch08-02-strings.html#updating-a-string)

A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`, if you push more data into it. In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `String` values.

> We can grow a `String` by using the `push_str` method to append a string slice

    let mut s = String::from("foo");
    s.push_str("bar");

_The `push_str` method takes a string slice because we don’t necessarily want to take ownership of the parameter._

> The `push` method takes a single character as a parameter and adds it to the `String`. Listing below shows code that adds the letter "l" to a `String` using the `push` method.

    let mut s = String::from("lo");
    s.push('l');

## [Concatenation with the + Operator or the format! Macro](https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro)

Often, you’ll want to combine two existing strings. One way is to use the `+` operator.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

The reason we’re able to use `&s2` in the call to `add` is that the compiler can coerce the `&String` argument into a `&str`. 


> If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

For more complicated string combining, we can use the `format!` macro:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

The `format!` macro works in the same way as `println!`, but instead of printing the output to the screen, it returns a `String` with the contents. **macro uses references so that this call doesn’t take ownership of any of its parameters.**

## [Indexing into Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings)

In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.

The error and the note tell the story: Rust strings don’t support indexing.

## [Internal Representation](https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation)

A `String` is a wrapper over a `Vec<u8>`.

    let hello = String::from("Hola");

In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long.

    let hello = String::from("Здравствуйте");

Asked how long the string is, you might say 12. However, Rust’s answer is 24, because each Unicode scalar value in that string takes 2 bytes of storage.

## [Bytes and Scalar Values and Grapheme Clusters! Oh My!](https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my)

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as **bytes**, **scalar values**, and **grapheme clusters** (the closest thing to what we would call letters).

> If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

## [Slicing Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings)

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.

To be more specific in your indexing and indicate that you want a string slice, rather than indexing using `[]` with a single number, you can use `[]` with a range to create a string slice containing particular bytes:


    let hello = "Здравствуйте";

    let s = &hello[0..4];

Note that here, `s` will be a `&str` that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes, which means `s` will be `Зд`.

What would happen if we used `&hello[0..1]`? The answer: Rust would panic at runtime in the same way as if an invalid index were accessed in a vector

_You should use ranges to create string slices with caution, because doing so can crash your program._

## [Methods for Iterating Over Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#methods-for-iterating-over-strings)

If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the `chars` method. Calling `chars` on “नमस्ते” separates out and returns six values of type `char`, and you can iterate over the result to access each element:

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

> The `bytes` method returns each raw byte, which might be appropriate for your domain:

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library. Crates are available on [crates.io](https://crates.io/) if this is the functionality you need.

## [Strings Are Not So Simple](https://doc.rust-lang.org/book/ch08-02-strings.html#strings-are-not-so-simple)

To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of `String` data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

Let’s switch to something a bit less complex: hash maps!

## [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#storing-keys-with-associated-values-in-hash-maps)

The last of our common collections is the hash map. The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.

For example, in a game, you could keep track of each team’s score in a hash map in which **each key is a team’s name** and **the values are each team’s score**. Given a team name, you can retrieve its score.

## [Creating a New Hash Map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#creating-a-new-hash-map)

You can create an empty hash map with `new` and add elements with `insert`.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

_Note that we need to first use the HashMap from the collections portion of the standard library._

+ Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude.
+ Hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.

> Just like vectors, hash maps store their data on the **heap**.

Just like vectors, hash maps store their data on the heap. This `HashMap` has keys of type `String` and values of type `i32`. Like vectors, hash maps are **homogeneous**: all of the **keys must have the same type**, and all of the **values must have the same type**.

> Another way of constructing a hash map is by using iterators and the `collect` method on a vector of tuples, where each tuple consists of a key and its value.

For example, if we had the team names and initial scores in two separate vectors, we could use the `zip` method to create an iterator of tuples where _“Blue” is paired with 10_, and so forth. Then we could use the collect `method` to turn that iterator of tuples into a hash map.

    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

The type annotation `HashMap<_, _>` is needed here because it’s possible to `collect`  into many different data structures and Rust doesn’t know which you want unless you specify. However, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

## [Hash Maps and Ownership](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hash-maps-and-ownership)

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be **moved** and the hash map will be the **owner** of those values

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

## [Accessing Values in a Hash Map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map)

We can get a value out of the hash map by providing its key to the `get` method.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

Here, `score` will have the value that’s associated with the Blue team, and the result will be` Some(&10)`. The result is wrapped in `Some` because `get` returns an `Option<&V>`; if there’s no value for that key in the hash map, `get` will return `None`.

We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

## [Updating a Hash Map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-hash-map)

Although the number of keys and values is growable, each key can only have **one** value associated with it at a time.

 + You could replace the old value with the new value, completely disregarding the old value.
 + You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. 
 + Or you could combine the old value and the new value.

## [Overwriting a Value](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#overwriting-a-value)

If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

## [Only Inserting a Value If the Key Has No Value](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value)

It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for it. Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter.
The return value of the `entry` method is an enum called `Entry` that represents a value that might or might not exist.

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.

## [Updating a Value Based on the Old Value](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value)

Another common use case for hash maps is to look up a key’s value and then update it based on the old value. For instance, Listing below shows code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

## [Hashing Functions](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hashing-functions)

By default, `HashMap` uses a hashing function called **SipHash** that can provide resistance to Denial of Service (DoS) attacks involving hash tables1.