use proc_macro2::Span;
use syn::parse::Parse;

use super::{kw, Disposition, Stage};

#[derive(Clone, Debug)]
pub struct Run {
	pub commands: Vec<Stage>,
	pub disposition: Option<Disposition>,
	pub span: Span,
}

impl Parse for Run {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let start_span = if input.peek(kw::run) {
			let start = input.parse::<kw::run>()?;
			Some(start.span)
		} else {
			None
		};

		let mut commands: Vec<Stage> = Vec::new();

		let disposition = loop {
			commands.push(input.parse()?);

			if input.is_empty() {
				break None;
			}

			let la = input.lookahead1();
			if la.peek(syn::Token![|]) {
				let _pipe = input.parse::<syn::Token![|]>();
			} else if la.peek(syn::Token![as]) {
				break Some(input.parse()?);
			} else if la.peek(syn::Token![;]) {
				break None;
			} else {
				return Err(la.error());
			};
		};

		let span = start_span.unwrap_or_else(|| commands[0].span);
		Ok(Run {
			commands,
			disposition,
			span,
		})
	}
}
