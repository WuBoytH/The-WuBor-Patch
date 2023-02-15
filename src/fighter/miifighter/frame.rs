use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::{lua_const::*, *}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{cancels::*, vars::*, table_const::*}
};

unsafe fn miifighter_fgc_exception(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END
    && VarModule::is_flag(fighter.battle_object, fighter::instance::flag::IS_FGC)
    && MotionModule::frame(fighter.module_accessor) > 27.0 {
        let situation = fighter.global_table[SITUATION_KIND].get_i32();
        if special_cancel_common(fighter, situation.into(), [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec()).get_bool() {
            return;
        }
        jump_cancel_common(fighter, situation.into());
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER, main )]
fn miifighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        miifighter_fgc_exception(fighter);
    }
}

pub fn install() {
    install_agent_frames!(
        miifighter_frame
    );
}