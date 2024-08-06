

pub fn enumerations() -> Option<u8> {
     let mut opt1: Option<u8> = None;
     opt1 = Some(10);
     return opt1;
}

pub fn test_option_string() -> Option<String> {
    let mut opt1: Option<String> = None;
    opt1 = Some("Bostone Ochieng".to_string());
    return opt1;
}

pub fn test_option_chartype() -> Option<CharacterType> {
    let mut chartype: Option<CharacterType> = None;
    chartype = Some(CharacterType::Mage);
    return chartype;
}

pub enum CharacterType {
    Archer,
    Warior,
    Mage
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer",
            CharacterType::Warior => "Warior",
            CharacterType::Mage => "Mage",
        }.to_string()
    }
}