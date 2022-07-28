// Note: Dr. Mario's specials have been rotated, but
// the files are still labelled for vanilla.

// Special N has been moved to Special S.
// Special S has been moved to Special LW.
// Special LW has been moved to Special N.

mod special_lw;
mod special_n;
mod special_s;

pub fn install() {
    special_lw::install();
    special_n::install();
    special_s::install();
}