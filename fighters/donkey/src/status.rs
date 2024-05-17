use super::*;

mod special_s;

pub fn install(agent: &mut smashline::Agent) {
    special_s::install(agent);
}