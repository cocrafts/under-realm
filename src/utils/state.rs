#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
	AssetLoading,
	Splash,
	InGame,
}
