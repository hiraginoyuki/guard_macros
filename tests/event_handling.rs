#![allow(dead_code)]

/// Ignores any tokens inside the block.
macro_rules! comment {
    ( $($_:tt)* ) => {};
}

use guard_macros::guard;

#[cfg_attr(test, test)]
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
