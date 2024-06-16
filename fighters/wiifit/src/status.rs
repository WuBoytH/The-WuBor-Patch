use super::*;

mod special_hi_jump;
mod special_hi_end;

pub fn install(agent: &mut Agent) {
    special_hi_jump::install(agent);
    special_hi_end::install(agent);
}