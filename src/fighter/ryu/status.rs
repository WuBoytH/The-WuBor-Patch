mod rebirth;
mod dash_back;
mod attack;
// mod attack_hi3;
mod attack_lw4_start;
mod attack_lw4;
mod special_n;
mod special_n2;
mod special_s;
mod special_hi;
mod special_lw;
mod special_lw_step;

pub fn install() {
    rebirth::install();
    dash_back::install();
    attack::install();
    // attack_hi3::install();
    attack_lw4_start::install();
    attack_lw4::install();
    special_n::install();
    special_n2::install();
    special_s::install();
    special_hi::install();
    special_lw::install();
    special_lw_step::install();
}