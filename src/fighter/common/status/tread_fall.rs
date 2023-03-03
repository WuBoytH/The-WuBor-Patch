use crate::imports::status_imports::*;

// We want to disable teching while getting footstooled.

// #[skyline::hook(replace = L2CFighterCommon_status_TreadFall)]
// unsafe fn status_treadfall(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if !MotionModule::motion_kind(fighter.module_accessor) == 0xd88a289d5
//     || MotionModule::is_end(fighter.module_accessor) {
//         MotionModule::change_motion(
//             fighter.module_accessor,
//             Hash40::new_raw(0x9b5c6425d),
//             0.0,
//             1.0,
//             false,
//             0.0,
//             false,
//             false
//         );
//     }
//     // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
//     // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
//     if !StopModule::is_stop(fighter.module_accessor) {
//         fighter.sub_tread_fall_uniq_check();
//     }
//     fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_tread_fall_uniq_check as *const () as _));
//     fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TreadFall_Main as *const () as _))
// }

#[skyline::hook(replace = L2CFighterCommon_status_TreadFall_Main)]
unsafe fn status_treadfall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_RECOVERY) {
        // if fighter.sub_AirChkPassive().get_bool() {
        //     return 0.into();
        // }
    }
    else {
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        let cancel = fighter.sub_air_check_fall_common().get_bool();
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if cancel {
            return 0.into();
        }
        // if fighter.sub_AirChkPassive().get_bool() {
        //     return 0.into();
        // }
        if fighter.sub_transition_group_check_air_landing().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::motion_kind(fighter.module_accessor) == 0xd88a289d5
        && MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(0x9b5c6425d),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            // status_treadfall,
            status_treadfall_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}