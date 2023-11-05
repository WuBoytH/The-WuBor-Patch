use crate::imports::acmd_imports::*;
use super::super::super::{helper::*, vl, vtable_hook::*};

#[acmd_script( agent = "dolly_burst", script = "game_superspecial", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn dolly_burst_superspecial(agent: &mut L2CAgentBase) {
    let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let owner_module_accessor = sv_battle_object::module_accessor(otarget_id);
    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *oboma) == *FIGHTER_KIND_DOLLY
    && VarModule::is_flag(owner_module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL) {
        VarModule::on_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL);
    }
    if macros::is_excute(agent) {
        let mut damage = 26.0;
        let mut bkb = 100;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL) {
            damage *= vl::param_private::special_cancel_damage_mul;
            bkb = (bkb as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 70, 41, 0, bkb, 1.0, 0.0, 2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 70, 41, 0, bkb, 3.0, 0.0, 3.0, -8.0, Some(0.0), Some(3.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut damage1 = 26.0;
        let mut damage2 = 23.0;
        let mut damage3 = 20.0;
        let mut bkb = 100;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL) {
            damage1 *= vl::param_private::special_cancel_damage_mul;
            damage2 *= vl::param_private::special_cancel_damage_mul;
            damage3 *= vl::param_private::special_cancel_damage_mul;
            bkb = (bkb as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage1, 70, 41, 0, bkb, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(10.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage2, 70, 42, 0, bkb, 11.0, 0.0, 25.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), damage3, 70, 44, 0, bkb, 8.0, 0.0, 40.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let mut damage1 = 19.0;
        let mut damage2 = 17.0;
        let mut damage3 = 15.0;
        let mut bkb1 = 120;
        let mut bkb2 = 110;
        let mut bkb3 = 100;
        if VarModule::is_flag(agent.module_accessor, dolly_wave::instance::flag::FROM_CANCEL) {
            damage1 *= vl::param_private::special_cancel_damage_mul;
            damage2 *= vl::param_private::special_cancel_damage_mul;
            damage3 *= vl::param_private::special_cancel_damage_mul;
            bkb1 = (bkb1 as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
            bkb2 = (bkb2 as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
            bkb3 = (bkb3 as f32 * vl::param_private::special_cancel_bkb_mul) as i32;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage1, 70, 41, 0, bkb1, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(10.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage2, 70, 42, 0, bkb2, 11.0, 0.0, 25.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), damage3, 70, 44, 0, bkb3, 8.0, 0.0, 40.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_SUPERSPECIAL01, *ATTACK_REGION_ENERGY);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    burst.game_acmd("game_superspecial", dolly_burst_superspecial);
}