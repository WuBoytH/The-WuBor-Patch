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
    custom_var::*,
    wubor_utils::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l02"));
                let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor);
                macros::SET_SPEED_EX(fighter, speedx, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
            if ![
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT,
                *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE
            ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
                VarModule::set_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND, mario::SPECIAL_LW_KIND_LONG_JUMP);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        mario_frame
    );
}