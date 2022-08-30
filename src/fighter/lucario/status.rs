mod special_n;
mod auraball;
mod special_s;
mod special_hi;
mod special_lw;
mod escape_air;
mod landing;

pub fn install() {
    special_n::install();
    auraball::install();
    special_s::install();
    special_hi::install();
    special_lw::install();
    escape_air::install();
    landing::install();
}