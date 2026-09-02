use super::*;

unsafe extern "C" fn special_hi_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("control_frame"));
    WorkModule::set_int(fighter.module_accessor, control_frame, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_INT_COUNTER);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        let mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, mul);
    }

    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi")
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            1.4
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.5 * lr,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_init);
}
