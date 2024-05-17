use super::*;

mod special_hi_loop;
mod special_hi_end;

pub fn install(agent: &mut smashline::Agent) {
    special_hi_loop::install(agent);
    special_hi_end::install(agent);
}