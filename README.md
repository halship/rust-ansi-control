# rust-ansi-control
rust-ansi-control is the library controlling cursor and screen in terminal.

### [Documentation]()


## Example

```rust
use ansi_control::*;

println!("This is a sample text.");
print!("{}", clear_display(Pos::Both));

```
