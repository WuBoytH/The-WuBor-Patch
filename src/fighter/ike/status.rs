mod special_n_loop;
pub mod special_n_end;
mod special_s_dash;
mod rebirth;
pub mod helper;

pub fn install() {
    special_n_loop::install();
    special_n_end::install();
    special_s_dash::install();
    rebirth::install();
}