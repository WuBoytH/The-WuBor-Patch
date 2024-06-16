use super::*;

mod special_n_hit_end;

pub fn install(agent: &mut Agent) {
    special_n_hit_end::install(agent);
}