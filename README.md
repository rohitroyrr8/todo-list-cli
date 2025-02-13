# TODO List CLI
allows you to add, remove, list, and mark tasks as done using a simple command-line interface.

## To Run 

```
cargo run -- list
cargo run -- add "Buy groceries"
cargo run -- done 1
cargo run -- remove 1
```


## Features & Improvements
- Persistent storage (Saves tasks in todo_list.json)
- Simple CLI interface
- Error handling (Handles invalid task IDs)
- Task management with completion trackin


## Install Cargo and Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
cargo --version
export PATH="$HOME/.cargo/bin:$PATH"

```

