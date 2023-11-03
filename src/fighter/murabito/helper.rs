use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::table_const::*
};

pub unsafe extern "C" fn special_n_pocket_set_flag(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    let target_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET {
        if target_id == *BATTLE_OBJECT_ID_INVALID as u32 {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x29660ac06a), false);
        }
    }
    // println!("checking pocketed object!");
    if target_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let pocket_object_cat = sv_battle_object::category(target_id);
        if pocket_object_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
            // println!("pocketed a weapon!");
            VarModule::pocket_vars(fighter.module_accessor, target_id);
        }
    }
    0.into()
}
