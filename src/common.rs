use syn::{
    parse::{Parse, ParseStream},
    Expr, Token,
};

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
    pub fn expr(&self) -> Option<&Expr> {
        if let Self::Expr { expr, .. } = self {
            Some(expr)
        } else {
            None
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
