use ctsh::ctsh;

fn main() {
	// If the output of a command is a valid Rust expression, it can be used as such
	let four = ctsh!(
		"echo" "4" as expr
	);
	println!("{four:?}"); // -> 4

	// When multiple expressions are requested, the output will be a tuple of all expressions
	let tuple = ctsh!(
		"echo" "4" as expr;
		"echo" "\"str\"" as expr;
	);
	println!("{tuple:?}"); // -> (4, "str")

	// `as expr` can be mixed with other expression-generating dispositions, such as `as str`
	let tuple = ctsh!(
		"echo" "4" as expr;
		"echo" "\"str\"" as str;
	);
	println!("{tuple:?}"); // -> (4, "\"str\"\n")

	// Since `as str` captures _all_ the output as a string, some changes are necessary to create the same result as with
	// `as expr`
	let tuple = ctsh!(
		"echo" "4" as expr;
		"echo" "str" | "tr" "-d" "\n" as str;
	);
	println!("{tuple:?}"); // -> (4, "str")
}
