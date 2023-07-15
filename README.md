`ctsh!` runs commands at compile time and (optionally) captures their output.

```rust
use ctsh::ctsh;
let result = ctsh!("echo" "Hello" "World" as str);
assert_eq!(result.trim(), "Hello World");
```
