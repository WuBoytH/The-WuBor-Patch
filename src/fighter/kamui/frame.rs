use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::wua_bind::*,
    super::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL);
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER);
        }

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL) > 0.0 {
            if !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                WarkModule::count_down(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL, 1.0);
            }
            else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                WarkModule::add_f32(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL, 2.0);
            }
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                MiscModule::wall_jump_check(fighter);
            }
        }

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER) > 0.0 {
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER) == 24.0 {
                macros::EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
                macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
                macros::EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
                macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
            }
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER) == 18.0 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("handr"), 2, 0, 0, 0, 0, 0, 0.7, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            }
            WarkModule::add_f32(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER, -1.0);
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL) > 0.0 {
                if WorkModule::get_float(fighter.module_accessor, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER) == 0.0 {
                    WorkModule::set_float(fighter.module_accessor, 24.0, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL_TIMER);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        kamui_frame
    );
}