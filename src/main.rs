#![allow(clippy::needless_return)]
use std::{
    collections::HashMap,
    fmt::Display,
};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PokeType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Effectiveness {
    Super,
    Not,
    Immune,
    Normal,
}

impl Display for Effectiveness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e: f32 = f32::from(*self);
        write!(f, "{e}")
    }
}

impl From<Effectiveness> for f32 {
    fn from(value: Effectiveness) -> Self {
        match value {
            Effectiveness::Immune => 0.0,
            Effectiveness::Not => 0.5,
            Effectiveness::Normal => 1.0,
            Effectiveness::Super => 2.0,
        }
    }
}

fn type_relationships(
    relationships: Vec<(PokeType, Effectiveness)>,
) -> HashMap<PokeType, Effectiveness> {
    let mut defender_map = HashMap::new();
    for (defender, effectiveness) in relationships {
        defender_map.insert(defender, effectiveness);
    }
    return defender_map;
}

fn type_effectiveness() -> HashMap<PokeType, HashMap<PokeType, Effectiveness>> {
    let mut type_effectiveness: HashMap<PokeType, HashMap<PokeType, Effectiveness>> =
        HashMap::new();
    type_effectiveness.insert(
        PokeType::Normal,
        type_relationships(vec![
            (PokeType::Rock, Effectiveness::Not),
            (PokeType::Ghost, Effectiveness::Immune),
        ]),
    );
    type_effectiveness.insert(
        PokeType::Fire,
        type_relationships(vec![
            (PokeType::Fire, Effectiveness::Not),
            (PokeType::Water, Effectiveness::Not),
            (PokeType::Rock, Effectiveness::Not),
            (PokeType::Dragon, Effectiveness::Not),
            (PokeType::Grass, Effectiveness::Super),
            (PokeType::Ice, Effectiveness::Super),
            (PokeType::Bug, Effectiveness::Super),
            (PokeType::Steel, Effectiveness::Super),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Water,
        type_relationships(vec![
            (PokeType::Fire, Effectiveness::Super),
            (PokeType::Water, Effectiveness::Not),
            (PokeType::Grass, Effectiveness::Not),
            (PokeType::Ground, Effectiveness::Super),
            (PokeType::Rock, Effectiveness::Super),
            (PokeType::Dragon, Effectiveness::Not),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Electric,
        type_relationships(vec![
            (PokeType::Flying, Effectiveness::Super),
            (PokeType::Water, Effectiveness::Super),
            (PokeType::Electric, Effectiveness::Not),
            (PokeType::Grass, Effectiveness::Not),
            (PokeType::Dragon, Effectiveness::Not),
            (PokeType::Ground, Effectiveness::Immune),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Grass,
        type_relationships(vec![
            (PokeType::Water, Effectiveness::Super),
            (PokeType::Ground, Effectiveness::Super),
            (PokeType::Rock, Effectiveness::Super),
            (PokeType::Fire, Effectiveness::Not),
            (PokeType::Grass, Effectiveness::Not),
            (PokeType::Poison, Effectiveness::Not),
            (PokeType::Flying, Effectiveness::Not),
            (PokeType::Bug, Effectiveness::Not),
            (PokeType::Dragon, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Ice,
        type_relationships(vec![
            (PokeType::Fire, Effectiveness::Not),
            (PokeType::Water, Effectiveness::Not),
            (PokeType::Ice, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
            (PokeType::Grass, Effectiveness::Super),
            (PokeType::Ground, Effectiveness::Super),
            (PokeType::Flying, Effectiveness::Super),
            (PokeType::Dragon, Effectiveness::Super),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Fighting,
        type_relationships(vec![
            (PokeType::Normal, Effectiveness::Super),
            (PokeType::Ice, Effectiveness::Super),
            (PokeType::Dark, Effectiveness::Super),
            (PokeType::Rock, Effectiveness::Super),
            (PokeType::Steel, Effectiveness::Super),
            (PokeType::Poison, Effectiveness::Not),
            (PokeType::Flying, Effectiveness::Not),
            (PokeType::Psychic, Effectiveness::Not),
            (PokeType::Bug, Effectiveness::Not),
            (PokeType::Fairy, Effectiveness::Not),
            (PokeType::Ghost, Effectiveness::Immune),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Poison,
        type_relationships(vec![
            (PokeType::Grass, Effectiveness::Super),
            (PokeType::Fairy, Effectiveness::Super),
            (PokeType::Poison, Effectiveness::Not),
            (PokeType::Ground, Effectiveness::Not),
            (PokeType::Rock, Effectiveness::Not),
            (PokeType::Ghost, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Immune),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Ground,
        type_relationships(vec![
            (PokeType::Fire, Effectiveness::Super),
            (PokeType::Electric, Effectiveness::Super),
            (PokeType::Poison, Effectiveness::Super),
            (PokeType::Rock, Effectiveness::Super),
            (PokeType::Steel, Effectiveness::Super),
            (PokeType::Grass, Effectiveness::Not),
            (PokeType::Bug, Effectiveness::Not),
            (PokeType::Flying, Effectiveness::Immune),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Flying,
        type_relationships(vec![
            (PokeType::Grass, Effectiveness::Super),
            (PokeType::Fighting, Effectiveness::Super),
            (PokeType::Bug, Effectiveness::Super),
            (PokeType::Electric, Effectiveness::Not),
            (PokeType::Rock, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Psychic,
        type_relationships(vec![
            (PokeType::Fighting, Effectiveness::Super),
            (PokeType::Poison, Effectiveness::Super),
            (PokeType::Psychic, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
            (PokeType::Dark, Effectiveness::Immune),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Bug,
        type_relationships(vec![
            (PokeType::Grass, Effectiveness::Super),
            (PokeType::Psychic, Effectiveness::Super),
            (PokeType::Dark, Effectiveness::Super),
            (PokeType::Fire, Effectiveness::Not),
            (PokeType::Fighting, Effectiveness::Not),
            (PokeType::Poison, Effectiveness::Not),
            (PokeType::Flying, Effectiveness::Not),
            (PokeType::Ghost, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
            (PokeType::Fairy, Effectiveness::Not),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Rock,
        type_relationships(vec![
            (PokeType::Fire, Effectiveness::Super),
            (PokeType::Ice, Effectiveness::Super),
            (PokeType::Flying, Effectiveness::Super),
            (PokeType::Bug, Effectiveness::Super),
            (PokeType::Fighting, Effectiveness::Not),
            (PokeType::Ground, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Ghost,
        type_relationships(vec![
            (PokeType::Psychic, Effectiveness::Super),
            (PokeType::Ghost, Effectiveness::Super),
            (PokeType::Dark, Effectiveness::Not),
            (PokeType::Normal, Effectiveness::Immune),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Dragon,
        type_relationships(vec![
            (PokeType::Dragon, Effectiveness::Super),
            (PokeType::Steel, Effectiveness::Not),
            (PokeType::Fairy, Effectiveness::Immune),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Dark,
        type_relationships(vec![
            (PokeType::Psychic, Effectiveness::Super),
            (PokeType::Ghost, Effectiveness::Super),
            (PokeType::Dark, Effectiveness::Not),
            (PokeType::Fighting, Effectiveness::Not),
            (PokeType::Fairy, Effectiveness::Not),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Steel,
        type_relationships(vec![
            (PokeType::Ice, Effectiveness::Super),
            (PokeType::Rock, Effectiveness::Super),
            (PokeType::Fairy, Effectiveness::Super),
            (PokeType::Fire, Effectiveness::Not),
            (PokeType::Water, Effectiveness::Not),
            (PokeType::Electric, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
        ]),
    );

    type_effectiveness.insert(
        PokeType::Fairy,
        type_relationships(vec![
            (PokeType::Fighting, Effectiveness::Super),
            (PokeType::Dragon, Effectiveness::Super),
            (PokeType::Dark, Effectiveness::Super),
            (PokeType::Fire, Effectiveness::Not),
            (PokeType::Poison, Effectiveness::Not),
            (PokeType::Steel, Effectiveness::Not),
        ]),
    );
    return type_effectiveness;
}

#[must_use]
/// Returns the effectiveness of `attacker` against `defender`.
///
/// # Panics
///
/// If `attacker` is not in the effectiveness map.
pub fn effectiveness(attacker: &PokeType, defender: &PokeType) -> Effectiveness {
    let m = type_effectiveness();
    let atk_type = m
        .get(attacker)
        .expect("Attacker not found in type_effectiveness chart");
    atk_type
        .get(defender)
        .copied()
        .unwrap_or(Effectiveness::Normal)
}

fn main() {
    let type_effectiveness = type_effectiveness();
    println!("{type_effectiveness:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        attacker: &'a PokeType,
        defender: &'a PokeType,
        want: Effectiveness,
    }

    #[test]
    fn effectiveness_calculations() {
        let test_cases = [
            TestCase {
                attacker: &PokeType::Fire,
                defender: &PokeType::Water,
                want: Effectiveness::Not,
            },
            TestCase {
                attacker: &PokeType::Ice,
                defender: &PokeType::Dragon,
                want: Effectiveness::Super,
            },
            TestCase {
                attacker: &PokeType::Ghost,
                defender: &PokeType::Normal,
                want: Effectiveness::Immune,
            },
            TestCase {
                attacker: &PokeType::Flying,
                defender: &PokeType::Psychic,
                want: Effectiveness::Normal,
            },
        ];

        for case in test_cases {
            let TestCase {
                attacker,
                defender,
                want,
            } = case;
            let got = effectiveness(attacker, defender);
            assert_eq!(
                got, want,
                "Failure: attacker = {:?}, defender = {:?}, want = {}, got = {}",
                attacker, defender, want, got
            );
        }
    }
}
