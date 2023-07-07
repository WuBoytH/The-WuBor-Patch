mod special_n_escape;
mod special_n_jump;
pub mod helper;

pub fn install() {
    special_n_escape::install();
    special_n_jump::install();
}