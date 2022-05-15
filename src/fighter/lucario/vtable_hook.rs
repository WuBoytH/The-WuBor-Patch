use {
    smash::{
        hash40,
        app::{Fighter, lua_bind::{StatusModule, WorkModule}},
        lib::lua_const::*
    },
    smash_rs::{
        phx::Hash40,
        app::LinkEventCapture
    }
};

#[skyline::hook(offset = 0xc5d580)]
pub unsafe extern "C" fn lucario_on_grab(_vtable: u64, fighter: &mut Fighter, capture_event: &mut LinkEventCapture) -> u64 {
    if capture_event.link_event_kind.as_u64() == hash40("capture") {
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
            // param_3 + 0x30
            capture_event.node = Hash40::new("throw");
            // param_3[0x28]
            capture_event.result = true;
            let offset = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
            // param_3 + 0x44
            capture_event.motion_offset = offset;
            let offset_lw = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
            // param_3 + 0x48
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request(module_accessor, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, false);
            return 0;
        }
    }
    1
}

pub fn install() {
    skyline::install_hooks!(
        lucario_on_grab
    );
}