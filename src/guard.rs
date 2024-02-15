use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{discouraged::Speculative, Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    token::Brace,
    Expr, Pat, Token,
};

use crate::common::{RefuteHandler, RefuteHandlerInheritable};

/// Assign expression found in `let`-`else`.
#[cfg_attr(feature = "debug-print", derive(Debug))]
struct LetElseClause {
    pat: Pat,
    _eq: Token![=],
    expr: Expr,
}
impl Parse for LetElseClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: Pat::parse_single(input)?,
            _eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}
impl LetElseClause {
    fn expand(&self, refute_handler: &Expr) -> TokenStream {
        let Self { pat, expr, .. } = self;
        quote! {
            let #pat = #expr else {
                #refute_handler
            };
        }
    }
}

/// Boolean expression found after `if`.
#[cfg_attr(feature = "debug-print", derive(Debug))]
struct IfClause {
    expr: Expr,
}
impl Parse for IfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
        })
    }
}
impl IfClause {
    fn expand(&self, refute_handler: &Expr) -> TokenStream {
        let Self { expr } = self;
        quote! {
            if !( #expr ) {
                #refute_handler
            }
        }
    }
}

/// Guard clause found in either `let`-`else` or `if ...`.
#[cfg_attr(feature = "debug-print", derive(Debug))]
enum GuardClause {
    /// Assign expression expected in `let`-`else`.
    LetElse(LetElseClause),
    /// Boolean expression expected after `if`.
    If(IfClause),
}
impl Parse for GuardClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // TODO: parse arbitrary amount of attributes?

        let fork1 = input.fork();
        let fork2 = input.fork();
        if let Ok(clause) = fork1.parse() {
            input.advance_to(&fork1);
            Ok(Self::LetElse(clause))
        } else if let Ok(clause) = fork2.parse() {
            input.advance_to(&fork2);
            Ok(Self::If(clause))
        } else {
            #[cfg(feature = "debug-print")]
            {
                println!("{:#?}", input);
            }
            Err(input.error("expected `let`-`else` or `if` clause"))
        }
    }
}
impl GuardClause {
    fn expand(&self, refute_handler: &Expr) -> TokenStream {
        match self {
            GuardClause::LetElse(clause) => clause.expand(refute_handler),
            GuardClause::If(clause) => clause.expand(refute_handler),
        }
    }
}

#[cfg_attr(feature = "debug-print", derive(Debug))]
enum GuardDecl {
    Block {
        _brace: Brace,
        body: GuardBody,
        refute_handler: RefuteHandlerInheritable,
    },
    Clause {
        clause: GuardClause,
        refute_handler: Option<RefuteHandler>,
    },
}
impl Parse for GuardDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            let fork = input.fork();
            let interior;
            let brace = braced!(interior in fork);

            if let Some(handler) = RefuteHandlerInheritable::try_parse(&fork)? {
                input.advance_to(&fork);
                return Ok(Self::Block {
                    _brace: brace,
                    body: interior.parse()?,
                    refute_handler: handler,
                });
            }
        }

        Ok(Self::Clause {
            clause: input.parse()?,
            refute_handler: RefuteHandler::try_parse(input)?,
        })
    }
}
impl GuardDecl {
    fn expand(&self, refute_handler: &Expr) -> TokenStream {
        match self {
            Self::Block {
                body,
                refute_handler: handler,
                ..
            } => {
                let handler = handler.expr().unwrap_or(refute_handler);
                let body = body.expand(handler);
                quote! {
                    { #body }
                }
            }
            Self::Clause {
                clause,
                refute_handler: handler,
            } => {
                let handler = match handler {
                    Some(handler) => handler.expr(),
                    None => refute_handler,
                };
                clause.expand(handler)
            }
        }
    }
}

// TODO: Docs
#[cfg_attr(feature = "debug-print", derive(Debug))]
pub struct GuardBody {
    guards: Punctuated<GuardDecl, Token![,]>,
}
impl Parse for GuardBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            guards: Punctuated::parse_terminated(input)?,
        })
    }
}
impl GuardBody {
    pub fn expand(&self, refute_handler: &Expr) -> TokenStream {
        self.guards
            .iter()
            .map(|guard| guard.expand(refute_handler))
            .collect()
    }
    pub fn expand_default(&self) -> TokenStream {
        let default_handler = parse_quote! {
            return ::core::default::Default::default()
        };

        self.expand(&default_handler)
    }
}
