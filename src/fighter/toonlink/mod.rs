mod acmd;
mod frame;
mod status;
mod vl;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}