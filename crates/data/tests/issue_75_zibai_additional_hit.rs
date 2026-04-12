use genshin_calc_core::{Element, ScalingStat};
use genshin_calc_data::find_character;

const EXPECTED: [f64; 15] = [
    0.2946, 0.3167, 0.3387, 0.3682, 0.3903, 0.4124, 0.4418, 0.4713, 0.5008, 0.5302, 0.5597, 0.5891,
    0.6259, 0.6628, 0.6996,
];

#[test]
fn zibai_skill_has_fourth_hit_additional_def_scaling() {
    let zibai = find_character("zibai").expect("Zibai should exist");
    let skill = &zibai.talents.elemental_skill;

    let additional = skill
        .scalings
        .iter()
        .find(|scaling| scaling.name == "月相転移4段追加ダメージ")
        .expect("Zibai skill should include the fourth-hit additional damage entry");

    assert_eq!(skill.scalings.len(), 10);
    assert_eq!(additional.scaling_stat, ScalingStat::Def);
    assert_eq!(additional.damage_element, Some(Element::Geo));
    assert_eq!(additional.values, EXPECTED);
}
