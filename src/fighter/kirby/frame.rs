use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Incin Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLAG_JUMP_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IN_HITLAG].get_bool() {
            FGCModule::jump_cancel_check_exception(fighter);
        }

        // Give Kirby back Dark Deception if he is on the ground or grabbing ledge.

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N)
        && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N);
        }

        // Taunt Movement

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_loop") {
            if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L | *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                let lr = PostureModule::lr(fighter.module_accessor);
                let mot = if lr < 0.0 {
                    WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L)
                }
                else {
                    WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R)
                };
                let restart_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME) as f32;
                MotionModule::change_motion_force_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot),
                    restart_frame,
                    1.0,
                    0.0
                );
            }
            else {
                let stickx = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
                let mut spin = 0.5 * stickx;
                if spin.abs() > 0.5 {
                    if spin < 0.0 {
                        spin = -0.5;
                    }
                    else {
                        spin = 0.5;
                    }
                }
                macros::SET_SPEED_EX(fighter, spin, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        kirby_frame
    );
}