mod acmd;
mod status;
mod vtable_hook;
pub mod helper;

pub fn install() {
    acmd::install();
    status::install();
    vtable_hook::install();
}