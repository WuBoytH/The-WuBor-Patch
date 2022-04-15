use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::{
        wua_bind::*,
        table_const::*
    },
    super::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_MARTH )]
fn marth_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && !MiscModule::is_damage_check(fighter.module_accessor, false)
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_STANCE_CHANGE_LOCKOUT) <= 0 {
            let stance = WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
            WorkModule::set_flag(fighter.module_accessor, !stance, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
            macros::EFFECT(fighter, Hash40::new("marth_counter_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
            if !stance {
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
            }
            else {
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 1.0);
            }
            WorkModule::set_int(fighter.module_accessor, 10, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_STANCE_CHANGE_LOCKOUT);
            ControlModule::clear_command_one(
                fighter.module_accessor,
                *FIGHTER_PAD_COMMAND_CATEGORY1,
                *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW
            );
        }
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE)
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_STANCE_CHANGE_LOCKOUT) <= 0 {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
                macros::EFFECT(fighter, Hash40::new("marth_counter_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.2, 1.0);
                WorkModule::set_int(fighter.module_accessor, 10, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_STANCE_CHANGE_LOCKOUT);
            }
        }
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_STANCE_CHANGE_LOCKOUT) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_INT_STANCE_CHANGE_LOCKOUT);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        marth_frame
    );
}