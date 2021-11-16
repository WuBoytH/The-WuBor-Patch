use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Incin Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N
        && MotionModule::frame(fighter.module_accessor) > 19.0
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }

        // Down Tilt Bounce

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3") {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                macros::EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                BOUNCE[entry_id(fighter.module_accessor)] = true;
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
        if BOUNCE[entry_id(fighter.module_accessor)]
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("jump_b") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_b"), 22.0, 1.0, false, 0.0, false, false);
            macros::SET_SPEED_EX(fighter, -1.0, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_b")
        && MotionModule::frame(fighter.module_accessor) >= 30.0)
        || MotionModule::motion_kind(fighter.module_accessor) != hash40("jump_b") {
            BOUNCE[entry_id(fighter.module_accessor)] = false;
        }

        // Give Ganondorf back Dark Deception if he is on the ground or grabbing ledge (or if Funny Mode is enabled).

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            DISABLE_SPECIAL_N[entry_id(fighter.module_accessor)] = false;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        kirby_frame
    );
}