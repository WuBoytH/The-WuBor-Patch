use super::*;

mod special_hi_loop;

pub fn install(agent: &mut Agent) {
    special_hi_loop::install(agent);
}