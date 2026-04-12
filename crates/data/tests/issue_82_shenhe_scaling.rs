use genshin_calc_core::BuffableStat;
use genshin_calc_data::find_talent_buffs;

#[test]
fn shenhe_icy_quill_scaling_matches_honeyhunter_mirror() {
    let buffs = find_talent_buffs("shenhe").expect("Shenhe talent buffs should exist");
    let icy_quill = buffs
        .iter()
        .find(|buff| buff.stat == BuffableStat::NormalAtkFlatDmg)
        .expect("Shenhe should have an Icy Quill Normal Attack flat damage buff");

    let expected = [
        0.4566, 0.4908, 0.5250, 0.5707, 0.6049, 0.6392, 0.6848, 0.7305, 0.7762, 0.8218, 0.8675,
        0.9131, 0.9702, 1.0273, 1.0843,
    ];

    assert_eq!(
        icy_quill.talent_scaling,
        Some(&expected),
        "Shenhe Icy Quill scaling should match the honeyhunter mirror at Lv1-15"
    );
}
