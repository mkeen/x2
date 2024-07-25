extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ImplItem, ItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn request(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    // Check if the `request` method is implemented
    // let has_request_method = input.items.iter().any(|item| {
    //     if let ImplItem::Fn(method) = item {
    //         method.sig.ident == "request"
    //     } else {
    //         false
    //     }
    // });

    // if !has_request_method {
    //     return TokenStream::from(quote! {
    //         compile_error!("You must implement the request method");
    //     });
    // }

    // Add the associated type definition and default implementations for `is_authorized` and `authorize`
    input.items.insert(
        0,
        ImplItem::Type(syn::parse_quote! {
            type Response = Response;
        }),
    );

    input.items.push(ImplItem::Fn(syn::ImplItemFn {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        defaultness: None,
        sig: syn::parse_quote! { fn is_authorized(&self) -> bool },
        block: syn::parse_quote! {{
            match self.credential {
                Credential::Unauthorized(_) => self.authorization.is_none(),
                _ => true,
            }
        }},
    }));

    // input.items.push(ImplItem::Fn(syn::ImplItemFn {
    //     attrs: vec![],
    //     vis: syn::Visibility::Inherited,
    //     defaultness: None,
    //     sig: syn::parse_quote! { fn authorize(&mut self) -> Result<(), XError> },
    //     block: syn::parse_quote! {{
    //         match self.credential.try_into() {
    //             Ok(request_credential) => {
    //                 self.authorization = request_credential;
    //                 Ok(())
    //             },
    //             Err(e) => e
    //         }

    //         self.authorization = c;
    //     }},
    // }));

    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn trailing_none(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let block = &input.block;
    let vis = &input.vis;
    let output = &input.sig.output;

    let fn_name_with_defaults = Ident::new(&format!("{}_with_defaults", fn_name), fn_name.span());

    let param_names: Vec<_> = inputs
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let name = format_ident!("arg{}", i);
            quote! { #name }
        })
        .collect();

    let param_defs: Vec<_> = inputs
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let name = format_ident!("arg{}", i);
            quote! { #name: Option<_> }
        })
        .collect();

    let mut arms = Vec::new();
    for i in 0..=inputs.len() {
        let args = param_names.iter().enumerate().map(|(j, name)| {
            if j < i {
                quote! { Some(#name) }
            } else {
                quote! { None }
            }
        });

        let args_list = quote! { #(#args),* };

        arms.push(quote! {
            (#(#param_names),*) => #fn_name(#args_list),
        });
    }

    let expanded = quote! {
        #vis fn #fn_name(#inputs) #output {
            #block
        }

        pub fn #fn_name_with_defaults(#(#param_defs),*) #output {
            match (#(#param_names),*) {
                #(#arms)*
                _ => panic!("Invalid number of arguments"),
            }
        }
    };

    TokenStream::from(expanded)
}
