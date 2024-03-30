use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

#[repr(C)]
struct TroopManager {
  _x0: u64,
  max_pikmin_count: usize, // always 3
  current_pikmin_count: usize,
  pikmin_objects: *mut *mut BattleObject,
  pikmin: [*mut BattleObject; 3],
  // remainder that we don't care about
  // big shoutouts to Blujay for this one
}

unsafe extern "C" fn pikmin_antenna_indicator(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ENTRY
    || sv_information::is_ready_go() {
        let troops = WorkModule::get_int64(fighter.module_accessor, 0x100000C0);
        let troopmanager = troops as *const TroopManager;
        let count = (*troopmanager).current_pikmin_count;
        let pikmin;
        let pikmin_id;
        if count > 0 {
            pikmin = (*troopmanager).pikmin[0];
            pikmin_id = (*pikmin).battle_object_id;
        }
        else {
            pikmin = std::ptr::null_mut();
            pikmin_id = *BATTLE_OBJECT_ID_INVALID as u32;
        }
        let antenna_eff = WorkModule::get_int(fighter.module_accessor, 0x100000C4) as u32;
        if pikmin_id != *BATTLE_OBJECT_ID_INVALID as u32
        && sv_battle_object::is_active(pikmin_id) {
            let variation = WorkModule::get_int((*pikmin).module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
            let color = match variation {
                0 => Vector3f{x: 1.0, y: 0.2, z: 0.2},
                1 => Vector3f{x: 1.0, y: 1.0, z: 0.2},
                2 => Vector3f{x: 0.2, y: 0.2, z: 1.0},
                3 => Vector3f{x: 0.8, y: 0.8, z: 0.8},
                _ => Vector3f{x: 0.4, y: 0.2, z: 0.8}
            };
            EffectModule::set_alpha(fighter.module_accessor, antenna_eff, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, antenna_eff, color.x, color.y, color.z);
        }
        else {
            EffectModule::set_rgb(fighter.module_accessor, antenna_eff, 0.8, 0.8, 0.8);
            EffectModule::set_alpha(fighter.module_accessor, antenna_eff, 0.05);
        }
    }
}

unsafe extern "C" fn pikmin_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    pikmin_antenna_indicator(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, pikmin_frame);
}