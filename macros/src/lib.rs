use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Equivalent, attributes(equivalent))]
pub fn derive_equivalent(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let equivalent_paths: Vec<syn::Path> = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("equivalent"))
        .map(|attr| attr.parse_args().expect("Invalid path in equivalent attribute"))
        .collect();

    if equivalent_paths.is_empty() {
        panic!("Missing #[equivalent(...)] attribute");
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let make_impls = |path: &syn::Path| {
        let (to_matches, from_matches) = match input.data {
            Data::Struct(ref data) => {
                match &data.fields {
                    Fields::Named(fields) => {
                        let field_names = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
                        let fields = quote! { 
                            Self { #(#field_names: value.#field_names.into()),* }
                        };
                        (fields.clone(), fields)
                    },
                    Fields::Unnamed(fields) => {
                        let indices = (0..fields.unnamed.len())
                            .map(syn::Index::from)
                            .collect::<Vec<_>>();
                        let fields = quote! {
                            Self(#(value.#indices.into()),*)
                        };
                        (fields.clone(), fields)
                    },
                    Fields::Unit => {
                        let fields = quote! { Self };
                        (fields.clone(), fields)
                    }
                }
            },
            Data::Enum(ref data) => {
                let to_arms = data.variants.iter().map(|v| {
                    let variant_ident = &v.ident;
                    match &v.fields {
                        Fields::Unit => {
                            quote! { #name::#variant_ident => #path::#variant_ident }
                        },
                        Fields::Unnamed(fields) => {
                            let vars = (0..fields.unnamed.len())
                                .map(|i| format_ident!("v{}", i))
                                .collect::<Vec<_>>();
                            quote! { 
                                #name::#variant_ident(#(#vars),*) => #path::#variant_ident(#(#vars.into()),*) 
                            }
                        },
                        Fields::Named(fields) => {
                            let field_names = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
                            quote! {
                                #name::#variant_ident { #(#field_names),* } => 
                                    #path::#variant_ident { #(#field_names: #field_names.into()),* }
                            }
                        }
                    }
                });

                let from_arms = data.variants.iter().map(|v| {
                    let variant_ident = &v.ident;
                    match &v.fields {
                        Fields::Unit => {
                            quote! { #path::#variant_ident => #name::#variant_ident }
                        },
                        Fields::Unnamed(fields) => {
                            let vars = (0..fields.unnamed.len())
                                .map(|i| format_ident!("v{}", i))
                                .collect::<Vec<_>>();
                            quote! { 
                                #path::#variant_ident(#(#vars),*) => #name::#variant_ident(#(#vars.into()),*) 
                            }
                        },
                        Fields::Named(fields) => {
                            let field_names = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
                            quote! {
                                #path::#variant_ident { #(#field_names),* } => 
                                    #name::#variant_ident { #(#field_names: #field_names.into()),* }
                            }
                        }
                    }
                });

                (
                    quote! { match value { #(#to_arms),* } },
                    quote! { match value { #(#from_arms),* } }
                )
            },
            _ => panic!("Only structs and enums are supported")
        };

        quote! {
            impl #impl_generics From<#name #ty_generics> for #path #ty_generics #where_clause {
                fn from(value: #name #ty_generics) -> Self {
                    #to_matches
                }
            }

            impl #impl_generics From<#path #ty_generics> for #name #ty_generics #where_clause {
                fn from(value: #path #ty_generics) -> Self {
                    #from_matches
                }
            }
        }
    };

    let all_impls = equivalent_paths.iter().map(make_impls);

    let expanded = quote! {
        #(#all_impls)*
    };

    TokenStream::from(expanded)
}
