use crate::imports::status_imports::*;

#[skyline::hook(offset = 0xca14f0)]
pub unsafe extern "C" fn luigi_change_motion_callback(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {
    // nothing lmao
}

#[skyline::hook(offset = 0xca0e50)]
pub unsafe extern "C" fn luigi_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        SoundModule::play_se(fighter.battle_object.module_accessor, Hash40::new("se_common_cliff_catch"), true, false, false, false, enSEType(0));
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