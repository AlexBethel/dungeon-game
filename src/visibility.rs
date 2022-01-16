//! Code for determining which cells the player and monsters can see.

/// The light transmission properties of a cell in the world.
#[derive(Debug, PartialEq)]
pub enum CellVisibility {
    /// This cell allows light to pass through: monsters can see
    /// through this cell as if it is air.
    Transparent,

    /// This cell blocks all light.
    Blocking,
}

/// How well-lit a cell is.
#[derive(Debug, PartialEq)]
pub enum Lighting {
    /// Monsters can only see in this cell if the cell is immediately
    /// adjacent to the monster.
    Dark,

    /// Monsters can see in this cell from far away.
    Lit,
}

/// Calculates whether a monster standing at `origin` can see the
/// contents of cell `cell`. We assume the monster can see `radius`
/// cells away at best (None for unlimited range), that `cell_map`
/// represents whether a cell transmits light, and that `light_map`
/// represents how well-lit a cell is.
pub fn visible(
    origin: (i32, i32),
    cell: (i32, i32),
    radius: Option<i32>,
    cell_map: impl Fn((i32, i32)) -> CellVisibility,
    light_map: impl Fn((i32, i32)) -> Lighting,
) -> bool {
    let dx = cell.0 - origin.0;
    let dy = cell.1 - origin.1;

    radius
        .map(|radius| dx * dx + dy * dy < radius * radius)
        .unwrap_or(true)
        && (light_map(cell) == Lighting::Lit)
        && (line(origin, cell).all(|tile| cell_map(tile) == CellVisibility::Transparent))
}

/// Constructs an iterator over the cells in a straight line from
/// `start` to `end`. The line will include `start`, but not `end`.
fn line(start: (i32, i32), end: (i32, i32)) -> Box<dyn Iterator<Item = (i32, i32)>> {
    // We could use a dedicated iterator type here eventually and
    // avoid the `Box` allocations, but I'm gonna assume it's not a
    // significant problem until proven otherwise.

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    // Transform the world so we're working from left to right, with
    // slope magnitude less than 1.
    if dx.abs() < dy.abs() {
        Box::new(line((start.1, start.0), (end.1, end.0)).map(|(x, y)| (y, x)))
    } else if dx < 0 {
        Box::new(line((-start.0, start.1), (-end.0, end.1)).map(|(x, y)| (-x, y)))
    } else {
        // Move the destination over by 0.5 cells on each axis, to
        // navigate to the corner rather than the center of the target
        // cell. It's weird but it makes things work way better.
        let dx = dx as f64 - 0.5;
        let dy = if dy > 0 {
            dy as f64 - 0.5
        } else if dy < 0 {
            dy as f64 + 0.5
        } else {
            dy as f64
        };

        // Now use float math to step along the line, one cell at a
        // time.
        let slope = dy as f64 / dx as f64;
        Box::new(
            std::iter::successors(Some((start.0, start.1 as f64)), move |&(x, y)| {
                Some((x + 1, y + slope))
            })
            // Add 0.5 here to round to nearest rather than rounding
            // towards zero (eliminates some bias).
            .map(|(x, y)| (x, (y + 0.5) as i32))
            .take_while(move |(x, _y)| x < &end.0),
        )
    }
}
