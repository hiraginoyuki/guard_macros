use quote::quote;
use proc_macro::TokenStream;
use syn::{
    braced, parse::{discouraged::Speculative, Parse, ParseStream}, parse_macro_input, parse_quote, punctuated::Punctuated, token::Brace, Expr, Ident, Pat, Token
};

/// ```no_run
/// stmt      := $(guard),+?
/// on_refute := `=>` $:expr 
/// guard     := $( clause | { stmt } ),+ $(on_refute)?
/// clause    := | $:pat = $:expr  // let ... else
///              | $:expr          // if ...
/// ```

#[proc_macro]
pub fn guard(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as GuardStatement).expand(&parse_quote!(return ()))
}

#[proc_macro]
pub fn make_guard(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as MakeGuardExpr).expand()
}

struct OnRefute {
    _arrow: Token![=>],
    expr: Expr,
}
impl Parse for OnRefute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _arrow: input.parse()?,
            expr: input.parse()?,
        })
    }
}
impl OnRefute {
    fn try_parse(input: ParseStream) -> syn::Result<Option<Self>> {
        match input.parse() {
            Ok(arrow) => Ok(Some(Self {
                _arrow: arrow,
                expr: input.parse()?,
            })),
            Err(_) => Ok(None),
        }
    }
}

/// Assign expression found in `let ... else`.
struct LetElseClause {
    pat: Pat,
    _eq: Token![=],
    expr: Expr,
}
impl Parse for LetElseClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: Pat::parse_multi(input)?,
            _eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}

/// Boolean expression found after `if`.
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

/// Guard clause found in either `let ... else` or `if ...`.
enum GuardClauseInner {
    /// Assign expression expected in `let ... else`.
    LetElse(LetElseClause),
    /// Boolean expression expected after `if`.
    If(IfClause),
}
impl Parse for GuardClauseInner {
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
            todo!("expected assign or expr")
        }
    }
}

/// Guard clause found in either `let ... else` or `if ...`,
/// optionally with "on_refute" expression succeeding `=>`.
struct GuardClause {
    clause: GuardClauseInner,
    on_refute: Option<OnRefute>,
}
impl Parse for GuardClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            clause: input.parse()?,
            on_refute: OnRefute::try_parse(input)?,
        })
    }
}
impl GuardClause {
    fn expand(&self, on_refute: &Expr) -> TokenStream {
        let on_refute = match self.on_refute {
            Some(ref on_refute) => &on_refute.expr,
            None => on_refute,
        };

        match &self.clause {
            GuardClauseInner::LetElse(LetElseClause { pat, expr, .. }) => {
                quote! {
                    let #pat = #expr else {
                        #on_refute
                    };
                }
            }
            GuardClauseInner::If(IfClause { expr }) => {
                quote! {
                    if !( #expr ) {
                        #on_refute
                    }
                }
            }
        }.into()
    }
}

enum GuardClauseOrBlock {
    Block {
        _brace: Brace,
        guard: Box<GuardStatement>,
    },
    Clause(GuardClause),
}
impl Parse for GuardClauseOrBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            let interior;
            Ok(Self::Block {
                _brace: braced!(interior in input),
                guard: Box::new(interior.parse()?)
            })
        } else {
            Ok(Self::Clause(input.parse()?))
        }
    }
}
impl GuardClauseOrBlock {
    fn expand(&self, on_refute: &Expr) -> TokenStream {
        match self {
            Self::Block { guard, .. } => guard.expand(on_refute),
            Self::Clause(clause) => clause.expand(on_refute),
        }
    }
}

struct Guard {
    expr_or_block: GuardClauseOrBlock,
    on_refute: Option<OnRefute>,
}
impl Parse for Guard {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr_or_block: input.parse()?,
            on_refute: OnRefute::try_parse(input)?,
        })
    }
}
impl Guard {
    fn expand(&self, on_refute: &Expr) -> TokenStream {
        let on_refute = match &self.on_refute {
            Some(on_refute) => &on_refute.expr,
            None => on_refute,
        };

        self.expr_or_block.expand(on_refute)
    }
}

// TODO: Docs
struct GuardStatement {
    guards: Punctuated<Guard, Token![,]>,
}
impl Parse for GuardStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            guards: Punctuated::parse_terminated(input)?,
        })
    }
}
impl GuardStatement {
    fn expand(&self, on_refute: &Expr) -> TokenStream {
        self.guards
            .iter()
            .map(|guard| guard.expand(on_refute))
            .collect()
    }
}

struct MakeGuardExpr {
    ident: Ident,
    _sep: Token![=>],
    on_refute: Expr,
}
impl Parse for MakeGuardExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _sep: input.parse()?,
            on_refute: input.parse()?,
        })
    }
}
impl MakeGuardExpr {
    fn expand(&self) -> TokenStream {
        let Self { ident, on_refute, .. } = self;

        quote! {
            macro_rules! #ident {
                ($($tt:tt)*) => {
                    ::guard_macros::guard!(
                        { $($tt)* } => #on_refute
                    )
                }
            }
        }.into()
    }
}