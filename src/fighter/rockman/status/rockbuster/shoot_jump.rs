use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "rockman", status = FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn rockman_rockbuster_shoot_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    rockman_rockbuster_main_helper(fighter, true.into(), false.into(), L2CValue::Void(), L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_rockbuster_shoot_jump_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_rockbuster_shoot_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = rockman_rockbuster_main_loop_helper(fighter, true.into(), false.into()).get_bool();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if helper_ret {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if sit == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        rockman_rockbuster_shoot_jump_main
    );
}