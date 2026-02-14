use super::*;

pub mod helper;
mod special_n;
mod special_n_escape;
mod special_n_jump;

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
    special_n_escape::install(agent);
    special_n_jump::install(agent);
}
