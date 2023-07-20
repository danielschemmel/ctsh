//! [`ctsh!`] runs commands at compile time and (optionally) captures their output.
//!
//! ```
//! use ctsh::ctsh;
//!
//! let result = ctsh!("echo" "Hello" "World" as str);
//! assert_eq!(result.trim(), "Hello World");
//! ```
//!
//! Commands can be piped together.
//!
//! ```
//! # use ctsh::ctsh;
//! let result = ctsh!("echo" "Hello" "World" | "tr" "-d" "l" as str);
//! assert_eq!(result.trim(), "Heo Word");
//! ```
//!
//! Multiple commands can be executed with semicolons. If their outputs are used for expressions, they generate a tuple
//! of all outputs.
//!
//! ```
//! # use ctsh::ctsh;
//! let result = ctsh!{
//!   "echo" "1" as str;
//!   "echo" "foo" as str;
//! };
//! assert_eq!(result.0.trim(), "1");
//! assert_eq!(result.1.trim(), "foo");
//! ```
//!
//! There are multiple dispositions available for commands.
//!
//! ```
//! # use ctsh::ctsh;
//! let result = ctsh!{
//!   "echo" "moo";           // execute command and ignore its output
//!   "echo" "oink" as str;   // capture output as string literal (`&'static str`)
//!   "echo" "meow" as bytes; // capture output as byte literal (`&'static [u8; _]`)
//!   "echo" "()" as expr;    // capture output as Rust expression
//! };
//! assert_eq!(result, ("oink\n", b"meow\n", ()));
//!
//! ctsh!{
//!   "echo" r#"
//!     #[derive(Debug)]
//!     struct Foo;
//!     let result = format!("{:?}", Foo);
//!   "# as stmts; // capture output as Rust statements (cannot be mixed with expression capturing)
//! }
//! assert_eq!(result, "Foo");
//! ```
//!
//! Every [`ctsh!`] starts executing commands in `CARGO_MANIFEST_DIR`.
//!
//! ```
//! # use ctsh::ctsh;
//! let result = ctsh!("pwd" as str);
//! assert_eq!(result.trim(), env!("CARGO_MANIFEST_DIR"));
//! ```
//!
//! Use `cd` to change to a different directory. Note: `cd` uses canonicalized paths with all symlinks resolved.
//!
//! ```
//! # use ctsh::ctsh;
//! let result = ctsh!{
//!   cd "/";
//!   "pwd" as str;
//! };
//! assert_eq!(result.trim(), "/");
//! ```
//! 
//! You can use the special form `cd temp` to create and go to a temporary directory that will be cleaned up afterwards.
//! 
//! ```
//! # use ctsh::ctsh;
//! ctsh!{
//!   cd temp;
//!   "touch" "garbage"; // does not leave any `garbage` files on your computer
//! }
//! ```
//!
//! [`ctsh!`] will cause a compile-time error if your command fails.
//!
//! ```compile_fail
//! # use ctsh::ctsh;
//! ctsh!("false")
//! ```
//!
//! ```compile_fail
//! # use ctsh::ctsh;
//! ctsh!("false" | "true")
//! ```
//!
//! Mark each command that might fail with `?` to ignore errors.
//!
//! ```
//! # use ctsh::ctsh;
//! ctsh!("false" ? | "true")
//! ```
//!
//! ```compile_fail
//! # use ctsh::ctsh;
//! ctsh!("false" | "true" ?)
//! ```

/// The proc-macro that it is all about.
///
/// See [the module-level documentation](self) for usage details.
pub use ctsh_proc::ctsh;
