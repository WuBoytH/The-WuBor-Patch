mod special_s;
mod special_s_charge;
mod special_s_end;
mod special_hi_drop;
mod catch;
pub mod helper;

pub fn install() {
    special_s::install();
    special_s_charge::install();
    special_s_end::install();
    special_hi_drop::install();
    catch::install();
}