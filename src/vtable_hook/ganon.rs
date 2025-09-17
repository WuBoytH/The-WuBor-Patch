use crate::imports::*;
use smash_rs::app::{LinkEvent, LinkEventCapture};

#[skyline::hook(offset = 0xaa6990)]
pub unsafe extern "C" fn ganon_link_event(_vtable: u64, fighter: &mut Fighter, log: *mut u64) -> u64 {
    let event : &mut LinkEvent = std::mem::transmute(log);
    let module_accessor = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let event_kind = event.link_event_kind.0;
    // println!("event: {:#x}", event_kind);
    if event_kind == hash40("capture") {
        let capture_event : &mut LinkEventCapture = std::mem::transmute(event);
        if status == *FIGHTER_STATUS_KIND_SPECIAL_HI && capture_event.status == *FIGHTER_STATUS_KIND_CLUNG_GANON {
            if LinkModule::is_link(module_accessor, 0) {
                capture_event.result = false;
                return 0;
            }
            StatusModule::change_status_request(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, false);
            capture_event.result = true;
            capture_event.constraint = false;
            let object_id = capture_event.sender_id;
            CatchModule::set_catch(module_accessor, object_id);
            if !LinkModule::is_link(module_accessor, 0) {
                return 0;
            }
            let ptr = MiscModule::get_module_vtable_func(module_accessor, 0x130, 0x80);
            let func: extern "C" fn(catch_module: *mut u64) = std::mem::transmute(ptr);
            let catch_module = (module_accessor as *mut u64).add(0x130 / 0x8);
            func(catch_module);

            let mut offset = (0.0, 0.0);
            if object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                let object = MiscModule::get_battle_object_from_id(object_id);
                let vtable = *(object as *const u64) as *const u64;
                let func : fn(*mut BattleObject) -> bool = std::mem::transmute(*vtable);
                if !func(object) {
                    if (*object).battle_object_id >> 0x1c == 0 {
                        offset.0 = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_x"));
                        offset.1 = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_y"));
                    }
                }
            }
            LinkModule::set_model_constraint_flag(module_accessor, 0x2003);
            LinkModule::set_constraint_translate_offset(module_accessor, &Vector3f{x: offset.0, y: offset.1, z: 0.0});
            return 0;
        }

        if status == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if capture_event.status == *FIGHTER_STATUS_KIND_CATCHED_GANON {
                StatusModule::change_status_request(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, false);
                capture_event.result = true;
                capture_event.node = smash_rs::phx::Hash40::new("throw");
                return 0;
            }
            if capture_event.status == *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
                StatusModule::change_status_request(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, false);
                capture_event.result = true;
                capture_event.node = smash_rs::phx::Hash40::new("throw");
                return 0;
            }
        }
    }
    else if event_kind == 0xa84e26287 {
        if status == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
            CatchModule::set_send_cut_event(module_accessor, false);
            CatchModule::cling_cut(module_accessor, false);
            if event.padding3[0] != 0 {
                return 0;
            }
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_CUT, false);
            return 0;
        }
    }
    else if event_kind == 0xdac7c579e {
        if status == *FIGHTER_GANON_STATUS_KIND_FINAL_ATTACK {
            let object_id = event.sender_id;
            let object = MiscModule::get_battle_object_from_id(object_id);
            let vtable = *(object as *const u64) as *const u64;
            let func : fn(*mut BattleObject) -> bool = std::mem::transmute(*vtable);
            if !func(object)
            && (*object).battle_object_id >> 0x1c == 1
            && WorkModule::get_int(module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_BEAST_BEAST_TASK_ID) as u32 == (*object).battle_object_id {
                WorkModule::on_flag(module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_BEAST_END);
                return 1;
            }
        }
    }

    1
}

#[skyline::hook(offset = 0xaa6800)]
pub unsafe extern "C" fn ganon_status_transition(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as u64;
    let status = StatusModule::status_kind(module_accessor) as u64;
    if prev_status < 0x36 {
        if 1 << (prev_status & 0x3f) & 0xe00000000000u64 != 0
        && status & 0xfffffffe != 0x2e {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
        if 1 << (prev_status & 0x3f) & 0x7000000000000u64 != 0
        && 1 < status - 0x31 {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
        if 1 << (prev_status & 0x3f) & 0x38000000000000u64 != 0
        && status & 0xfffffffe != 0x34 {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
        if prev_status == 0x2b {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
    }
    if prev_status == 0x24 {
        if status != 0x25 {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
    }
    else if prev_status == 0x25 {
        ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
    }
    else if [0x2b, 0x2f, 0x32, 0x35].contains(&prev_status) {
        if status == 0x24 {
            ArticleModule::generate_article_enable(module_accessor, 1, false, -1);
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        ganon_link_event,
        ganon_status_transition
    );
}