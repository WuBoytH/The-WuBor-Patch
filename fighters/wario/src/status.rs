use super::*;

mod rebirth;

mod throw;

mod special_lw;

pub fn install(agent: &mut Agent) {
    rebirth::install(agent);

    throw::install(agent);

    special_lw::install(agent);
}