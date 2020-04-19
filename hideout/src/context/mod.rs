mod server_state;
pub mod session_store;

pub use server_state::ServerState;

use std::rc::Rc;

pub struct Context<S> {
    pub server_state: ServerState<S>,
    pub request: crate::http::Request,
    pub session: Option<Rc<S>>,
}

impl<S> Context<S> {
    pub fn new(server_state: ServerState<S>, request: crate::http::Request) -> Self {
        let session = request
            .cookie()
            .get("SID")
            .and_then(|sid| server_state.get_session(sid.as_ref()));
        Self {
            server_state,
            request,
            session,
        }
    }
}
