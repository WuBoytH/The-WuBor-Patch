mod wait;

mod dash_back;

mod guard_off;

mod escape;

mod attack;

mod attack_dash;

mod attack_s3;

mod attack_hi3;

mod attack_lw3;

mod attack_air;

mod special_n;

mod special_s;

mod special_hi;

mod special_lw;

mod superspecial;

mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    wait::install(agent);

    dash_back::install(agent);

    guard_off::install(agent);

    escape::install(agent);

    attack::install(agent);

    attack_dash::install(agent);

    attack_s3::install(agent);

    attack_hi3::install(agent);

    attack_lw3::install(agent);

    attack_air::install(agent);

    special_n::install(agent);

    special_s::install(agent);

    special_hi::install(agent);

    special_lw::install(agent);

    superspecial::install(agent);

    appeal::install(agent);
}