# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## NOTE: open-api-generator

In some `new` function, if you get `Option<T>` from function's params, and struct type is `Option<Box<T>>`, `open-api-generator` generates like:
```rust
struct A {
    ...
    speaker: Option<Box<CertainType>>
    ...
}
    fn new(..., speaker: Option<CertainType>) -> Self {
        Self {
            ... // <--- some struct members
            speaker: Box::new(speaker)
            ... // <--- some struct members
        }
    }
```
but correct generation is like:
```rust
struct A {
    ...
    speaker: Option<Box<CertainType>>
    ...
}
    fn new(..., speaker: Option<CertainType>) -> Self {
        Self {
            ... // <--- some struct members
            speaker: speaker.map(Box::new)
            ... // <--- some struct members
        }
    }
```
so, i manually fixed these kinds of codes