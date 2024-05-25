use super::*;

mod special_hi_attack;
mod special_hi_reflect;

pub fn install(agent: &mut Agent) {
    special_hi_attack::install(agent);
    special_hi_reflect::install(agent);
}