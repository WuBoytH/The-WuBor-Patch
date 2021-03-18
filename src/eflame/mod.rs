use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash_script::macros;
use crate::IS_FUNNY;

static mut CALLBACK : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
unsafe fn eflame_frame(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id < 8 {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_S
        && MotionModule::frame(boma) >= 20.0 {
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                WorkModule::enable_transition_term(boma,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
                WorkModule::enable_transition_term(boma,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::enable_transition_term(boma,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
            }
            if ControlModule::check_button_trigger(boma,*CONTROL_PAD_BUTTON_SPECIAL) == true {
                CALLBACK[entry_id] = true;
                StatusModule::change_status_request_from_script(boma,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_S_CATCH,true);
            }
        }

        if IS_FUNNY[entry_id] {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_EFLAME_ESWORD )]
unsafe fn eflame_esword_frame(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let o_entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if CALLBACK[o_entry_id] {
        StatusModule::change_status_request_from_script(boma,*WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_BACK,true);
        if StatusModule::status_kind(boma) == *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_BACK {
            CALLBACK[o_entry_id] = false;
        }
    }
}

#[script( agent = "eflame", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn eflame_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 1.333);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if ArticleModule::is_exist(boma, 17452) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(boma, 17452, 60296, Hash40::new_raw(0x07439e926b), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) == true {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(boma, 60300);
        }
    }
    sv_animcmd::frame(lua_state, 15.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 3.0, 0.0, 0.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -1.0, 7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -2.0, 11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -3.0, 14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 3.0, 0.0, 2.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 16.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 3.0, 0.0, -2.0, -2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -7.0, -1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -10.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -13.0, 1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 3.0, 0.0, 2.0, -3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 17.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 1.0, -11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -2.0, -13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -5.0, -14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -8.0, -15.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 4.0, -7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 3.0, 0.0, 6.0, -13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 2.4, 0.0, 5.0, -16.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 2.4, 0.0, 4.0, -19.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 2.4, 0.0, 3.0, -22.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        smash_script::macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 3.0, 0.0, 8.0, -8.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 26.0);
    if ArticleModule::is_exist(boma, 17452) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(boma, 17452, 60296, Hash40::new_raw(0x08183db0f4), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(boma) == true {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(boma, 60300);
        }
    }
    sv_animcmd::frame(lua_state, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    smash_script::replace_scripts!(
        eflame_dair
    );
    smash_script::replace_fighter_frames!(eflame_frame);
    smash_script::replace_weapon_frames!(eflame_esword_frame);
}