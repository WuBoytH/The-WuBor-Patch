use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ryu", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_appealhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if DamageModule::damage(fighter.module_accessor, 0) >= 180.0 {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_aura"));
            VarModule::on_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_STATE);
            VarModule::on_flag(fighter.battle_object, ryu::instance::flag::EX_FLASH);
            VarModule::set_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER, 0);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
            macros::BURN_COLOR(fighter, 0.0, 0.55, 1.0, 0.7);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            DamageModule::set_damage_lock(fighter.module_accessor, true);
        }
    }
    frame(fighter.lua_state_agent, 30.0);
    if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_STATE) {
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ryu_savingattack_aura"), false, true);
            macros::BURN_COLOR_NORMAL(fighter);
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            DamageModule::set_damage_lock(fighter.module_accessor, false);
            VarModule::off_flag(fighter.battle_object, ryu::instance::flag::EX_FLASH);
            macros::COL_NORMAL(fighter);
            VarModule::off_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_STATE);
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ryu_appealhi
    );
}