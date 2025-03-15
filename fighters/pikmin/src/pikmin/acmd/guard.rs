use super::*;

unsafe extern "C" fn game_guardcancelattackjump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let variation = WorkModule::get_int(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        if variation == *WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -0.4, y: 0.0, z: 0.0});
        }
        let game_params = pikmin_game_helper(agent, 8.0, 4.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("head1"), game_params.damage, 361, 0, 0, 80, game_params.hitbox_size, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -10, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, game_params.hit_effect, *ATTACK_SOUND_LEVEL_L, game_params.sound_attr, *ATTACK_REGION_PIKMIN);
    }
    wait(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_guardcancelattackjump(agent: &mut L2CAgentBase) {
    let effect_params = pikmin_effect_helper(agent);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pikmin_smash_trail"), Hash40::new("top"), 0, 4, 3, 0, 0, 0, 1, true);
        if let Some(trail) = effect_params.trail {
            macros::LAST_EFFECT_SET_COLOR(agent, trail.r, trail.g, trail.b);
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("pikmin_attack_flash"), Hash40::new("top"), 0, 4, 1.6, 0, 0, 0, 1, true);
        if let Some(trail) = effect_params.trail {
            macros::LAST_EFFECT_SET_COLOR(agent, trail.r, trail.g, trail.b);
        }
    }
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.8, 1, 0.5, 0);
            macros::FLASH_FRM(agent, 2, 0.8, 1, 0.5, effect_params.flash);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 2, 0.8, 1, 0.5, 0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
    }
}

unsafe extern "C" fn sound_guardcancelattackjump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s02"));
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_smash_s04"));
    }
}

pub fn install(agent: &mut Agent) {
    pikmin_acmd(agent, "game_guardcancelattackjump", game_guardcancelattackjump);
    pikmin_acmd(agent, "effect_guardcancelattackjump", effect_guardcancelattackjump);
    pikmin_acmd(agent, "sound_guardcancelattackjump", sound_guardcancelattackjump);
}