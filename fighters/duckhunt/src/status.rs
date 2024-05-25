use super::*;

mod special_hi_fly;

pub fn install(agent: &mut Agent) {
    special_hi_fly::install(agent);
}