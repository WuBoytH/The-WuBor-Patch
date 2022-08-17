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

pub fn install() {
    dash_back::install();
    guard_off::install();
    escape::install();
    attack::install();
    attack_dash::install();
    attack_s3::install();
    attack_hi3::install();
    attack_lw3::install();
    attack_air::install();
    special_n::install();
    special_s::install();
    special_hi::install();
    special_lw::install();
    superspecial::install();
    appeal::install();
}