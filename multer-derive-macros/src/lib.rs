use proc_macro::TokenStream;
use syn::DeriveInput;

mod impls;

/// Provide an implementation of `FromMultipart` for construct types from [`multer::Multipart`].
#[proc_macro_derive(FromMultipart, attributes(multer))]
pub fn derive_from_multipart(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    match impls::derive_from_multipart(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
