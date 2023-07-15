#![forbid(unsafe_code)]

use std::{
	io::Write,
	path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use syn::parse_macro_input;

mod syntax;
use self::syntax::{Disposition, Syntax};

#[proc_macro_error]
#[proc_macro]
pub fn ctsh(input: TokenStream) -> TokenStream {
	let Syntax { commands } = parse_macro_input!(input as Syntax);

	let mut dir: PathBuf = std::env::var_os("CARGO_MANIFEST_DIR")
		.expect("Could not determine CARGO_MANIFEST_DIR")
		.into();
	dir = dir.canonicalize().expect("CARGO_MANIFEST_DIR is not a valid directory");

	let items = {
		let mut stmts = false;
		let mut exprs = false;
		for command in &commands {
			if let syntax::Item::Run(syntax::Run {
				disposition: Some(disposition),
				..
			}) = command
			{
				if let syntax::Disposition::Stmts { .. } = disposition {
					stmts = true;
				} else {
					exprs = true;
				}
			}
		}

		if stmts && exprs {
			abort_call_site!("Cannot mix expressions and items");
		}
		stmts
	};

	let mut tokens = Vec::new();
	let mut tempdirs = Vec::new();
	for command in commands {
		use syntax::Item::*;
		match command {
			File(command) => file(&dir, &command),
			Run(command) => tokens.extend(run(&dir, &command)),
			Cd(command) => tempdirs.extend(cd(&mut dir, command)),
		}
	}
	drop(tempdirs);

	match tokens.len() {
		0 => quote::quote!().into(),
		1 => tokens.pop().unwrap().into(),
		_ => {
			if items {
				quote::quote!(#(#tokens)*).into()
			} else {
				quote::quote!((#(#tokens,)*)).into()
			}
		}
	}
}

fn file(dir: &Path, syntax::File { name, content }: &syntax::File) {
	let path = dir.join(name);
	let mut file =
		std::fs::File::create(path).unwrap_or_else(|err| abort_call_site!("Cannot create file {}: {}", name, err));
	file
		.write_all(content)
		.unwrap_or_else(|err| abort_call_site!("Cannot write to file {}: {}", name, err));
}

fn run(
	dir: &Path,
	syntax::Run {
		commands,
		disposition,
		span,
	}: &syntax::Run,
) -> Option<proc_macro2::TokenStream> {
	let cwd = dir.to_path_buf();

	let mut cmd_iter = commands.iter().rev();
	let last_command = cmd_iter.next().expect("There should be at least one command");
	let pipe = last_command.build(&cwd).stdout_capture();

	let pipe = cmd_iter.fold(pipe, |pipe, stage| stage.build(&cwd).pipe(pipe));

	let result = pipe
		.run()
		.unwrap_or_else(|err| abort!(span, "Cannot run pipe: {}", err));

	disposition
		.as_ref()
		.map(|disposition| output_to_tokens(result.stdout, disposition))
}

fn output_to_tokens(output: Vec<u8>, disposition: &Disposition) -> proc_macro2::TokenStream {
	match disposition {
		Disposition::Bytes { span } => {
			let tokens = syn::LitByteStr::new(&output, *span);
			quote::quote!(#tokens)
		}
		Disposition::String { span } => {
			let value = String::from_utf8(output).unwrap_or_else(|_err| abort!(span, "Output of command is not valid UTF-8"));
			let tokens = syn::LitStr::new(&value, *span);
			quote::quote!(#tokens)
		}
		Disposition::Expr { span } => {
			let value = String::from_utf8(output).unwrap_or_else(|_err| abort!(span, "Output of command is not valid UTF-8"));
			let tokens: syn::Expr = syn::parse_str(&value)
				.unwrap_or_else(|err| abort!(span, "Output of command is not a valid rust expression: {}", err));
			quote::quote!(#tokens)
		}
		Disposition::Stmts { span } => {
			let value = String::from_utf8(output).unwrap_or_else(|_err| abort!(span, "Output of command is not valid UTF-8"));
			let syntax::Stmts { items } = syn::parse_str(&value)
				.unwrap_or_else(|err| abort!(span, "Output of command is not a valid rust expression: {}", err));
			quote::quote!(#(#items)*)
		}
	}
}

fn cd(dir: &mut PathBuf, syntax::Cd { target, span }: syntax::Cd) -> Option<tempfile::TempDir> {
	match target {
		syntax::CdTarget::Path(target) => {
			dir.push(&target);
			*dir = dir
				.canonicalize()
				.unwrap_or_else(|err| abort!(span, "Could not change directory to {:?}: {}", &target, err));
			None
		}
		syntax::CdTarget::Temp => {
			let tempdir =
				tempfile::tempdir().unwrap_or_else(|err| abort!(span, "Could not create temporary directory: {}", err));
			*dir = tempdir
				.path()
				.canonicalize()
				.unwrap_or_else(|err| abort!(span, "Could not change directory to {:?}: {}", &target, err));
			Some(tempdir)
		}
	}
}
