use crate::imports::status_imports::*;

// unsafe extern "C" fn bayonetta_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let is_air_f = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F;
//     if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S)
//     || is_air_f {
//         if bayonetta_specials_pre_count(fighter).get_bool() {
//             if !is_air_f
//             || (is_air_f && CancelModule::is_enable_cancel(fighter.module_accessor)) {
//                 let reuse = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_REUSE_FRAME);
//                 if reuse == 0 {
//                     return false.into();
//                 }
//             }
//         }
//         if [
//             *FIGHTER_STATUS_KIND_JUMP,
//             *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP,
//             *FIGHTER_STATUS_KIND_CLIFF_JUMP2
//         ].contains(&fighter.global_table[STATUS_KIND_INTERRUPT].get_i32()) {
//             let jump_after_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_DISABLE_JUMP_AFTER_FRAME);
//             if 0 < jump_after_frame {
//                 return false.into();
//             }
//         }
//         return true.into();
//     }
//     false.into()
// }

// unsafe extern "C" fn bayonetta_specials_pre_count(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
//     return (0 < count).into();
// }

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::add_reset_statuses(
        fighter.battle_object_id,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        vec![
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F
        ]
    );
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}
