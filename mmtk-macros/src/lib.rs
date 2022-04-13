
extern crate proc_macro;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Field, spanned::Spanned};

fn field_has_attr(f: &Field, attr_name: &str) -> bool {
    f.attrs.iter().any(|a| {
        a.path.is_ident(attr_name)
    })
}

fn get_fields_with_attribute<'f>(fields: &'f FieldsNamed, attr_name: &str) -> Vec<&'f Field> {
    fields.named.iter().filter(|f| {
        field_has_attr(f, attr_name)
    }).collect::<Vec<_>>()
}

fn get_unique_field_with_attribute<'f>(fields: &'f FieldsNamed, attr_name: &str) -> Option<&'f Field> {
    let mut result = None;

    'each_field: for field in fields.named.iter() {
        for attr in field.attrs.iter() {
            if attr.path.is_ident(attr_name) {
                if result.is_none() {
                    result = Some(field);
                    continue 'each_field;
                } else {
                    let span = attr.path.span();
                    abort! { span, "At most one field in a struct can have the #[{}] attribute.", attr_name };
                }
            }
        }
    }

    result
}

fn generate_trace_object<'a>(space_fields: Vec<&'a Field>, parent_field: Option<&'a Field>) -> TokenStream {
    let space_field_handler = space_fields.iter().map(|f| {
        let f_ident = f.ident.as_ref().unwrap();
        quote! {
            if self.#f_ident.is_in_space(__mmtk_objref) {
                return self.#f_ident.trace_object(__mmtk_objref);
            }
        }
    });

    let parent_field_delegator = match parent_field {
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

#[proc_macro_error]
#[proc_macro_derive(HasSpaces, attributes(space_field, parent_field))]
pub fn derive_has_spaces(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let output = if let syn::Data::Struct(syn::DataStruct{
        fields: syn::Fields::Named(ref fields),
        ..
    }) = input.data {
        let space_fields = get_fields_with_attribute(&fields, "space_field") ;
        let parent_field = get_unique_field_with_attribute(&fields, "parent_field") ;

        let trace_object_function = generate_trace_object(space_fields, parent_field);

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
