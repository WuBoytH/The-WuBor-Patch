#![allow(non_snake_case)]

use crate::imports::*;
use smash_rs::app::{LinkEvent, LinkEventCapture};

#[skyline::hook(offset = 0xb36ee0)]
pub unsafe extern "C" fn jack_damage_callback(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb36ea0)]
pub unsafe extern "C" fn jack_damage_callback2(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0x21b3440)]
pub unsafe extern "C" fn jack_damage_callback3(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb34450)]
pub unsafe extern "C" fn jack_handle_gun_dodge_staling(_vtable: u64, _fighter: &mut Fighter) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0x21b35f0)]
pub unsafe extern "C" fn jack_call_summon_dispatch(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

unsafe extern "C" fn jack_on_attack(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32) {
    let module_accessor = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    if [1, 2].contains(&collision_kind)
    && !WorkModule::is_flag(module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        let mul = if collision_kind == 2 {
            0.1
        }
        else {
            1.0
        };
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        FighterSpecializer_Jack::add_rebel_gauge(module_accessor, FighterEntryID(entry_id), damage * mul);
    }
    jack_on_attack_inner(vtable, fighter, log)
}

#[skyline::from_offset(0xb33d30)]
unsafe extern "C" fn jack_on_attack_inner(vtable: u64, fighter: &mut Fighter, log: u64);

#[skyline::hook(offset = 0xb33820)]
pub unsafe extern "C" fn jack_on_grab(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let event : &mut LinkEvent = std::mem::transmute(log);
    let module_accessor = fighter.battle_object.module_accessor;
    // param_3 + 0x10
    if event.link_event_kind.0 == 0x1653b9bb3a {
        if StatusModule::status_kind(module_accessor) == jack::status::SPECIAL_S_CATCH_JUMP {
            CatchModule::set_send_cut_event(module_accessor, false);
            CatchModule::cling_cut(module_accessor, false);
            return 0;
        }
    }
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut LinkEventCapture = std::mem::transmute(event);
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
            StatusModule::change_status_request(module_accessor, jack::status::SPECIAL_S_CATCH, false);
            capture_event.result = true;
            capture_event.constraint = false;
            CatchModule::set_catch(module_accessor, capture_event.sender_id);
            if LinkModule::is_link(module_accessor, *LINK_NO_CAPTURE) {
                let ptr = MiscModule::get_module_vtable_func(module_accessor, 0x130, 0x80);
                let func: extern "C" fn(catch_module: *mut u64) = std::mem::transmute(ptr);
                let catch_module = (module_accessor as *mut u64).add(0x130 / 0x8);
                func(catch_module);
            }
            LinkModule::set_attribute(
                module_accessor,
                *LINK_NO_CAPTURE,
                LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_COLOR_BLEND as u8},
                false
            );
            let capture_object = MiscModule::get_battle_object_from_id(capture_event.sender_id);
            let capture_id = (*capture_object).battle_object_id;
            if capture_id >> 0x1c != 0
            && capture_id & 0xf0000000 != 0x40000000 {
                return 0;
            }

            let constraint = LinkModule::get_model_constraint_flag(module_accessor) as u32;
            LinkModule::set_model_constraint_flag(module_accessor, constraint | 0x2000); // CONSTRAINT_FLAG_OFFSET_ROT

            let offset = if capture_id >> 0x1c == 0 {
                diddy_get_special_s_offset(singletons::FighterParamAccessor2(), (*capture_object).kind)
            }
            else {
                0.0
            };

            let offset = &mut Vector3f{x: offset, y: offset, z: 0.0};
            if MotionModule::is_flip((*capture_object).module_accessor) {
                offset.x *= -1.0;
            }
            let scale = PostureModule::scale((*capture_object).module_accessor);
            offset.y *= scale;

            LinkModule::set_constraint_translate_offset(module_accessor, offset);

            return 0;
        }
    }
    original!()(vtable, fighter, log)
}

#[skyline::from_offset(0x721240)]
fn diddy_get_special_s_offset(param_accessor_2: *mut smash::app::FighterParamAccessor2, kind: u32) -> f32;

pub fn install() {
    // Disables passive meter gain
    skyline::patching::Patch::in_text(0xb31620).data(0x17FFFF6Eu32);
    // Disables automatically summoning Arsene
    skyline::patching::Patch::in_text(0xb3153c).data(0x14000035u32);
    skyline::patching::Patch::in_text(0xb30dd4).data(0x14000031u32);
    skyline::install_hooks!(
        jack_damage_callback,
        jack_damage_callback2,
        jack_damage_callback3,
        jack_handle_gun_dodge_staling,
        jack_call_summon_dispatch,
        jack_on_grab
    );
    MiscModule::patch_vtable_function(0x4fc71b8, jack_on_attack as u64);
}