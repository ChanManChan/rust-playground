trait Attacker {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "thai chi".to_string()
        }
    }

    fn choose_weapon(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits() {
       let my_character: Character = Character::Archer;
       let chosen_fighting_style: String = my_character.choose_style();
       dbg!(chosen_fighting_style);
    }
}