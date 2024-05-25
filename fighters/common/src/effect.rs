// use super::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon31bind_hash_call_effect_ChargeMaxEPN3lib8L2CAgentERNS1_7utility8VariadicEPKcSt9__va_list"]
    fn bind_hash_call_effect_ChargeMax();
}

#[skyline::hook(replace = bind_hash_call_effect_ChargeMax)]
unsafe extern "C" fn effect_chargemax() {
    // nothing
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            effect_chargemax
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}