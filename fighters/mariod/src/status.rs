use super::*;

mod special_lw;
mod rebirth;

pub fn install(agent: &mut Agent) {
    special_lw::install(agent);
    rebirth::install(agent);
}