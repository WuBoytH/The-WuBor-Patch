use smash::{
    lua2cpp::{L2CFighterCommon, L2CAgentBase},
    phx::Hash40,
    app::{lua_bind::*/*, sv_animcmd::*, **/},
    lib::{lua_const::*, L2CValue}
};
use smash_script::*;
use smashline::*;
use crate::{
    commonfuncs::*,
    globals::*,
    vars::*
};

#[status_script(agent = "purin", status = FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HIT_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn purin_specialnhitend(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hit_end"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_PURIN_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE);
    fighter.sub_shift_status_main(L2CValue::Ptr(purin_specialnhitendmain as *const () as _))
}

unsafe extern "C" fn purin_specialnhitendmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_air_check_fall_common();
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[acmd_script( agent = "purin", script = "game_shieldbreakfly", category = ACMD_GAME, low_priority )]
unsafe fn purin_shieldbreak(fighter: &mut L2CAgentBase) {
    if IS_FUNNY[entry_id(fighter.module_accessor)] {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 1000, 500, 1000, 50.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
    }
}

pub fn install() {
    install_status_scripts!(
        purin_specialnhitend
    );
    install_acmd_scripts!(
        purin_shieldbreak
    );
}