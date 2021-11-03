use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_b") {
            if MotionModule::frame(fighter.module_accessor) >= 10.0 && MotionModule::frame(fighter.module_accessor) < 57.0 {
                let stickx = ControlModule::get_stick_x(fighter.module_accessor);
                let lr = PostureModule::lr(fighter.module_accessor);
                macros::SET_SPEED_EX(fighter, 1.1 * lr * stickx, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) == 57.0 {
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            FINISH_SIGN[entry_id(fighter.module_accessor)] = 0;
        }

        // Wario can now move during his back throw.

        if (MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_l")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_r"))
        && MotionModule::frame(fighter.module_accessor) == 10.0 {
            FINISH_SIGN[entry_id(fighter.module_accessor)] += 1;
            if FINISH_SIGN[entry_id(fighter.module_accessor)] > 15 {
                FINISH_SIGN[entry_id(fighter.module_accessor)] = 15;
            }
        }

        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
        && StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            FINISH_SIGN[entry_id(fighter.module_accessor)] = 0;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        wario_frame
    );
}