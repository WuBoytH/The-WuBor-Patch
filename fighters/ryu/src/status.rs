use super::*;

mod rebirth;

// mod wait;

// mod walk;

mod dash_back;

// mod squat_wait;

mod landing;

mod guard_on;
mod guard;

mod attack;

mod attack_s3;
mod attack_hi3;
mod attack_lw3;

mod attack_lw4_start;
mod attack_lw4;

mod attack_air;

mod special_n;

mod special_n2;

mod special_s;
mod special_s_loop;
mod special_s_end;

mod special_hi;

mod special_lw;

// Reused for Denjin Rush
mod special_lw_step_f;

// Reused for Denjin Impact / Denjin Reversal
mod special_lw_step_b;

mod r#final;

pub fn install(agent: &mut Agent) {
    rebirth::install(agent);

    // wait::install(agent);

    // walk::install(agent);

    dash_back::install(agent);

    // squat_wait::install(agent);

    landing::install(agent);

    guard_on::install(agent);
    guard::install(agent);

    attack::install(agent);

    attack_s3::install(agent);
    attack_hi3::install(agent);
    attack_lw3::install(agent);

    attack_lw4_start::install(agent);
    attack_lw4::install(agent);

    attack_air::install(agent);

    special_n::install(agent);

    special_n2::install(agent);

    special_s::install(agent);
    special_s_loop::install(agent);
    special_s_end::install(agent);

    special_hi::install(agent);

    special_lw::install(agent);

    special_lw_step_f::install(agent);

    special_lw_step_b::install(agent);

    r#final::install(agent);
}