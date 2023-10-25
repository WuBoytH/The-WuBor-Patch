use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    crate::fighter::common::agent_inits::*
};

pub unsafe extern "C" fn ryu_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::DENJIN_CHARGE)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
        let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
        if pad_flag & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0
        && pad_flag & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), true.into());
            return true.into();
        }
    }
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_N_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND.into(), true.into());
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

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_RYU {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(ryu_check_special_command as *const () as _));
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(speciallw_pre_generic as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}