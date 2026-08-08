use super::*;

unsafe extern "C" fn effect_richterspecialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("richter_bottle_release"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_richterspecialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("richter_bottle_release"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_richterspecialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kirby_copy_richter_01"));
        macros::PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn expression_richterspecialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_richterspecialnblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("richter_bottle_blank"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_richterspecialnblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_richter_special_l04"));
    }
}

unsafe extern "C" fn expression_richterspecialnblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_richterspecialn", effect_richterspecialn, Priority::Low);
    agent.acmd("sound_richterspecialn", sound_richterspecialn, Priority::Low);
    agent.acmd("expression_richterspecialn", expression_richterspecialn, Priority::Low);

    agent.acmd("effect_richterspecialairn", effect_richterspecialairn, Priority::Low);
    agent.acmd("sound_richterspecialairn", sound_richterspecialn, Priority::Low);
    agent.acmd("expression_richterspecialairn", expression_richterspecialn, Priority::Low);

    agent.acmd("effect_richterspecialnblank", effect_richterspecialnblank, Priority::Low);
    agent.acmd("sound_richterspecialnblank", sound_richterspecialnblank, Priority::Low);
    agent.acmd("expression_richterspecialnblank", expression_richterspecialnblank, Priority::Low);

    agent.acmd("effect_richterspecialairnblank", effect_richterspecialnblank, Priority::Low);
    agent.acmd("sound_richterspecialairnblank", sound_richterspecialnblank, Priority::Low);
    agent.acmd("expression_richterspecialairnblank", expression_richterspecialnblank, Priority::Low);
}
