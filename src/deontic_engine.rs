use std::cmp::Ordering;

// --- Core Types: The Language of Logic ---

/// Деонтическая модальность: Что мы обязаны делать, а что запрещено.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeonticModality {
    Obligation,  // Мы ОБЯЗАНЫ это сделать (например, спасти протокол)
    Prohibition, // Нам ЗАПРЕЩЕНО это делать (например, вывести > 10%)
    Permission,  // Нам РАЗРЕШЕНО (нейтрально)
}

/// Структура Нормы (Правила).
/// У каждой нормы есть вес (priority). Логика побеждает силу.
#[derive(Debug, Clone)]
pub struct Norm {
    pub id: String,
    pub modality: DeonticModality,
    pub priority: u8, // 0 - 255. Чем выше, тем важнее правило.
    pub description: String,
}

/// Состояние системы (Snapshot of the World).
/// В реальной версии здесь будут данные из блокчейна.
pub struct WorldState {
    pub treasury_balance: f64,
    pub collateral_ratio: f64, // Коэффициент обеспечения
    pub is_emergency_mode: bool,
}

/// Результат работы Логического Файервола.
#[derive(Debug)]
pub struct LogicVerdict {
    pub allowed: bool,
    pub trace: Vec<String>, // Лог доказательства решения
}

// --- The Engine ---

pub struct DeonticEngine {
    norms: Vec<Norm>,
}

impl DeonticEngine {
    pub fn new() -> Self {
        Self { norms: Vec::new() }
    }

    pub fn add_norm(&mut self, norm: Norm) {
        self.norms.push(norm);
    }

    /// Главная функция: Разрешение конфликтов.
    /// Принимает действие и состояние мира, возвращает Вердикт.
    pub fn evaluate_transaction(&self, action_amount: f64, state: &WorldState) -> LogicVerdict {
        let mut trace = Vec::new();
        trace.push(format!("Evaluating transaction: Withdraw {}", action_amount));

        // 1. Собираем активные Прогибиции (Запреты)
        let mut active_prohibitions = Vec::new();
        // Пример логики: Если вывод > 10% от баланса -> Запрет
        if action_amount > (state.treasury_balance * 0.10) {
            if let Some(norm) = self.norms.iter().find(|n| n.id == "MAX_WITHDRAWAL_LIMIT") {
                active_prohibitions.push(norm);
                trace.push(format!("VIOLATION DETECTED: Action violates norm '{}' (Priority: {})", norm.id, norm.priority));
            }
        }

        // 2. Собираем активные Облигации (Обязательства)
        let mut active_obligations = Vec::new();
        // Пример логики: Если Collateral Ratio < 150%, мы ОБЯЗАНЫ ребалансировать
        if state.collateral_ratio < 1.5 {
            if let Some(norm) = self.norms.iter().find(|n| n.id == "MAINTAIN_SOLVENCY") {
                active_obligations.push(norm);
                trace.push(format!("OBLIGATION ACTIVE: Action serves norm '{}' (Priority: {})", norm.id, norm.priority));
            }
        }

        // 3. Logic Resolution (Разрешение конфликта)
        if active_prohibitions.is_empty() {
            trace.push("No prohibitions found. Access GRANTED.".to_string());
            return LogicVerdict { allowed: true, trace };
        }

        // Если есть запреты, ищем оправдание (Обязательство с более высоким приоритетом)
        let max_prohibition_priority = active_prohibitions.iter().map(|n| n.priority).max().unwrap_or(0);
        let max_obligation_priority = active_obligations.iter().map(|n| n.priority).max().unwrap_or(0);

        trace.push(format!("Conflict detected: Prohibition Priority ({}) vs Obligation Priority ({})", 
            max_prohibition_priority, max_obligation_priority));

        if max_obligation_priority > max_prohibition_priority {
            trace.push("RESOLUTION: Obligation overrides Prohibition. Logic Integrity maintained.".to_string());
            trace.push("Access GRANTED (Emergency Override).".to_string());
            LogicVerdict { allowed: true, trace }
        } else {
            trace.push("RESOLUTION: Prohibition stands. Obligation insufficient to override.".to_string());
            trace.push("Access DENIED.".to_string());
            LogicVerdict { allowed: false, trace }
        }
    }
}

// --- Test Scenario (To show "The Boys" on GitHub) ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paradox_resolution() {
        let mut engine = DeonticEngine::new();

        // Норма 1: Обычное правило безопасности (Запрет)
        // Нельзя выводить много денег сразу. Вес: 50.
        engine.add_norm(Norm {
            id: "MAX_WITHDRAWAL_LIMIT".to_string(),
            modality: DeonticModality::Prohibition,
            priority: 50,
            description: "Withdrawals > 10% are forbidden".to_string(),
        });

        // Норма 2: Высшая аксиома (Обязательство)
        // Протокол должен жить. Вес: 100.
        engine.add_norm(Norm {
            id: "MAINTAIN_SOLVENCY".to_string(),
            modality: DeonticModality::Obligation,
            priority: 100,
            description: "Must prevent liquidation at all costs".to_string(),
        });

        // Сценарий: Рынок рухнул.
        let world_state = WorldState {
            treasury_balance: 1000.0,
            collateral_ratio: 1.2, // Критически мало! (нужно 1.5)
            is_emergency_mode: true,
        };

        // Агент пытается вывести 150 (15%), чтобы спасти залог.
        // Это нарушает правило 1 (>10%), но спасает правило 2.
        let verdict = engine.evaluate_transaction(150.0, &world_state);

        println!("--- LOGIC FIREWALL TRACE ---");
        for log in verdict.trace {
            println!("{}", log);
        }

        assert!(verdict.allowed); // Система должна разрешить нарушение ради спасения!
    }
}
