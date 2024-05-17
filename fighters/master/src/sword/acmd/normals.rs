use super::*;

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 6.0, *WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLOAT_2ND_GRAVITY);
        WorkModule::set_float(agent.module_accessor, 0.0, *WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLOAT_2ND_AIR_RESISTANCE);
    }
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1);
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLAG_PHYSICS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLAG_PHYSICS);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
}