extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();

    let ast = parse_macro_input!(input_clone as DeriveInput);
    let name_literal = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let component_impl = TokenStream::from(quote! {
        impl #impl_generics ::moecs::component::Component for #name_literal #ty_generics #where_clause {}
    });

    let property_id_impl = derive_property_id(input.clone());

    TokenStream::from_iter(vec![component_impl, property_id_impl])
}

#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    derive_property_id(input)
}

#[proc_macro_derive(SystemParam)]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();

    let ast = parse_macro_input!(input_clone as DeriveInput);
    let name_literal = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let system_param_impl = TokenStream::from(quote! {
        impl #impl_generics ::moecs::system::SystemParam for #name_literal #ty_generics #where_clause {}
    });

    let property_id_impl = derive_property_id(input.clone());

    TokenStream::from_iter(vec![system_param_impl, property_id_impl])
}

fn derive_property_id(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name_literal = &ast.ident;
    let name_string = &ast.ident.to_string();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics ::moecs::util::PropertyId for #name_literal #ty_generics #where_clause {
            fn property_string() -> &'static str
            where Self: Sized {
                static PROPERTY_STRING: &'static str = concat!(module_path!(), "::", #name_string);
                PROPERTY_STRING
            }

            fn property_id() -> u64
            where Self: Sized {
                use ::std::collections::hash_map::DefaultHasher;
                use ::std::hash::{Hash, Hasher};
                use ::std::sync::OnceLock;

                static HASH: OnceLock<u64> = OnceLock::new();
                *HASH.get_or_init(|| {
                    let key = #name_literal::property_string();
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    hasher.finish()
                })
            }

            fn self_property_id(&self) -> u64 {
                #name_literal::property_id()
            }

            fn self_property_string(&self) -> &'static str {
                #name_literal::property_string()
            }
        }
    })
}
