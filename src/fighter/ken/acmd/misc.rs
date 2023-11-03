use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ken", script = "game_run", category = ACMD_GAME, low_priority )]
unsafe fn ken_run(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if VarModule::get_int(agent.module_accessor, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
            macros::FT_MOTION_RATE(agent, 0.7);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if VarModule::get_int(agent.module_accessor, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_LW_STEP_KICK);
        }
    }
    frame(agent.lua_state_agent, 31.0);
    if VarModule::get_int(agent.module_accessor, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ken_run
    );
}