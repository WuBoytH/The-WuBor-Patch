mod acmd;
mod status;
// mod frame;

pub fn install() {
    acmd::install();
    status::install();
    // frame::install();
}