use bevy::prelude::*;

pub const GRID_SIZE: f32 = 96.;
pub const GAP: f32 = 10.;
pub const LINE_WIDTH: f32 = 2.;
pub const SCALE: f32 = 1.;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum AppState {
    #[default]
    WaitStart,
    Restart,
    Playing,
    GameOver,
}

// 棋盘大小和雷的数量
#[derive(Component)]
struct Board {
    size: Vec2, // 棋盘大小
    mines: u32, // 雷的数量
}

impl Board {
    pub fn new(size: Vec2, mines: u32) -> Self {
        Self { size, mines }
    }
}

// 定义一个枚举类型，表示格子的状态
#[derive(Component)]
enum CellState {
    Covered,       // 被覆盖，未揭开
    Uncovered(u8), // 被揭开，显示周围的雷数
    Flagged,       // 被标记，猜测有雷
}

// 定义一个结构体，表示格子的属性
#[derive(Component)]
struct Cell {
    state: CellState, // 格子的状态
    has_mine: bool,   // 格子是否有雷
}

// 为Cell结构体实现一些方法
impl Cell {
    // 创建一个新的格子，初始状态为覆盖，没有雷
    fn new() -> Cell {
        Cell {
            state: CellState::Covered,
            has_mine: false,
        }
    }

    // 设置格子是否有雷
    fn set_mine(&mut self, has_mine: bool) {
        self.has_mine = has_mine;
    }

    // 切换格子的标记状态
    fn toggle_flag(&mut self) {
        match self.state {
            CellState::Covered => self.state = CellState::Flagged, // 如果是覆盖状态，就标记
            CellState::Flagged => self.state = CellState::Covered, // 如果是标记状态，就取消标记
            _ => (),                                               // 其他状态不做操作
        }
    }

    // 揭开格子，返回是否触雷
    fn uncover(&mut self, mines: u8) -> bool {
        match self.state {
            CellState::Covered => {
                if self.has_mine {
                    true // 如果有雷，就触雷
                } else {
                    self.state = CellState::Uncovered(mines); // 如果没有雷，就显示周围的雷数
                    false // 不触雷
                }
            }
            _ => false, // 其他状态不做操作，不触雷
        }
    }
}

// 定义一个函数，打印格子的显示字符
fn print_cell(cell: &Cell) {
    match cell.state {
        CellState::Covered => print!("■"),          // 覆盖状态显示方块
        CellState::Uncovered(0) => print!(" "),     // 揭开状态且周围没有雷显示空格
        CellState::Uncovered(n) => print!("{}", n), // 揭开状态且周围有雷显示雷数
        CellState::Flagged => print!("⚑"),          // 标记状态显示旗帜
    }
}
