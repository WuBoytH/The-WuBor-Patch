use super::*;

mod entry;
pub mod helper;

pub fn install(agent: &mut Agent) {
    entry::install(agent);
}