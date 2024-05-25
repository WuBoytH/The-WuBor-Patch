use super::*;

mod special_s;
mod special_s_charge;
mod special_s_end;
mod special_hi_drop;
mod catch;
pub mod helper;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
    special_s_charge::install(agent);
    special_s_end::install(agent);
    special_hi_drop::install(agent);
    catch::install(agent);
}