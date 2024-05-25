use super::*;

mod jump;
mod throw;

mod special_s;

mod special_air_s_start;
mod special_air_s_hold;
mod special_air_s_end;

mod trick;

pub fn install(agent: &mut Agent) {
    jump::install(agent);
    throw::install(agent);

    special_s::install(agent);

    special_air_s_start::install(agent);
    special_air_s_hold::install(agent);
    special_air_s_end::install(agent);

    trick::install(agent);
}