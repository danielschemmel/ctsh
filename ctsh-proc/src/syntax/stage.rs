use std::path::Path;

use proc_macro2::Span;
use syn::parse::Parse;

#[derive(Clone, Debug)]
pub struct Stage {
	pub command: Vec<String>,
	pub unchecked: bool,
	pub span: Span,
}

impl Stage {
	pub fn build(&self, dir: &Path) -> duct::Expression {
		let mut expr = duct::cmd(&self.command[0], &self.command[1..]).dir(dir);
		if self.unchecked {
			expr = expr.unchecked();
		}
		expr
	}
}

impl Parse for Stage {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let la = input.lookahead1();
		let (mut command, span) = if la.peek(syn::LitStr) {
			let arg: syn::LitStr = input.parse()?;
			(vec![arg.value()], arg.span())
		} else {
			return Err(la.error());
		};

		let mut unchecked = false;
		while !input.is_empty() {
			let la = input.lookahead1();
			if la.peek(syn::LitStr) {
				let arg: syn::LitStr = input.parse()?;
				command.push(arg.value());
			} else if la.peek(syn::Token![?]) {
				if unchecked {
					return Err(input.error("Duplicate unchecked mark"));
				}
				let _unchecked_mark = input.parse::<syn::Token![?]>();
				unchecked = true;
			} else {
				break;
			};
		}

		Ok(Stage {
			command,
			unchecked,
			span,
		})
	}
}
