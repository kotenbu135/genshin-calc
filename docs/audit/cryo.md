# Cryo Audit — Issues Only

## Wriothesley
- [ ] **Charged Attack Lv8**: 2.4736(247.36%)→2.4474(244.74%)。差+2.62pp

## Layla
- [ ] **Charged Attack 2 Lv14**: 1.3200(132.00%)→1.3320(133.20%)。差-1.20pp
- Minor: N1 Lv1 0.5120→0.5122（0.02pp rounding）

## Freminet
- [ ] **Skill Lv4 Shattering Pressure 元素間違い**: `Element::Cryo`→Physical DMG
- [ ] **constellation_pattern**: `C3NormalC5Burst`→`C3NormalC5Skill`（C5=Pressurized Floe/Skill）
  - `C3NormalC5Skill` variant がenumに存在しない→追加必要

## Escoffier
- [ ] **C1**: `CritDmg`→`ElementalCritDmg(Element::Cryo)`（HH: "Cryo DMG CRIT DMG +60%"）
- [ ] **C2**: `BuffTarget::Team`→`BuffTarget::TeamExcludeSelf`（HH: "apart from Escoffier"）

## Mika
- [ ] **C6**: `CritDmg`→Physical-specific CritDmg（HH: "Physical CRIT DMG +60%"）
  - 余計なPhysical DMG+10%バフあり（C6説明にない）

## Eula（Note）
- Skill RES shred 固定25%/stack — 実際は16-25%タレントレベル依存（簡略化）

## Shenhe（Note）
- 同上パターン

## Ayaka（Note）
- A4説明 "after burst"→実際はsprint Cryo application trigger。値正しい
