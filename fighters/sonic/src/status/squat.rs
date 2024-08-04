use super::*;

unsafe extern "C" fn sonic_squat_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STICK_Y].get_f32() > 0.0 {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, vars::sonic::status::LOOK_UP);
        return 1.into();
    }
    fighter.status_pre_Squat()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SQUAT, sonic_squat_pre);
}