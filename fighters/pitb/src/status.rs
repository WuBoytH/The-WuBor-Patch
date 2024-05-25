use super::*;

mod special_n_charge;
mod special_n_shoot;

pub fn install(agent: &mut Agent) {
    special_n_charge::install(agent);
    special_n_shoot::install(agent);
}