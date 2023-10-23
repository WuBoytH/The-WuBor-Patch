mod rebirth;

mod dash_back;

mod guard_on;
mod guard;
mod guard_damage;

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

pub fn install() {
    rebirth::install();

    dash_back::install();

    guard_on::install();
    guard::install();
    guard_damage::install();

    attack::install();

    attack_s3::install();
    attack_hi3::install();
    attack_lw3::install();

    attack_lw4_start::install();
    attack_lw4::install();

    attack_air::install();

    special_n::install();

    special_n2::install();

    special_s::install();
    special_s_loop::install();
    special_s_end::install();

    special_hi::install();

    special_lw::install();

    special_lw_step_f::install();

    special_lw_step_b::install();
}