use std::collections::BTreeMap;

/// Serializable state ready to be consumed by Eww as Json
///
/// The json should have: Output -> Workspaces -> Windows { positons }
use serde::Serialize;

use crate::State;

#[derive(Serialize)]
pub(crate) struct SerializableState {
    outputs: BTreeMap<String, Output>,
}

#[derive(Serialize)]
struct Output {
    workspaces: BTreeMap<u64, Workspace>,
}
#[derive(Serialize)]
struct Workspace {
    id: u64,
    columns: BTreeMap<usize, Column>,
    is_active: bool,
}

#[derive(Serialize)]
struct Column {
    index: usize,
    windows: Vec<Window>,
    num_windows: usize,
    has_focused_window: bool,
}

#[derive(Serialize)]
struct Window {
    id: u64,
    column: usize,
    is_focused: bool,
}

impl From<&State> for SerializableState {
    fn from(state: &State) -> Self {
        // first create the workspaces - without windows, then populate the windows
        let mut outputs = BTreeMap::<String, Output>::new();
        for workspace in state.workspaces.iter() {
            let output_name = if let Some(output) = &workspace.output {
                output
            } else {
                continue;
            };

            let output = outputs
                .entry(output_name.clone())
                .or_insert_with(|| Output {
                    workspaces: BTreeMap::new(),
                });

            output.workspaces.insert(
                workspace.id,
                Workspace {
                    id: workspace.id,
                    columns: BTreeMap::new(),
                    is_active: workspace.is_active,
                },
            );
        }

        // populate the windows
        for window in state.windows.iter() {
            // We only care about non-floating windows
            if window.is_floating {
                continue;
            }
            // We only care about windows with a workspace (that exists)
            let workspace = match window.workspace_id {
                Some(workspace_id) => outputs
                    .values_mut()
                    .flat_map(|output| output.workspaces.values_mut())
                    .find(|workspace| workspace.id == workspace_id)
                    .expect("Workspace id set for window not found in state's workspaces"),
                None => continue,
            };

            let column_index = window
                .location
                .tile_pos_in_scrolling_layout
                .expect(
                    "Tile position not set, something is wrong, non-floating windows should have a tile position",
                )
                .0;

            let column = workspace
                .columns
                .entry(column_index)
                .or_insert_with(|| Column {
                    index: column_index,
                    windows: Vec::new(),
                    num_windows: 0,
                    has_focused_window: false,
                });

            if window.is_focused {
                column.has_focused_window = true;
            }
            column.windows.push(Window {
                id: window.id,
                column: column_index,
                is_focused: window.is_focused,
            });
        }

        SerializableState { outputs }
    }
}
