use smash::hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::{L2CFighterCommon/*, L2CFighterBase*/};
// use smash_script::*;
use smashline::*;
use crate::commonfuncs::*;
use smash::app::*;
use smash::lib::L2CValue;
use crate::globals::*;
use crate::vars::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn common_status_damagefall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_DamageFall_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 1.0, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(common_status_damagefall_main as *const () as _))
}

unsafe extern "C" fn common_status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == true
    || fighter.check_damage_fall_transition().get_bool() == true {
        return L2CValue::I32(0);
    }
    let tech : bool;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND) == false {
        tech = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    }
    else {
        // let mut flame_choke_tech_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_passive_trigger_frame")) as f32;
        // let tech_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0 as u64);
        // flame_choke_tech_frame *= tech_mul;
        tech = fighter.sub_check_passive_button(L2CValue::I32(0x30)).get_bool();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB) {
        if FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value")) <= fighter.global_table[STICK_X].get_f32().abs() {
                if tech {
                    fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
                    return L2CValue::Bool(true);
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE) {
        if FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) == false {
                if tech {
                    fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
                    return L2CValue::Bool(true);
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), true.into());
            return L2CValue::I32(0);
        }
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21status_DamageAir_MainEv")]
unsafe fn damage_air_main(fighter: &mut L2CFighterCommon) {
    ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    original!()(fighter);
}

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);

        if utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_DONKEY {
            IS_DK[entry_id(fighter.module_accessor)] = true;
        }

        // The code to set up Funny Mode.

        if ItemModule::is_attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_USAGIHAT))
        && IS_FUNNY[entry_id(fighter.module_accessor)] == false {
            IS_FUNNY[entry_id(fighter.module_accessor)] = true;
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP);
        }
        if (status == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false)
        && IS_FUNNY[entry_id(fighter.module_accessor)] {
            IS_FUNNY[entry_id(fighter.module_accessor)] = false;
        }

        // The code to set up FGC Mode.

        if FighterUtil::is_hp_mode(fighter.module_accessor) {
            IS_FGC[entry_id(fighter.module_accessor)] = true;
        }
        else {
            IS_FGC[entry_id(fighter.module_accessor)] = false;
        }

        // Remove an OPPONENT_BOMA if the opponent is dead.

        if entry_id(fighter.module_accessor) < 8 {
            if OPPONENT_BOMA[entry_id(fighter.module_accessor)] != 0 {
                if StatusModule::status_kind(OPPONENT_BOMA[entry_id(fighter.module_accessor)] as *mut BattleObjectModuleAccessor) == *FIGHTER_STATUS_KIND_DEAD {
                    OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
                }
            }
        }
        
        // Platform Dropping while in shield.

        if (status == *FIGHTER_STATUS_KIND_GUARD
        || status == *FIGHTER_STATUS_KIND_GUARD_ON)
        && ControlModule::get_stick_y(fighter.module_accessor) < -0.8
        && GroundModule::is_passable_ground(fighter.module_accessor) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
        }

        // Whifflag???

        if [
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F
        ].contains(&status) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                AIR_WHIFF[entry_id(fighter.module_accessor)] = false;
            }
            else if AttackModule::is_attack(fighter.module_accessor, 0, false) {
                AIR_WHIFF[entry_id(fighter.module_accessor)] = true;
            }
        }
        else {
            AIR_WHIFF[entry_id(fighter.module_accessor)] = false;
        }
        
        // Command Inputs

        let dir = get_command_stick_direction(fighter.module_accessor, true);
        if INPUT_TIMER[entry_id(fighter.module_accessor)] <= 10 {
            INPUT_TIMER[entry_id(fighter.module_accessor)] += 1;
        }
        else {
            QCF[entry_id(fighter.module_accessor)] = 0;
            QCB[entry_id(fighter.module_accessor)] = 0;
        }

        // Quarter Circle Back

        if QCB[entry_id(fighter.module_accessor)] == 0 {
            if dir == 2 {
                QCB[entry_id(fighter.module_accessor)] = 1;
                INPUT_TIMER[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCB[entry_id(fighter.module_accessor)] == 1 {
            if dir == 1 {
                QCB[entry_id(fighter.module_accessor)] = 2;
            }
            else if dir != 4
            && dir != 1
            && dir != 2 {
                QCB[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCB[entry_id(fighter.module_accessor)] == 2 {
            if dir == 4 {
                QCB[entry_id(fighter.module_accessor)] = 3;
            }
            else if dir != 4
            && dir != 1
            && dir != 7 {
                QCB[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else {
            if dir != 4
            && dir != 7 {
                QCB[entry_id(fighter.module_accessor)] = 0;
            }
        }

        // Quarter Circle Forward

        if QCF[entry_id(fighter.module_accessor)] == 0 {
            if dir == 2 {
                QCF[entry_id(fighter.module_accessor)] = 1;
                INPUT_TIMER[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCF[entry_id(fighter.module_accessor)] == 1 {
            if dir == 1 {
                QCF[entry_id(fighter.module_accessor)] = 2;
            }
            else if dir != 6
            && dir != 3
            && dir != 2 {
                QCF[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCF[entry_id(fighter.module_accessor)] == 2 {
            if dir == 6 {
                QCF[entry_id(fighter.module_accessor)] = 3;
            }
            else if dir != 6
            && dir != 3
            && dir != 9 {
                QCF[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else {
            if dir != 6
            && dir != 9 {
                QCF[entry_id(fighter.module_accessor)] = 0;
            }
        }

        // The Counter-Hit Code (only applicable to Jabs, Tilts, and Smash Attacks)

        if [
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_LANDING,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER,
            *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_S3,
            *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_LW3,
        ].contains(&status) {
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 1;
        }
        else {
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 0;
        }
    }
}

// Use this for general per-frame weapon-level hooks
//pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
//    unsafe {
//        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;
//
//        if frame % 10 == 0 {
//            println!("[Weapon Hook] Frame : {}", frame);
//        }
//    }
//}

pub fn install() {
    // skyline::install_hook!(damage_air_main);
    smashline::install_hook!(
        damage_air_main
    );
    smashline::install_status_scripts!(
        common_status_damagefall
    );
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
//    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}