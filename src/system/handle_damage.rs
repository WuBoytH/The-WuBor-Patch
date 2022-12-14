use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    crate::{
        fighter::{
            dolly::helper::*,
            gaogaen::helper::*,
            ken::helper::*,
            lucina::helper::*
        }
    },
    wubor_utils::wua_bind::*
};

#[skyline::hook(offset = 0x6310a0)]
unsafe fn fighter_handle_damage_hook(object: *mut BattleObject, arg: *const u8) {
    let module_accessor = (*object).module_accessor;
    // let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    // let hitstun = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    // println!("histun remaining: {}", hitstun);
    call_original!(object, arg);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) - damage_received;
    let attacker_ids = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    // println!("attacker ids: {}", attacker_ids);
    let fighter_kind = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if fighter_kind == *FIGHTER_KIND_GAOGAEN {
        handle_revenge(object, module_accessor, attacker_ids);
    }
    for x in 0..8 {
        if attacker_ids & (1 << x) == 0 {
            continue;
        }
        if let Some(object_id) = MiscModule::get_active_battle_object_id_from_entry_id(x) {
            let object = MiscModule::get_battle_object_from_id(object_id);
            let module_accessor = (*object).module_accessor;
            let kind = utility::get_kind(&mut *module_accessor);
            if kind == *FIGHTER_KIND_LUCINA {
                add_sp(object, module_accessor, damage_received);
            }
            else if kind == *FIGHTER_KIND_KEN {
                add_vgauge(object, module_accessor, damage_received);
            }
            else if kind == *FIGHTER_KIND_DOLLY {
                add_go(object, module_accessor, damage_received);
            }
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        fighter_handle_damage_hook
    );
    
}