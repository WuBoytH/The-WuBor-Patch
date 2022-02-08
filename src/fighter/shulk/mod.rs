mod acmd;
mod frame;
mod status;
// mod vtable_hook;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    // vtable_hook::install();
}