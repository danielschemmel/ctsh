use syn::parse::Parse;

mod kw {
	syn::custom_keyword!(bytes);
	syn::custom_keyword!(cd);
	syn::custom_keyword!(expr);
	syn::custom_keyword!(file);
	syn::custom_keyword!(run);
	syn::custom_keyword!(stmts);
	syn::custom_keyword!(str);
	syn::custom_keyword!(temp);
}

mod script;
pub use script::Script;

mod item;
pub use item::Item;

mod cd;
pub use cd::{Cd, CdTarget};

mod file;
pub use file::File;

mod run;
pub use run::Run;

mod stage;
pub use stage::Stage;

mod disposition;
pub use disposition::Disposition;

#[derive(Clone)]
pub struct Stmts {
	pub items: Vec<syn::Stmt>,
}

impl Parse for Stmts {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut items = Vec::new();
		while !input.is_empty() {
			items.push(input.parse()?);
		}
		Ok(Stmts { items })
	}
}
