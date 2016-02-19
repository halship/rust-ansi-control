# rust-ansi-control
This is a library for controlling cursor and screen on ANSI terminals.


## Example

```rust
use ansi_control::*;

println!("This is a sample text.");
print!("{}", clear_display(Pos::Both));

```
