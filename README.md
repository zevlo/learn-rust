# Learn Rust

A collection of Rust examples and exercises for learning Rust programming fundamentals.

## Overview

This repository contains hands-on examples organized by topic to help learn Rust concepts through practical code examples. Each example demonstrates a specific Rust feature or programming pattern.

## Repository Structure

The examples are organized into the following categories:

### **basics/**
- `calculator_function.rs` - Basic function implementation with conditional logic

### **for-loop/**
- `factorial_function.rs` - Loop iteration and factorial calculation

### **iterators/**
- `dedup.rs` - Deduplicating vector elements using HashSet
- `vector_iteration.rs` - Iterating over vectors

### **options/**
- `vector_modification.rs` - Working with Option type for safe vector operations

### **sets/**
- `hashset.rs` - HashSet basic operations
- `hashset_membership.rs` - Checking membership in HashSets
- `safe_casting.rs` - Type casting with safety
- `extend_collection.rs` - Extending collections with HashSet and Vec

### **tuples/**
- `tuple_return.rs` - Returning multiple values using tuples

### **type/**
- `downast.rs` - Type downcasting examples

### **vectors/**
- `palindrome.rs` - Palindrome checking with vectors
- `sorted_vector.rs` - Sorting vectors
- `clone.rs` - Cloning vectors
- `reuse_vector.rs` - Reusing vector memory
- `convert_vector.rs` - Converting between vector types
- `copy.rs` - Copy semantics with vectors

## Running Examples

Since these are standalone Rust files, you can run them individually using `rustc` or `cargo`:

### Using rustc:

```bash
# Compile and run a single file
rustc basics/calculator_function.rs
./calculator_function
```

### Using cargo script:

```bash
# Run a file directly
cargo script basics/calculator_function.rs
```

### Using rust-script (if installed):

```bash
rust-script basics/calculator_function.rs
```

## Topics Covered

- **Functions**: Defining and calling functions, return values
- **Control Flow**: If/else statements, loops
- **Data Structures**: Vectors, HashSets, tuples
- **Iterators**: Working with collections efficiently
- **Option Type**: Safe handling of optional values
- **Type System**: Casting, cloning, and copy semantics
- **Collections**: Extending, converting, and manipulating data structures

## Learning Path

If you're new to Rust, consider exploring the examples in this order:

1. Start with **basics/** to understand functions and control flow
2. Move to **vectors/** to learn about the most common collection type
3. Explore **iterators/** to work with collections efficiently
4. Check out **options/** to understand Rust's approach to null safety
5. Learn about **tuples/** for returning multiple values
6. Dive into **sets/** for unique collections
7. Experiment with **type/** examples for type system features

## Requirements

- Rust toolchain (rustc and cargo)
- Install from [rust-lang.org](https://www.rust-lang.org/)

## Contributing

Feel free to add more examples or improve existing ones!

## License

This is a learning repository - use freely for educational purposes.
