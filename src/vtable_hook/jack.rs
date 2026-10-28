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
    skyline::install_hooks!(
        jack_damage_callback3,
        jack_call_summon_dispatch
    );
}
