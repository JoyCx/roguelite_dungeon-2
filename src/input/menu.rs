use ratatui::widgets::ListState;

/// Move selection up with wrapping
pub fn move_selection_up(state: &mut ListState, total_items: usize) {
    let s = state.selected().unwrap_or(0);
    state.select(Some(if s == 0 { total_items - 1 } else { s - 1 }));
}

/// Move selection down with wrapping
pub fn move_selection_down(state: &mut ListState, total_items: usize) {
    let s = state.selected().unwrap_or(0);
    state.select(Some(if s >= total_items - 1 { 0 } else { s + 1 }));
}
