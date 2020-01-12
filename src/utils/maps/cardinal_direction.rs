/// This enum is used to represent the four possible options for the cardinal directions.
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}
    
impl CardinalDirection {
    /// Gets the direction achieved when rotating from self by 90 degrees in the direction
    /// specified (true for clockwise, false for counter-clockwise).
    pub fn get_90deg_rotated_direction(&self, rotate_cw: bool) -> CardinalDirection {
        match self {
            CardinalDirection::North => {
                if rotate_cw {
                    return CardinalDirection::East;
                } else {
                    return CardinalDirection::West;
                }
            },
            CardinalDirection::East => {
                if rotate_cw {
                    return CardinalDirection::South;
                } else {
                    return CardinalDirection::North;
                }
            },
            CardinalDirection::South => {
                if rotate_cw {
                    return CardinalDirection::West;
                } else {
                    return CardinalDirection::East;
                }
            },
            CardinalDirection::West => {
                if rotate_cw {
                    return CardinalDirection::North;
                } else {
                    return CardinalDirection::South;
                }
            }
        }
    }
}
