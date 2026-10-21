use crate::imports::*;
use wubor_utils::offsets;

#[skyline::hook(offset = offsets::fighter::dolly::CHECK_SUPER_SPECIAL)]
pub unsafe extern "C" fn dolly_check_super_special(work: u64, _damage: u64) -> u64 {
    let module_accessor = &mut *(*((work as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    // if smashball::is_training_mode() {
    //     return 1;
    // }
    let go_meter = VarModule::get_float(module_accessor, vars::dolly::instance::float::GO_METER);
    // println!("go_meter: {}", go_meter);
    if go_meter >= 100.0 {
        return 1;
    }
    0
}

pub fn install() {
    skyline::install_hooks!(
        dolly_check_super_special
    );
}
