# Defining
Structs are similar to tuples. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean. As a result of these names, structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

This topic is nice to review when you face a problem.

# Method Syntax
Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions **in that they’re defined within the context of a struct** (or an enum or a trait object), and their first parameter is **always self**, which represents the instance of the struct the method is being called on.

> To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

In the signature for area, we use &self instead of rectangle: &Rectangle. *The &self is actually short for self: &Self.*

Often, but not always, methods with the same name as a field will be defined to only return the value in the field and do nothing else. Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do. Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type’s public API.

> Where’s the -> Operator?
Check documentation.

> Methods with More Parameters

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

> Associated Functions

All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can **define associated functions that don’t have self as their first parameter** (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this, the String::from function, that’s defined on the String type.

For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example. This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.

> Multiple impl Blocks

Each struct is allowed to have multiple impl blocks. For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block.

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

**There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax. We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.**