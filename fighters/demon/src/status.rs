use super::*;

pub unsafe extern "C" fn demon_attack_loop_common(fighter: &mut L2CFighterCommon, status: L2CValue) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(status, false.into());
        return true.into();
    }

    false.into()
}

mod wait;

mod walk;

mod dash;

mod squat_wait;

mod landing;

mod attack;

mod attack_s3;
mod attack_lw3;

mod attack_stand_2;

mod attack_squat_2;

mod attack_step_2s;

mod landing_attack_air;

mod down;

pub fn install(agent: &mut Agent) {
    wait::install(agent);

    walk::install(agent);

    dash::install(agent);

    squat_wait::install(agent);

    landing::install(agent);

    attack::install(agent);

    attack_s3::install(agent);
    attack_lw3::install(agent);

    attack_stand_2::install(agent);

    attack_squat_2::install(agent);

    attack_step_2s::install(agent);

    landing_attack_air::install(agent);

    down::install(agent);
}