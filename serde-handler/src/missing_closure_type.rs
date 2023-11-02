use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    channel::{Message, Responder},
    Result,
};

pub trait Api: Serialize {
    /// The service whose API is extended with this implementation.
    ///
    /// `Request`s of the implementing type will be sent to this service.
    const SERVICE: &'static str;

    /// The unique name of the API that identifies the kind of `Request` to the `SERVICE`.
    const NAME: &'static str;

    /// The request body.
    type Request<'de>: Serialize + Deserialize<'de>;

    /// The data returned to answer a `Request`.
    type Reply: Serialize + DeserializeOwned;
}

pub struct ApiRouter {
    handlers: HashMap<&'static str, BoxedHandler>,
}

impl ApiRouter {
    /// Create a new `Router`.
    ///
    /// Unless you add additional routes via [`register_handler`](APIRouter::register_handler), this
    /// will respond with `InvalidRequest` to all requests.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Add a new handler for API requests of type `A`.
    ///
    /// This will make the router route all requests of type `A` to the given `handler` if the
    /// request data can be successfully deserialized into [`A::Request`](Api::Request).
    /// The `handler` may be a function name or a closure.
    pub fn register_handler<A: Api, H: Handler<A>>(mut self, handler: H) -> Self
    where
        H: 'static,
    {
        self.handlers
            .insert(A::NAME, BoxedHandler::from_handler(handler));
        self
    }

    /// Perpetually waits for incoming requests on `socket` and handles them with the handler
    /// registered for their route (see [register_handler](Api::register_handler)), sending
    /// back the computed reply.
    pub fn serve_on(mut self, socket: Responder) -> Result<()> {
        loop {
            let Message { api_name, data } = socket.next_request()?;

            let reply = match self
                .handlers
                .get_mut(api_name.as_str())
                .ok_or_else(|| format!("No handler for '{api_name}'",))
                .and_then(|handler| (handler.0)(&data))
            {
                Ok(reply) => reply,
                Err(e) => {
                    let error_message = e.to_string().into_bytes();
                    let error_response = Message {
                        api_name,
                        data: error_message,
                    };
                    if let Err(e) = socket.send_response(error_response) {
                        eprintln!("Failed to reply to invalid request: {e}",);
                    }
                    continue;
                }
            };

            let response = Message {
                api_name,
                data: reply,
            };
            socket.send_response(response)?;
        }
    }
}

/// A function that can handle [`A::Request<'de>`](API::Request) for `'de == 'req`.
///
/// See the [implementation notes](API#implementation-notes) on `API` for more information.
pub trait HandlerOn<'req, A: Api>: FnMut(A::Request<'req>) -> A::Reply {}
impl<'req, A: Api, F: FnMut(A::Request<'req>) -> A::Reply> HandlerOn<'req, A> for F {}

/// A function that can handle [`A::Request<'de>`](API::Request) for any `'de`.
///
/// See the [implementation notes](API#implementation-notes) on `API` for more information.
pub trait Handler<A: Api>: for<'req> HandlerOn<'req, A> {}
impl<A: Api, F: for<'req> HandlerOn<'req, A>> Handler<A> for F {}

type BoxedRequestHandler = Box<dyn FnMut(&[u8]) -> Result<Vec<u8>>>;
struct BoxedHandler(BoxedRequestHandler);

impl BoxedHandler {
    fn from_handler<A: Api, H: Handler<A>>(mut handler: H) -> Self
    where
        H: 'static,
    {
        let handler = move |request_data: &[u8]| -> Result<Vec<u8>> {
            let request: A::Request<'_> = serde_json::from_slice(request_data)
                .map_err(|e| format!("Deserialize error: {e}"))?;
            let reply = handler(request);
            let reply =
                serde_json::to_vec_pretty(&reply).map_err(|e| format!("Serialize error: {e}"))?;
            Ok(reply)
        };
        Self(Box::new(handler))
    }
}
