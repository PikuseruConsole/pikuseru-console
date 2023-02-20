use std::net::SocketAddr;
use bytemuck::{Pod, Zeroable};

use ggrs::{Config, PlayerType};

use pikuseru::input::{Buttons, InputState, MouseState};
use crate::PikuseruConsole;

#[derive(Clone)]
pub struct PikuseruConsoleState {
    pub(crate) previous_buttons: Box<[Buttons]>,
}

pub struct SaveStateDefinition {
    pub(crate) memories: Vec<String>,
    pub(crate) mutable_globals: Vec<String>,
}

#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NetworkInputState {
    pub input_state: InputState,
    pub mouse_state: MouseState,
}

impl Config for PikuseruConsole {
    type Input = NetworkInputState;
    type State = PikuseruConsoleState;
    type Address = SocketAddr;
}

#[derive(Clone)]
pub struct SessionDescriptor {
    pub num_players: usize,
    pub player_types: Box<[PlayerType<SocketAddr>]>,
    pub port: u16,
}