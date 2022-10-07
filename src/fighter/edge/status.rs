mod special_hi;
mod special_hi_rush;
mod special_hi_end;
mod special_hi_landing;
pub mod helper;

pub fn install() {
    special_hi::install();
    special_hi_rush::install();
    special_hi_end::install();
    special_hi_landing::install();
}