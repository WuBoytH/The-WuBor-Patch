mod acmd;
mod status;

pub fn install() {
    acmd::install();
    status::install();
}