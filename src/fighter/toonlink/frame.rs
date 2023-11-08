use {
    crate::imports::status_imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn toonlink_attack_air_lw_bounce(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.module_accessor, toonlink::status::flag::ATTACK_AIR_LW_BOUNCE);
        }
        if VarModule::is_flag(fighter.module_accessor, toonlink::status::flag::ATTACK_AIR_LW_BOUNCE) {
            macros::SET_SPEED_EX(fighter, 0.0, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            KineticModule::suspend_energy_all(fighter.module_accessor);
            if !fighter.global_table[IS_STOP].get_bool()
            && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
            && MotionModule::frame(fighter.module_accessor) < 65.0 {
                MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 65.0, true, true, false);
            }
            else if MotionModule::frame(fighter.module_accessor) > 66.0 {
                KineticModule::resume_energy_all(fighter.module_accessor);
                VarModule::off_flag(fighter.module_accessor, toonlink::status::flag::ATTACK_AIR_LW_BOUNCE);
                MotionModule::set_rate(fighter.module_accessor, 0.4);
            }
        }
    }
}

unsafe extern "C" fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    toonlink_attack_air_lw_bounce(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, toonlink_frame);
}