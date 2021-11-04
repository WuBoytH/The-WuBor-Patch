mod acmd;
// mod frame;
mod status;

pub fn install() {
    acmd::install();
    // frame::install();
    status::install();
}