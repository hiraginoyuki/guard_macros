use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Token,
};

#[cfg_attr(feature = "debug-print", derive(Debug))]
struct MakeGuardExpr {
    ident: Ident,
    _sep: Token![=>],
    refute_handler: Expr,
}
impl Parse for MakeGuardExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _sep: input.parse()?,
            refute_handler: input.parse()?,
        })
    }
}
impl MakeGuardExpr {
    fn expand(&self) -> TokenStream {
        let Self {
            ident,
            refute_handler: handler,
            ..
        } = self;

        quote! {
            macro_rules! #ident {
                ($($tt:tt)*) => {
                    ::guard_macros::guard!(
                        { $($tt)* } => #handler
                    )
                }
            }
        }
    }
}

#[cfg_attr(feature = "debug-print", derive(Debug))]
pub struct MakeGuardBody {
    exprs: Punctuated<MakeGuardExpr, Token![,]>,
}
impl Parse for MakeGuardBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            exprs: Punctuated::parse_terminated(input)?,
        })
    }
}
impl MakeGuardBody {
    pub fn expand(&self) -> TokenStream {
        self.exprs.iter().map(MakeGuardExpr::expand).collect()
    }
}
