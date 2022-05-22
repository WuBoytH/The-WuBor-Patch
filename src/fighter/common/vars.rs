#![allow(unused_must_use)]

use wubor_utils::vars::*;

fn unload_nro_hook(_info: &skyline::nro::NroInfo) {
    unsafe {
        FGC_TRAINING = false;
    }
}

pub fn install() {
    skyline::nro::add_unload_hook(unload_nro_hook);
}
