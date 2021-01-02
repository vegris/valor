use super::creature_stack::CreatureStack;

pub fn calculate_strike_damage(attacker: &CreatureStack, defender: &CreatureStack) -> u32 {
    let (damage_min, damage_max) = attacker.base_stats().damage;
    let (damage_min, damage_max) = (damage_min as u32, damage_max as u32);
    
    // Базовый урон
    let base_damage = (damage_max + damage_min) / 2 * attacker.count;
    // dbg!(base_damage);

    // Эффекты, модифицирующие атаку атакующего
    let current_attack = attacker.base_stats().attack;
    // dbg!(current_attack);

    // Эффекты, модифицирующие защиту защищающегося
    let current_defence = defender.base_stats().defence;
    // dbg!(current_defence);

    // основной модификатор урона в двух видах
    let md_1 = 
        if current_attack > current_defence {
            0.05 * (current_attack - current_defence) as f32
        } else {
            0.0
        };
    let md_1 = f32::min(md_1, 3.0);
    // dbg!(md_1);

    let md_2 =
        if current_defence > current_attack {
            1.0 - 0.025 * (current_defence - current_attack) as f32
        } else {
            1.0
        };
    let md_2 = f32::max(md_2, 0.3);
    // dbg!(md_2);

    // Модификатор вторичного навыка Нападение/Стрельба
    let m_off = 0.0;
    // dbg!(m_off);
    
    // Модификатор специализации 
    let m_spec = 0.0;
    // dbg!(m_spec);

    // Модификатор удачи
    let m_luck = 0.0;
    // dbg!(m_luck);

    let m_at = 0.0;
    // dbg!(m_at);

    // Модификатор доспехов
    let m_armor = 1.0;
    // dbg!(m_armor);

    // Модификатор защитных заклинаний
    let m_spell = 0.0;
    // dbg!(m_spell);

    // Модификатор штрафа стрелков
    let m_arch_penalty = 1.0;
    // dbg!(m_arch_penalty);

    // Прочие модификаторы защиты
    let m_prot = 1.0;
    // dbg!(m_prot);
    
    let damage = base_damage as f32 *
        (1.0 + md_1 + m_off + m_spec + m_luck + m_at) *
        md_2 * m_armor * m_spell * m_arch_penalty * m_prot;

    damage.round() as u32
}
