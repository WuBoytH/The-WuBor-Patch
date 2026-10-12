use crate::imports::*;
use wubor_utils::offsets;

#[skyline::hook(offset = offsets::fighter::wario::VTABLE_FUNC_2)]
pub unsafe extern "C" fn wario_vtable_func2(_vtable: u64, _module_accessor: &mut BattleObjectModuleAccessor) {

}

pub fn install() {
    skyline::install_hooks!(
        wario_vtable_func2
    );
}