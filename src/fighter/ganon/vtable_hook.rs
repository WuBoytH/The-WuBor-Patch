use {
    smash::{
        hash40,
        phx::Vector3f,
        app::{Fighter/*, Article*/, lua_bind::*},
        lib::lua_const::*
    },
    smash_rs::{
        phx::Hash40,
        app::{LinkEvent, LinkEventCapture}
    },
    wubor_utils::wua_bind::*
};

#[skyline::hook(offset = 0xaa6970)]
pub unsafe extern "C" fn ganon_on_grab(_vtable: u64, fighter: &mut Fighter, event: u64) -> u64 {
    // param_3 + 0x10
    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let link_event : &mut LinkEvent = std::mem::transmute(event);
    if link_event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut LinkEventCapture = std::mem::transmute(event);
        if status == *FIGHTER_STATUS_KIND_SPECIAL_HI && capture_event.status == *FIGHTER_STATUS_KIND_CLUNG_GANON {
            if LinkModule::is_link(module_accessor, *LINK_NO_CAPTURE) {
                capture_event.result = false;
                return 0;
            }
            StatusModule::change_status_request(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, false);
            capture_event.result = true;
            capture_event.constraint = false;
            CatchModule::set_catch(module_accessor, capture_event.sender_id);
            if !LinkModule::is_link(module_accessor, *LINK_NO_CAPTURE) {
                return 0;
            }
            let catch_module = *(module_accessor as *const u64).offset(0x120 / 8) as *const u64;
            let func : fn(*mut smash_rs::app::Module) = std::mem::transmute(catch_module.offset(0x80 / 8));
            let catch_module = *(module_accessor as *const u64).offset(0x120 / 8) as *mut smash_rs::app::Module;
            func(catch_module);
            let mut offset_x = 0.0;
            let mut offset_y = 0.0;
            if capture_event.sender_id != 0x50000000 {
                let object = MiscModule::get_battle_object_from_id(capture_event.sender_id);
                if !object.is_null() /*|| *((object as *const u8).offset())*/ {
                    // skipping a check that will probably always be 0?
                    if (*object).battle_object_id >> 0x1c == 0 {
                        offset_x = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_x"));
                        offset_y = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_y"));
                    }
                }
            }
            LinkModule::set_model_constraint_flag(module_accessor, 0x2003);
            LinkModule::set_constraint_translate_offset(module_accessor, &Vector3f{x: offset_x, y: offset_y, z: 0.0});
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
    else if link_event.link_event_kind.0 == 0xa84e26287 {
        if status == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
            CatchModule::set_send_cut_event(module_accessor, false);
            CatchModule::cling_cut(module_accessor, false);
            if *(event as *const u8).offset(0x29) != 0 {
                return 0;
            }
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_CUT, false);
        }
    }
    else if link_event.link_event_kind.0 == 0xdac7c579e {
        if status == *FIGHTER_GANON_STATUS_KIND_FINAL_ATTACK {
            let object = MiscModule::get_battle_object_from_id(link_event.sender_id);
            if !object.is_null()
            && (*object).battle_object_id >> 0x1c == 1
            && (*object).battle_object_id == WorkModule::get_int(module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_BEAST_BEAST_TASK_ID) as u32 {
                WorkModule::on_flag(module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_BEAST_END);
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