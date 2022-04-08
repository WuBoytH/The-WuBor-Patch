use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_SZEROSUIT )]
fn szerosuit_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SZEROSUIT_STATUS_SUPER_JUMP_PUNCH_DECIDE_MOTION) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    let mot = MotionModule::motion_kind(fighter.module_accessor);
                    let mot2;
                    if mot == hash40("special_hi") {
                        mot2 = Hash40::new("special_hi_2");
                    }
                    else {
                        mot2 = Hash40::new("special_air_hi_2");
                    }
                    MotionModule::change_motion_inherit_frame(
                        fighter.module_accessor,
                        mot2,
                        -1.0,
                        1.0,
                        0.0,
                        false,
                        false
                    );
                }
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_SZEROSUIT_STATUS_SUPER_JUMP_PUNCH_DECIDE_MOTION);
            }
            if [
                hash40("special_hi_2"),
                hash40("special_air_hi_2")
            ].contains(&MotionModule::motion_kind(fighter.module_accessor))
            && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && MotionModule::frame(fighter.module_accessor) > 30.0 {
                WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME)
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        szerosuit_frame
    );
}