use crate::imports::acmd_imports::*;

unsafe extern "C" fn ryu_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if DamageModule::damage(agent.module_accessor, 0) >= 180.0 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_6c_aura"));
            VarModule::on_flag(agent.module_accessor, ryu::instance::flag::SEC_SEN_STATE);
            VarModule::on_flag(agent.module_accessor, ryu::instance::flag::EX_FLASH);
            VarModule::set_int(agent.module_accessor, ryu::instance::int::FLASH_TIMER, 0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
            macros::BURN_COLOR(agent, 0.0, 0.55, 1.0, 0.7);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            DamageModule::set_damage_lock(agent.module_accessor, true);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if VarModule::is_flag(agent.module_accessor, ryu::instance::flag::SEC_SEN_STATE) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, true);
            macros::BURN_COLOR_NORMAL(agent);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            DamageModule::set_damage_lock(agent.module_accessor, false);
            VarModule::off_flag(agent.module_accessor, ryu::instance::flag::EX_FLASH);
            macros::COL_NORMAL(agent);
            VarModule::off_flag(agent.module_accessor, ryu::instance::flag::SEC_SEN_STATE);
            HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ryu_appealhi
    );
}