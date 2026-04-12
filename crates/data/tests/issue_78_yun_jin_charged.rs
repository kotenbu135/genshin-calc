use genshin_calc_data::find_character;

#[test]
fn yun_jin_charged_scalings_match_honeyhunter_mirror() {
    let yun_jin = find_character("yun_jin").expect("Yun Jin character data should exist");
    let charged = yun_jin
        .talents
        .normal_attack
        .charged
        .iter()
        .find(|scaling| scaling.name == "重撃ダメージ")
        .expect("Yun Jin charged attack scaling should exist");

    let expected = [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.6001,
        2.8289, 3.0577, 3.2865, 3.5361,
    ];

    assert_eq!(
        charged.values, expected,
        "Yun Jin charged attack values should match honeyhunter mirror at Lv1-15"
    );
}
