use super::*;

mod start;
mod shoot;

pub fn install(agent: &mut Agent) {
    start::install(agent);
    shoot::install(agent);
}