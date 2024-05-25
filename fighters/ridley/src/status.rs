use super::*;

mod special_hi_charge;

pub fn install(agent: &mut Agent) {
    special_hi_charge::install(agent);
}