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

pub fn install(agent : &mut smashline::Agent) {
    attack_dash::install(agent);
    special_n::install(agent);
    special_n_loop::install(agent);
    special_n_end::install(agent);
    special_s::install(agent);
    special_s2::install(agent);
    special_s4::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    special_lw_hit::install(agent);
    rebirth::install(agent);
}