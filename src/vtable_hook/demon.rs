use crate::imports::*;
// use smash_rs::app::{LinkEvent, LinkEventCapture};

// #[skyline::hook(offset = 0x933800)]
// pub unsafe extern "C" fn demon_link_event(vtable: u64, fighter: &mut Fighter, event: u64) -> u64 {
//     let link_event : &mut LinkEvent = std::mem::transmute(event);
//     // param_3 + 0x10
//     if link_event.link_event_kind.0 == hash40("capture") {
//         let capture_event : &mut LinkEventCapture = std::mem::transmute(event);
//         let module_accessor = fighter.battle_object.module_accessor;
//         if [
//             *FIGHTER_STATUS_KIND_CATCH,
//             *FIGHTER_STATUS_KIND_CATCH_DASH,
//             *FIGHTER_STATUS_KIND_CATCH_TURN
//         ].contains(&StatusModule::status_kind(module_accessor)) {
//             println!("capture event status: {:#x}", capture_event.status);
//             return 1;
//         }
//     }
//     original!()(vtable, fighter, event)
// }

#[skyline::hook(offset = 0x934310)]
pub unsafe extern "C" fn demon_some_event(_vtable: u64, _fighter: &mut Fighter, event: u64) -> u64 {
    event
}

pub fn install() {
    skyline::install_hooks!(
        // demon_link_event,
        demon_some_event
    );
}