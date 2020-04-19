mod server_state;
pub mod session_store;

pub use server_state::ServerState;

pub struct Context<S> {
    pub server_state: ServerState<S>,
    pub request: crate::http::Request,
}

impl<S> Context<S> {
    pub fn new(server_state: ServerState<S>, request: crate::http::Request) -> Self {
        Self {
            server_state,
            request,
        }
    }
}
