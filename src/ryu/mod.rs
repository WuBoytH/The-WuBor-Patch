use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
// use smash_script::macros;
use smash::phx::Vector3f;
use smash::app::lua_bind::EffectModule;

static mut SPECIAL_LW : [bool; 8] = [false; 8];
static mut CANCEL : [bool; 8] = [false; 8];
static mut EX_FLASH : [bool; 8] = [false; 8];
static mut SPECIAL_LW_TIMER : [i16; 8] = [-1; 8];

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
unsafe fn ryu_frame(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id < 8 {

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
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ControlModule::get_stick_y(boma) < -0.5 {
                if MotionModule::motion_kind(boma) == smash::hash40("special_n")
                && MotionModule::frame(boma) > 13.0 && CANCEL[entry_id] == false {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                    CANCEL[entry_id] = true;
                }
                if MotionModule::motion_kind(boma) == smash::hash40("special_s_start") || MotionModule::motion_kind(boma) == smash::hash40("special_s") {
                    if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
                    && CANCEL[entry_id] == false {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                        CANCEL[entry_id] = true;
                    }
                }
                if MotionModule::motion_kind(boma) == smash::hash40("special_hi") || MotionModule::motion_kind(boma) == smash::hash40("special_hi_command") {
                    if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
                    && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
                    && CANCEL[entry_id] == false {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                        CANCEL[entry_id] = true;
                    }
                }
            }
        }
        
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW && CANCEL[entry_id] == true {
            EX_FLASH[entry_id] = true;
            SPECIAL_LW_TIMER[entry_id] = 1200;
            SPECIAL_LW[entry_id] = true;
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

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && EX_FLASH[entry_id] == true {
            if MotionModule::frame(boma) <= 8.0 {
                macros::FLASH(fighter, 1, 1, 0.0, 1.0);
            }
            else {
                macros::COL_NORMAL(fighter);
            }
        }

        if StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_LW
        && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_N
        && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_HI
        && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_S {
            EX_FLASH[entry_id] = false;
        }
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        ryu_frame
    );
}