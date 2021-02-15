//use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, /*acmd_func*/};
//use smash::phx::Vector3f;
//use smash::app::BattleObjectModuleAccessor;
//use smash::app::lua_bind::EffectModule;

pub fn special_hi_move(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_TOONLINK {
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") {
                if MotionModule::frame(module_accessor) > 6.0 && MotionModule::frame(module_accessor) < 46.0 {
                    let facing_dirn = PostureModule::lr(module_accessor);
                    if facing_dirn > 0.0 {
                        acmd!(lua_state, {
                            SET_SPEED_EX(1.76 * ControlModule::get_stick_x(module_accessor), 0.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
                        });
                    }
                    else{
                        acmd!(lua_state, {
                            SET_SPEED_EX(-1.76 * ControlModule::get_stick_x(module_accessor), 0.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
                        });
                    }
                }
            }
        }
    }
}

pub fn install() {
    //acmd::add_hooks!(
        
    //);
    acmd::add_custom_hooks!(
        special_hi_move
    );
}