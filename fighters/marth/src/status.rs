use super::*;

mod special_s;

mod special_hi;

mod stance;

pub mod helper;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);

    special_hi::install(agent);

    stance::install(agent);
}