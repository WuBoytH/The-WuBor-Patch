use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::lucina::MARTH_LUCINA_ON_ATTACK)]
unsafe extern "C" fn marth_lucina_on_attack(vtable: u64, fighter: &mut Fighter, log: u64);

unsafe extern "C" fn lucina_init(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    VarModule::set_float(module_accessor, vars::yu::instance::float::SP_GAUGE_MAX, 100.0);
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
    let module_accessor = fighter.battle_object.module_accessor;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor) {
        VarModule::countdown_int(module_accessor, vars::yu::instance::int::SP_GAIN_PENALTY, 0);
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
            helper::add_sp(module_accessor, damage * mul);
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            let opponent_object = MiscModule::get_battle_object_from_id((*collision_log).opponent_object_id);
            let opponent_module_accessor = (*opponent_object).module_accessor;
            helper::handle_slow(module_accessor, opponent_module_accessor);
        }
    }
    marth_lucina_on_attack(vtable, fighter, log)
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_LUCINA,
        4,
        lucina_init as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_LUCINA,
        13,
        lucina_per_frame as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_LUCINA,
        36,
        lucina_on_attack as *const () as _
    );
}
