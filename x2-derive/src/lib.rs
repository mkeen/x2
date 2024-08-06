 extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AngleBracketedGenericArguments, Data, DeriveInput, Fields, PathArguments, Type, TypePath};

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
            fn builder(&mut self) -> Option<RequestBuilder<#lifetime>> {
                self.builder.take()
            }

            fn update_builder(&mut self, builder: RequestBuilder<#lifetime>) {
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

#[proc_macro_derive(XData)]
pub fn derive_xdata(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let getters = match input.data { 
        Data::Struct(ref data_struct) => {
            match data_struct.fields {
                Fields::Named(ref fields) => {
                    fields.named.iter().map(|f| {
                        let field_name = &f.ident;
                        let field_type = &f.ty;

                        if let Type::Path(TypePath { path, .. }) = field_type {
                            if let Some(segment) = path.segments.last() {
                                if segment.ident == "Option" {
                                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
                                        if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                                            let getter_name = quote::format_ident!("{}", field_name.as_ref().unwrap());
                                            
                                            return quote! {
                                                pub fn #getter_name(&self) -> Option<&#inner_type> {
                                                    self.#field_name.as_ref()
                                                }
                                            };
                                        }
                                    }
                                }
                            }
                        }

                        // Fallback to the original type if it's not Option<T>
                        let getter_name = quote::format_ident!("{}", field_name.as_ref().unwrap());
                        quote! {
                            pub fn #getter_name(&self) -> Option<&#field_type> {
                                self.#field_name.as_ref()
                            }
                        }
                    }).collect::<Vec<_>>()
                }
                _ => Vec::new(),
            }
        }
        _ => Vec::new(),
    };
    
    let expanded = quote! {
        impl #name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}
