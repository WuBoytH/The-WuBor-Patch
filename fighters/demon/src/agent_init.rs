use super::*;

unsafe extern "C" fn check_attack_3_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if [
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_1,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_3,
        *FIGHTER_DEMON_STATUS_KIND_SQUAT_TURN_AUTO,
    ].contains(&status) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        return 0.into();
    }

    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)
    || WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S.into(), true.into());
            return 1.into();
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
            let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
                FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE
            }
            else {
                FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L
            };
            fighter.change_status(status.into(), true.into());
            return 1.into();
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S.into(), true.into());
            return 1.into();
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F.into(), true.into());
            return 1.into();
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2.into(), true.into());
            return 1.into();
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623NB != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB) {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP.into(), false.into());
                return 1.into();
            }
        }

        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND) {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1.into(), true.into());
                return 1.into();
            }
        }
    }

    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_ATTACK_3_UNIQ].assign(&L2CValue::Ptr(check_attack_3_uniq as *const () as _));
    fighter.global_table[CHECK_ATTACK_S4_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_ATTACK_HI4_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_ATTACK_LW4_UNIQ].assign(&L2CValue::Bool(false));
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(check_special_command as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}