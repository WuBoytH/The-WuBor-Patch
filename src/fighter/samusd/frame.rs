use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Vector3f,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
    
        // Morph Ball Drop Bounce
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") {
            if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
            && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUSD_STATUS_SPECIAL_LW_FLAG_BOUNCE) {
                MotionModule::set_frame_sync_anim_cmd(
                    fighter.module_accessor,
                    44.0,
                    true,
                    true,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                macros::SET_SPEED_EX(fighter, speed_x, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0,y: 0.5,z: 0.0});
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUSD_STATUS_SPECIAL_LW_FLAG_BOUNCE);
            }
        }

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_N_FLOAT) {
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_N_FLOAT);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        samusd_frame
    );
}