mod escape;

mod special_lw;
mod special_lw_out;

pub fn install() {
    escape::install();

    special_lw::install();
    special_lw_out::install();
}