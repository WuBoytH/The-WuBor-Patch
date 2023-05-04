mod dengekidama;
mod special_s;
mod special_s_warp;
mod special_hi;
mod special_lw;

pub fn install() {
    dengekidama::install();
    special_s::install();
    special_s_warp::install();
    special_hi::install();
    special_lw::install();
}