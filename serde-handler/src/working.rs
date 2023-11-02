use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    channel::{Message, Requester, Responder},
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
    /// Unless you add additional routes via [`register_handler`](ApiRouter::register_handler), this
    /// will respond with `InvalidRequest` to all requests.
    #[allow(clippy::new_without_default)]
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
    /// registered for their route (see [register_handler](ApiRouter::register_handler)), sending
    /// back the computed reply.
    pub fn serve_on(mut self, socket: Responder) -> Result<()> {
        loop {
            let Message { api_name, data } = socket.next_request()?;

            let handler = self
                .handlers
                .get_mut(api_name.as_str())
                .ok_or_else(|| format!("No handler for '{api_name}'",))?;
            let reply = (handler.0)(&data)?;
            let response = Message {
                api_name,
                data: reply,
            };
            socket.send_response(response)?;
        }
    }
}

/// A function that can handle [`A::Request<'de>`](Api::Request) for `'de == 'req`.
pub trait HandlerOn<'req, A: Api>: FnMut(A::Request<'req>) -> A::Reply {}
impl<'req, A: Api, F: FnMut(A::Request<'req>) -> A::Reply> HandlerOn<'req, A> for F {}

/// A function that can handle [`A::Request<'de>`](Api::Request) for any `'de`.
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

impl Requester {
    pub fn request<'a, A: Api<Request<'a> = A>>(&self, request: A) -> Result<A::Reply> {
        let data =
            serde_json::to_vec_pretty(&request).map_err(|e| format!("Serialize error: {e}"))?;
        let request = Message {
            api_name: A::NAME.to_string(),
            data,
        };
        self.outgoing
            .send(request)
            .map_err(|e| format!("Failed to send request: {e}"))?;
        let response = self
            .incoming
            .recv()
            .map_err(|e| format!("Error receiving response: {e}"))?;
        assert_eq!(response.api_name, A::NAME);
        serde_json::from_slice(&response.data).map_err(|e| format!("Deserialize error: {e}"))
    }
}
