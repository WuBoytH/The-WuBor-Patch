use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::brave::REMOVE_PSYCHE_UP)]
extern "C" fn remove_psyche_up(fighter: &mut Fighter);

unsafe extern "C" fn brave_on_frame6(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    if !WorkModule::is_flag(module_accessor, 0x200000ea)
    || !WorkModule::is_flag(module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_CRITICAL_HIT) {
        return;
    }
    let status = StatusModule::status_kind(module_accessor);
    let mot = MotionModule::motion_kind(module_accessor);
    let attack_s3 = status == *FIGHTER_STATUS_KIND_ATTACK_S3 && mot == hash40("attack_s3_s");
    let attack = status == *FIGHTER_STATUS_KIND_ATTACK && (mot == hash40("attack_11") || mot == hash40("attack_12"));
    let is_hit = AttackModule::is_infliction(module_accessor, 6);
    if attack_s3 || attack {
        if !is_hit {
            return;
        }
        // (*fighter as *const bool).add(0xf9) = true;
        *(fighter as *const Fighter as *mut bool).add(0xf9) = true;
        return;
    }
    if !is_hit && !*(fighter as *const Fighter as *const bool).add(0xf8) {
        return;
    }
    remove_psyche_up(fighter);
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_BRAVE,
        FighterVtableSlot::OnFrame6,
        brave_on_frame6 as *const () as _
    );
}
