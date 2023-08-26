mod dash_back;
mod attack;
// mod attack_hi3;
mod special_s;
mod special_s_loop;
mod special_hi;
mod special_lw_step;

pub fn install() {
    dash_back::install();
    attack::install();
    // attack_hi3::install();
    special_s::install();
    special_s_loop::install();
    special_hi::install();
    special_lw_step::install();
}