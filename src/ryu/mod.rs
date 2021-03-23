use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
// use smash_script::macros;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::app::lua_bind::EffectModule;
use crate::IS_FUNNY;

static mut SPECIAL_LW : [bool; 8] = [false; 8];
static mut CANCEL : [bool; 8] = [false; 8];
static mut EX_FLASH : [bool; 8] = [false; 8];
pub static mut SECRET_SENSATION : [bool; 8] = [false; 8];
static mut CAMERA : [bool; 8] = [false; 8];
pub static mut OPPONENT_POS : [Vector2f; 8] = [Vector2f{ x: 0.0, y: 0.0}; 8];
pub static mut OPPONENT_GA : [i32; 8] = [0; 8];
static mut RYU_POS : [Vector2f; 8] = [Vector2f{ x: 0.0, y: 0.0}; 8];
static mut SPECIAL_LW_TIMER : [i16; 8] = [-1; 8];
static mut FLASH_TIMER : [i16; 8] = [-1; 8];
static mut SEC_SEN_TIMER : [f32; 8] = [-0.4; 8];
static mut OPPONENT_DIRECTION : [f32; 8] = [12.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
unsafe fn ryu_frame(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id < 8 {

        if IS_FUNNY[entry_id] {
            if (MotionModule::motion_kind(boma) == smash::hash40("appeal_hi_r")
            || MotionModule::motion_kind(boma) == smash::hash40("appeal_hi_l"))
            && MotionModule::frame(boma) <= 30.0 {
                DamageModule::set_damage_mul(boma, 0.000001);
            }
            else {
                DamageModule::set_damage_mul(boma, 1.0);
            }
        }

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
            if CAMERA[entry_id] == false {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_RESET);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 1.1, 0.0, 0.0);
                macros::SLOW_OPPONENT(fighter, 100.0, 32.0);
                SlowModule::set_whole(boma, 4, 0);
                JostleModule::set_status(boma, true);
                // Also use MotionModule::change_motion
                MotionModule::change_motion_inherit_frame_keep_rate(boma,Hash40::new("turn"),0.0,0.0,0.0);
                RYU_POS[entry_id] = PostureModule::pos_2d(boma);
                if RYU_POS[entry_id].x < OPPONENT_POS[entry_id].x {
                    OPPONENT_DIRECTION[entry_id] = 12.0;
                }
                else {
                    OPPONENT_DIRECTION[entry_id] = -12.0;
                }
                CAMERA[entry_id] = true;
            }
            if SEC_SEN_TIMER[entry_id] >= 0.0 {
                if StatusModule::situation_kind(boma) != OPPONENT_GA[entry_id] {
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(OPPONENT_GA[entry_id]), true);
                }
                PostureModule::set_pos_2d(boma, &Vector2f{
                    x: (((OPPONENT_POS[entry_id].x + OPPONENT_DIRECTION[entry_id]) * SEC_SEN_TIMER[entry_id]) + RYU_POS[entry_id].x * (1.0 - SEC_SEN_TIMER[entry_id])),
                    y: (((OPPONENT_POS[entry_id].y + 12.0) * SEC_SEN_TIMER[entry_id]) + RYU_POS[entry_id].y * (1.0 - SEC_SEN_TIMER[entry_id]))
                });
            }
            SEC_SEN_TIMER[entry_id] += 0.1;
            if SEC_SEN_TIMER[entry_id] > 1.0 {
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                    PostureModule::reverse_lr(boma);
                }
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                macros::CAM_ZOOM_OUT(fighter);
                SlowModule::clear_whole(boma);
                JostleModule::set_status(boma, false);
                SEC_SEN_TIMER[entry_id] = -0.4;
                SECRET_SENSATION[entry_id] = false;
                CAMERA[entry_id] = false;
            }
        }
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        ryu_frame
    );
}