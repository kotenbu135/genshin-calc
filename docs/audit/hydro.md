# Hydro Audit — Issues Only

## Ayato
- [ ] **constellation_pattern**: `C3BurstC5Skill`→`C3SkillC5Burst`（C3=Kamisato Art: Kyouka/Skill, C5=Kamisato Art: Suiyuu/Burst）

## Neuvillette
- [ ] **constellation_pattern**: `C3BurstC5Skill`→`C3NormalC5Burst`（C3=As Water Seeks Equilibrium/Normal, C5=O Tides, I Have Returned/Burst）
- [ ] **A1/A4バフ名入れ替わり**
  - A1: code="Tidal Affinity"（存在しない名前）→正: "Heir to the Ancient Sea's Authority"
  - A4: code="Heir to the Ancient Sea's Authority"→正: "Discipline of the Supreme Arbitration"
  - 値は正しい

## Nilou
- [ ] **Whirling Steps scalings未実装**
  - Whirling Steps 1-Hit: 3.26%...7.75% Max HP
  - Whirling Steps 2-Hit: 3.96%...9.41% Max HP
  - Water Wheel: 5.06%...12.02% Max HP
  - Sword Dance pathのみ実装済み

## Mona
- [ ] **C2説明不一致**: code "Party EM+80 on Charged ATK hit"→HH C2はIllusory Torrent duration延長。EM buffの出典不明
- [ ] **C4**: code CritDmg+15%含む→HH C4はCritRate+15%のみ。CritDmg+15%の根拠不明

## Columbina
- [ ] **A1未実装**: CR+5%×3 stacks after Gravity Interference
- [ ] **Burst Lunar Reaction DMG Bonus未実装**: 13-55% per talent level

## Sigewinne（Minor）
- Burst scalings 微小誤差（0.0001-0.0005）複数Lv — 要否判断

## Tartaglia（Minor）
- Plunge Lv4: 0.8178→0.8177, Low Lv8: 2.1853→2.1851（微小）
