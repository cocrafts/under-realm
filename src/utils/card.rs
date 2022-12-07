#[derive(Clone, Copy, PartialEq)]
pub enum CardType {
	Hero,
	Troop,
	Spell,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ClassType {
	Tanker,
	Warrior,
	Assassin,
	Wizard,
	Summoner,
}
