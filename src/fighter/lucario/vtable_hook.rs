use {
    smash::{
        hash40,
        app::{BattleObjectModuleAccessor, BattleObject, Fighter/*, Article*/, lua_bind::*, sv_battle_object},
        lib::lua_const::*
    },
    smash_rs::{
        phx::Hash40,
        app::{LinkEvent, LinkEventCapture}
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*},
    super::vl
};

#[skyline::hook(offset = 0xc5bff0)]
pub unsafe extern "C" fn lucario_check_aura(module_accessor: *mut BattleObjectModuleAccessor) -> f32 {
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = MiscModule::get_battle_object_from_id((*module_accessor).battle_object_id);
    get_aura(object)
}

#[skyline::hook(offset = 0xc5be20)]
pub unsafe extern "C" fn lucario_check_aura2(module: u64) -> f32 {
    let module_accessor = &mut *(*((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = MiscModule::get_battle_object_from_id((*module_accessor).battle_object_id);
    get_aura(object)
}

#[skyline::hook(offset = 0xc5e530)]
pub unsafe extern "C" fn lucario_handle_aura(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = get_aura(object);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

#[skyline::hook(offset = 0xc5e6d0)]
pub unsafe extern "C" fn lucario_handle_aura2(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let object_id = object.battle_object_id;
    let module_accessor = sv_battle_object::module_accessor(object_id);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = get_aura(object);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    // let prev_charge = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
    // let mut charge_frame;
    // if !ArticleModule::is_exist(module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL) {
    //     charge_frame = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    //     if !WorkModule::is_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD) {
    //         charge_frame = -1;
    //     }
    // }
    // else {
    //     let article = ArticleModule::get_article(module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL);
    //     if !article.is_null() {
    //         let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    //         let object = MiscModule::get_battle_object_from_id(object_id);
    //         let auraball = sv_battle_object::module_accessor(object_id);
    //         charge_frame = WorkModule::get_int(auraball, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    //     }
    //     else {
    //         charge_frame = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    //         if !WorkModule::is_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_AURABALL_HAD) {
    //             charge_frame = -1;
    //         }
    //     }
    // }
    // if prev_charge != charge_frame {
    //     let max_charge_frame = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    //     if charge_frame < max_charge_frame as i32 {
    //         let left_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L) as u32;
    //         if left_eff != 0 {
    //             EffectModule::remove(module_accessor, left_eff, 0);
    //             WorkModule::set_int(module_accessor, 0, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L);
    //         }
    //         let right_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R) as u32;
    //         if right_eff != 0 {
    //             EffectModule::remove(module_accessor, right_eff, 0);
    //             WorkModule::set_int(module_accessor, 0, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R);
    //         }
    //     }
    //     else {
    //         let right_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R) as u32;
    //         if right_eff == 0 {
    //             let bone = if object.kind == 6 {
    //                 hash40("haver")
    //             }
    //             else {
    //                 hash40("handr")
    //             };
    //             let eff = EffectModule::req_follow(
    //                 module_accessor,
    //                 smash::phx::Hash40::new_raw(0x16b1f651c2),
    //                 smash::phx::Hash40::new_raw(bone),
    //                 &ZERO_VECTOR,
    //                 &ZERO_VECTOR,
    //                 1.0,
    //                 false,
    //                 0,
    //                 0,
    //                 -1,
    //                 0,
    //                 0,
    //                 false,
    //                 false
    //             ) as u32;
    //             WorkModule::set_int(module_accessor, eff as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_R);
    //         }
    //         let left_eff = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L) as u32;
    //         if left_eff == 0 {
    //             let bone = if object.kind == 6 {
    //                 hash40("havel")
    //             }
    //             else {
    //                 hash40("handl")
    //             };
    //             let eff = EffectModule::req_follow(
    //                 module_accessor,
    //                 smash::phx::Hash40::new_raw(0x164bf96ca1),
    //                 smash::phx::Hash40::new_raw(bone),
    //                 &ZERO_VECTOR,
    //                 &ZERO_VECTOR,
    //                 1.0,
    //                 false,
    //                 0,
    //                 0,
    //                 -1,
    //                 0,
    //                 0,
    //                 false,
    //                 false
    //             ) as u32;
    //             WorkModule::set_int(module_accessor, eff as i32, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_AURABALL_MAX_L);
    //         }
    //     }
    // }
    // WorkModule::set_int(module_accessor, charge_frame, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_PREV_AURABALL_CHARGE_FRAME);
    // WorkModule::set_int(module_accessor, charge_frame, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
}

#[skyline::hook(offset = 0xc5d580)]
pub unsafe extern "C" fn lucario_on_grab(_vtable: u64, fighter: &mut Fighter, event: &mut LinkEvent) -> u64 {
    // param_3 + 0x10
    if event.link_event_kind.as_u64() == hash40("capture") {
        let capture_event : &mut LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
            // param_3 + 0x30
            capture_event.node = Hash40::new("throw");
            // param_3[0x28]
            capture_event.result = true;
            let offset = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
            // param_3 + 0x44
            capture_event.motion_offset = offset;
            let offset_lw = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
            // param_3 + 0x48
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request(module_accessor, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, false);
            return 0;
        }
    }
    1
}

unsafe extern "C" fn get_aura(object: *mut BattleObject) -> f32 {
    let module_accessor = (*object).module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        1.0
    }
    else {
        let mut charge = VarModule::get_int(object, lucario::instance::int::AURA_LEVEL) as f32;
        charge += VarModule::get_int(object, lucario::status::int::AURA_ENHANCED_BY) as f32;
        let min_aurapower = vl::aurapower::MIN_AURAPOWER;
        let max_aurapower = vl::aurapower::MAX_AURAPOWER;
        let diff = max_aurapower - min_aurapower;
        let max_charge = vl::private::AURA_CHARGE_MAX as f32;
        min_aurapower + (diff * charge.clamp(0.0, max_charge) / max_charge)
    }
}

#[skyline::hook(offset = 0xc5ce20)]
pub unsafe extern "C" fn lucario_set_effect_scale(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    let effect = WorkModule::get_int64(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_KIND);
    if effect != hash40("null") {
        let left = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_LHADOU) as u32;
        let right = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_RHADOU) as u32;
        let aurapower = WorkModule::get_float(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
        let ratio = (aurapower - vl::aurapower::MIN_AURAPOWER) / (vl::aurapower::MAX_AURAPOWER - vl::aurapower::MIN_AURAPOWER);
        let diff = vl::aurapower::MAX_SCALE - vl::aurapower::MIN_SCALE;
        let scale = vl::aurapower::MIN_SCALE + (diff * ratio);
        if left != 0 {
            EffectModule::set_scale(module_accessor, left, &smash::phx::Vector3f{x: scale, y: scale, z: scale});
        }
        if right != 0 {
            EffectModule::set_scale(module_accessor, right, &smash::phx::Vector3f{x: scale, y: scale, z: scale});
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        lucario_check_aura,
        lucario_check_aura2,
        lucario_handle_aura,
        lucario_handle_aura2,
        lucario_on_grab,
        lucario_set_effect_scale
    );
}