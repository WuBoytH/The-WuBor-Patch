use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN )]
fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
        && MotionModule::frame(fighter.module_accessor) > 19.0
        && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY))
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
            reset_i32(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE) > 0
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE) < 3
        && AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false) >= 20.0 {
            critical_zoom(fighter, 0, 2.0, 1.5);
            WorkModule::set_int(fighter.module_accessor, 3, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        gaogaen_frame
    );
}