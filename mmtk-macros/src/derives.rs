use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{spanned::Spanned, Attribute, DeriveInput, Field, FieldsNamed};

fn get_field_attribute<'f>(field: &'f Field, attr_name: &str) -> Option<&'f Attribute> {
    let attrs = field
        .attrs
        .iter()
        .filter(|a| a.path.is_ident(attr_name))
        .collect::<Vec<_>>();
    if attrs.len() > 1 {
        let second_attr = attrs.get(1).unwrap();
        abort! { second_attr.path.span(), "Duplicated attribute: #[{}]", attr_name }
    };

    attrs.get(0).cloned()
}

fn get_fields_with_attribute<'f>(fields: &'f FieldsNamed, attr_name: &str) -> Vec<&'f Field> {
    fields
        .named
        .iter()
        .filter(|f| get_field_attribute(f, attr_name).is_some())
        .collect::<Vec<_>>()
}

fn get_unique_field_with_attribute<'f>(
    fields: &'f FieldsNamed,
    attr_name: &str,
) -> Option<&'f Field> {
    let mut result = None;

    'each_field: for field in fields.named.iter() {
        if let Some(attr) = get_field_attribute(field, attr_name) {
            if result.is_none() {
                result = Some(field);
                continue 'each_field;
            } else {
                let span = attr.path.span();
                abort! { span, "At most one field in a struct can have the #[{}] attribute.", attr_name };
            }
        }
    }

    result
}

fn generate_trace_object<'a>(
    space_fields: &[&'a Field],
    parent_field: &Option<&'a Field>,
) -> TokenStream {
    let space_field_handler = space_fields.iter().map(|f| {
        let f_ident = f.ident.as_ref().unwrap();
        quote! {
            if self.#f_ident.is_in_space(__mmtk_objref) {
                return self.#f_ident.trace_object(__mmtk_objref);
            }
        }
    });

    let parent_field_delegator = if let Some(f) = parent_field {
        let f_ident = f.ident.as_ref().unwrap();
        quote! {
            self.#f_ident.trace_object(__mmtk_objref)
        }
    } else {
        quote! {
            panic!("No more spaces to try")
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

fn generate_find_space_dyn<'a>(
    space_fields: &[&'a Field],
    parent_field: &Option<&'a Field>,
) -> TokenStream {
    let space_field_handler = space_fields.iter().map(|f| {
        let f_ident = f.ident.as_ref().unwrap();
        quote! {
            if self.#f_ident.is_in_space(__mmtk_objref) {
                return ::std::option::Option::Some(&self.#f_ident);
            }
        }
    });

    let parent_field_delegator = if let Some(f) = parent_field {
        let f_ident = f.ident.as_ref().unwrap();
        quote! {
            self.#f_ident.find_space_dyn(__mmtk_objref)
        }
    } else {
        quote! {
            ::std::option::Option::None
        }
    };

    quote! {
        fn find_space_dyn(&self, __mmtk_objref: usize) -> Option<&dyn crate::spaces::Space> {
            use crate::spaces::Space;
            #(#space_field_handler)*
            #parent_field_delegator
        }
    }
}

pub(crate) fn has_spaces(input: DeriveInput) -> TokenStream {
    let ident = input.ident;

    let output = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = input.data
    {
        let space_fields = get_fields_with_attribute(fields, "space_field");
        let parent_field = get_unique_field_with_attribute(fields, "parent_field");

        let trace_object_function = generate_trace_object(&space_fields, &parent_field);
        let find_space_dyn_function = generate_find_space_dyn(&space_fields, &parent_field);

        quote! {
            impl crate::plans::HasSpaces for #ident {
                #trace_object_function
                #find_space_dyn_function
            }
        }
    } else {
        abort_call_site!("`#[derive(HasSpaces)]` only supports structs with named fields.")
    };

    output
}
