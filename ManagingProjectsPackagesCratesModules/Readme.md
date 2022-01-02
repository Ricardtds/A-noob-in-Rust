# [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

+ As a package grows, you can extract parts into separate crates that become external dependencies.
+ You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.

> Features:

1. Packages: A Cargo feature that lets you build, test, and share crates
2. Crates: A tree of modules that produces a library or executable
3. Modules and use: Let you control the organization, scope, and privacy of paths
4. Paths: A way of naming an item, such as a struct, function, or module

## [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

+ If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package.
+ A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

_A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects._

For example, the rand crate we used in [Chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number) provides functionality that generates random numbers.


## [Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

> In this section, we’ll talk about modules and other parts of the module system, namely paths that allow you to name items.

+ the **use** keyword that brings a path _into scope_; and the **`pub`** keyword to make items _public_. We’ll also discuss the **as** keyword, external packages, and the glob operator.

Explanation | privacy 
-- | --
which is whether an item can be used by outside code | Public
internal implementation detail and not available for outside use | Private

1. Create a new library named restaurant by running cargo new --lib restaurant
2. then put the code in Listing 7-1 into src/lib.rs to define some modules and function signatures.
> A restaurant example

    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn serve_order() {}

            fn take_payment() {}
        }
    }
_Filename: src/lib.rs_

+ We define a module by starting with the **`mod`** keyword and then specify the **name of the module**.

_Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree._

    crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment
_module tree_

+ Attention to the implicit module named **crate**.

## [Paths for Referring to an Item in the Module Tree](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)

A path can take two forms:
+ An absolute path starts from a crate root by using a crate name or a literal **`crate`**.
+ A relative path starts from the current module and uses **`self`**, **`super`**, or an identifier in the current module.

_Both absolute and relative paths are followed by one or more identifiers separated by double colons (::)._

> So... How do we call the add_to_waitlist function?

_Choosing whether to use a relative or absolute path is a decision you’ll make based on your project._

+ The decision should depend on whether you’re more likely to move item definition code separately from or together with the code that uses the item.
  + Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.

>Modules aren’t useful only for organizing your code. They also define Rust’s privacy boundary: the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.

+ The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are **private by default**. _Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules_.
+ Check documentation for a better explanation!

## [Exposing Paths with the pub Keyword](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword)

>Let’s also make the add_to_waitlist function public by adding the pub keyword before its definition, like the mod hosting.

    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }

_Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the function from eat_at_restaurant_

## [Starting Relative Paths with super](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#starting-relative-paths-with-super)

We can also construct relative paths that begin in the parent module by using **`super`** at the start of the path. This is like starting a filesystem path with the **`..`** syntax. Why would we want to do this?

    fn serve_order() {}
    
    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }
    
        fn cook_order() {}
    }

_The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent module of back_of_house, which in this case is crate, the root._

## [Making Structs and Enums Public](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)

 We can also use `pub` to designate structs and enums as public, but there are a few extra details. If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private.

    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }
_Filename: src/lib.rs_

>  if we make an enum public, all of its variants are then public. We only need the pub before the `enum` keyword.

    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }

_Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad` variants in `eat_at_restaurant`_
