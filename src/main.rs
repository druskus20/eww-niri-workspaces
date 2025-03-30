use niri_ipc::{socket::Socket, Event, Window, Workspace};

mod serializable;

fn main() {
    let mut state = State::new();
    let niri_socket_env = std::env::var("NIRI_SOCKET");
    let connection = if let Ok(niri_socket) = niri_socket_env {
        Socket::connect_to(niri_socket).unwrap()
    } else {
        Socket::connect().unwrap()
    };
    let (r, mut block_read_next_event) = connection.send(niri_ipc::Request::EventStream).unwrap();
    match r {
        Ok(_) => loop {
            let event = block_read_next_event().unwrap();
            dbg!(get_event_name_str(&event));
            state.update_with_event(event);
            let serializable_state = serializable::SerializableState::from(&state);
            let json = serde_json::to_string(&serializable_state).unwrap();
            dbg!(json);
        },
        Err(e) => {
            eprintln!("Niri error: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_event_name_str(e: &Event) -> &'static str {
    match e {
        Event::WorkspacesChanged { .. } => "WorkspacesChanged",
        Event::WorkspaceActivated { .. } => "WorkspaceActivated",
        Event::WorkspaceActiveWindowChanged { .. } => "WorkspaceActiveWindowChanged",
        Event::WindowsChanged { .. } => "WindowsChanged",
        Event::WindowOpenedOrChanged { .. } => "WindowOpenedOrChanged",
        Event::WindowClosed { .. } => "WindowClosed",
        Event::WindowFocusChanged { .. } => "WindowFocusChanged",
        Event::WindowsLocationsChanged { .. } => "WindowsLocationsChanged",
        Event::KeyboardLayoutsChanged { .. } => "KeyboardLayoutsChanged",
        Event::KeyboardLayoutSwitched { .. } => "KeyboardLayoutSwitched",
    }
}

#[derive(Debug, Default)]
struct State {
    workspaces: Vec<Workspace>,
    windows: Vec<Window>,
}

impl State {
    fn new() -> Self {
        Self::default()
    }

    /// https://yalter.github.io/niri/niri_ipc/enum.Event.html
    fn update_with_event(&mut self, e: Event) {
        match e {
            Event::WorkspacesChanged { workspaces } => self.workspaces = workspaces,
            Event::WorkspaceActivated { id, focused } => {
                if focused {
                    // All other workspaces become not focused
                    for workspace in self.workspaces.iter_mut() {
                        workspace.is_focused = false;
                    }
                }
                if let Some(workspace) = self.workspaces.iter_mut().find(|w| w.id == id) {
                    workspace.is_active = true;
                    workspace.is_focused = focused;
                }
            }
            Event::WorkspaceActiveWindowChanged {
                workspace_id,
                active_window_id,
            } => {
                if let Some(workspace) = self.workspaces.iter_mut().find(|w| w.id == workspace_id) {
                    workspace.active_window_id = active_window_id;
                }
            }
            Event::WindowsChanged { windows } => self.windows = windows,
            Event::WindowOpenedOrChanged { window } => {
                if window.is_focused {
                    // All other windows become not focused
                    for window in self.windows.iter_mut() {
                        window.is_focused = false;
                    }
                }

                // Change or add window
                if let Some(w) = self.windows.iter_mut().find(|w| w.id == window.id) {
                    *w = window;
                } else {
                    self.windows.push(window);
                }
            }
            Event::WindowClosed { id } => {
                self.windows.retain(|w| w.id != id);
            }
            Event::WindowFocusChanged { id } => {
                // All other windows become not focused
                for window in self.windows.iter_mut() {
                    window.is_focused = false;
                }

                // If a window is meant to be focused
                if let Some(id) = id {
                    if let Some(window) = self.windows.iter_mut().find(|w| w.id == id) {
                        window.is_focused = true;
                    }
                }
            }
            Event::WindowsLocationsChanged { changes } => {
                for (id, window_location) in changes {
                    if let Some(window) = self.windows.iter_mut().find(|w| w.id == id) {
                        window.location = window_location;
                    }
                }
            }
            Event::KeyboardLayoutsChanged { .. } => { /* Do nothing */ }
            Event::KeyboardLayoutSwitched { .. } => { /* Do nothing */ }
        }
    }
}
