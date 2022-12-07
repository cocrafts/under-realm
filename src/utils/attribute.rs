#[derive(Clone, Copy, PartialEq)]
pub enum ElementalType {
	Metal,
	Wood,
	Water,
	Fire,
	Earth,
	Dark,
	Light,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ActivationType {
	Summon,
	Death,
	Passive,
	Attack,
	Defense,
	Glory,
	Prefight,
	Postfight,
	Charge,
	Inspire,
	Banner,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InspireSource {
	Metal,
	Wood,
	Water,
	Fire,
	Earth,
	Dark,
	Light,
	Summon,
	Death,
	Spell,
	Skill,
}
