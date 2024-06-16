use super::*;

mod jump_squat;
mod fall;

pub fn install(agent: &mut Agent) {
    jump_squat::install(agent);
    fall::install(agent);
}