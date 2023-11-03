mod dash_back;

mod attack;

mod attack_lw4_start;
mod attack_lw4;

mod special_hi;

mod special_lw;

pub fn install() {
    dash_back::install();

    attack::install();

    attack_lw4_start::install();
    attack_lw4::install();

    special_hi::install();

    special_lw::install();
}