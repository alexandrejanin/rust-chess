use super::{Sprite, SpriteSheet, Transform};
use cuivre::maths::Point3f;

#[derive(Copy, Clone)]
pub enum Team {
    White,
    Black,
}

impl Team {
    fn piece(self, id: i32, position: (u8, u8)) -> Piece {
        Piece {
            team: self,
            id,
            x: position.0,
            y: self.get_row(position.1),
        }
    }

    fn get_row(self, row: u8) -> u8 {
        match self {
            Team::White => row,
            Team::Black => 7 - row,
        }
    }
}

pub struct Piece {
    team: Team,
    id: i32,
    x: u8,
    y: u8,
}

impl Piece {
    pub fn name(&self) -> &str {
        match self.id {
            0 => "Pawn",
            1 => "Knight",
            2 => "Bishop",
            3 => "Rook",
            4 => "Queen",
            5 => "King",
            _ => "UNKNOWN",
        }
    }

    pub fn sprite<'s>(&self, sheet: &'s SpriteSheet) -> Sprite<'s> {
        let y = match self.team {
            Team::White => 0,
            Team::Black => 1,
        };

        sheet.sprite(self.id, y)
    }

    pub fn transform(&self) -> Transform {
        Transform::from_position(Point3f::new(self.x as f32 + 0.5, self.y as f32 + 0.5, 0.0))
    }
}

pub struct PiecesManager {
    pub pieces: Vec<Piece>,
}

impl Default for PiecesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PiecesManager {
    pub fn new() -> Self {
        let mut pieces = Vec::new();

        for team in &[Team::White, Team::Black] {
            //Pawns
            for i in 0..8 {
                pieces.push(team.piece(0, (i, 1)));
            }

            //Knights
            pieces.push(team.piece(1, (1, 0)));
            pieces.push(team.piece(1, (6, 0)));

            //Bishops
            pieces.push(team.piece(2, (2, 0)));
            pieces.push(team.piece(2, (5, 0)));

            //Rooks
            pieces.push(team.piece(3, (0, 0)));
            pieces.push(team.piece(3, (7, 0)));

            //Queen
            pieces.push(team.piece(4, (3, 0)));
            //King
            pieces.push(team.piece(5, (4, 0)));
        }

        Self { pieces }
    }
}
