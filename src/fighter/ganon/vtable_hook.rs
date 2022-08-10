use {
    smash::{
        hash40,
        phx::Vector3f,
        app::{Fighter/*, Article*/, lua_bind::*},
        lib::lua_const::*
    },
    smash_rs::{
        phx::Hash40,
        app::LinkEventCapture
    },
    wubor_utils::wua_bind::*
};

#[skyline::hook(offset = 0xaa6970)]
pub unsafe extern "C" fn ganon_on_grab(_vtable: u64, fighter: &mut Fighter, capture_event: &mut LinkEventCapture) -> u64 {
    // param_3 + 0x10
    let module_accessor = fighter.battle_object.module_accessor;
    if capture_event.link_event_kind.as_u64() == hash40("capture") {
        let status = StatusModule::status_kind(module_accessor);
        if status == *FIGHTER_STATUS_KIND_SPECIAL_HI && capture_event.status == *FIGHTER_STATUS_KIND_CLUNG_GANON {
            if LinkModule::is_link(module_accessor, 0) {
                capture_event.result = false;
                return 0;
            }
            StatusModule::change_status_request(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, false);
            capture_event.result = true;
            capture_event.constraint = false;
            CatchModule::set_catch(module_accessor, capture_event.sender_id);
            if !LinkModule::is_link(module_accessor, 0) {
                return 0;
            }
            let catchmodule = (module_accessor as *mut u64).offset(0x110) as *mut smash_rs::app::Module;
            crate::function_hooks::some_catch(catchmodule);
            let mut x_offset = 0.0;
            let mut y_offset = 0.0;
            if capture_event.sender_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                if let Some(object) = MiscModule::get_battle_object_from_entry_id(capture_event.sender_id) {
                    if (object as *mut u64).offset(0x10).add(0x2) as u64 > 4
                    && capture_event.sender_id >> 0x1c == 0 {
                        let sender_boma = (*object).module_accessor;
                        x_offset = WorkModule::get_param_float(sender_boma, hash40("param_motion"), hash40("ganon_special_hi_offset_x"));
                        y_offset = WorkModule::get_param_float(sender_boma, hash40("param_motion"), hash40("ganon_special_hi_offset_y"));
                    }
                }
            }
            LinkModule::set_model_constraint_flag(module_accessor, 0x2003);
            LinkModule::set_constraint_translate_offset(module_accessor, &Vector3f{x: x_offset, y: y_offset, z: 0.0});
            return 0;
        }
        if status == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if capture_event.status == *FIGHTER_STATUS_KIND_CATCHED_GANON {
                StatusModule::change_status_request(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, false);
                capture_event.result = true;
                capture_event.node = Hash40::new("throw");
                return 0;
            }
            if capture_event.status != *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
                StatusModule::change_status_request(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, false);
                capture_event.result = true;
                capture_event.node = Hash40::new("throw");
                return 0;
            }
        }
    }
    else {
        if capture_event.link_event_kind.as_u64() == 0xa84e26287 {
            if StatusModule::status_kind(module_accessor) == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
                CatchModule::set_send_cut_event(module_accessor, false);
                CatchModule::cling_cut(module_accessor, false);
                // if ((*(capture_event.to_owned())) as *mut u64).add(0x29) as u64 != 0 {
                //     return 0;
                // }
                StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_CUT, false);
                return 0;
            }
        }
        if capture_event.link_event_kind.as_u64() == 0xdac7c579e {
            if StatusModule::status_kind(module_accessor) == *FIGHTER_GANON_STATUS_KIND_FINAL_ATTACK {
                if let Some(object) = MiscModule::get_battle_object_from_entry_id(capture_event.sender_id) {
                    let sender_boma = (*object).module_accessor;
                    if capture_event.sender_id >> 0x1c == 1
                    && WorkModule::get_int(sender_boma, 0x11000005) as u32 == capture_event.sender_id {
                        WorkModule::on_flag(sender_boma, 0x21000012);
                        return 1;
                    }
                }
            }
        }
    }
    1
}

pub fn install() {
    skyline::install_hooks!(
        ganon_on_grab
    );
}