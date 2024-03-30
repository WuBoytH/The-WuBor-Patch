use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame,
    super::vl
};

unsafe extern "C" fn marth_allow_stance_toggle(fighter: &mut L2CFighterCommon) -> bool {
    !VarModule::is_flag(fighter.module_accessor, marth::status::flag::DISABLE_STANCE_CHANGE)
}

unsafe extern "C" fn marth_stance_toggle_handler(fighter: &mut L2CFighterCommon, stance: bool) {
    VarModule::set_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE, !stance);
    macros::EFFECT(fighter, Hash40::new("marth_counter_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
    if stance {
        macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
    }
    else {
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 1.0);
    }
    VarModule::set_int(fighter.module_accessor, marth::instance::int::STANCE_CHANGE_LOCKOUT, vl::param_stance::stance_change_lockout_frame);
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW
    );
}

unsafe extern "C" fn marth_check_stance(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        VarModule::off_flag(fighter.module_accessor, marth::instance::flag::AIR_STANCE);
    }

    if VarModule::get_int(fighter.module_accessor, marth::instance::int::STANCE_CHANGE_LOCKOUT) <= 0 {
        let stance = VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE);
        let mut change = false;
        if !stance {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            && marth_allow_stance_toggle(fighter) {
                change = true;
            }
        }
        else {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            || (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH))
            && marth_allow_stance_toggle(fighter) {
                change = true;
            }
        }
        if change {
            marth_stance_toggle_handler(fighter, stance);
        }
    }
    if VarModule::get_int(fighter.module_accessor, marth::instance::int::STANCE_CHANGE_LOCKOUT) > 0 {
        VarModule::dec_int(fighter.module_accessor, marth::instance::int::STANCE_CHANGE_LOCKOUT);
    }
}

unsafe extern "C" fn marth_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    marth_check_stance(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, marth_frame);
}