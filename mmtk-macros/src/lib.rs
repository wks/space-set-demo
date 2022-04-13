extern crate proc_macro;

mod derives;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_error]
#[proc_macro_derive(HasSpaces, attributes(space_field, parent_field))]
pub fn derive_has_spaces(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::has_spaces(input).into()
}
