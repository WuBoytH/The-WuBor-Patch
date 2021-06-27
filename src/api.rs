use arcropolis_api::{arc_callback, hash40};
use sli::SliFile;
use prc;

#[arc_callback]
fn api_callback(hash: u64, data: &mut [u8]) -> Option<usize> {
    // if hash == hash40("ui/param/database/ui_chara_db.prc").as_u64() {
    //     let mut reader = std::io::Cursor::new(data);
    //     match prc::read_stream(&mut reader)
    //     {
    //         Ok(f) => {

    //             let mut charaprc = f;

    //             match prc::write_stream(&mut reader, &charaprc)
    //             {
    //                 Ok(_f) => {
    //                     return Some(reader.position() as usize);
    //                 }
    //                 Err(_e) => return None,
    //             }

    //         }

    //         Err(_e) => return None,
    //     };
    // }
    if hash == hash40("sound/param/soundlabelinfo.sli").as_u64() {
        let mut reader = std::io::Cursor::new(data);
        match SliFile::read(&mut reader)
        {
            Ok(f) => {

                let mut soundlabel = f;
        
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("se_ryu_6c_aura").as_u64(),
                    nus3bank_id: 2068,
                    tone_id: 83
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("se_ryu_6c_exec").as_u64(),
                    nus3bank_id: 2068,
                    tone_id: 84
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_appeal01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 36
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_appeal02").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 37
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_attack01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 38
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_attack02").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 39
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_attack03").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 40
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_attack04").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 41
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_attack05").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 42
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_attack06").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 43
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_attack07").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 44
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_cliffcatch").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 45
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_damage_twinkle").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 46
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_damage01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 47
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_damage02").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 48
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_damage03").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 49
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_damagefly01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 50
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_damagefly02").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 51
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_final").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 52
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_furafura").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 53
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_furasleep").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 54
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_heavyget").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 55
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_jump01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 56
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_missfoot01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 57
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_missfoot02").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 58
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_ottotto").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 59
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_passive").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 60
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_special_h01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 61
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_special_l01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 62
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_special_l02").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 63
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_special_n01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 64
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_swimup").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 65
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_win01").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 66
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_win02").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 67
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_win03").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 68
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_win_marth").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 69
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_win_ike").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 70
                });
                soundlabel.entries_mut().push(sli::Entry {
                    tone_name: hash40("vc_shadow_knockout").as_u64(),
                    nus3bank_id: 4118,
                    tone_id: 71
                });
        
                match soundlabel.write(&mut reader)
                {
                    Ok(_f) => {
                        return Some(reader.position() as usize);
                    }
                    Err(_e) => return None,
                }

            }

            Err(_e) => return None,
        }
    }
    else {
        None
    }
}

const SOUNDLABELSIZE: usize = 0x61A80;
// const CHARASIZE: usize = 0x9C40;

pub fn install() {
    // api_callback::install("ui/param/database/ui_chara_db.prc", CHARASIZE);
    api_callback::install("sound/param/soundlabelinfo.sli", SOUNDLABELSIZE);
}