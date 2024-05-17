use super::*;

mod special_n_loop;
mod special_n_end;

pub fn install(agent: &mut smashline::Agent) {
    special_n_loop::install(agent);
    special_n_end::install(agent);
}