use super::*;

mod special_hi;
mod special_hi_air_end;

pub fn install(agent: &mut smashline::Agent) {
    special_hi::install(agent);
    special_hi_air_end::install(agent);
}