//! # guard_macros – *Early Returns Made Easy*
//!
//! ## Table of Contents
//! - [Usage](#usage)
//! - [Overview](#overview)
//! - [Example](#example)
//! - [Specification](#specification)
//!
//!
//! ## Usage
//! ```
//! # use guard_macros::guard;
//! # fn main() {
//! # let expr = false;
//! // returns when (expr) is evaluated to false
//! guard!( (expr) );
//!
//! // returns when refuted (i.e. (expr) doesn't match (pat))
//! guard!( (pat) = (expr) );
//!
//! // panics instead of returning (called "Refute Handler")
//! guard!( (expr) => panic!("false") );
//! guard!( (pat) = (expr) => panic!("refuted") );
//!
//! guard! {
//!     // can be repeated
//!     (expr),
//!     (pat) = (expr) => panic!("refuted"),
//!
//!     // can be grouped and nested
//!     {
//!         (expr),
//!         (pat) = (expr),
//!         {
//!             (expr),
//!             (pat) = (expr) => panic!("baz"),
//!         } => _, // inherit refute handler
//!     } => panic!("foo"),
//! }
//! # }
//! ```
//!
//!
//! ## Overview
//!
//! `guard_macros` provides two macros:
//! - [`guard!`](./macro.guard.html) which replaces recurring `let`-`else` and
//!   `if` statement(s).
//! - [`make_guard!`](./macro.make_guard.html) which defines new guard macro(s)
//!   with different default [Refute Handlers](#refute-handler).
//!
//! #### Refute Handler
//!
//! A "Refute Handler" is an expression that is executed when the condition of
//! a clause is not met. It can be specified by appending `=>` followed by an
//! expression, either:
//! - to a single clause
//!   ```
//!   # use guard_macros::guard;
//!   # fn main() {
//!   # let expr = true;
//!   guard! {
//!   #   false,
//!       (pat) = (expr) => panic!("refuted"),
//!       (expr) => panic!("false"),
//!   }
//!   # }
//!   ```
//! - or to a group of clauses enclosed by `{` `}`.
//!   ```
//!   # use guard_macros::guard;
//!   # fn main() {
//!   # let expr = true;
//!   guard! {
//!   #   false,
//!       {
//!           (pat) = (expr),
//!           (expr),
//!       } => panic!("unmet")
//!   }
//!   # }
//!   ```
//!
//! ## Example
//!
//! ```
#![doc = include_str!("../examples/event_handling.rs")]
//! ```
//!
//!
//! ## Specification
//!
//! - [`guard!`](./macro.guard.html)
//!   > <sup>**Syntax**</sup>
//!   >
//!   > _GuardBody_ : \
//!   > &nbsp;&nbsp; _GuardDecl_ ( `,` _GuardDecl_ )* `,`<sup>?</sup>
//!   >
//!   > _GuardDecl_ : \
//!   > &nbsp;&nbsp; &nbsp;&nbsp; `{` _GuardBody_ `}` _RefuteHandlerInheritable_ \
//!   > &nbsp;&nbsp;            | _GuardClause_ _RefuteHandler_<sup>?</sup>
//!   >
//!   > _GuardClause_ : \
//!   > &nbsp;&nbsp; &nbsp;&nbsp; [_PatternNoTopAlt_] `=` [_Expression_] \
//!   > &nbsp;&nbsp;            | [_Expression_]
//!   >
//!   > _RefuteHandler_ : \
//!   > &nbsp;&nbsp; `=>` [_Expression_]
//!   >
//!   > _RefuteHandlerInheritable_ : \
//!   > &nbsp;&nbsp; &nbsp;&nbsp; _RefuteHandler_ \
//!   > &nbsp;&nbsp;            | `=>` `_`
//!
//!   [_Expression_]: https://doc.rust-lang.org/stable/reference/expressions.html
//!   [_PatternNoTopAlt_]: https://doc.rust-lang.org/stable/reference/patterns.html
//!
//! - [`make_guard!`](./macro.make_guard.html)
//!   > <sup>**Syntax**</sup>
//!   >
//!   > _MakeGuardBody_ : \
//!   > &nbsp;&nbsp; _MakeGuardDecl_ ( `,` _MakeGuardDecl_ )* `,`<sup>?</sup>
//!   >
//!   > _MakeGuardDecl_ : \
//!   > &nbsp;&nbsp; [_Identifier_] _RefuteHandler_
//!
//!   [_Identifier_]: https://doc.rust-lang.org/stable/reference/identifiers.html

use proc_macro::TokenStream;
use syn::parse_macro_input;

pub(crate) mod common;
mod guard;
use guard::GuardBody;
mod make_guard;
use make_guard::MakeGuardBody;

/// A macro that replaces `let`-`else` and `if` clause(s).
///
/// For more information, refer to the [crate-level documentation](./index.html).
#[proc_macro]
pub fn guard(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as GuardBody);
    #[cfg(feature = "debug-print")]
    {
        println!("{:#?}", parsed);
    }
    parsed.expand_default().into()
}

/// A macro that defines new guard macro(s).
///
/// For more information, refer to the [crate-level documentation](./index.html).
#[proc_macro]
pub fn make_guard(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as MakeGuardBody);
    #[cfg(feature = "debug-print")]
    {
        println!("{:#?}", parsed);
    }
    parsed.expand().into()
}
