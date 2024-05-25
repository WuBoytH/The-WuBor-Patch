use super::*;

mod special_hi_move;

pub fn install(agent: &mut Agent) {
    special_hi_move::install(agent);
}