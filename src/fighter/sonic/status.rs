mod dash;
mod jump;
mod throw;

pub fn install() {
    dash::install();
    jump::install();
    throw::install();
}