use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::{lua_const::*, *}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_GANON, main )]
fn ganon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_N)
        && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
            VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_N);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW
        && !CatchModule::is_catch(fighter.module_accessor)
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ganon_frame
    );
}