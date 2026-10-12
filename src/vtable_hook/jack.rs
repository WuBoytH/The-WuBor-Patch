use wubor_utils::offsets;

#[skyline::hook(offset = offsets::fighter::jack::DAMAGE_CALLBACK_3)]
pub unsafe extern "C" fn jack_damage_callback3(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = offsets::fighter::jack::CALL_SUMMON_DISPATCH)]
pub unsafe extern "C" fn jack_call_summon_dispatch(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

pub fn install() {
    unsafe {
        // Disables passive meter gain
        skyline::patching::Patch::in_text(offsets::fighter::jack::PASSIVE_METER_GAIN_PATCH).data(0x17FFFF6Eu32);
        // Disables automatically summoning Arsene
        skyline::patching::Patch::in_text(offsets::fighter::jack::AUTO_SUMMON_ARSENE_PATCH_1).data(0x14000035u32);
        skyline::patching::Patch::in_text(offsets::fighter::jack::AUTO_SUMMON_ARSENE_PATCH_2).data(0x14000031u32);
        skyline::install_hooks!(
            jack_damage_callback3,
            jack_call_summon_dispatch
        );
    }
}
