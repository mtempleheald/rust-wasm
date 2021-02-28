// pub type Player = u8; // Player is a synonym for u8, just to make code easier to reason about

// pub type Column = u32;

// pub enum Action {
//     None = 0,
//     MoveUp = 1,
//     MoveDown = 2,
// }

// pub struct GameState {
//     pub current_player: Player, // player whose turn it is gets precedence during processing
//     pub is_stable: bool, // optimisation - board will be reviewed recursively after change until stable
//     pub selected_column: Column,
//     pub next_action: Action,
// }