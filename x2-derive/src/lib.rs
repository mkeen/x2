extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Built)]
pub fn build_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl super::Request<Response> for #name {
            fn builder(&mut self) -> Option<RequestBuilder> {
                self.builder.take()
            }

            fn update_builder(&mut self, builder: RequestBuilder) {
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

    let expanded = quote! {
        impl super::Authorized<Response> for #name {}
    };

    TokenStream::from(expanded)
}
