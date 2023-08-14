use {
    smash::{
        lua2cpp::*,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*, table_const::*}
};

#[line("kirby", main)]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Incin Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N
        && VarModule::is_flag(fighter.battle_object, fighter::status::flag::JUMP_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IS_STOP].get_bool() {
            FGCModule::jump_cancel_check_exception(fighter);
        }

        // Give Kirby back Dark Deception if he is on the ground or grabbing ledge.

        if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_N)
        && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
            VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_N);
        }

        // Taunt Movement

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_loop") {
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

pub fn install() {
    kirby_frame::install();
}