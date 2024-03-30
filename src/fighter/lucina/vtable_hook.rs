use crate::imports::*;
use wubor_utils::app::*;
use super::helper::*;

unsafe extern "C" fn lucina_init(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        return;
    }
    WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    VarModule::set_float(module_accessor, yu::instance::float::SP_GAUGE_MAX, 100.0);
    FGCModule::set_command_input_button(module_accessor, 0, 2);
    FGCModule::set_command_input_button(module_accessor, 1, 2);
    FGCModule::set_command_input_button(module_accessor, 2, 2);
    FGCModule::set_command_input_button(module_accessor, 3, 2);
    FGCModule::set_command_input_button(module_accessor, 8, 2);
    FGCModule::set_command_input_button(module_accessor, 9, 2);
    FGCModule::set_command_input_button(module_accessor, 10, 2);
    FGCModule::set_command_input_button(module_accessor, 11, 2);
}

unsafe extern "C" fn lucina_per_frame(_vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        return;
    }
    let module_accessor = fighter.battle_object.module_accessor;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor) {
        VarModule::countdown_int(module_accessor, yu::instance::int::SP_GAIN_PENALTY, 0);
    }
}

unsafe extern "C" fn lucina_on_attack(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32) {
    let module_accessor = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    if fighter.battle_object.kind == *FIGHTER_KIND_LUCINA as u32 {
        let collision_kind = (*collision_log).collision_kind;
        if [1, 2].contains(&collision_kind) {
            let mul = if collision_kind == 2 {
                0.1
            }
            else {
                1.0
            };
            add_sp(module_accessor, damage * mul);
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            let opponent_object = MiscModule::get_battle_object_from_id((*collision_log).opponent_object_id);
            let opponent_module_accessor = (*opponent_object).module_accessor;
            handle_slow(module_accessor, opponent_module_accessor);
        }
    }
    marth_lucina_on_attack(vtable, fighter, log)
}

#[skyline::from_offset(0xcd9ea0)]
unsafe extern "C" fn marth_lucina_on_attack(vtable: u64, fighter: &mut Fighter, log: u64);

pub fn install() {
    MiscModule::patch_vtable_function(0x4fe7fc0, lucina_init as u64);
    MiscModule::patch_vtable_function(0x4fe8008, lucina_per_frame as u64);
    MiscModule::patch_vtable_function(0x4fe80c0, lucina_on_attack as u64);
}