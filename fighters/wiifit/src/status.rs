use super::*;

mod special_hi_jump;

pub fn install(agent: &mut Agent) {
    special_hi_jump::install(agent);
}