use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ken", script = "game_run", category = ACMD_GAME, low_priority )]
unsafe fn ken_run(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
            macros::FT_MOTION_RATE(fighter, 0.7);
        }
    }
    frame(fighter.lua_state_agent, 22.0);
    if VarModule::get_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.battle_object, ken::status::flag::SPECIAL_LW_STEP_KICK);
        }
    }
    frame(fighter.lua_state_agent, 31.0);
    if VarModule::get_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ken_run
    );
}