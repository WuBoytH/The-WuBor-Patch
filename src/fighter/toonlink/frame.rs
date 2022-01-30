use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Down Air Bounce

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw"){
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_ATTACK_AIR_LW_FLAG_BOUNCE);
            }
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_ATTACK_AIR_LW_FLAG_BOUNCE) {
                macros::SET_SPEED_EX(fighter, 0.0, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::suspend_energy_all(fighter.module_accessor);
                if !fighter.global_table[IN_HITLAG].get_bool()
                && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
                && MotionModule::frame(fighter.module_accessor) < 65.0 {
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 65.0, true, true, false);
                }
                else if MotionModule::frame(fighter.module_accessor) > 66.0 {
                    KineticModule::resume_energy_all(fighter.module_accessor);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_ATTACK_AIR_LW_FLAG_BOUNCE);
                    MotionModule::set_rate(fighter.module_accessor, 0.4);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        toonlink_frame
    );
}