extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name_literal = &ast.ident;
    let name_string = &ast.ident.to_string();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics Component for #name_literal #ty_generics #where_clause {}
        impl #impl_generics ::moecs::util::PropertyId for #name_literal #ty_generics #where_clause {
            fn property_id() -> u64
            where Self: Sized {
                use ::std::hash::{Hash, Hasher};
                use ::std::collections::hash_map::DefaultHasher;

                let key = format!("{}::{}", module_path!(), #name_string);
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                hasher.finish()
            }

            fn self_property_id(&self) -> u64 {
                Self::property_id()
            }
        }
    })
}

#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name_literal = &ast.ident;
    let name_string = &ast.ident.to_string();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics ::moecs::util::PropertyId for #name_literal #ty_generics #where_clause {
            fn property_id() -> u64
            where Self: Sized {
                use ::std::hash::{Hash, Hasher};
                use ::std::collections::hash_map::DefaultHasher;

                let key = format!("{}::{}", module_path!(), #name_string);
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                hasher.finish()
            }

            fn self_property_id(&self) -> u64 {
                Self::property_id()
            }
        }
    })
}

#[proc_macro_derive(SystemParam)]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name_literal = &ast.ident;
    let name_string = &ast.ident.to_string();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics SystemParam for #name_literal #ty_generics #where_clause {}
        impl #impl_generics ::moecs::util::PropertyId for #name_literal #ty_generics #where_clause {
            fn property_id() -> u64
            where Self: Sized {
                use ::std::hash::{Hash, Hasher};
                use ::std::collections::hash_map::DefaultHasher;

                let key = format!("{}::{}", module_path!(), #name_string);
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                hasher.finish()
            }

            fn self_property_id(&self) -> u64 {
                Self::property_id()
            }
        }
    })
}
