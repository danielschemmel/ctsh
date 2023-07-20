use syn::parse::Parse;

use super::Item;

#[derive(Clone, Debug)]
pub struct Script {
	pub commands: Vec<Item>,
}

impl Parse for Script {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let commands = input.parse_terminated(Item::parse, syn::Token![;])?;

		Ok(Self {
			commands: commands.into_iter().collect(),
		})
	}
}
