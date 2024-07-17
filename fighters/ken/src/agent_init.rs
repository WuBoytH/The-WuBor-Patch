use super::*;

extern "C" {
    #[link_name = "speciallw_pre_generic"]
    pub fn speciallw_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub unsafe extern "C" fn ken_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL) {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0 {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x229a8a3811));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                VarModule::on_flag(fighter.module_accessor, vars::ken::instance::flag::SKIP_FINAL_PROX_CHECK);
                fighter.change_status(FIGHTER_RYU_STATUS_KIND_FINAL2.into(), true.into());
                return true.into();
            }
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0 {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x229a8a3811));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                VarModule::on_flag(fighter.module_accessor, vars::ken::instance::flag::SKIP_FINAL_PROX_CHECK);
                fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
                return true.into();
            }
        }
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1 != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool()
    && FighterSpecializer_Ryu::check_special_air_s_command(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(ken_check_special_command as *const () as _));
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(speciallw_pre_generic as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}