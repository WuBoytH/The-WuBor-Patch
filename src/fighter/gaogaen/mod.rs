mod acmd;
mod frame;
mod vtable_hook;

pub fn install() {
    acmd::install();
    frame::install();
    vtable_hook::install();
}