use super::*;

mod special_hi;
mod special_hi_keep;

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
    special_hi_keep::install(agent);
}