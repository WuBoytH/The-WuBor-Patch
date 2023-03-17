use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars}
};

#[skyline::hook(offset = vars::FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module: u64, param_type: u64, param_hash: u64) -> f32 {
    let ret = original!()(module, param_type, param_hash);
    let module_accessor = &mut *(*((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let category = utility::get_category(&mut *module_accessor);
    let kind = utility::get_kind(&mut *module_accessor);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let object = MiscModule::get_battle_object_from_id(module_accessor.battle_object_id);
        if [
            hash40("jump_y")
        ].contains(&param_type) {
            let mut mul = 1.0;
            if VarModule::is_flag(object, vars::fighter::instance::flag::SUPER_JUMP) {
                mul *= 1.2;
            }
            return ret * mul;
        }
    }
    else if category == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if kind == *WEAPON_KIND_KAMUI_RYUSENSYA {
            let otarget_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            if sv_battle_object::is_active(otarget_id)
            && sv_battle_object::kind(otarget_id) == *FIGHTER_KIND_KAMUI {
                let object = MiscModule::get_battle_object_from_id(otarget_id);
                if param_hash == hash40("speed_max") {
                    if VarModule::get_float(object, vars::kamui::instance::float::DRAGON_INSTALL) > 0.0 {
                        return 1.2;
                    }
                }
                else if param_hash == hash40("life_max") {
                    if VarModule::get_float(object, vars::kamui::instance::float::DRAGON_INSTALL) > 0.0 {
                        return 150.0;
                    }
                }
                else if param_hash == hash40("scale_max") {
                    if VarModule::get_float(object, vars::kamui::instance::float::DRAGON_INSTALL) > 0.0 {
                        return 1.7;
                    }
                }
            }
        }
    }
    ret
}

pub fn install() {
    skyline::install_hooks!(
        get_param_float_replace
    );
    
}