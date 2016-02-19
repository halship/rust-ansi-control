# rust-ansi-control [![ansi_control on crates.io](http://meritbadge.herokuapp.com/ansi_control)](https://crates.io/crates/ansi_control) [![Build Status](https://travis-ci.org/halship/rust-ansi-control.svg?branch=master)](https://travis-ci.org/halship/rust-ansi-control)
This is a library for controlling cursor and screen on ANSI terminals.

### [Documentation](http://halship.github.io/rust-ansi-control/ansi_control/)


## Example

```rust
use ansi_control::*;

println!("This is a sample text.");
print!("{}", clear_display(Pos::Both));

```
