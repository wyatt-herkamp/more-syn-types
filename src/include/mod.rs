/*!
include_str!() and include_bytes!()
 */
use proc_macro2::TokenStream;
use std::borrow::Cow;
use syn::LitStr;
use thiserror::Error;

pub mod include_bytes;
pub mod include_str;
/// An Error that is either an IO Error or a Parse Error
#[derive(Error, Debug)]
pub enum IOOrParseError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] syn::Error),
}

/// Implemented on Types that include an external file
#[cfg(feature = "executing")]
pub trait IncludeMacro<'a>: Sized {
    fn get_inner_tokens(&self) -> Cow<'a, TokenStream>;
    fn get_lit_str(&self) -> Result<LitStr, syn::Error> {
        syn::parse2::<LitStr>(self.get_inner_tokens().clone().into_owned())
    }
    fn get_string(&self) -> Result<String, syn::Error> {
        let value = self.get_lit_str()?.value();
        Ok(value)
    }
    fn get_path_buf(&self) -> Result<std::path::PathBuf, syn::Error> {
        let value = self.get_string()?;
        Ok(std::path::PathBuf::from(value))
    }
}
