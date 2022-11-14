mod dash_back;
mod attack;
mod special_s;
mod special_s_loop;
mod special_hi;
mod special_lw;

pub fn install() {
    dash_back::install();
    attack::install();
    special_s::install();
    special_s_loop::install();
    special_hi::install();
    special_lw::install();
}