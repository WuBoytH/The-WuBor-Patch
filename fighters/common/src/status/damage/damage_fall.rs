use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFall)]
unsafe extern "C" fn status_pre_damagefall(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FALL_FLOAT,
        *FS_SUCCEEDS_KEEP_SOUND
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_DamageFall)]
unsafe extern "C" fn bind_address_call_status_damagefall(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_DamageFall()
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFall)]
unsafe extern "C" fn status_damagefall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_DamageFall_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 1.25, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_DamageFall_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFall_Main)]
unsafe extern "C" fn status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool()
    || fighter.check_damage_fall_transition().get_bool() {
        return 0.into();
    }

    if fighter.is_enable_passive().get_bool() {
        let tech = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND) {
            let trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_passive_trigger_frame"));
            let frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
            fighter.sub_check_passive_button((trigger_frame as f32 * frame_mul).into()).get_bool()
        }
        else {
            let passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("passive_trigger_frame")) as f32;
            let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
            fighter.sub_check_passive_button((passive_trigger_frame * passive_trigger_frame_mul).into()).get_bool()
        };
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
        && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value")) <= fighter.global_table[STICK_X].get_f32().abs()
        && tech {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
        && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor)
        && tech {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
            return true.into();
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), true.into());
        return 0.into();
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    0.into()
}

// #[skyline::hook(replace = L2CFighterCommon_set_damage_fall_transition)]
// unsafe extern "C" fn set_damage_fall_transition(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
//     if !param_1.get_bool() {
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_TRANSITION) {
//             return;
//         }
//         WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
//         WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
//         WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
//         WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_WALL_JUMP);
//         WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
//         // WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
//         WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
//         WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_TRANSITION);
//     }
//     else {
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_TRANSITION) {
//             if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
//                 if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_WORK_INT_NO_TRANSITION_FRAME) > 0 {
//                     return;
//                 }
//                 WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
//                 WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
//                 WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
//                 WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_WALL_JUMP);
//                 WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
//                 // WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
//                 WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO);
//                 WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FALL_FLAG_TRANSITION);
//             }
//         }
//     }
// }

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_damagefall,
            bind_address_call_status_damagefall,
            status_damagefall,
            status_damagefall_main,
            // set_damage_fall_transition
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}