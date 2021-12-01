use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            DRAGON_INSTALL[entry_id(fighter.module_accessor)] = 0.0;
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = 0;
        }

        if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
            if !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                DRAGON_INSTALL[entry_id(fighter.module_accessor)] -= 1.0;
            }
            else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                DRAGON_INSTALL[entry_id(fighter.module_accessor)] += 2.0;
            }
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                wall_jump_check(fighter);
            }
        }

        if _TIME_COUNTER[entry_id(fighter.module_accessor)] > 0 {
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 24 {
                macros::EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
                macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
                macros::EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
                macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
            }
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 18 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_start"), Hash40::new("handr"), 2, 0, 0, 0, 0, 0, 0.7, true);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            }
            _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
            if DRAGON_INSTALL[entry_id(fighter.module_accessor)] > 0.0 {
                if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] = 24;
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