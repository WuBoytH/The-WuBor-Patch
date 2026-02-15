use super::*;

extern "C" {
    #[link_name = "belmont_mot_kinetic_helper"]
    pub fn belmont_mot_kinetic_helper(
        fighter: &mut L2CFighterCommon,
        some_bool: L2CValue,
        mot_g: L2CValue,
        mot_a: L2CValue,
        kinetic_g: L2CValue,
        kinetic_a: L2CValue,
        correct_g: L2CValue,
        correct_a: L2CValue
    ) -> L2CValue;
}

// mod attack;
mod attack_lw3;
mod attack_lw32;
mod slide_bounce;

mod attack_air;

mod special_n;

mod special_s;

mod special_hi;

mod special_lw;
mod special_air_lw_landing;

pub fn install(agent: &mut Agent) {
    // attack::install(agent);
    attack_lw3::install(agent);
    attack_lw32::install(agent);
    slide_bounce::install(agent);

    attack_air::install(agent);

    special_n::install(agent);

    special_s::install(agent);

    special_hi::install(agent);

    special_lw::install(agent);
    special_air_lw_landing::install(agent);
}