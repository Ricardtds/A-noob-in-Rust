# Control Flow

Deciding whether or not to run some code depending on if a condition is true and deciding to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

# if Expressions

|Condition|symbol|
| - | - |
| Less | < |
| Greater| > |
| Different | != |
| Equal | == |

## Simple Use
```
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

## Multiple Conditions with else if
```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
   

```
## Using if in a let Statement

> Because if is an expression, we can use it on the right side of a let statement.

```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```
Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole if expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the **same type**.


# Repetition with Loops

It’s often useful to execute a block of code more than once. For this task, Rust provides several loops. A loop runs through the code inside the loop body to the end and then starts immediately back at the beginning. To experiment with loops, let’s make a new project called loops.

Rust has three kinds of loops: _loop_, _while_, and _for_.

## Loop

>break

    You can place the break keyword within the loop to tell the program when to stop executing the loop.

> continue

    The continue keyword within a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

> Returning Values from Loops</h4>

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

# Conditional Loops with while

It’s often useful for a program to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. This loop type could be implemented using a combination of loop, if, else, and break; you could try that now in a program, if you’d like.

```
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

## Looping Through a Collection with for
```
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```
> A more concise alternative, you can use a for loop and execute some code for each item in a collection.
```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

```

> Using range in a for loop
```
fn main() {
    \\ The rev is used to reverse the range.
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```