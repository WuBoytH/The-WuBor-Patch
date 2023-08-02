mod special_hi_jump;
mod special_lw;
mod special_lw_select;
mod special_lw_start;
mod special_lw_crash_end;
pub mod helper;

pub fn install() {
    special_hi_jump::install();
    special_lw::install();
    special_lw_select::install();
    special_lw_start::install();
    special_lw_crash_end::install();
}