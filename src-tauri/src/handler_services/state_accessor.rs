use std::sync::Mutex;

use tauri::{AppHandle, Manager};
use crate::app_state::AppState;

pub trait StateAccessor {
    fn with_state<F, R>(&self, f: F) -> R 
    where
        F: FnOnce(&AppState) -> R;

    fn with_state_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut AppState) -> R;
}

impl StateAccessor for AppHandle {
    fn with_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AppState) -> R,
    {
        let state = self.state::<Mutex<AppState>>();
        let state_guard = state.lock().unwrap();
        f(&state_guard)   
    }

    fn with_state_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut AppState) -> R,
    {
        let state = self.state::<Mutex<AppState>>();
        let mut state_guard = state.lock().unwrap();
        f(&mut state_guard) 
    }
}