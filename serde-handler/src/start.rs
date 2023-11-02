use serde::{de::DeserializeOwned, Serialize};

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
    type Request;

    /// The data returned to answer a `Request`.
    type Reply;
}

impl Requester {
    pub fn request<A: Api<Request = A>>(&self, request: A) -> Result<A::Reply>
    where
        A::Request: Serialize,
        A::Reply: DeserializeOwned,
    {
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

impl Responder {
    /// Perpetually waits for incoming requests on `self` and handles them with the given `handler`,
    /// sending back the computed reply.
    pub fn serve_forever<A: Api, H>(self, mut handler: H) -> Result<()>
    where
        H: FnMut(&A::Request) -> A::Reply,
        A::Request: DeserializeOwned,
        A::Reply: Serialize,
    {
        loop {
            let Message { api_name, data } = self.next_request()?;
            let data =
                serde_json::from_slice(&data).map_err(|e| format!("Deserialize error: {e}"))?;
            let reply = handler(&data);
            let data =
                serde_json::to_vec_pretty(&reply).map_err(|e| format!("Serialize error: {e}"))?;
            let response = Message { api_name, data };
            self.send_response(response)?;
        }
    }
}
