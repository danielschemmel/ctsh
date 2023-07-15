use std::path::PathBuf;

use proc_macro2::Span;
use syn::parse::Parse;

use super::kw;

#[derive(Clone, Debug)]
pub struct Cd {
	pub target: CdTarget,
	pub span: Span,
}

#[derive(Clone, Debug)]
pub enum CdTarget {
	Path(PathBuf),
	Temp,
}

impl Parse for Cd {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let start = input.parse::<kw::cd>()?;

		let la = input.lookahead1();
		if la.peek(syn::LitStr) {
			let value = input.parse::<syn::LitStr>()?;
			Ok(Cd {
				target: CdTarget::Path(value.value().into()),
				span: start.span,
			})
		} else if la.peek(kw::temp) {
			let _temp = input.parse::<kw::temp>()?;
			Ok(Cd {
				target: CdTarget::Temp,
				span: start.span,
			})
		} else {
			Err(la.error())
		}
	}
}
