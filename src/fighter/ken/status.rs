mod dash_back;

mod attack;

mod attack_lw4_start;
mod attack_lw4;

mod special_s;
mod special_s_loop;
mod special_s_end;

mod special_hi;

mod special_lw;

pub fn install() {
    dash_back::install();

    attack::install();

    attack_lw4_start::install();
    attack_lw4::install();

    special_s::install();
    special_s_loop::install();
    special_s_end::install();

    special_hi::install();

    special_lw::install();
}