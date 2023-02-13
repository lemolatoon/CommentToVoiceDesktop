# CommentToVoiceDesktop

## 開発環境構築

### VOICEVOX を立ち上げる
```bash
docker compose up -d
```

### tauri を立ち上げる
```bash
yarn tauri dev
```

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
so, i manually fixed these kinds of codes.
Also, binary strings are tried to be converted into text by open api generator, so i fixed as they return ByteArray `Vec<u8>`