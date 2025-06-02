# Rust Scope

## Scope

Now that you understand how functions work, it is important to learn how variables act inside and outside of functions.

Scope refers to where a variable is allowed to be used.

A variable only lives inside the block where it was created. A block is anything inside curly braces { }.

### Variable Inside a Function

A variable created inside a function only exists inside that function:
Example:  

```rust  
fn myFunction() {
  let message = "Hello!";
  println!("{}", message);  // You can access the message variable here
}

myFunction();

println!("{}", message); // Error - you cannot access the message variable outside of the function
```  

Note: The variable message only exists inside the function. Trying to use it outside the function will cause an error.
Variable Inside a Block

You can also create blocks inside other code, like in if statements or loops. Variables created in these blocks are only valid inside them.
Example:  

```rust  
let score = 80;

if score > 50 {
  let result = "Pass";
  println!("Result: {}", result);
}

println!("Result: {}", result); // Error: result is out of scope here
```

### Variables in the Same Scope

Two variables cannot have the same name in the same scope.
Example:  

```rust  
let x = 5;
let x = 10; // Error: name already used in this scope

But inside a new block, you can use the same variable name again:
Example
let x = 5;

{
  let x = 10;
  println!("Inside block: {}", x);
}

println!("Outside block: {}", x);
```

Here, the two x variables are in different scopes, so it is allowed. But if you can, try to use different names to make your code easier to read and understand.

## Why Scope Matters

Understanding scope helps you:

- Know where a variable can be used
- Prevent naming conflicts
- Avoid errors when working with functions, loops, and conditionals
