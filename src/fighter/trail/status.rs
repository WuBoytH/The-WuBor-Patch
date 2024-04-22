mod attack_air_n;

mod landing_attack_air;

pub fn install(agent: &mut smashline::Agent) {
    attack_air_n::install(agent);

    landing_attack_air::install(agent);
}