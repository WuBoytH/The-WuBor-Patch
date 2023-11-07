mod fall_special;

mod attack_air;

mod special_hi;

mod special_lw;

mod uniq_float;

pub fn install(agent: &mut smashline::Agent) {
    fall_special::install(agent);

    attack_air::install(agent);

    special_hi::install(agent);

    special_lw::install(agent);

    uniq_float::install(agent);
}