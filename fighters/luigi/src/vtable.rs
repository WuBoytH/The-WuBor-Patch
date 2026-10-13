use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::luigi::LINK_EVENT)]
unsafe extern "C" fn luigi_on_link_event_inner(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64;

pub unsafe extern "C" fn luigi_on_change_motion(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {
    // nothing lmao
}

pub unsafe extern "C" fn luigi_on_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        return 1;
    }
    luigi_on_link_event_inner(vtable, fighter, event)
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_LUIGI,
        FighterVtableSlot::OnLinkEvent,
        luigi_on_link_event as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_LUIGI,
        FighterVtableSlot::OnChangeMotion,
        luigi_on_change_motion as *const () as _
    );
}
