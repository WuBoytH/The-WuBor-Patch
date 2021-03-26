use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::app::lua_bind::EffectModule;
use crate::IS_FUNNY;

static mut _TIME_COUNTER: [i32; 8] = [0; 8];

static mut SPECIAL_LW : [bool; 8] = [false; 8];
static mut CANCEL : [bool; 8] = [false; 8];
static mut EX_FLASH : [bool; 8] = [false; 8];
pub static mut SECRET_SENSATION : [bool; 8] = [false; 8];
pub static mut CAMERA : [bool; 8] = [false; 8];
pub static mut OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_BOMA : [u64; 8] = [0; 8];
static mut RYU_X : [f32; 8] = [0.0; 8];
static mut RYU_Y : [f32; 8] = [0.0; 8];
static mut SPECIAL_LW_TIMER : [i16; 8] = [-1; 8];
static mut FLASH_TIMER : [i16; 8] = [-1; 8];
static mut SEC_SEN_TIMER : [f32; 8] = [-0.4; 8];
static mut OPPONENT_DIRECTION : [f32; 8] = [12.0; 8];
static mut VERT_EXTRA : [f32; 8] = [12.0; 8];
static mut SEC_SEN_STATE : [bool; 8] = [false; 8];
static mut SEC_SEN_DIREC : [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
unsafe fn ryu_frame(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id < 8 {

        // Secret Sensation Prep Code

        // if IS_FUNNY[entry_id] {
            if (MotionModule::motion_kind(boma) == smash::hash40("appeal_hi_r")
            || MotionModule::motion_kind(boma) == smash::hash40("appeal_hi_l"))
            && MotionModule::frame(boma) <= 30.0 {
                SEC_SEN_STATE[entry_id] = true;
            }
            else if SECRET_SENSATION[entry_id] {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
                DamageModule::set_damage_lock(boma, true);
                DamageModule::set_no_reaction_no_effect(boma, true);
                HitModule::set_hit_stop_mul(boma, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
            }
            else if SECRET_SENSATION[entry_id] == false
            && SEC_SEN_STATE[entry_id] {
                DamageModule::set_damage_lock(boma, false);
                DamageModule::set_no_reaction_no_effect(boma, false);
                HitModule::set_hit_stop_mul(boma, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                SEC_SEN_STATE[entry_id] = false;
                macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            }

            if SEC_SEN_STATE[entry_id] {
                DamageModule::set_damage_lock(boma, true);
                DamageModule::set_no_reaction_no_effect(boma, true);
                HitModule::set_hit_stop_mul(boma, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
            }
        // }

        // Jump Cancel Heavy Up-Tilt

        if MotionModule::motion_kind(boma) == smash::hash40("attack_hi3_s") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }

        // Reset Vars

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            SPECIAL_LW[entry_id] = false;
            CANCEL[entry_id] = false;
            EX_FLASH[entry_id] = false;
            SPECIAL_LW_TIMER[entry_id] = -1;
            SECRET_SENSATION[entry_id] = false;
            SEC_SEN_STATE[entry_id] = false;
        }

        // EX Focus Attack Check
        if SPECIAL_LW[entry_id] == false {
            if (MotionModule::motion_kind(boma) == smash::hash40("special_n")
            && MotionModule::frame(boma) > 13.0)
            || (MotionModule::motion_kind(boma) == smash::hash40("special_s_start") || MotionModule::motion_kind(boma) == smash::hash40("special_s")
            && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)))
            || (MotionModule::motion_kind(boma) == smash::hash40("special_hi") || MotionModule::motion_kind(boma) == smash::hash40("special_hi_command")
            && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
            && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND)
            && CANCEL[entry_id] == false {
                CANCEL[entry_id] = true;
            }
            else if CANCEL[entry_id]
            && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    CANCEL[entry_id] = false;
            }
        }

        if ControlModule::get_command_flag_cat(boma,0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && CANCEL[entry_id] {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
        }

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && CANCEL[entry_id] {
            EX_FLASH[entry_id] = true;
            if !IS_FUNNY[entry_id] {
                SPECIAL_LW_TIMER[entry_id] = 1200;
                SPECIAL_LW[entry_id] = true;
            }
            CANCEL[entry_id] = false;
        }

        if SPECIAL_LW_TIMER[entry_id] > 0 {
            SPECIAL_LW_TIMER[entry_id] = SPECIAL_LW_TIMER[entry_id] - 1;
        }
        else if SPECIAL_LW_TIMER[entry_id] == 0 {
            SPECIAL_LW_TIMER[entry_id] = -1;
            let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
            let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
            let focuseff: u32 = EffectModule::req_follow(boma, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rgb(boma, focuseff, 0.0, 0.0, 0.0);
            SPECIAL_LW[entry_id] = false;
        }

        // EX Flash I Hope

        if EX_FLASH[entry_id] == true {
            if FLASH_TIMER[entry_id] < 0 {
                FLASH_TIMER[entry_id] = 12;
            }
            else if FLASH_TIMER[entry_id] == 0 {
                macros::COL_NORMAL(fighter);
                EX_FLASH[entry_id] = false;
                FLASH_TIMER[entry_id] -= 1;
            }
            else {
                macros::FLASH(fighter, 1, 1, 0.0, 1.0);
                FLASH_TIMER[entry_id] -= 1;
            }
        }

        // Secret Sensation???

        if SECRET_SENSATION[entry_id] {
            JostleModule::set_status(boma, false);
            if CAMERA[entry_id] == false {
                macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 1.5, 0.0, 0.0);
                macros::SLOW_OPPONENT(fighter, 100.0, 32.0);
                SlowModule::set_whole(boma, 4, 0);
                macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                RYU_X[entry_id] = PostureModule::pos_x(boma);
                RYU_Y[entry_id] = PostureModule::pos_y(boma);
                if RYU_X[entry_id] == OPPONENT_X[entry_id] {
                    OPPONENT_DIRECTION[entry_id] = -12.0 * PostureModule::lr(boma);
                    SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B;
                }
                else if RYU_X[entry_id] < OPPONENT_X[entry_id] {
                    OPPONENT_DIRECTION[entry_id] = 12.0;
                    if PostureModule::lr(boma) == -1.0 {
                        PostureModule::reverse_lr(boma);
                    }
                    SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                }
                else {
                    OPPONENT_DIRECTION[entry_id] = -12.0;
                    if PostureModule::lr(boma) == 1.0 {
                        PostureModule::reverse_lr(boma);
                    }
                    SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                }
                println!("Difference between Ys: {}", (RYU_Y[entry_id] - OPPONENT_Y[entry_id]).abs());
                if (RYU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() <= 12.0
                && StatusModule::situation_kind(OPPONENT_BOMA[entry_id] as *mut smash::app::BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                    VERT_EXTRA[entry_id] = 0.0;
                }
                else {
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                    VERT_EXTRA[entry_id] = 12.0;
                    RYU_Y[entry_id] += 2.0;
                    PostureModule::add_pos_2d(boma, &Vector2f{
                        x: 0.0,
                        y: 2.0
                    });
                }
                CAMERA[entry_id] = true;
            }
            if (RYU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() <= 8.0
            && StatusModule::situation_kind(OPPONENT_BOMA[entry_id] as *mut smash::app::BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            if StatusModule::status_kind(boma) != SEC_SEN_DIREC[entry_id] {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_RESET);
                StatusModule::change_status_request_from_script(boma, SEC_SEN_DIREC[entry_id], true);
            }
            if SEC_SEN_TIMER[entry_id] >= 0.0 {
                if RYU_Y[entry_id] != OPPONENT_Y[entry_id]{
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                }
                PostureModule::set_pos_2d(boma, &Vector2f{
                    x: (((OPPONENT_X[entry_id] + OPPONENT_DIRECTION[entry_id]) * SEC_SEN_TIMER[entry_id]) + RYU_X[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id])),
                    y: (((OPPONENT_Y[entry_id] + VERT_EXTRA[entry_id]) * SEC_SEN_TIMER[entry_id]) + RYU_Y[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id]))
                });
            }
            SEC_SEN_TIMER[entry_id] += 0.1;
            if SEC_SEN_TIMER[entry_id] > 1.0 {
                SECRET_SENSATION[entry_id] = false;
                CAMERA[entry_id] = false;
                WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_TURN_AUTO, true);
                }
                macros::CAM_ZOOM_OUT(fighter);
                macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
                SlowModule::clear_whole(boma);
                JostleModule::set_status(boma, true);
                SEC_SEN_TIMER[entry_id] = -0.4;
            }
        }
    }
}

#[script( agent = "ryu", scripts = [ "game_speciallwstepf", "game_speciallwstepb" ], category = ACMD_GAME )]
unsafe fn ryu_dspecialstep(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SECRET_SENSATION[entry_id] == false {
        if macros::is_excute(fighter) {
            smash_script::macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 10.0, 0.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_BODY_HEAD, false);
        }
        sv_animcmd::frame(lua_state, 15.0);
        if macros::is_excute(fighter) {
            smash_script::search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        }
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        ryu_frame
    );
    smash_script::replace_scripts!(
        ryu_dspecialstep
    );
}