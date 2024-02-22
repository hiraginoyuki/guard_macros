use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

pub trait RefuteHandlerExt {
    fn fallback<'a>(&'a self, fallback: &'a Expr) -> &'a Expr;
}

#[cfg_attr(feature = "debug-print", derive(Debug))]
pub struct RefuteHandler {
    _arrow: Token![=>],
    expr: Expr,
}
impl Parse for RefuteHandler {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _arrow: input.parse()?,
            expr: input.parse()?,
        })
    }
}
impl RefuteHandler {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
    pub fn try_parse(input: ParseStream) -> syn::Result<Option<Self>> {
        match input.parse() {
            Ok(arrow) => Ok(Some(Self {
                _arrow: arrow,
                expr: input.parse()?,
            })),
            Err(_) => Ok(None),
        }
    }
}
impl RefuteHandlerExt for Option<RefuteHandler> {
    fn fallback<'a>(&'a self, fallback: &'a Expr) -> &'a Expr {
        match self {
            Some(handler) => handler.expr(),
            None => fallback,
        }
    }
}

#[cfg_attr(feature = "debug-print", derive(Debug))]
pub enum RefuteHandlerInheritable {
    Expr {
        _arrow: Token![=>],
        expr: Expr,
    },
    Inherit {
        _arrow: Token![=>],
        _underscore: Token![_],
    },
}
impl Parse for RefuteHandlerInheritable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let arrow = input.parse()?;

        if let Ok(underscore) = input.parse() {
            Ok(Self::Inherit {
                _arrow: arrow,
                _underscore: underscore,
            })
        } else {
            Ok(Self::Expr {
                _arrow: arrow,
                expr: input.parse()?,
            })
        }
    }
}
impl RefuteHandlerInheritable {
    pub fn fallback<'a>(&'a self, fallback: &'a Expr) -> &'a Expr {
        match self {
            Self::Expr { expr, .. } => expr,
            _ => fallback,
        }
    }
    pub fn try_parse(input: ParseStream) -> syn::Result<Option<Self>> {
        match input.parse() {
            Ok(arrow) => {
                if let Ok(underscore) = input.parse() {
                    Ok(Some(Self::Inherit {
                        _arrow: arrow,
                        _underscore: underscore,
                    }))
                } else {
                    Ok(Some(Self::Expr {
                        _arrow: arrow,
                        expr: input.parse()?,
                    }))
                }
            }
            Err(_) => Ok(None),
        }
    }
}
impl RefuteHandlerExt for RefuteHandlerInheritable {
    fn fallback<'a>(&'a self, fallback: &'a Expr) -> &'a Expr {
        match self {
            Self::Expr { expr, .. } => expr,
            _ => fallback,
        }
    }
}
