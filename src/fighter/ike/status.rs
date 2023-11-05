mod special_n_loop;
pub mod special_n_end;
mod special_s_dash;
mod rebirth;
pub mod helper;

pub fn install(agent : &mut smashline::Agent) {
    special_n_loop::install(agent);
    special_n_end::install(agent);
    special_s_dash::install(agent);
    rebirth::install(agent);
}