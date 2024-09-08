#![allow(clippy::needless_return)]
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
enum PokeType {
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

#[derive(Debug)]
enum Effectiveness {
    Super,
    Not,
    Immune,
}

fn type_relationships(relationships: Vec<(PokeType, Effectiveness)> ) -> HashMap<PokeType, Effectiveness> {
    let mut defender_map = HashMap::new();
    for (defender, effectiveness) in relationships {
        defender_map.insert(defender, effectiveness);
    }
    return defender_map;
}


fn type_effectiveness() -> HashMap<PokeType, HashMap<PokeType, Effectiveness>> {
    let mut type_effectiveness : HashMap<PokeType, HashMap<PokeType, Effectiveness>> = HashMap::new();
    type_effectiveness.insert(
        PokeType::Normal,
        type_relationships(
            vec![
                (PokeType::Rock, Effectiveness::Not),
                (PokeType::Ghost, Effectiveness::Immune),
            ]
        )
    );
    type_effectiveness.insert(
        PokeType::Fire,
        type_relationships(
            vec![
                (PokeType::Fire, Effectiveness::Not),
                (PokeType::Water, Effectiveness::Not),
                (PokeType::Rock, Effectiveness::Not),
                (PokeType::Dragon, Effectiveness::Not),
                (PokeType::Grass, Effectiveness::Super),
                (PokeType::Ice, Effectiveness::Super),
                (PokeType::Bug, Effectiveness::Super),
                (PokeType::Steel, Effectiveness::Super),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Water,
        type_relationships(
            vec![
                (PokeType::Fire, Effectiveness::Super),
                (PokeType::Water, Effectiveness::Not),
                (PokeType::Grass, Effectiveness::Not),
                (PokeType::Ground, Effectiveness::Super),
                (PokeType::Rock, Effectiveness::Super),
                (PokeType::Dragon, Effectiveness::Not),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Electric,
        type_relationships(
            vec![
                (PokeType::Flying, Effectiveness::Super),
                (PokeType::Water, Effectiveness::Super),
                (PokeType::Electric, Effectiveness::Not),
                (PokeType::Grass, Effectiveness::Not),
                (PokeType::Dragon, Effectiveness::Not),
                (PokeType::Ground, Effectiveness::Immune),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Grass,
        type_relationships(
            vec![
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
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Ice,
        type_relationships(
            vec![
                (PokeType::Fire, Effectiveness::Not),
                (PokeType::Water, Effectiveness::Not),
                (PokeType::Ice, Effectiveness::Not),
                (PokeType::Steel, Effectiveness::Not),
                (PokeType::Grass, Effectiveness::Super),
                (PokeType::Ground, Effectiveness::Super),
                (PokeType::Flying, Effectiveness::Super),
                (PokeType::Dragon, Effectiveness::Super),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Fighting,
        type_relationships(
            vec![
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
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Poison,
        type_relationships(
            vec![
                (PokeType::Grass, Effectiveness::Super),
                (PokeType::Fairy, Effectiveness::Super),
                (PokeType::Poison, Effectiveness::Not),
                (PokeType::Ground, Effectiveness::Not),
                (PokeType::Rock, Effectiveness::Not),
                (PokeType::Ghost, Effectiveness::Not),
                (PokeType::Steel, Effectiveness::Immune),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Ground,
        type_relationships(
            vec![
                (PokeType::Fire, Effectiveness::Super),
                (PokeType::Electric, Effectiveness::Super),
                (PokeType::Poison, Effectiveness::Super),
                (PokeType::Rock, Effectiveness::Super),
                (PokeType::Steel, Effectiveness::Super),
                (PokeType::Grass, Effectiveness::Not),
                (PokeType::Bug, Effectiveness::Not),
                (PokeType::Flying, Effectiveness::Immune),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Flying,
        type_relationships(
            vec![
                (PokeType::Grass, Effectiveness::Super),
                (PokeType::Fighting, Effectiveness::Super),
                (PokeType::Bug, Effectiveness::Super),
                (PokeType::Electric, Effectiveness::Not),
                (PokeType::Rock, Effectiveness::Not),
                (PokeType::Steel, Effectiveness::Not),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Psychic,
        type_relationships(
            vec![
                (PokeType::Fighting, Effectiveness::Super),
                (PokeType::Poison, Effectiveness::Super),
                (PokeType::Psychic, Effectiveness::Not),
                (PokeType::Steel, Effectiveness::Not),
                (PokeType::Dark, Effectiveness::Immune),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Bug,
        type_relationships(
            vec![
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
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Rock,
        type_relationships(
            vec![
                (PokeType::Fire, Effectiveness::Super),
                (PokeType::Ice, Effectiveness::Super),
                (PokeType::Flying, Effectiveness::Super),
                (PokeType::Bug, Effectiveness::Super),
                (PokeType::Fighting, Effectiveness::Not),
                (PokeType::Ground, Effectiveness::Not),
                (PokeType::Steel, Effectiveness::Not),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Ghost,
        type_relationships(
            vec![
                (PokeType::Psychic, Effectiveness::Super),
                (PokeType::Ghost, Effectiveness::Super),
                (PokeType::Dark, Effectiveness::Not),
                (PokeType::Normal, Effectiveness::Immune),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Dragon,
        type_relationships(
            vec![
                (PokeType::Dragon, Effectiveness::Super),
                (PokeType::Steel, Effectiveness::Not),
                (PokeType::Fairy, Effectiveness::Immune),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Dark,
        type_relationships(
            vec![
                (PokeType::Psychic, Effectiveness::Super),
                (PokeType::Ghost, Effectiveness::Super),
                (PokeType::Dark, Effectiveness::Not),
                (PokeType::Fighting, Effectiveness::Not),
                (PokeType::Fairy, Effectiveness::Not),
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Steel,
        type_relationships(
            vec![
                (PokeType::Ice, Effectiveness::Super),
                (PokeType::Rock, Effectiveness::Super),
                (PokeType::Fairy, Effectiveness::Super),
                (PokeType::Fire, Effectiveness::Not),
                (PokeType::Water, Effectiveness::Not),
                (PokeType::Electric, Effectiveness::Not),
                (PokeType::Steel, Effectiveness::Not),  
            ]
        )
    );

    type_effectiveness.insert(
        PokeType::Fairy,
        type_relationships(
            vec![
                (PokeType::Fighting, Effectiveness::Super),
                (PokeType::Dragon, Effectiveness::Super),
                (PokeType::Dark, Effectiveness::Super),
                (PokeType::Fire, Effectiveness::Not),
                (PokeType::Poison, Effectiveness::Not),
                (PokeType::Steel, Effectiveness::Not),
            ]
        )
    );
       return type_effectiveness;
}

#[allow(dead_code)]
fn effectiveness(attacker: &PokeType, defender: &PokeType) -> f32 {
    let m = type_effectiveness();
    let atk_type = m.get(attacker).expect("Attacker not found in type_effectiveness chart");
    match atk_type.get(defender) {
        Some(Effectiveness::Super) => return 2.0,
        Some(Effectiveness::Not) => return 0.5,
        Some(Effectiveness::Immune) => return 0.0,
        _ => return 1.0,
    }
}

fn main() {
    let type_effectiveness = type_effectiveness();
    println!("{:?}", type_effectiveness);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestCase<'a> {
        attacker: &'a PokeType,
        defender: &'a PokeType,
        want: f32,
    }


    #[test]
    fn effectiveness_calculations() {
        let test_cases = [
            TestCase { attacker: &PokeType::Fire, defender: &PokeType::Water, want: 0.5 },
            TestCase { attacker: &PokeType::Ice,  defender: &PokeType::Dragon, want: 2.0 },
            TestCase { attacker: &PokeType::Ghost, defender: &PokeType::Normal, want: 0.0 },
            TestCase { attacker: &PokeType::Flying, defender: &PokeType::Psychic, want: 1.0 },
        ];

        for case in test_cases {
            let TestCase {attacker, defender, want} = case;
            let got = effectiveness(attacker, defender);
            assert_eq!(
                got,
                want,
                "Failure: attacker = {:?}, defender = {:?}, want = {}, got = {}", attacker, defender, want, got
            )
        }

    }
}
