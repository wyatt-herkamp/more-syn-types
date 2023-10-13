#[cfg(feature = "quote")]
pub mod quote {
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens, TokenStreamExt};
    use syn::Attribute;

    pub fn to_tokens_function_like_macro_parenthesis(
        tokens: &mut TokenStream,
        name: &impl ToTokens,
        arguments: &impl ToTokens,
        attributes: &[Attribute],
    ) {
        let value = quote! {
            #(#attributes)*
            #name!(#arguments)
        };
        tokens.append_all(value);
    }
}
