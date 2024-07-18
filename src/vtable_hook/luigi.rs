use crate::imports::*;

#[skyline::hook(offset = 0xca1510)]
pub unsafe extern "C" fn luigi_change_motion_callback(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {
    // nothing lmao
}

#[skyline::hook(offset = 0xca0e70)]
pub unsafe extern "C" fn luigi_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        return 1;
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
    skyline::install_hooks!(
        luigi_change_motion_callback,
        luigi_link_event
    );
}