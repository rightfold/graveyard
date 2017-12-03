use direction::Direction;
use world::Area;

use bincode;

use std::io;

const MESSAGE_SIZE_LIMIT: bincode::Bounded = bincode::Bounded(1_000_000);

/// Receive a request.
pub fn receive_request<R: io::Read>(r: &mut R) -> bincode::Result<Request> {
    bincode::deserialize_from(r, MESSAGE_SIZE_LIMIT)
}

/// Send a request.
pub fn send_request<W: io::Write>(w: &mut W, request: &Request) -> bincode::Result<()> {
    bincode::serialize_into(w, request, MESSAGE_SIZE_LIMIT)
}

/// Receive a response.
pub fn receive_response<R: io::Read>(r: &mut R) -> bincode::Result<Response> {
    bincode::deserialize_from(r, MESSAGE_SIZE_LIMIT)
}

/// Send a response.
pub fn send_response<W: io::Write>(w: &mut W, response: &Response) -> bincode::Result<()> {
    bincode::serialize_into(w, response, MESSAGE_SIZE_LIMIT)
}

/// A request sent by the client to the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    Look,
    Move(Direction),
}

/// A response sent by the server to the client.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    Area(Area),
}
