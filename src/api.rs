use arcropolis_api::*;
use prc::hash40::{Hash40, to_hash40};
use prc::*;
use sli::*;

#[arc_callback]
fn chara_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    load_original_file(hash, &mut data);
    // with the param data ready,
    let mut reader = std::io::Cursor::new(&mut data);
    let mut root = prc::read_stream(&mut reader).unwrap();

    // enter the first and only node of the file ("db_root")
    let (db_root_hash, db_root) = &mut root.0[0];
    assert_eq!(*db_root_hash, to_hash40("db_root"));

    let db_root_list = db_root.try_into_mut::<ParamList>().unwrap();

    // iterate the list to find the param with Lucina's data
    // we could go to the exact index, but this is subject to change across game updates.
    db_root_list.0.iter_mut().for_each(|param| {
        let ui_chara_struct = param.try_into_ref::<ParamStruct>().unwrap();
        let (_, ui_chara_id) = &ui_chara_struct.0[0];
        let ui_chara_hash = ui_chara_id.try_into_ref::<Hash40>().unwrap();
        if *ui_chara_hash == to_hash40("ui_chara_lucina") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_persona");
                }
            });
        }
    });
    let mut writer = std::io::Cursor::new(data);
    write_stream(&mut writer, &root).unwrap();
    return Some(writer.position() as usize);
}

#[arc_callback]
fn sli_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    load_original_file(hash, &mut data);
    // with the param data ready,
    let mut reader = std::io::Cursor::new(&mut data);
    let mut soundlabel = SliFile::read(&mut reader).unwrap();

    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("se_ryu_6c_aura").0,
        nus3bank_id: 2068,
        tone_id: 83
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("se_ryu_6c_exec").0,
        nus3bank_id: 2068,
        tone_id: 84
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_appeal01").0,
        nus3bank_id: 4118,
        tone_id: 36
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_appeal02").0,
        nus3bank_id: 4118,
        tone_id: 37
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_attack01").0,
        nus3bank_id: 4118,
        tone_id: 38
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_attack02").0,
        nus3bank_id: 4118,
        tone_id: 39
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_attack03").0,
        nus3bank_id: 4118,
        tone_id: 40
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_attack04").0,
        nus3bank_id: 4118,
        tone_id: 41
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_attack05").0,
        nus3bank_id: 4118,
        tone_id: 42
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_attack06").0,
        nus3bank_id: 4118,
        tone_id: 43
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_attack07").0,
        nus3bank_id: 4118,
        tone_id: 44
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_cliffcatch").0,
        nus3bank_id: 4118,
        tone_id: 45
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_damage_twinkle").0,
        nus3bank_id: 4118,
        tone_id: 46
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_damage01").0,
        nus3bank_id: 4118,
        tone_id: 47
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_damage02").0,
        nus3bank_id: 4118,
        tone_id: 48
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_damage03").0,
        nus3bank_id: 4118,
        tone_id: 49
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_damagefly01").0,
        nus3bank_id: 4118,
        tone_id: 50
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_damagefly02").0,
        nus3bank_id: 4118,
        tone_id: 51
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_final").0,
        nus3bank_id: 4118,
        tone_id: 52
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_furafura").0,
        nus3bank_id: 4118,
        tone_id: 53
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_furasleep").0,
        nus3bank_id: 4118,
        tone_id: 54
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_heavyget").0,
        nus3bank_id: 4118,
        tone_id: 55
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_jump01").0,
        nus3bank_id: 4118,
        tone_id: 56
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_missfoot01").0,
        nus3bank_id: 4118,
        tone_id: 57
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_missfoot02").0,
        nus3bank_id: 4118,
        tone_id: 58
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_ottotto").0,
        nus3bank_id: 4118,
        tone_id: 59
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_passive").0,
        nus3bank_id: 4118,
        tone_id: 60
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_special_h01").0,
        nus3bank_id: 4118,
        tone_id: 61
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_special_l01").0,
        nus3bank_id: 4118,
        tone_id: 62
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_special_l02").0,
        nus3bank_id: 4118,
        tone_id: 63
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_special_n01").0,
        nus3bank_id: 4118,
        tone_id: 64
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_swimup").0,
        nus3bank_id: 4118,
        tone_id: 65
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_win01").0,
        nus3bank_id: 4118,
        tone_id: 66
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_win02").0,
        nus3bank_id: 4118,
        tone_id: 67
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_win03").0,
        nus3bank_id: 4118,
        tone_id: 68
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_win_marth").0,
        nus3bank_id: 4118,
        tone_id: 69
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_win_ike").0,
        nus3bank_id: 4118,
        tone_id: 70
    });
    soundlabel.entries_mut().push(sli::Entry {
        tone_name: to_hash40("vc_shadow_knockout").0,
        nus3bank_id: 4118,
        tone_id: 71
    });

    let mut writer = std::io::Cursor::new(data);
    soundlabel.write(&mut writer).unwrap();
    return Some(writer.position() as usize);
}

pub fn install() {
    chara_callback::install("ui/param/database/ui_chara_db.prc",  0xFFFF);
    sli_callback::install("sound/param/soundlabelinfo.sli",  0x57000);
}