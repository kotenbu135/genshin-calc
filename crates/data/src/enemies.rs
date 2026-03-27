use crate::types::{EnemyData, ResistanceTemplate};

// ---------------------------------------------------------------------------
// Resistance Templates
// ---------------------------------------------------------------------------

/// 標準: 全耐性10% (ヒルチャール、宝盗団など大半の敵)
pub const ALL_10: ResistanceTemplate = ResistanceTemplate {
    name: "標準 (全耐性10%)",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 物理50% 元素10% (遺跡守衛、遺跡ハンターなど)
pub const PHYS_50_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理50% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.50,
};

/// 物理70% 元素10% (遺跡重機など)
pub const PHYS_70_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理70% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.70,
};

/// 物理30% 元素10% (ファデュイ先遣隊など)
pub const PHYS_30_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理30% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.30,
};

/// 物理-20% 元素10% (ファデュイ・デットエージェントなど)
pub const PHYS_MINUS20_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理-20% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: -0.20,
};

/// 炎70% 他10% (爆炎樹など)
pub const PYRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "炎70% 他10%",
    pyro: 0.70,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 氷70% 他10% (急凍樹など)
pub const CRYO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "氷70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.70,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷70% 他10% (雷音権現など)
pub const ELECTRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.70,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 水70% 他10% (純水精霊など)
pub const HYDRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "水70% 他10%",
    pyro: 0.10,
    hydro: 0.70,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 草70% 他10% (翠翎恐蕈など)
pub const DENDRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "草70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.70,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 岩70% 他10% (無相の岩など)
pub const GEO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "岩70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.70,
    physical: 0.10,
};

/// 風70% 他10% (無相の風など)
pub const ANEMO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "風70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.70,
    geo: 0.10,
    physical: 0.10,
};

/// 全耐性の一覧
pub const ALL_TEMPLATES: &[&ResistanceTemplate] = &[
    &ALL_10,
    &PHYS_50_ELEM_10,
    &PHYS_70_ELEM_10,
    &PHYS_30_ELEM_10,
    &PHYS_MINUS20_ELEM_10,
    &PYRO_70_REST_10,
    &CRYO_70_REST_10,
    &ELECTRO_70_REST_10,
    &HYDRO_70_REST_10,
    &DENDRO_70_REST_10,
    &GEO_70_REST_10,
    &ANEMO_70_REST_10,
];

// ---------------------------------------------------------------------------
// Enemy Data
// ---------------------------------------------------------------------------

pub const HILICHURL: EnemyData = EnemyData {
    id: "hilichurl",
    name: "ヒルチャール",
    resistance: &ALL_10,
};

pub const MITACHURL: EnemyData = EnemyData {
    id: "mitachurl",
    name: "ヒルチャール暴徒",
    resistance: &ALL_10,
};

pub const LAWACHURL: EnemyData = EnemyData {
    id: "lawachurl",
    name: "ヒルチャール・岩兜の王",
    resistance: &ALL_10,
};

pub const TREASURE_HOARDER: EnemyData = EnemyData {
    id: "treasure_hoarder",
    name: "宝盗団",
    resistance: &ALL_10,
};

pub const FATUI_SKIRMISHER: EnemyData = EnemyData {
    id: "fatui_skirmisher",
    name: "ファデュイ先遣隊",
    resistance: &PHYS_30_ELEM_10,
};

pub const FATUI_AGENT: EnemyData = EnemyData {
    id: "fatui_agent",
    name: "ファデュイ・デットエージェント",
    resistance: &PHYS_MINUS20_ELEM_10,
};

pub const RUIN_GUARD: EnemyData = EnemyData {
    id: "ruin_guard",
    name: "遺跡守衛",
    resistance: &PHYS_50_ELEM_10,
};

pub const RUIN_HUNTER: EnemyData = EnemyData {
    id: "ruin_hunter",
    name: "遺跡ハンター",
    resistance: &PHYS_50_ELEM_10,
};

pub const RUIN_GRADER: EnemyData = EnemyData {
    id: "ruin_grader",
    name: "遺跡重機",
    resistance: &PHYS_70_ELEM_10,
};

pub const PYRO_REGISVINE: EnemyData = EnemyData {
    id: "pyro_regisvine",
    name: "爆炎樹",
    resistance: &PYRO_70_REST_10,
};

pub const CRYO_REGISVINE: EnemyData = EnemyData {
    id: "cryo_regisvine",
    name: "急凍樹",
    resistance: &CRYO_70_REST_10,
};

pub const THUNDER_MANIFESTATION: EnemyData = EnemyData {
    id: "thunder_manifestation",
    name: "雷音権現",
    resistance: &ELECTRO_70_REST_10,
};

pub const HYDRO_HYPOSTASIS: EnemyData = EnemyData {
    id: "hydro_hypostasis",
    name: "無相の水",
    resistance: &HYDRO_70_REST_10,
};

pub const GEO_HYPOSTASIS: EnemyData = EnemyData {
    id: "geo_hypostasis",
    name: "無相の岩",
    resistance: &GEO_70_REST_10,
};

pub const ANEMO_HYPOSTASIS: EnemyData = EnemyData {
    id: "anemo_hypostasis",
    name: "無相の風",
    resistance: &ANEMO_70_REST_10,
};

/// 全敵データの一覧
pub const ALL_ENEMIES: &[&EnemyData] = &[
    &HILICHURL,
    &MITACHURL,
    &LAWACHURL,
    &TREASURE_HOARDER,
    &FATUI_SKIRMISHER,
    &FATUI_AGENT,
    &RUIN_GUARD,
    &RUIN_HUNTER,
    &RUIN_GRADER,
    &PYRO_REGISVINE,
    &CRYO_REGISVINE,
    &THUNDER_MANIFESTATION,
    &HYDRO_HYPOSTASIS,
    &GEO_HYPOSTASIS,
    &ANEMO_HYPOSTASIS,
];

#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::Element;

    #[test]
    fn test_all_10_template() {
        let eps = 1e-10;
        assert!((ALL_10.pyro - 0.10).abs() < eps);
        assert!((ALL_10.physical - 0.10).abs() < eps);
        assert!((ALL_10.get(Some(Element::Pyro)) - 0.10).abs() < eps);
        assert!((ALL_10.get(None) - 0.10).abs() < eps);
    }

    #[test]
    fn test_phys_50_template() {
        let eps = 1e-10;
        assert!((PHYS_50_ELEM_10.physical - 0.50).abs() < eps);
        assert!((PHYS_50_ELEM_10.pyro - 0.10).abs() < eps);
        assert!((PHYS_50_ELEM_10.get(None) - 0.50).abs() < eps);
        assert!((PHYS_50_ELEM_10.get(Some(Element::Hydro)) - 0.10).abs() < eps);
    }

    #[test]
    fn test_negative_resistance_template() {
        let eps = 1e-10;
        assert!((PHYS_MINUS20_ELEM_10.physical - (-0.20)).abs() < eps);
        assert!((PHYS_MINUS20_ELEM_10.get(None) - (-0.20)).abs() < eps);
    }

    #[test]
    fn test_enemy_to_core() {
        let enemy = RUIN_GUARD.to_enemy(Some(Element::Pyro), 90, 0.0);
        let eps = 1e-10;
        assert_eq!(enemy.level, 90);
        assert!((enemy.resistance - 0.10).abs() < eps);
        assert!((enemy.def_reduction - 0.0).abs() < eps);
    }

    #[test]
    fn test_enemy_physical_resistance() {
        let enemy = RUIN_GUARD.to_enemy(None, 85, 0.0);
        let eps = 1e-10;
        assert!((enemy.resistance - 0.50).abs() < eps);
    }

    #[test]
    fn test_all_enemies_unique_ids() {
        for (i, a) in ALL_ENEMIES.iter().enumerate() {
            for (j, b) in ALL_ENEMIES.iter().enumerate() {
                if i != j {
                    assert_ne!(a.id, b.id, "Duplicate enemy id: {}", a.id);
                }
            }
        }
    }

    #[test]
    fn test_all_templates_count() {
        assert_eq!(ALL_TEMPLATES.len(), 12);
    }

    #[test]
    fn test_all_enemies_count() {
        assert_eq!(ALL_ENEMIES.len(), 15);
    }
}
