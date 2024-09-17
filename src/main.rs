use niri_ipc::{socket::Socket, Event, Response};
use std::{collections::HashMap, io};

fn main() {
    let mut state = State::new();
    let niri_socket_env = std::env::var("NIRI_SOCKET");
    let mut niri_socket = if *&niri_socket_env.is_ok() {
        Socket::connect_to(&niri_socket_env.unwrap()).unwrap()
    } else {
        Socket::connect().unwrap()
    };
    let r = niri_socket.send(niri_ipc::Request::EventStream);
    let (_, mut f) = handle_niri_response(r).unwrap();

    loop {
        let event = f().unwrap();
        state.update_with_event(event);
    }
    todo!()
}

type Id = u64;

struct State {
    _windows_per_workspace: HashMap<Id, Vec<Id>>,
    focused_workspace_id: Id,
    active_workspaces_id: Vec<Id>,
    focused_window_id: Id,
}

impl State {
    fn new() -> Self {
        Self {
            _windows_per_workspace: Default::default(),
            focused_workspace_id: Default::default(),
            active_workspaces_id: Default::default(),
            focused_window_id: Default::default(),
        }
    }

    fn update_with_event(&mut self, e: Event) {}
}

fn handle_niri_response(
    r: Result<
        (
            Result<Response, String>,
            impl FnMut() -> Result<Event, std::io::Error>,
        ),
        std::io::Error,
    >,
) -> Result<(Response, impl FnMut() -> io::Result<Event>), String> {
    match r {
        Ok((Ok(r), f)) => Ok((r, f)),
        Ok((Err(e), _)) => Err(e),
        Err(e) => Err(e.to_string()),
    }
}
