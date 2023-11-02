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
    handlers: HashMap<&'static str, Box<dyn Handler>>,
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
    pub fn register_handler<A: Api, H: Handler<Api = A>>(mut self, handler: H) -> Self
    where
        H: 'static,
    {
        self.handlers.insert(A::NAME, handler);
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
            let reply = handler(&data)?;
            let response = Message {
                api_name,
                data: reply,
            };
            socket.send_response(response)?;
        }
    }
}

/// A function that can handle [`A::Request<'de>`](Api::Request) for `'de == 'req`.
pub trait HandlerOn<'req>:
    FnMut(
    <<Self as HandlerOn<'req>>::Api as Api>::Request<'req>,
) -> <<Self as HandlerOn<'req>>::Api as Api>::Reply
{
    type Api: Api;
}
impl<'req, A: Api, F: FnMut(A::Request<'req>) -> A::Reply> HandlerOn<'req> for F {
    type Api = A;
}

/// A function that can handle [`A::Request<'de>`](Api::Request) for any `'de`.
pub trait Handler: for<'req> HandlerOn<'req> {}
impl<F: for<'req> HandlerOn<'req>> Handler for F {}

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
