use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*
    },
    wubor_utils::{wua_bind::*, table_const::*}
};

pub unsafe extern "C" fn wolf_fgc(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    // let mut special_cancels : Vec<i32> = [].to_vec();
    // let mut normal_cancels : Vec<i32> = [].to_vec();
    let mut jump_cancel = 0;
    if [
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
    ].contains(&status) {
        jump_cancel = 1;
    }
    if !FGCModule::cancel_system(fighter, [].to_vec(), [].to_vec(), false, jump_cancel).get_bool() {
        crate::fighter::common::common_fgc::common_fgc(fighter);
    }
}
