use super::*;

mod jump_squat;
mod fall;

pub fn install(agent: &mut smashline::Agent) {
    jump_squat::install(agent);
    fall::install(agent);
}