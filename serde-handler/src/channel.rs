use std::sync::mpsc::{self, Receiver, SyncSender};

use crate::Result;

pub struct Message {
    pub api_name: String,
    pub data: Vec<u8>,
}

pub struct Requester {
    pub(crate) outgoing: SyncSender<Message>,
    pub(crate) incoming: Receiver<Message>,
}

pub struct Responder {
    incoming: Receiver<Message>,
    outgoing: SyncSender<Message>,
}

pub fn new_pair() -> (Requester, Responder) {
    let (send1, recv1) = mpsc::sync_channel(1);
    let (send2, recv2) = mpsc::sync_channel(1);
    let req = Requester {
        outgoing: send1,
        incoming: recv2,
    };
    let rep = Responder {
        incoming: recv1,
        outgoing: send2,
    };
    (req, rep)
}

impl Responder {
    pub fn next_request(&self) -> Result<Message> {
        self.incoming.recv().map_err(|e| format!("Recv error: {e}"))
    }

    pub fn send_response(&self, message: Message) -> Result<()> {
        self.outgoing
            .send(message)
            .map_err(|e| format!("Failed to send: {e}"))
    }
}
