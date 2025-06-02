# Ownership

Rust uses "ownership" to manage memory in a safe way.

Every value in Rust has an owner. The owner is usually a variable.

## Ownership Rules

- Each value has one owner
- When the owner goes out of scope, the value is deleted
- You can only have one owner at a time, unless you borrow it (covered in the next chapter)

## Basic Ownership Example

In this example, a owns the string. Then we move it to b:

```rust  
let a = String::from("Hello");
let b = a;

// println!("{}", a); Error: a no longer owns the value
println!("{}", b); // Ok: b now owns the value
```

When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid.

But simple types like numbers, characters and booleans are copied, not moved.

This means you can still use the original variable after assigning it to another:

```rust  
let a = 5;
let b = a;
println!("a = {}", a);  // Works
println!("b = {}", b);  // Works
```

Here, a is copied into b, not moved, so you can still use b.
Clone

For other types, like String, if you really want to keep the original value and also assign it to another variable, you can use the .clone() method, which makes a copy of the data:

```rust  
let a = String::from("Hello");
let b = a.clone(); // Now both have the same value

println!("a = {}", a);  // Works
println!("b = {}", b);  // Works
```

However, if you don't need to own the value twice, using a reference (&) is usually better than cloning, which you will learn more about in the next chapter.
Why Ownership Matters

- Rust uses ownership to automatically free memory when it's no longer needed
- It prevents bugs like using memory that's already been deleted
- It is one of the reasons Rust is so safe and fast

Next: Learn about borrowing - how to let other parts of your program use a value without taking ownership.  
