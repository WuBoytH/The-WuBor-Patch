use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "rockman", status = FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn rockman_rockbuster_shoot_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    rockman_rockbuster_main_helper(fighter, true.into(), false.into(), L2CValue::Void(), L2CValue::Void());
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_rockbuster_shoot_landing_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_rockbuster_shoot_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = rockman_rockbuster_main_loop_helper(fighter, true.into(), false.into()).get_bool();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if sit == *SITUATION_KIND_AIR {
        let status = if helper_ret {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    if fighter.sub_check_button_jump().get_bool()
    || fighter.sub_check_button_frick().get_bool() {
        let frame = MotionModule::frame(fighter.module_accessor);
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
        if landing_frame < frame {
            let status = if helper_ret {
                FIGHTER_STATUS_KIND_JUMP_SQUAT
            }
            else {
                FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT
            };
            fighter.change_status(status.into(), true.into());
            return 1.into()
        }
    }
    let status = if helper_ret {
        FIGHTER_STATUS_KIND_WAIT
    }
    else {
        FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT
    };
    fighter.change_status(status.into(), false.into());
    1.into()
}

pub fn install() {
    install_status_scripts!(
        rockman_rockbuster_shoot_landing_main
    );
}