
extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Field};

fn get_fields_with_annotation<'f>(fields: &'f FieldsNamed, name: &str) -> Vec<&'f Field> {
    fields.named.iter().filter(|f| {
        f.attrs.iter().any(|a| {
            a.path.is_ident(name)
        })
    }).collect::<Vec<_>>()
}

fn generate_trace_object<'a>(space_fields: Vec<&'a Field>, opt_parent_field: Option<&'a Field>) -> TokenStream {
    let space_field_handler = space_fields.iter().map(|f| {
        let f_ident = f.ident.as_ref().unwrap();
        quote! {
            if self.#f_ident.is_in_space(__mmtk_objref) {
                return self.#f_ident.trace_object(__mmtk_objref);
            }
        }
    });

    let parent_field_delegator = match opt_parent_field {
        Some(f) => {
            let f_ident = f.ident.as_ref().unwrap();
            quote! {
                self.#f_ident.trace_object(__mmtk_objref)
            }
        },
        None => {
            quote! {
                panic!("No more spaces to try")
            }
        }
    };

    quote! {
        fn trace_object(&self, __mmtk_objref: usize) -> usize {
            use crate::spaces::Space;
            #(#space_field_handler)*
            #parent_field_delegator
        }
    }
}

#[proc_macro_derive(HasSpaces, attributes(space_field, parent_field))]
pub fn derive_has_spaces(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let output = if let syn::Data::Struct(syn::DataStruct{
        fields: syn::Fields::Named(ref fields),
        ..
    }) = input.data {
        let space_fields = get_fields_with_annotation(&fields, "space_field") ;
        let parent_fields = get_fields_with_annotation(&fields, "parent_field") ;

        if parent_fields.len() > 1 {
            panic!("There are multiple parent fields!");
        }

        let opt_parent_field = parent_fields.get(0).copied();

        let trace_object_function = generate_trace_object(space_fields, opt_parent_field);

        quote! {
            impl crate::plans::HasSpaces for #ident {
                #trace_object_function
            }
        }
    } else {
        panic!("This derive macro only supports structs with named fields.")
    };

    output.into()
}
