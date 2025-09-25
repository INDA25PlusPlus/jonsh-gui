use chess_api::{
    bitboards, perform_moves,
    state::{GameState, History},
    visualize,
};
use ggez::{
    Context, ContextBuilder, GameResult, conf, event,
    graphics::{self, Color, DrawParam, Image, Mesh, Rect, Text, TextFragment},
    winit::event_loop,
};
use std::{io, str::Chars};

struct Mainstate {
    state: GameState,
    history: History,
    last_move: Option<(i8, i8)>,
    grid: Vec<(f32, f32, Color)>,
    image: [Image; 12],
}

impl Mainstate {
    fn new(_ctx: &mut Context) -> GameResult<Mainstate> {
        let mut grid: Vec<(f32, f32, Color)> = Vec::new();
        for i in 0..=7 {
            for j in 0..=7 {
                if (j + i) % 2 == 1 {
                    grid.push((50.0 * i as f32, 50.0 * j as f32, Color::WHITE));
                } else {
                    grid.push((50.0 * i as f32, 50.0 * j as f32, Color::MAGENTA));
                }
            }
        }
        let image = [
            Image::from_path(_ctx, "/white-pawn.png").unwrap(),
            Image::from_path(_ctx, "/white-knight.png").unwrap(),
            Image::from_path(_ctx, "/white-bishop.png").unwrap(),
            Image::from_path(_ctx, "/white-rook.png").unwrap(),
            Image::from_path(_ctx, "/white-queen.png").unwrap(),
            Image::from_path(_ctx, "/white-king.png").unwrap(),
            Image::from_path(_ctx, "/black-pawn.png").unwrap(),
            Image::from_path(_ctx, "/black-knight.png").unwrap(),
            Image::from_path(_ctx, "/black-bishop.png").unwrap(),
            Image::from_path(_ctx, "/black-rook.png").unwrap(),
            Image::from_path(_ctx, "/black-queen.png").unwrap(),
            Image::from_path(_ctx, "/black-king.png").unwrap(),
        ];
        Ok(Mainstate {
            state: GameState::new(),
            history: History::new(),
            last_move: None,
            grid,
            image,
        })
    }

    fn make_move(&mut self, from: i8, to: i8) {
        perform_moves::make_move(from, to, &mut self.state, &mut self.history, true);
        self.last_move = Some((from, to));
    }
}

impl event::EventHandler for Mainstate {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut chars = input.trim().chars();
        let file_char = chars.next().unwrap();
        let rank_char = chars.next().unwrap();
        let rank = (rank_char as u8).wrapping_sub(b'1') as i8;
        let file = (file_char as u8).wrapping_sub(b'a') as i8;
        let f = (file) + (rank * 8);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut chars = input.trim().chars();
        let file_char = chars.next().unwrap();
        let rank_char = chars.next().unwrap();
        let rank = (rank_char as u8).wrapping_sub(b'1') as i8;
        let file = (file_char as u8).wrapping_sub(b'a') as i8;
        let t = (file) + (rank * 8);
        self.make_move(f, t);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb_u32(0xE83D84));

        visualize::print_board(&self.state.board);

        let text = Text::new(format!("Last Move: {:?}", self.last_move));

        canvas.draw(&text, graphics::DrawParam::default().dest([100.0, 100.0]));

        for (x, y, color) in &self.grid {
            let square = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(*x, *y, 50.0, 50.0),
                *color,
            )
            .unwrap();
            canvas.draw(&square, graphics::DrawParam::default());
            println!("{:?}", *x);
        }
        let white_bitboards = [
            (self.state.board.white_pawns, 0),
            (self.state.board.white_knights, 1),
            (self.state.board.white_bishops, 2),
            (self.state.board.white_rooks, 3),
            (self.state.board.white_queens, 4),
            (self.state.board.white_king, 5),
        ];
        for (piece, symbol) in white_bitboards.iter() {
            let mut m = 0;
            for l in 0..64 {
                if (piece >> l) & 1 == 1 {
                    let row = l / 8;
                    let col = l % 8;

                    let x = col as f32 * 50.0;
                    let y = (7 - row) as f32 * 50.0;
                    canvas.draw(
                        &self.image[*symbol],
                        DrawParam::default().dest([x, y]).scale([0.4, 0.4]),
                    );
                }
            }
        }
        let black_bitboards = [
            (self.state.board.black_pawns, 6),
            (self.state.board.black_knights, 7),
            (self.state.board.black_bishops, 8),
            (self.state.board.black_rooks, 9),
            (self.state.board.black_queens, 10),
            (self.state.board.black_king, 11),
        ];
        for (piece, symbol) in black_bitboards.iter() {
            for n in 0..64 {
                if (piece >> n) & 1 == 1 {
                    let row = n / 8;
                    let col = n % 8;

                    let x = col as f32 * 50.0;
                    let y = (7 - row) as f32 * 50.0;
                    canvas.draw(
                        &self.image[*symbol],
                        DrawParam::default().dest([x, y]).scale([0.4, 0.4]),
                    );
                }
            }
        }

        let pawns: Vec<bool> = (0..64)
            .map(|i| (self.state.board.black_pawns >> i) & 1 == 1)
            .collect();
        println!("{:?}", pawns);

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Chess", "Me")
        .window_setup(conf::WindowSetup::default().title("My First ggez App"))
        .window_mode(conf::WindowMode::default().dimensions(400.0, 400.0))
        .add_resource_path("./resources")
        .build()?;
    let state = Mainstate::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

// let mut state = GameState::new();
//     let mut history = History::new();
//     loop {
//         chess_api::visualize::print_board(&state.board);
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         let mut chars = input.trim().chars();
//         let file_char = chars.next().unwrap();
//         let rank_char = chars.next().unwrap();
//         let rank = (rank_char as u8).wrapping_sub(b'1') as i8;
//         let file = (file_char as u8).wrapping_sub(b'a') as i8;
//         let f = (file) + (rank * 8);
//         input.clear();
//         io::stdin().read_line(&mut input).unwrap();
//         let mut chars = input.trim().chars();
//         let file_char = chars.next().unwrap();
//         let rank_char = chars.next().unwrap();
//         let rank = (rank_char as u8).wrapping_sub(b'1') as i8;
//         let file = (file_char as u8).wrapping_sub(b'a') as i8;
//         let t = (file) + (rank * 8);
//         println!("{:?}", f);
//         println!("{:?}", t);
//         chess_api::perform_moves::make_move(f, t, &mut state, &mut history, true);
//     }
