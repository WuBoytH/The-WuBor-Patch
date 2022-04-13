mod acmd;
mod frame;
mod status;
// mod vtable_hook;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    // vtable_hook::install();
}