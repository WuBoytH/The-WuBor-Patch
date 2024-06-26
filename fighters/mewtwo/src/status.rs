use super::*;

mod escape_air;

mod special_hi_2;

pub fn install(agent: &mut Agent) {
    escape_air::install(agent);

    special_hi_2::install(agent);
}