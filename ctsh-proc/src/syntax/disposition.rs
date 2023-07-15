use proc_macro2::Span;
use syn::parse::Parse;

use super::kw;

#[derive(Clone, Debug)]
pub enum Disposition {
	String { span: Span },
	Bytes { span: Span },
	Expr { span: Span },
	Stmts { span: Span },
}

impl Parse for Disposition {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let start = input.parse::<syn::Token![as]>()?;

		let la = input.lookahead1();
		if la.peek(kw::expr) {
			input.parse::<kw::expr>()?;
			Ok(Disposition::Expr { span: start.span })
		} else if la.peek(kw::stmts) {
			input.parse::<kw::stmts>()?;
			Ok(Disposition::Stmts { span: start.span })
		} else if la.peek(kw::str) {
			input.parse::<kw::str>()?;
			Ok(Disposition::String { span: start.span })
		} else if la.peek(kw::bytes) {
			input.parse::<kw::bytes>()?;
			Ok(Disposition::Bytes { span: start.span })
		} else {
			Err(la.error())
		}
	}
}
