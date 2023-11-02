use std::{
    io::{self, BufRead},
    thread,
};

use serde::{Deserialize, Serialize};

use serde_handler::{channel, zero_copy2::*};

const TEXT_SERVICE: &str = "text";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct UppercaseRequest<'a>(&'a str);

impl<'de> Api for UppercaseRequest<'de> {
    type Reply = String;
    type Request = UppercaseRequest<'de>;

    const NAME: &'static str = "upper";
    const SERVICE: &'static str = TEXT_SERVICE;
}

fn main() {
    let (requester, responder) = channel::new_pair();

    thread::spawn(|| {
        responder
            .serve_forever::<UppercaseRequest, _>(|request| request.0.to_uppercase().to_string());
    });

    // Repeatedly read lines from the terminal and send them off as requests
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                // Text requests
                let upper_request = UppercaseRequest(&input);
                let upper = requester.request(upper_request).unwrap();
                println!("Uppercase: {upper}");
            }
            Err(e) => eprintln!("Input error: {e}"),
        }
    }
}
