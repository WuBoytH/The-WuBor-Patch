use smash::lua2cpp::L2CFighterCommon;
use smash::lib::L2CValue;
use smash::lib::lua_const::*;
use smash::app::*;
use smashline::*;

#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        fighter.global_table["time_counter"] = L2CValue::new_int(0);
        fighter.global_table["is_funny"] = L2CValue::new_bool(false);
        fighter.global_table["is_fgc"] = L2CValue::new_bool(false);
        fighter.global_table["counter_hit_state"] = L2CValue::new_int(0);
        fighter.global_table["counter_hit_helper"] = L2CValue::new_num(0.0);
        let kind = utility::get_kind(&mut *fighter.module_accessor);
        if kind == FIGHTER_KIND_KEN {
            fighter.global_table["quick_step_state"] = L2CValue::new_int(0);
            fighter.global_table["vs1_cancel"] = L2CValue::Bool(false);
            fighter.global_table["v_shift"] = L2CValue::Bool(false);
            fighter.global_table["v_trigger"] = L2CValue::Bool(false);
            fighter.global_table["vt_activation"] = L2CValue::Bool(false);
            fighter.global_table["vt1_cancel"] = L2CValue::Bool(false);
            fighter.global_table["v_gauge"] = L2CValue::new_int(0);
            fighter.global_table["dmg_ratio"] = L2CValue::new_num(0.8);
            fighter.global_table["damage_taken"] = L2CValue::new_num(0.0);
            fighter.global_table["damage_taken_prev"] = L2CValue::new_num(0.0);
            fighter.global_table["shoryureppa"] = L2CValue::new_int(0);
            fighter.global_table["curr_loops"] = L2CValue::new_int(0);
            fighter.global_table["diff_x"] = L2CValue::new_num(0.0);
        }
    }
}

#[installer]
fn install() {
  install_agent_resets!(
    fighter_reset
  );
}