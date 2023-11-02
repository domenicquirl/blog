use std::{
    io::{self, BufRead},
    thread,
};

use serde::{Deserialize, Serialize};

use serde_handler::{channel, working::*};

const TEXT_SERVICE: &str = "text";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct UppercaseRequest<'a>(&'a str);

impl Api for UppercaseRequest<'_> {
    type Reply = String;
    type Request<'de> = UppercaseRequest<'de>;

    const NAME: &'static str = "upper";
    const SERVICE: &'static str = TEXT_SERVICE;
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LowercaseRequest<'a>(&'a str);

impl Api for LowercaseRequest<'_> {
    type Reply = String;
    type Request<'de> = LowercaseRequest<'de>;

    const NAME: &'static str = "lower";
    const SERVICE: &'static str = TEXT_SERVICE;
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TrimRequest<'a>(&'a str);

impl Api for TrimRequest<'_> {
    type Reply = String;
    type Request<'de> = TrimRequest<'de>;

    const NAME: &'static str = "trim";
    const SERVICE: &'static str = TEXT_SERVICE;
}

fn text_service_router() -> ApiRouter {
    ApiRouter::new()
        .register_handler::<UppercaseRequest, _>(|req| req.0.to_uppercase())
        .register_handler::<LowercaseRequest, _>(|req| req.0.to_lowercase())
        .register_handler::<TrimRequest, _>(|req| req.0.trim().to_string())
}

fn main() {
    let (requester, responder) = channel::new_pair();

    // Have the service run (forever)
    thread::spawn(move || {
        let router = text_service_router();
        router.serve_on(responder).expect("to run forever");
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

                let lower_request = LowercaseRequest(&input);
                let lower = requester.request(lower_request).unwrap();
                println!("Lowercase: {lower}");

                let trim_request = TrimRequest(&input);
                let trimmed = requester.request(trim_request).unwrap();
                println!("Trimmed  : {trimmed}");
            }
            Err(e) => eprintln!("Input error: {e}"),
        }
    }
}
