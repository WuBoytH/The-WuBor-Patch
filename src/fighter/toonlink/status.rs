mod boomerang;
mod special_hi;
mod special_lw;

pub fn install() {
    boomerang::install();
    special_hi::install();
    special_lw::install();
}