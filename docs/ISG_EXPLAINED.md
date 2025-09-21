# Understanding the Interface Signature Graph (ISG)

## What is an ISG?

Think of the ISG as a **map of your Rust code** that shows:
1. **What exists** (functions, structs, traits)
2. **How they connect** (who calls what, who implements what)

It's like a family tree, but for code!

## Real Example

Let's look at this simple Rust code:

```rust
// A struct representing a user
pub struct User {
    pub name: String,
    pub age: u32,
}

// A trait for displaying things
pub trait Display {
    fn fmt(&self) -> String;
}

// User implements Display
impl Display for User {
    fn fmt(&self) -> String {
        format!("{} (age {})", self.name, self.age)
    }
}

// A function that creates users
pub fn create_user(name: String, age: u32) -> User {
    User { name, age }
}

// Main function
fn main() {
    let user = create_user("Alice".to_string(), 30);
    println!("{}", user.fmt());
}
```

## How ISG Sees This Code

The ISG breaks this down into:

### NODES (The "Things")
```
1. main (Function)
   - Signature: fn main()
   - File: src/main.rs:1

2. User (Struct) 
   - Signature: struct User { name: String, age: u32 }
   - File: src/lib.rs:5

3. Display (Trait)
   - Signature: trait Display { fn fmt(&self) -> String; }
   - File: src/lib.rs:10

4. create_user (Function)
   - Signature: fn create_user(name: String, age: u32) -> User
   - File: src/lib.rs:15
```

### EDGES (The "Relationships")
```
User --Implements--> Display
  (User struct implements the Display trait)

main --Calls--> create_user  
  (main function calls create_user function)

create_user --Uses--> User
  (create_user function returns/uses User struct)
```

## Visual Representation

```
    main()
      |
      | Calls
      ↓
  create_user()
      |
      | Uses
      ↓
    User ---------> Display
         Implements
```

## What Can You Do With This?

### 1. **Find Implementors**
Question: "What implements the Display trait?"
Answer: "User struct"

### 2. **Blast Radius Analysis**
Question: "If I change the User struct, what else might break?"
Answer: "create_user function and anything that implements Display"

### 3. **Dependency Analysis**
Question: "What does the User struct depend on?"
Answer: "Display trait (because it implements it)"

### 4. **LLM Context Generation**
When you ask an AI about User, it gets:
- The User struct definition
- That it implements Display
- That create_user() returns it
- That main() uses it indirectly

## Why This Matters

Instead of searching through text files, you can:
- **Instantly find** all implementations of a trait
- **Understand impact** of changes before making them  
- **Navigate code** by following relationships
- **Generate perfect context** for AI tools (no hallucinations!)

## Performance

The ISG is super fast because it's all in memory:
- **Node operations**: ~6 microseconds
- **Simple queries**: <500 microseconds  
- **Complex queries**: <1 millisecond
- **File updates**: <12 milliseconds

## Try It Yourself

1. **See the sample**: `./target/debug/parseltongue debug --sample`
2. **Analyze real code**: `./target/debug/parseltongue ingest your_code.dump`
3. **Query relationships**: `./target/debug/parseltongue query what-implements Display`
4. **Generate context**: `./target/debug/parseltongue generate-context User --format json`

The ISG turns your code from a pile of text files into a **queryable knowledge graph**!