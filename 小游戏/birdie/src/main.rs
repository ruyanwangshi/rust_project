use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Palying,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Palying
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO
        ctx.cls();
        ctx.print_centered(5, "huann ying lai dao wo de rust xiao you xi");
        ctx.print_centered(8, "(P) kai shi you xi ");
        ctx.print_centered(9, "(Q) tui chu you xi");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        // TODO
        ctx.cls();
        ctx.print_centered(5, "ni yi jing si le shi fou ji xu!");
        ctx.print_centered(8, "(P) kai shi you xi ");
        ctx.print_centered(9, "(Q) tui chu you xi");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx.cls(); // 游戏执行开始的时候先清理屏幕
        // ctx.print(1, 1, "Hello, huan ying lai dao wo de rust xiao you xi!"); // x 坐标系 x y 坐标系 y 打印东西到窗口上面

        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Palying => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50() // 建立游戏窗口
        .with_title("xiao niao~") // 游戏名称
        .build()
        .unwrap(); // 构建 ? 是unwrap的语法糖，也可以使用unwrap

    main_loop(context, State::new())
}
