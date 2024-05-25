use super::*;

mod special_s;
pub mod helper;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
}