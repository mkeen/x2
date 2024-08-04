extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Built)]
pub fn build_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let lifetime = input.generics.lifetimes().next();

    let request_generics = match lifetime {
        Some(lifetime) => quote! { super::Request<#lifetime, Response> },
        None => quote! { super::Request<Response> },
    };

    let expanded = quote! {
        impl #impl_generics #request_generics for #name #ty_generics #where_clause {
            fn builder(&mut self) -> Option<RequestBuilder #impl_generics> {
                self.builder.take()
            }

            fn update_builder(&mut self, builder: RequestBuilder #impl_generics) {
                self.builder.replace(builder);
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Authorized)]
pub fn authorized_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let lifetime = input.generics.lifetimes().next();

    let authorized_generics = match lifetime {
        Some(lifetime) => quote! { super::Authorized<#lifetime, Response> },
        None => quote! { super::Authorized<Response> },
    };

    let expanded = quote! {
        impl #impl_generics #authorized_generics for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(UrlEndpoint)]
pub fn endpoint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl super::Endpoint for #name {}
    };

    TokenStream::from(expanded)
}
