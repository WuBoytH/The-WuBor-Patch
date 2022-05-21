use {
    smash::app::smashball,
    smash_rs::app::*,
    super::vars::*
};

#[skyline::hook(offset = 0x971230)]
pub unsafe extern "C" fn dolly_check_super_special(work: &mut WorkModule, _damage: &mut Module) -> u64 {
    if work.get_int(WorkId::from_u32_unchecked(0x10000000)) > 7 {
        std::process::abort();
    }
    if smashball::is_training_mode() {
        return 1;
    }
    let go_meter = work.get_float(WorkId::from_u32_unchecked(FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_GO_METER as u32));
    println!("go_meter: {}", go_meter);
    if go_meter >= 100.0 {
        return 1;
    }
    0
}

#[skyline::hook(offset = 0x970fd0)]
pub unsafe extern "C" fn dolly_check_super_special_pre(module_accessor: *mut smash::app::BattleObjectModuleAccessor, param_2: u8) {
    original!()(module_accessor, param_2)
}

pub fn install() {
    skyline::install_hooks!(
        dolly_check_super_special,
        dolly_check_super_special_pre
    );
}