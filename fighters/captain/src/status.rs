use super::*;

mod special_n;
mod special_s;
pub mod helper;

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
    special_s::install(agent);
}