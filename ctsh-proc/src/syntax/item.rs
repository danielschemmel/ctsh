use syn::parse::Parse;

use super::kw;

#[derive(Clone, Debug)]
pub enum Item {
	Run(super::Run),
	Cd(super::Cd),
	File(super::File),
}

impl Item {
	pub fn la_parse(input: syn::parse::ParseStream) -> Result<syn::Result<Self>, syn::parse::Lookahead1> {
		let la = input.lookahead1();
		if la.peek(kw::run) {
			Ok(input.parse().map(Item::Run))
		} else if la.peek(kw::cd) {
			Ok(input.parse().map(Item::Cd))
		} else if la.peek(kw::file) {
			Ok(input.parse().map(Item::File))
		} else if la.peek(syn::LitStr) {
			Ok(input.parse().map(Item::Run))
		} else {
			Err(la)
		}
	}
}

impl Parse for Item {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Self::la_parse(input).map_err(|la| la.error()).and_then(|result| result)
	}
}
