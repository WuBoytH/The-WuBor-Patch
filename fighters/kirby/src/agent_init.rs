use super::*;

unsafe extern "C" fn kirby_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        return 1.into();
    }
    else {
        let copy_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        if copy_kind == *FIGHTER_KIND_ROSETTA {
            let rosetta_interval = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_ROSETTA_SPECIAL_N_INTERVAL);
            if rosetta_interval <= 0 {
                return 1.into();
            }
            else {
                return 0.into();
            }
        }
        if copy_kind == *FIGHTER_KIND_GANON {
            if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_N) {
                return 0.into();
            }
            else {
                return 1.into();
            }
        }
        if copy_kind != *FIGHTER_KIND_PIT {
            if copy_kind != *FIGHTER_KIND_PITB {
                if copy_kind == *FIGHTER_KIND_INKLING {
                    let inkling_ink = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_INKLING_SPECIAL_N_INK);
                    if inkling_ink > 0.0 {
                        return 1.into();
                    }
                    else {
                        return 0.into();
                    }
                }
                return 1.into();
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY_STRANS_OFF) {
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn kirby_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        return 0.into();
    }
    let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();

    if copy_chara == *FIGHTER_KIND_RYU {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND.into(), true.into());
            return true.into();
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND.into(), true.into());
            return true.into();
        }
    }

    if copy_chara == *FIGHTER_KIND_KEN {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND.into(), true.into());
            return true.into();
        }
    }

    if copy_chara == *FIGHTER_KIND_DOLLY {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
            fighter.change_status(vars::kirby::status::DOLLY_SPECIAL_N_COMMAND.into(), true.into());
            return true.into();
        }
    }

    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(kirby_specialn_pre as *const () as _));
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(kirby_check_special_command as *const () as _));
    FGCModule::clone_command_input(fighter.module_accessor, Cat4::SPECIAL_S_COMMAND, Cat4::SPECIAL_N2_COMMAND);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
