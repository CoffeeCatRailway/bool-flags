bool-flags
==========

`bool-flags` adds a simple struct type to condense multiple booleans into fewer bytes.

1 boolean tends to equal 1 byte, while 1 `Flags8` equals 8 booleans in 1 byte.

There are five `Flags` types by default, `Flags8`, `Flags16`, `Flags32`, `Flags64` & `Flags128`.<br>
Check `Cargo Features` below for `FlagsUSize`.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
bool-flags = "0.1.0"
```

## Example
```rust
use bool_flags::Flags8;

fn main() {
    let flags = Flags8::none(); // Creates `Flags8` struct, this contains 8 flags or booleans all set to false
    flags.set(0); // Sets the first flag (left-most bit) to true
    
    if flags.get(0) { // Gets value of the first flag (left-most bit)
        println!("First flag is true");
    }
    
    if flags.get(1) { // Gets value of the second flag
        println!("Second flag is false");
    }
    
    if flags.get(8) { // Indices wrap around so index 8 is the same as index 0 
        println!("First flag is true"); 
    }
}
```

## Cargo Features
The `bool-flags` library defines two Cargo features:
- `inline`: Makes `Flags` struct functions inline.
- `usize`: Enables `FlagsUSize` type.