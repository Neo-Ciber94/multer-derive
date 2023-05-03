use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    Data, DeriveInput, Field, Fields, Ident,
};

pub fn derive_from_multipart(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_attrs = get_fields_attributes(&fields)?;

    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .cloned()
        .collect::<Vec<_>>();

    let mut field_parsers = Vec::new();

    for f in fields {
        let original_name = f.ident.as_ref().unwrap();
        let name_str = original_name.to_string();
        let attr = field_attrs.get(&name_str).cloned();
        let field_name = attr
            .clone()
            .and_then(|attr| attr.rename)
            .map(|s| Ident::new(&s, Span::call_site()))
            .unwrap_or(original_name.clone());

        let field_ty = f.ty;
        let parser = match attr.and_then(|s| s.with) {
            Some(with) => {
                let from_multipart_fn = match syn::parse_str::<syn::Path>(&with) {
                    Ok(p) => p,
                    Err(err) => {
                        return Err(err);
                    }
                };
                quote! { #from_multipart_fn ( multipart ) }
            }
            None => {
                quote! {
                    <#field_ty as ::multer_derive::FromMultipart>::from_multipart(
                        multipart,
                        ::multer_derive::FormContext {
                            field_name: Some( stringify!(#field_name) ),
                        },
                    )?
                }
            }
        };

        field_parsers.push(quote! {
            let #original_name = #parser;
        });
    }

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::multer_derive::FromMultipart for #name #ty_generics #where_clause {
            fn from_multipart<'a>(multipart: &::multer_derive::MultipartForm, ctx: ::multer_derive::FormContext<'_>) -> Result<Self, ::multer_derive::Error> {
                #(#field_parsers)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

#[derive(Debug, Clone)]
struct MulterAttribute {
    // #[multer(rename = "other_name")]
    rename: Option<String>,

    // #[multer(with = "path::to::function")]
    with: Option<String>,
}

impl Parse for MulterAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut multer_attribute = MulterAttribute {
            rename: None,
            with: None,
        };

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if input.peek(Ident) {
                let path: syn::Path = input.parse()?;

                // #[multer(rename = "...")]
                if path.is_ident("rename") {
                    let _: syn::Token![=] = input.parse()?;
                    let rename_value: syn::LitStr = input.parse()?;
                    multer_attribute.rename = Some(rename_value.value());
                }
                // #[multer(with = "...")]
                else if path.is_ident("with") {
                    let _: syn::Token![=] = input.parse()?;
                    let with_value: syn::LitStr = input.parse()?;
                    multer_attribute.with = Some(with_value.value());
                } else {
                    return Err(lookahead.error());
                }
            } else {
                return Err(lookahead.error());
            }

            if !input.is_empty() {
                let _: syn::Token![,] = input.parse()?;
            }
        }

        Ok(multer_attribute)
    }
}

fn get_fields_attributes(
    fields: &Punctuated<Field, Comma>,
) -> syn::Result<HashMap<String, MulterAttribute>> {
    let mut attrs = HashMap::new();

    for field in fields {
        for attr in &field.attrs {
            if !attr.path().is_ident("multer") {
                continue;
            }

            let multer_attr: MulterAttribute = attr.parse_args()?;
            let field_name = field.ident.as_ref().unwrap();
            attrs.insert(field_name.to_string(), multer_attr);
        }
    }

    Ok(attrs)
}
