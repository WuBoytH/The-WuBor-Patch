use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn samusd_special_lw_bounce(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw")
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") {
        if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !VarModule::is_flag(fighter.module_accessor, samusd::status::flag::SPECIAL_LW_BOUNCE) {
            MotionModule::set_frame_sync_anim_cmd(
                fighter.module_accessor,
                44.0,
                true,
                true,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0,y: 0.5,z: 0.0});
            VarModule::on_flag(fighter.module_accessor, samusd::status::flag::SPECIAL_LW_BOUNCE);
        }
    }
}

unsafe extern "C" fn samusd_reset_attack_air_n_float(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.module_accessor, samusd::instance::flag::ATTACK_AIR_N_FLOAT) {
            VarModule::off_flag(fighter.module_accessor, samusd::instance::flag::ATTACK_AIR_N_FLOAT);
        }
    }
}

unsafe extern "C" fn samusd_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    samusd_special_lw_bounce(fighter);
    samusd_reset_attack_air_n_float(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, samusd_frame);
}