use proc_macro::{TokenTree,TokenStream};
use syn::parse_macro_input;

fn make_hash(value:&str)->u64{
	use std::hash::Hasher;
	use std::hash::BuildHasher;
	let not_random_state=ahash::RandomState::with_seeds(0,0,0,0);
	let mut hasher=not_random_state.build_hasher();
	hasher.write(value.as_bytes());
	hasher.finish()
}

/// Takes a string literal as input and hashes it, returning a u64 literal.
#[proc_macro]
pub fn hash_literal(item:TokenStream)->TokenStream{
	let string_literal=parse_macro_input!(item as syn::LitStr);
	let string=string_literal.value();
	let hash=make_hash(&string);
	let u64_literal=proc_macro::Literal::u64_suffixed(hash);
	TokenStream::from(TokenTree::from(u64_literal))
}
