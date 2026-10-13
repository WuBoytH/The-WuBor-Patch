use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::shulk::ON_ATTACK)]
unsafe extern "C" fn shulk_on_attack_inner(vtable: u64, fighter: *mut Fighter, log: &mut CollisionLogScuffed, damage: f32) -> u64;

pub unsafe extern "C" fn shulk_on_attack(vtable: u64, fighter: *mut Fighter, log: &mut CollisionLogScuffed, damage: f32) -> u64 {
    let module_accessor = (*fighter).battle_object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    // if WorkModule::is_flag(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE)
    // && WorkModule::get_int(module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_JUMP {
    //     if [
    //         *FIGHTER_STATUS_KIND_SPECIAL_S,
    //         *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP,
    //         *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL,
    //         *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING
    //     ].contains(&status)
    //     && log.collision_kind == 1 {
    //         let opponent_object = MiscModule::get_battle_object_from_id(log.opponent_object_id);
    //         if !opponent_object.is_null()
    //         && sv_battle_object::category(log.opponent_object_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
    //             VarModule::on_flag((*opponent_object).module_accessor, vars::fighter::instance::flag::PURGED);
    //             VarModule::set_int((*opponent_object).module_accessor, vars::fighter::instance::int::PURGED_TIMER, 300);
    //         }
    //     }
    // }
    if ![
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_N_ACTION,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_ATTACK,
    ].contains(&status)
    && log.collision_kind == 1 {
        for art in 0..5 {
            WorkModule::count_down_int(module_accessor, 0x100000C9 + art, 0);
        }
    }
    shulk_on_attack_inner(vtable, fighter, log, damage)
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_SHULK,
        FighterVtableSlot::OnAttack,
        shulk_on_attack as *const () as _
    );
}
