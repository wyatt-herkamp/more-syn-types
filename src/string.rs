/*!
Implementation of stringify! macro

 */
pub const STRINGIFY_NAME: &str = "stringify";
pub mod keywords {
    use syn::custom_keyword;
    custom_keyword!(stringify);
}
use proc_macro2::{Ident, TokenStream};
use std::borrow::Cow;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{ExprMacro, LitStr};

#[derive(Clone)]
#[non_exhaustive]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct StringifyMacro<'a> {
    pub attributes: Cow<'a, [syn::Attribute]>,
    pub keyword: keywords::stringify,
    pub content: Ident,
}

#[cfg(feature = "quote")]
impl quote::ToTokens for StringifyMacro<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        crate::utils::quote::to_tokens_function_like_macro_parenthesis(
            tokens,
            &self.keyword,
            &self.content,
            &self.attributes,
        )
    }
}

#[cfg(feature = "executing")]
impl StringifyMacro<'_> {
    pub fn get_string(&self) -> String {
        self.content.to_string()
    }
    pub fn get_lit_str(&self) -> syn::LitStr {
        LitStr::new(&self.get_string(), self.content.span())
    }
}
impl Parse for StringifyMacro<'_> {
    fn parse(input: &syn::parse::ParseBuffer) -> syn::Result<Self> {
        let expr = input.parse::<ExprMacro>()?;
        return Self::try_from(expr);
    }
}

impl<'a> TryFrom<&'a ExprMacro> for StringifyMacro<'a> {
    type Error = syn::Error;

    fn try_from(value: &'a ExprMacro) -> Result<Self, Self::Error> {
        if !value.mac.path.is_ident(STRINGIFY_NAME) {
            return Err(syn::Error::new_spanned(value, "Expected stringify! macro"));
        }
        return Ok(StringifyMacro {
            attributes: Cow::Borrowed(&value.attrs),
            keyword: keywords::stringify(value.mac.path.span()),
            content: value.mac.parse_body::<Ident>()?,
        });
    }
}

impl TryFrom<ExprMacro> for StringifyMacro<'_> {
    type Error = syn::Error;

    fn try_from(value: ExprMacro) -> Result<Self, Self::Error> {
        if !value.mac.path.is_ident(STRINGIFY_NAME) {
            return Err(syn::Error::new_spanned(value, "Expected stringify! macro"));
        }
        return Ok(StringifyMacro {
            attributes: Cow::Owned(value.attrs),
            keyword: keywords::stringify(value.mac.path.span()),
            content: value.mac.parse_body::<Ident>()?,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::string::StringifyMacro;
    use syn::ExprMacro;

    #[test]
    fn test_stringify() {
        let stringify: StringifyMacro = syn::parse_quote! {
            stringify!(include_tests)
        };
        assert_eq!(stringify.get_string(), "include_tests");
    }
    #[test]
    fn test_stringify_from_expr() {
        let stringify_raw: ExprMacro = syn::parse_quote! {
            stringify!(include_tests)
        };
        let stringify: StringifyMacro = StringifyMacro::try_from(&stringify_raw).unwrap();
        assert_eq!(stringify.get_string(), "include_tests");
    }
}
