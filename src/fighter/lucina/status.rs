mod attack_dash;
mod special_n;
mod special_n_loop;
mod special_n_end;
mod special_s;
mod special_s2;
mod special_s4;
mod special_hi;
mod special_lw;
mod special_lw_hit;
mod rebirth;

pub fn install() {
    attack_dash::install();
    special_n::install();
    special_n_loop::install();
    special_n_end::install();
    special_s::install();
    special_s2::install();
    special_s4::install();
    special_hi::install();
    special_lw::install();
    special_lw_hit::install();
    rebirth::install();
}