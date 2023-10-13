use crate::include::IncludeMacro;
use proc_macro2::TokenStream;
use std::borrow::Cow;
use syn::parse::{Parse, ParseBuffer};
use syn::spanned::Spanned;
use syn::ExprMacro;

pub const INCLUDE_STR_NAME: &str = "include_bytes";
pub mod keywords {
    use syn::custom_keyword;
    custom_keyword!(include_bytes);
}
/// include_bytes!()
#[derive(Clone)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct IncludeBytesMacro<'a> {
    pub attributes: Cow<'a, [syn::Attribute]>,
    pub keyword: keywords::include_bytes,
    pub path: Cow<'a, TokenStream>,
}

#[cfg(feature = "executing")]
impl<'a> IncludeMacro<'a> for IncludeBytesMacro<'a> {
    fn get_inner_tokens(&self) -> Cow<'a, TokenStream> {
        self.path.clone()
    }
}
#[cfg(feature = "executing")]
impl IncludeBytesMacro<'_> {}
impl TryFrom<ExprMacro> for IncludeBytesMacro<'_> {
    type Error = syn::Error;

    fn try_from(value: ExprMacro) -> Result<Self, Self::Error> {
        if !value.mac.path.is_ident(INCLUDE_STR_NAME) {
            return Err(syn::Error::new_spanned(
                value,
                "Expected include_str! macro",
            ));
        }
        return Ok(IncludeBytesMacro {
            attributes: Cow::Owned(value.attrs),
            keyword: keywords::include_bytes(value.mac.path.span()),
            path: Cow::Owned(value.mac.tokens),
        });
    }
}
impl<'a> TryFrom<&'a ExprMacro> for IncludeBytesMacro<'a> {
    type Error = syn::Error;

    fn try_from(value: &'a ExprMacro) -> Result<Self, Self::Error> {
        if !value.mac.path.is_ident(INCLUDE_STR_NAME) {
            return Err(syn::Error::new_spanned(
                value,
                "Expected include_str! macro",
            ));
        }
        return Ok(IncludeBytesMacro {
            attributes: Cow::Borrowed(&value.attrs),
            keyword: keywords::include_bytes(value.mac.path.span()),
            path: Cow::Borrowed(&value.mac.tokens),
        });
    }
}
impl Parse for IncludeBytesMacro<'_> {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let expr = input.parse::<ExprMacro>()?;
        return Self::try_from(expr);
    }
}
#[cfg(feature = "quote")]
impl quote::ToTokens for IncludeBytesMacro<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        crate::utils::quote::to_tokens_function_like_macro_parenthesis(
            tokens,
            &self.keyword,
            &self.path,
            &self.attributes,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::include::include_bytes::IncludeBytesMacro;
    use crate::include::IncludeMacro;

    #[test]
    fn test_include_bytes() {
        let include_str: IncludeBytesMacro = syn::parse_quote! {
            include_bytes!("./test_data/include_tests.txt")
        };
        let as_path = include_str.get_path_buf().unwrap();
        assert_eq!(
            as_path,
            std::path::PathBuf::from("./test_data/include_tests.txt")
        );
        println!("as_path = {:?}", as_path);
    }
}
