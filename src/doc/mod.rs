/*!
Doc Attribute
*/

use crate::include_str::IncludeStrMacro;
use proc_macro2::TokenStream;
use std::borrow::Cow;

use crate::IOOrParseError;
use syn::parse::ParseStream;
use syn::{Attribute, DeriveInput, Field, LitStr};
use syn::{Expr, Lit, Meta};
pub const DOC_ATTRIBUTE_NAME: &str = "doc";
pub mod keywords {
    use syn::custom_keyword;
    custom_keyword!(doc);
    custom_keyword!(inline);
}
// TODO: Owned Parser
/// A Doc Attribute
#[derive(Clone)]
#[non_exhaustive]
#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub enum DocAttribute<'a> {
    /// `#[doc = "Hello World"]`
    LitStr(Cow<'a, LitStr>),
    /// `#[doc = include_str!("path/to/file")]`
    IncludeStr(IncludeStrMacro<'a>),
    /// `#[doc(inline)]`
    Inline,
    /// More Doc Attributes will be added
    Other(Cow<'a, Attribute>),
}
impl DocAttribute<'static> {
    /// Parses all attributes then filters out the doc attributes
    /// and parses them
    /// # Returns
    /// A Vec of DocAttributes
    pub fn parse_inner(input: ParseStream) -> syn::Result<Vec<DocAttribute<'static>>> {
        let attributes = Attribute::parse_inner(input)?;
        let mut results = Vec::with_capacity(attributes.len());
        for attribute in attributes {
            if !attribute.path().is_ident(DOC_ATTRIBUTE_NAME) {
                continue;
            }
            let result: DocAttribute = DocAttribute::try_from(attribute)?;

            results.push(result);
        }
        Ok(results)
    }
}
impl TryFrom<Attribute> for DocAttribute<'static> {
    type Error = syn::Error;

    fn try_from(attribute: Attribute) -> Result<Self, Self::Error> {
        if !attribute.path().is_ident(DOC_ATTRIBUTE_NAME) {
            return Err(syn::Error::new_spanned(attribute, "Expected doc attribute"));
        }
        match attribute.meta {
            Meta::NameValue(name) => match name.value {
                Expr::Lit(v) => {
                    if let Lit::Str(v) = v.lit {
                        Ok(DocAttribute::LitStr(Cow::Owned(v)))
                    } else {
                        Err(syn::Error::new_spanned(v, "Expected string literal"))
                    }
                }
                Expr::Macro(expr) => {
                    if expr.mac.path.is_ident(crate::include_str::INCLUDE_STR_NAME) {
                        Ok(DocAttribute::IncludeStr(IncludeStrMacro::try_from(expr)?))
                    } else {
                        Err(syn::Error::new_spanned(expr, "Expected include_str!()"))
                    }
                }
                _ => Err(syn::Error::new_spanned(
                    name,
                    "Expected string literal or include_str!()",
                )),
            },
            Meta::List(_) => attribute.parse_args_with(|input: ParseStream| {
                if input.peek(keywords::inline) {
                    input.parse::<keywords::inline>()?;
                    Ok(DocAttribute::Inline)
                } else {
                    Err(syn::Error::new(input.span(), "Expected inline"))
                }
            }),
            v => Err(syn::Error::new_spanned(v, "Expected doc attribute")),
        }
    }
}
impl<'a> TryFrom<&'a Attribute> for DocAttribute<'a> {
    type Error = syn::Error;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        if !attribute.path().is_ident(DOC_ATTRIBUTE_NAME) {
            return Err(syn::Error::new_spanned(attribute, "Expected doc attribute"));
        }
        let result = match &attribute.meta {
            Meta::NameValue(name) => match &name.value {
                Expr::Lit(v) => {
                    if let Lit::Str(v) = &v.lit {
                        Some(Ok(DocAttribute::LitStr(Cow::Borrowed(v))))
                    } else {
                        return Err(syn::Error::new_spanned(v, "Expected string literal"));
                    }
                }
                Expr::Macro(expr) => {
                    if expr.mac.path.is_ident(crate::include_str::INCLUDE_STR_NAME) {
                        Some(Ok(DocAttribute::IncludeStr(IncludeStrMacro::try_from(
                            expr,
                        )?)))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Meta::List(_) => attribute
                .parse_args_with(|input: ParseStream| {
                    if input.peek(keywords::inline) {
                        input.parse::<keywords::inline>()?;
                        Ok(Some(DocAttribute::Inline))
                    } else {
                        Ok(None)
                    }
                })
                .transpose(),
            _ => Some(Err(syn::Error::new_spanned(
                attribute,
                "Expected doc attribute",
            ))),
        };
        if let Some(result) = result {
            return result;
        }
        Ok(DocAttribute::Other(Cow::Borrowed(attribute)))
    }
}
#[cfg(feature = "executing")]
impl DocAttribute<'_> {
    /// Returns LitString as a String if it is a LitStr
    /// Reads the file if it is an include_str!()
    ///
    /// # Returns
    /// Ok(None) - the attribute does not have a comment or map to include_str!()
    pub fn to_string(&self) -> Result<Option<String>, IOOrParseError> {
        match self {
            DocAttribute::LitStr(v) => Ok(Some(v.value())),
            DocAttribute::IncludeStr(v) => Ok(Some(v.read_to_string()?)),
            _ => return Ok(None),
        }
    }
}
#[cfg(feature = "quote")]
impl quote::ToTokens for DocAttribute<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use quote::{quote, TokenStreamExt};
        let stream = match self {
            DocAttribute::LitStr(lit) => {
                quote! {
                    #[doc = #lit]
                }
            }
            DocAttribute::IncludeStr(v) => {
                quote! {
                    #[doc = #v]
                }
            }
            DocAttribute::Inline => {
                quote! {
                    #[doc(inline)]
                }
            }
            DocAttribute::Other(other) => other.to_token_stream(),
        };
        tokens.append_all(stream);
    }
}
/// Implemented on types that can have doc attributes
pub trait HasDocAttributes {
    /// Finds all doc attributes and parses them
    fn parse_doc_attributes(&self) -> Result<Vec<DocAttribute<'_>>, syn::Error>;

    /// Finds all doc attributes
    fn get_doc_attributes(&self) -> Vec<&Attribute>;
}
macro_rules! impl_has_doc_attributes {
    ($t:ty) => {
        impl HasDocAttributes for $t {
            fn parse_doc_attributes(&self) -> Result<Vec<DocAttribute<'_>>, syn::Error> {
                return utils::parse_doc_attributes(&self.attrs);
            }

            fn get_doc_attributes(&self) -> Vec<&Attribute> {
                return utils::get_comment_attributes(&self.attrs);
            }
        }
    };
}
impl_has_doc_attributes!(DeriveInput);
impl_has_doc_attributes!(Field);

pub mod utils {
    use crate::doc::DocAttribute;
    use syn::Attribute;

    /// Get all doc attributes as borrowed
    pub fn parse_doc_attributes(attrs: &[Attribute]) -> Result<Vec<DocAttribute<'_>>, syn::Error> {
        attrs
            .iter()
            .filter(|attr| attr.path().is_ident(super::DOC_ATTRIBUTE_NAME))
            .map(|attr| DocAttribute::try_from(attr))
            .collect()
    }
    /// Get all doc attributes
    pub fn get_comment_attributes(attrs: &[Attribute]) -> Vec<&Attribute> {
        attrs
            .iter()
            .filter(|attr| attr.path().is_ident(super::DOC_ATTRIBUTE_NAME))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use syn::{parse_quote, Attribute};

    #[test]
    fn test_get_comments_borrowed() {
        let doc_attribute: Attribute = syn::parse_quote! {
            #[doc = "Hello World"]
        };
        let _comments = super::utils::parse_doc_attributes(&[doc_attribute.clone()]);
    }
    #[test]
    fn test_parse_to_doc_type() {
        let doc_attribute: Vec<Attribute> = vec![
            parse_quote! {
                #[doc = include_str!("../test_data/include_tests.txt")]
            },
            parse_quote! {
                #[doc = "Hello World"]
            },
            parse_quote! {
                #[doc(inline)]
            },
            parse_quote! {
                #[async_trait]
            },
        ];
        let doc_type = super::utils::parse_doc_attributes(&doc_attribute).unwrap();
        for doc in doc_type {
            println!("doc_type = {:?}", doc);
        }
    }
}
