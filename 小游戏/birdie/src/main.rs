use bracket_lib::{prelude::*, random};

enum GameMode {
    Menu,
    Palying,
    End,
}

const SCREEN_WIDTH: i32 = 80; // 渲染画布宽度
const SCREEN_HEIGHT: i32 = 50; // 渲染画布高度
const FRAME_DURATION: f32 = 75.0; // 单位时间

// 当前游玩的对象的坐标信息
struct Player {
    x: i32,        // 水平位置
    y: i32,        // 垂直位置
    velocity: f32, // 垂直方向速度的属性
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            velocity: 0.0,
        }
    }

    // 渲染函数
    fn render(&mut self, ctx: &mut BTerm) {
        // 渲染的水平坐标， 纵向坐标， 前景色，背景色，把@字符转换成 unicode 437
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    // 这个是游戏角色的移动
    fn gravity_and_move(&mut self) {
        // 如果velocity 小于 2.0的时候
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // 先清除画布上面所有内容，并指定一种背景颜色。
        ctx.cls_bg(NAVY);
        // 这个地方frame_time_ms记录上次这个函数被调用一直到现在的时间
        // 最主要的作用就是让时间慢下来
        // 然后把这个时间段加到对应上一次游戏时间
        self.frame_time += ctx.frame_time_ms;

        // 如果这个frame_time时间就让frame_time时间重置
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            // 并且让游戏角色移动
            self.player.gravity_and_move();
        }

        // 如果用户按空格这个按键
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        // 并且一直渲染然后空格飞
        self.player.render(ctx);
        ctx.print(0, 0, "ji xu an kong ge fei"); // 游戏操作说明
        ctx.print(0, 1, &format!("dang qian ji fen {}", self.score)); // 游戏积分展示
        self.obstacle.render(ctx, self.player.x); // 渲染障碍物

        if self.player.x > self.obstacle.x { // 如果玩家当前的横坐标大于障碍的横坐标那么积分加一，并且初始化障碍物的横坐标
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_HEIGHT, self.score);
        }

        // 如果玩家的纵坐标大于当前游戏屏幕的高度，或者碰到障碍物那么就结束游戏
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Palying;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
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
        ctx.cls();
        ctx.print_centered(5, "ni yi jing si le shi fou ji xu!");
        ctx.print_centered(6, &format!("fen zhi wei {}", self.score));
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

// 添加障碍结构体
struct Obstacle {
    x: i32,     // 障碍的横坐标
    gap_y: i32, // 障碍中间的尺寸 // gap_y - (size / 2) 上面， gap_y + (size / 2) 下面
    size: i32,  // 障碍的尺寸
}

// 添加构造函数
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        // 伪随机库
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40), // 纵向的随机点放到随机的位置 10 - 40 之间不包含40
            size: i32::max(2, 20 - score), // 最小洞的尺寸，到最大洞的尺寸之间，随着玩家的积分越高洞就越小
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x; // 这个障碍的横坐标在生成的很坐标减去玩家当前的横坐标就是当前在屏幕上面的横坐标
        let half_size = self.size / 2; // 先计算障碍中间洞的大小

        // 然后生成上面障碍的大小
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // 生成下面障碍的大小
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }

    // 判断玩家是否撞到障碍物
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2; // 先计算中间洞的尺寸
        let does_x_match = player.x == self.x; // 然后判断玩家的横坐标是否等于当前障碍物所在的横坐标
        let player_above_gap = player.y < self.gap_y - half_size; // 如果玩家在上面障碍物y轴的范围之内
        let player_below_gap = player.y > self.gap_y + half_size; // 如果玩家在下面障碍物y轴的范围之内
        does_x_match && (player_above_gap || player_below_gap) // 必须y轴确定，然后确定是否在上面范围之内还是在下面范围之内
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50() // 建立游戏窗口
        .with_title("xiao niao~") // 游戏名称
        .build()
        .unwrap(); // 构建 ? 是unwrap的语法糖，也可以使用unwrap

    main_loop(context, State::new())
}
