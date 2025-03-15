use super::*;

mod jump_squat;
mod fall;
mod pass;

pub fn install(agent: &mut Agent) {
    jump_squat::install(agent);
    fall::install(agent);
    pass::install(agent);
}