use cuivre::{
    graphics::sprites::{Sprite, SpriteSheet},
    maths::Vector3f,
    transform::Transform,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
}

impl Team {
    fn piece(self, id: i32, x: usize, y: usize) -> Piece {
        Piece {
            team: self,
            id,
            x,
            y: self.get_row(y),
            alive: true,
        }
    }

    fn get_row(self, row: usize) -> usize {
        match self {
            Team::White => row,
            Team::Black => 7 - row,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MoveType {
    MoveOnly,
    CaptureOnly,
    MoveAndCapture,
}

/// Represents a move that could be executed by a piece,
/// but has to be checked for validity before being used.
#[derive(Debug)]
pub struct MoveAttempt {
    source_x: usize,
    source_y: usize,
    target_x: i32,
    target_y: i32,
    move_type: MoveType,
}

/// Represents a valid move that can be executed by a piece.
#[derive(Debug)]
pub struct Move {
    source_x: usize,
    source_y: usize,
    target_x: usize,
    target_y: usize,
}

impl Move {
    pub fn target_pos(&self) -> (usize, usize) {
        (self.target_x, self.target_y)
    }
}

#[derive(Debug)]
pub struct Piece {
    team: Team,
    id: i32,
    x: usize,
    y: usize,
    alive: bool,
}

impl Piece {
    pub fn sprite<'s>(&self, sheet: &'s SpriteSheet) -> Sprite<'s> {
        // Sprite row
        let y = match self.team {
            Team::White => 0,
            Team::Black => 1,
        };

        sheet.sprite(self.id, y)
    }

    pub fn transform(&self) -> Transform {
        Transform::from_position(Vector3f::new(self.x as f32 + 0.5, self.y as f32 + 0.5, 1.0))
    }

    // Creates a move attempt, relative to the current position.
    fn move_attempt(&self, x: i32, y: i32, move_type: MoveType) -> MoveAttempt {
        MoveAttempt {
            source_x: self.x,
            source_y: self.y,
            target_x: self.x as i32 + x,
            target_y: self.y as i32 + y,
            move_type,
        }
    }

    fn moves(&self, manager: &PiecesManager) -> Vec<Move> {
        // A list of possible moves that need to be validated.
        let mut move_tries = Vec::new();

        let dir = match self.team {
            Team::White => 1,
            Team::Black => -1,
        };

        match self.id {
            // Pawn
            0 => {
                // Move forward
                move_tries.push(self.move_attempt(0, dir, MoveType::MoveOnly));

                // Move 2 tiles forward
                if self.y == self.team.get_row(1) {
                    move_tries.push(self.move_attempt(0, 2 * dir, MoveType::MoveOnly));
                }

                // Diagonal capture
                for &x in &[self.x as i32 + 1, self.x as i32 - 1] {
                    move_tries.push(self.move_attempt(x, dir, MoveType::CaptureOnly))
                }
            }
            _ => {}
        }

        move_tries
            .iter()
            .filter_map(|ref move_attempt| {
                // Check bounds
                if move_attempt.target_x < 0
                    || move_attempt.target_x >= 8
                    || move_attempt.target_y < 0
                    || move_attempt.target_y >= 8
                    {
                        return None;
                    }

                let target_x = move_attempt.target_x as usize;
                let target_y = move_attempt.target_y as usize;

                // Check for pieces on the target tile
                if let Some(other_piece) = manager.piece_by_pos(target_x, target_y) {
                    if other_piece.team == self.team {
                        return None;
                    } else if move_attempt.move_type == MoveType::MoveOnly {
                        return None;
                    }
                } else if move_attempt.move_type == MoveType::CaptureOnly {
                    return None;
                }

                Some(Move {
                    target_x,
                    target_y,
                    source_x: move_attempt.source_x,
                    source_y: move_attempt.source_y,
                })
            })
            .collect()
    }
}

pub type PieceIndex = usize;

pub struct PiecesManager {
    pieces: Vec<Piece>,
    selected_piece_index: Option<PieceIndex>,
}

impl PiecesManager {
    pub fn new() -> Self {
        let mut pieces = Vec::new();

        for team in &[Team::White, Team::Black] {
            //Pawns
            for i in 0..8 {
                pieces.push(team.piece(0, i, 1))
            }

            //Knights
            pieces.push(team.piece(1, 1, 0));
            pieces.push(team.piece(1, 6, 0));

            //Bishops
            pieces.push(team.piece(2, 2, 0));
            pieces.push(team.piece(2, 5, 0));

            //Rooks
            pieces.push(team.piece(3, 0, 0));
            pieces.push(team.piece(3, 7, 0));

            //Queen
            pieces.push(team.piece(4, 3, 0));
            //King
            pieces.push(team.piece(5, 4, 0));
        }

        Self {
            pieces,
            selected_piece_index: None,
        }
    }

    pub fn pieces(&self) -> Vec<&Piece> {
        self.pieces.iter().filter(|piece| piece.alive).collect()
    }

    fn piece(&self, index: PieceIndex) -> &Piece {
        self.pieces
            .get(index)
            .unwrap_or_else(|| panic!("No piece found for index {}", index))
    }

    pub fn piece_by_pos(&self, x: usize, y: usize) -> Option<&Piece> {
        self.piece_index_by_pos(x, y).map(|index| self.piece(index))
    }

    fn piece_index_by_pos(&self, x: usize, y: usize) -> Option<PieceIndex> {
        for (index, piece) in self.pieces.iter().enumerate() {
            if piece.alive && piece.x == x && piece.y == y {
                return Some(index);
            }
        }

        None
    }
    pub fn selected_moves(&self) -> Vec<Move> {
        match &self.selected_piece_index {
            None => Vec::new(),
            Some(piece) => self.piece(*piece).moves(self),
        }
    }

    pub fn on_click(&mut self, x: usize, y: usize) {
        if x >= 8 || y >= 8 {
            self.selected_piece_index = None;
        } else {
            if let Some(selected_move) = self
                .selected_moves()
                .iter()
                .find(|mov| mov.target_pos() == (x, y))
                {
                    //TODO: Move piece
                } else {
                self.selected_piece_index = self.piece_index_by_pos(x, y)
            }
        }
    }
}
