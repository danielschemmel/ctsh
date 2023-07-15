use syn::parse::Parse;

use super::kw;

#[derive(Clone, Debug)]
pub struct File {
	pub name: String,
	pub content: Vec<u8>,
}

impl Parse for File {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		input.parse::<kw::file>()?;

		let la = input.lookahead1();
		let name = if la.peek(syn::LitStr) {
			let name: syn::LitStr = input.parse()?;
			Ok(name.value())
		} else {
			Err(la.error())
		}?;

		let la = input.lookahead1();
		let content = if la.peek(syn::LitStr) {
			let content: syn::LitStr = input.parse()?;
			Ok(content.value().into_bytes())
		} else if la.peek(syn::LitByteStr) {
			let content: syn::LitByteStr = input.parse()?;
			Ok(content.value())
		} else {
			Err(la.error())
		}?;

		Ok(File { name, content })
	}
}
