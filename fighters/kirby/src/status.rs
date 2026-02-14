use super::*;

mod attack_dash;
mod attack_lw3;
mod attack_lw3_bounce;

mod copy;

pub fn install(agent: &mut Agent) {
    attack_dash::install(agent);
    attack_lw3::install(agent);
    attack_lw3_bounce::install(agent);

    copy::install(agent);
}
