use crate::include::{IOOrParseError, IncludeMacro};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::borrow::Cow;
use syn::parse::{Parse, ParseBuffer};
use syn::{ExprMacro, LitStr};

pub const INCLUDE_STR_NAME: &str = "include_str";
pub mod keywords {
    use syn::custom_keyword;
    custom_keyword!(include_str);
}
/// include_str!()
#[derive(Clone)]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct IncludeStrMacro<'a> {
    pub attributes: Cow<'a, [syn::Attribute]>,
    pub path: Cow<'a, TokenStream>,
}

#[cfg(feature = "executing")]
impl<'a> IncludeMacro<'a> for IncludeStrMacro<'a> {
    fn get_inner_tokens(&self) -> Cow<'a, TokenStream> {
        self.path.clone()
    }
}
#[cfg(feature = "executing")]
impl IncludeStrMacro<'_> {
    pub fn read_to_string(&self) -> Result<String, IOOrParseError> {
        let path = self.get_path_buf()?;
        std::fs::read_to_string(path).map_err(IOOrParseError::IO)
    }

    pub fn read_to_lit_str(&self) -> Result<LitStr, IOOrParseError> {
        let value = self.read_to_string()?;
        Ok(LitStr::new(&value, self.get_lit_str()?.span()))
    }
}
impl TryFrom<ExprMacro> for IncludeStrMacro<'_> {
    type Error = syn::Error;

    fn try_from(value: ExprMacro) -> Result<Self, Self::Error> {
        if !value.mac.path.is_ident(INCLUDE_STR_NAME) {
            return Err(syn::Error::new_spanned(
                value,
                "Expected include_str! macro",
            ));
        }
        return Ok(IncludeStrMacro {
            attributes: Cow::Owned(value.attrs),
            path: Cow::Owned(value.mac.tokens),
        });
    }
}
impl<'a> TryFrom<&'a ExprMacro> for IncludeStrMacro<'a> {
    type Error = syn::Error;

    fn try_from(value: &'a ExprMacro) -> Result<Self, Self::Error> {
        if !value.mac.path.is_ident(INCLUDE_STR_NAME) {
            return Err(syn::Error::new_spanned(
                value,
                "Expected include_str! macro",
            ));
        }
        return Ok(IncludeStrMacro {
            attributes: Cow::Borrowed(&value.attrs),
            path: Cow::Borrowed(&value.mac.tokens),
        });
    }
}
impl Parse for IncludeStrMacro<'_> {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let expr = input.parse::<ExprMacro>()?;
        return Self::try_from(expr);
    }
}
#[cfg(feature = "quote")]
impl quote::ToTokens for IncludeStrMacro<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = &self.path;
        let attributes = &self.attributes;
        let value = quote! {
            #(#attributes)*
            include_str!(#path)
        };
        tokens.append_all(value);
    }
}

#[cfg(test)]
mod tests {
    use crate::include::IncludeMacro;
    use crate::include_str::IncludeStrMacro;

    #[test]
    fn test_include_str() {
        let include_str: IncludeStrMacro = syn::parse_quote! {
            include_str!("./test_data/include_tests.txt")
        };
        let as_path = include_str.get_path_buf().unwrap();
        assert_eq!(
            as_path,
            std::path::PathBuf::from("./test_data/include_tests.txt")
        );
        println!("as_path = {:?}", as_path);
        let string = include_str.read_to_string().unwrap();
        println!("string = {:?}", string);
    }
}
