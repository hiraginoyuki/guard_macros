# guard_macros – *Early Returns Made Easy*

## Table of Contents
- [Usage](#usage)
- [Overview](#overview)
- [Example](#example)
- [Specification](#specification)


## Usage
```rs
// returns when (expr) is evaluated to false
guard!( (expr) );

// returns when refuted (i.e. (expr) doesn't match (pat))
guard!( (pat) = (expr) );

// panics instead of returning (called "Refute Handler")
guard!( (expr) => panic!("false") );
guard!( (pat) = (expr) => panic!("refuted") );

guard! {
    // can be repeated
    (expr),
    (pat) = (expr) => panic!("refuted"),

    // can be grouped and nested
    {
        (expr),
        (pat) = (expr),
        {
            (expr),
            (pat) = (expr) => panic!("baz"),
        } => _, // inherit refute handler
    } => panic!("foo"),
}
```


## Overview

`guard_macros` provides two macros:
- `guard!` which replaces recurring `let`-`else` and
  `if` statement(s).
- `make_guard!` which defines new guard macro(s)
  with different default [Refute Handlers](#refute-handler).

#### Refute Handler

A "Refute Handler" is an expression that is executed when the condition of
a clause is not met. It can be specified by appending `=>` followed by an
expression, either:
- to a single clause
  ```
  guard! {
      (pat) = (expr) => panic!("refuted"),
      (expr) => panic!("false"),
  }
  ```
- or to a group of clauses enclosed by `{` `}`.
  ```
  guard! {
      {
          (pat) = (expr),
          (expr),
      } => panic!("unmet")
  }
  ```

## Example

```rs
#![allow(dead_code)]

/// Ignores any tokens inside the block.
macro_rules! comment {
    ( $($_:tt)* ) => {};
}

use guard_macros::guard;

fn main() {
    let event = Event::Message {
        author: "yuki".into(),
        message: "hello world".into(),
    };

    println!("{event:?}");
    handle_message(&event);
    handle_error(&event);
}

#[derive(Debug)]
enum Event {
    Message { author: String, message: String },
    Error { error: String },
}

enum Action {
    SendMessage { message: String },
}

fn handle_message(event: &Event) -> Option<Action> {
    guard!(Event::Message { author, message } = event);
    // expands to:
    comment! {
        let Event::Message { author, message } = event else {
            return ::core::default::Default::default()
        };
    }

    println!("{author}: {message}");

    guard! {
        *author == "yuki",
        message.starts_with("/ping"),
    };
    // expands to:
    comment! {
        if !(*author == "yuki") {
            return ::core::default::Default::default()
        }
        if !(message.starts_with("/ping")) {
            return ::core::default::Default::default()
        }
    }

    Some(Action::SendMessage {
        message: "pong!".into(),
    })
}

// make_guard!(hoge => return "noo");
// fn bar() -> i32 { hoge!(false); 0 }
// fn foo() -> u32 { hoge!(false); 0 }

fn handle_error(event: &Event) -> Option<Action> {
    guard!(Event::Error { error } = event);
    // expands to:
    comment! {
        let Event::Error { error } = event else {
            return ::core::default::Default::default()
        };
    }

    println!("ERR: {error}");

    None
}

```


## Specification

- [`guard!`](./macro.guard.html)
  > <sup>**Syntax**</sup>
  >
  > _GuardBody_ : \
  > &nbsp;&nbsp; _GuardDecl_ ( `,` _GuardDecl_ )* `,`<sup>?</sup>
  >
  > _GuardDecl_ : \
  > &nbsp;&nbsp; &nbsp;&nbsp; `{` _GuardBody_ `}` _RefuteHandlerInheritable_ \
  > &nbsp;&nbsp;            | _GuardClause_ _RefuteHandler_<sup>?</sup>
  >
  > _GuardClause_ : \
  > &nbsp;&nbsp; &nbsp;&nbsp; [_PatternNoTopAlt_] `=` [_Expression_] \
  > &nbsp;&nbsp;            | [_Expression_]
  >
  > _RefuteHandler_ : \
  > &nbsp;&nbsp; `=>` [_Expression_]
  >
  > _RefuteHandlerInheritable_ : \
  > &nbsp;&nbsp; &nbsp;&nbsp; _RefuteHandler_ \
  > &nbsp;&nbsp;            | `=>` `_`

  [_Expression_]: https://doc.rust-lang.org/stable/reference/expressions.html
  [_PatternNoTopAlt_]: https://doc.rust-lang.org/stable/reference/patterns.html

- [`make_guard!`](./macro.make_guard.html)
  > <sup>**Syntax**</sup>
  >
  > _MakeGuardBody_ : \
  > &nbsp;&nbsp; _MakeGuardDecl_ ( `,` _MakeGuardDecl_ )* `,`<sup>?</sup>
  >
  > _MakeGuardDecl_ : \
  > &nbsp;&nbsp; [_Identifier_] _RefuteHandler_

  [_Identifier_]: https://doc.rust-lang.org/stable/reference/identifiers.html
