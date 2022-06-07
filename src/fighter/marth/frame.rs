use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::vl
};

unsafe fn marth_allow_stance_toggle(fighter: &mut L2CFighterCommon) -> bool {
    if VarModule::is_flag(fighter.battle_object, marth::status::flag::DISABLE_STANCE_CHANGE) {
        false
    }
    else {
        true
    }
}

unsafe fn marth_stance_toggle_handler(fighter: &mut L2CFighterCommon, stance: bool) {
    VarModule::set_flag(fighter.battle_object, marth::instance::flag::IS_STANCE, !stance);
    macros::EFFECT(fighter, Hash40::new("marth_counter_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
    if stance {
        macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
    }
    else {
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 1.0);
    }
    VarModule::set_int(fighter.battle_object, marth::instance::int::STANCE_CHANGE_LOCKOUT, vl::param_stance::stance_change_lockout_frame);
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW
    );
}

#[fighter_frame( agent = FIGHTER_KIND_MARTH )]
fn marth_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            VarModule::off_flag(fighter.battle_object, marth::instance::flag::AIR_STANCE);
        }

        if VarModule::get_int(fighter.battle_object, marth::instance::int::STANCE_CHANGE_LOCKOUT) <= 0 {
            let stance = VarModule::is_flag(fighter.battle_object, marth::instance::flag::IS_STANCE);
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
        if VarModule::get_int(fighter.battle_object, marth::instance::int::STANCE_CHANGE_LOCKOUT) > 0 {
            VarModule::dec_int(fighter.battle_object, marth::instance::int::STANCE_CHANGE_LOCKOUT);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        marth_frame
    );
}