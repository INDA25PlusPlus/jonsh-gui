use chess_api::{
    perform_moves::{self, is_legal},
    state::{GameState, History},
};
use ggez::{
    Context, ContextBuilder, GameResult, conf, event,
    graphics::{self, Color, DrawParam, Image, Mesh, Rect},
};

use std::net::{TcpListener, TcpStream};
use std::{
    env::set_current_dir,
    io::{Read, Write},
};
use std::{
    i8,
    io::{self, ErrorKind},
};
pub mod network;
#[derive(Debug, PartialEq, Eq)]
enum Connection {
    Server,
    Client,
}
struct Mainstate {
    state: GameState,
    history: History,
    last_move: Option<(i8, i8)>,
    grid: Vec<(f32, f32, Color)>,
    image: [Image; 12],
    selected_square: Option<(i8, i8)>,
    pos1: Option<i8>,
    pos2: Option<i8>,
    available_squares: Vec<i8>,
    connection: Connection,
    stream: TcpStream,
}
const X: f32 = 2.0;
impl Mainstate {
    fn new(_ctx: &mut Context, connection: Connection, stream: TcpStream) -> GameResult<Mainstate> {
        let mut grid: Vec<(f32, f32, Color)> = Vec::new();
        for i in 0..=7 {
            for j in 0..=7 {
                if (j + i) % 2 == 1 {
                    grid.push((X * 50.0 * i as f32, X * 50.0 * j as f32, Color::GREEN));
                } else {
                    grid.push((
                        X * 50.0 * i as f32,
                        X * 50.0 * j as f32,
                        Color::from_rgb_u32(0xE83D84),
                    ));
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
            selected_square: None,
            pos1: None,
            pos2: None,
            available_squares: Vec::new(),
            connection,
            stream,
        })
    }

    fn make_move(&mut self, from: i8, to: i8) {
        perform_moves::make_move(from, to, &mut self.state, &mut self.history, true);
        self.last_move = Some((from, to));
    }

    fn i_have_to_come_up_with_new_names_but_i_have_run_out_of_ideas(&self, index: i8) -> char {
        let piece_bitboards_2 = [
            (self.state.board.white_pawns, 'P'),
            (self.state.board.white_knights, 'N'),
            (self.state.board.white_bishops, 'B'),
            (self.state.board.white_rooks, 'R'),
            (self.state.board.white_queens, 'Q'),
            (self.state.board.white_king, 'K'),
            (self.state.board.black_pawns, 'p'),
            (self.state.board.black_knights, 'n'),
            (self.state.board.black_bishops, 'b'),
            (self.state.board.black_rooks, 'r'),
            (self.state.board.black_queens, 'q'),
            (self.state.board.black_king, 'k'),
        ];
        for (bitboard, name) in piece_bitboards_2.iter() {
            if (bitboard >> index) & 1 == 1 {
                return *name;
            }
        }
        return ' ';
    }

    fn get_fen(&self) -> String {
        let mut listsakgrej: Vec<char> = Vec::new();
        for k in 0..8 {
            let mut variable = 0;
            for i in 0..8 {
                if self.i_have_to_come_up_with_new_names_but_i_have_run_out_of_ideas(
                    (i) + ((7 - k) * 8),
                ) == ' '
                {
                    variable += 1;
                    if self.i_have_to_come_up_with_new_names_but_i_have_run_out_of_ideas(
                        (i) + ((7 - k) * 8) + 1,
                    ) != ' '
                        && i != 7
                    {
                        listsakgrej.push(char::from_digit(variable, 10).unwrap());
                        variable = 0;
                    } else if i == 7 {
                        listsakgrej.push(char::from_digit(variable, 10).unwrap());
                        variable = 0;
                    }
                } else {
                    listsakgrej.push(
                        self.i_have_to_come_up_with_new_names_but_i_have_run_out_of_ideas(
                            (i) + ((7 - k) * 8),
                        ),
                    );
                }
            }
            if k != 7 {
                listsakgrej.push('/');
            }
        }
        let the_message = listsakgrej.into_iter().collect::<String>();
        return the_message;
    }
    fn set_piece(&mut self, index: i8, piece: char) {
        self.state.board.white_pawns &= !(1 << index);
        self.state.board.white_bishops &= !(1 << index);
        self.state.board.white_knights &= !(1 << index);
        self.state.board.white_rooks &= !(1 << index);
        self.state.board.white_queens &= !(1 << index);
        self.state.board.white_king &= !(1 << index);
        self.state.board.black_pawns &= !(1 << index);
        self.state.board.black_bishops &= !(1 << index);
        self.state.board.black_knights &= !(1 << index);
        self.state.board.black_rooks &= !(1 << index);
        self.state.board.black_queens &= !(1 << index);
        self.state.board.black_king &= !(1 << index);
        match piece {
            'P' => self.state.board.white_pawns |= 1 << index,
            'B' => self.state.board.white_bishops |= 1 << index,
            'N' => self.state.board.white_knights |= 1 << index,
            'R' => self.state.board.white_rooks |= 1 << index,
            'Q' => self.state.board.white_queens |= 1 << index,
            'K' => self.state.board.white_king |= 1 << index,
            'p' => self.state.board.black_pawns |= 1 << index,
            'b' => self.state.board.black_bishops |= 1 << index,
            'n' => self.state.board.black_knights |= 1 << index,
            'r' => self.state.board.black_rooks |= 1 << index,
            'q' => self.state.board.black_queens |= 1 << index,
            'k' => self.state.board.black_king |= 1 << index,
            _ => {}
        }
    }
}

impl event::EventHandler for Mainstate {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if _ctx.mouse.button_just_pressed(event::MouseButton::Left) {
            if (self.connection == Connection::Client
                && self.state.side_to_move == chess_api::state::Color::White)
                || (self.connection == Connection::Server
                    && self.state.side_to_move == chess_api::state::Color::Black)
            {
                // Set row, col and pos to mouse position
                let row = (_ctx.mouse.position().y / (X * 50.0)) as i8;
                let col = (_ctx.mouse.position().x / (X * 50.0)) as i8;
                let pos = (7 - row) * 8 + col;

                // let mut first_notation = String::new();
                // let mut second_notation = String::new();

                //Set selected square to the most recently clicked square/tile
                if self.selected_square == Some((row, col)) {
                    self.selected_square = None;
                } else {
                    self.selected_square = Some((row, col));
                }

                //Set pos1 to clicked square/tile and process available squares
                if self.pos1 == None {
                    self.pos1 = Some(pos);
                    // first_notation = Some(((b'A' + (col as u8)) as char).to_string()).unwrap()
                    // + (8 - row).to_string().as_str();
                    // println!("First: {:?}", first_notation);
                    for blablabla in 0..64 {
                        if is_legal(self.pos1.unwrap(), blablabla, &self.state) {
                            self.available_squares.push(blablabla);
                        }
                    }
                } else if self.pos2 == None && self.pos1 != Some(pos) {
                    // second_notation = Some(((b'A' + (col as u8)) as char).to_string()).unwrap()
                    // + (8 - row).to_string().as_str();
                    self.pos2 = Some(pos);
                    // println!("Second: {:?}", second_notation);
                    self.available_squares = Vec::new();
                } else {
                    self.pos1 = None;
                    self.available_squares = Vec::new();
                }
                if self.pos1 != None && self.pos2 != None && self.pos1 != self.pos2 {
                    if is_legal(self.pos1.unwrap(), self.pos2.unwrap(), &self.state) {
                        self.make_move(self.pos1.unwrap(), self.pos2.unwrap());

                        let first_notation =
                            Some(((b'A' + ((self.pos1.unwrap() % 8) as u8)) as char).to_string())
                                .unwrap()
                                + (1 + ((self.pos1.unwrap() - (self.pos1.unwrap() % 8)) as f32
                                    / 8.0) as u8)
                                    .to_string()
                                    .as_str();
                        let second_notation =
                            Some(((b'A' + ((self.pos2.unwrap() % 8) as u8)) as char).to_string())
                                .unwrap()
                                + (1 + ((self.pos2.unwrap() - (self.pos2.unwrap() % 8)) as f32
                                    / 8.0) as u8)
                                    .to_string()
                                    .as_str();
                        let move_message = first_notation + second_notation.as_str();
                        let the_message = self.get_fen();
                        // let mut fein = Vec::new();
                        // let piece_bitboards = [
                        //     (self.state.board.white_pawns, "P"),
                        //     (self.state.board.white_knights, "N"),
                        //     (self.state.board.white_bishops, "B"),
                        //     (self.state.board.white_rooks, "R"),
                        //     (self.state.board.white_queens, "Q"),
                        //     (self.state.board.white_king, "K"),
                        //     (self.state.board.black_pawns, "p"),
                        //     (self.state.board.black_knights, "n"),
                        //     (self.state.board.black_bishops, "b"),
                        //     (self.state.board.black_rooks, "r"),
                        //     (self.state.board.black_queens, "q"),
                        //     (self.state.board.black_king, "k"),
                        // ];
                        // for (piece, name) in piece_bitboards.iter() {
                        //     for k in 0..64 {
                        //         if (piece >> k) & 1 == 1 {
                        //             fein.push((k, *name));
                        //         }
                        //     }
                        // }
                        // for (k, name) in fein {
                        //     println!("PLEASE HELP ME OH GOD: {:?}", name);
                        // }
                        // let mut im_a_motherfucking_baller_get_it_right_fool = Vec::new();
                        // for i in 0..64 {
                        //     for j in 0..32 {}
                        // }

                        // println!("FUCK THSI BULLLSHIT: {:?}", fein);
                        let mut gamestatevar = "0-0:";
                        if self::perform_moves::is_checkmate_stalemate(&mut self.state) {
                            if self::perform_moves::is_check(
                                &mut self.state,
                                chess_api::state::Color::White,
                            ) {
                                gamestatevar = "1-0:";
                            } else if self::perform_moves::is_check(
                                &mut self.state,
                                chess_api::state::Color::Black,
                            ) {
                                gamestatevar = "0-1:";
                            } else {
                                gamestatevar = "1-1:";
                            }
                        }
                        let mut the_message_that_i_use = "ChessMOVE:".to_owned()
                            + &move_message
                            + "0:"
                            + gamestatevar
                            + &the_message
                            + ":";
                        if the_message_that_i_use.len() != 128 {
                            println!("WHY");
                            for i in 0..(128 - the_message_that_i_use.len()) {
                                the_message_that_i_use.push('0');
                            }
                        }
                        println!("{}", the_message_that_i_use);
                        println!("{}", the_message_that_i_use.len());
                        self.stream.write(the_message_that_i_use.as_bytes())?;
                        (self.pos1, self.pos2) = (None, None);
                        //NOOOOOOOOOOOOOOO
                    } else {
                        self.pos1 = Some(pos);
                        for blablabla in 0..64 {
                            if is_legal(self.pos1.unwrap(), blablabla, &self.state) {
                                self.available_squares.push(blablabla);
                            }
                        }
                        self.pos2 = None;
                    }
                }
            }
        }
        let mut buffer = [0; 128];
        match self.stream.read_exact(&mut buffer) {
            // Ok(n) => {
            //     let message = str::from_utf8(&buffer[0..n])
            //         .unwrap()
            //         .chars()
            //         .map(|character| character.to_digit(18).unwrap())
            //         .collect::<Vec<_>>();
            //     let pos1: i8 = ((message[0] - 10) + ((message[1] as u32 - 1) * 8)) as i8;
            //     let pos2: i8 = ((message[2] - 10) + ((message[3] as u32 - 1) * 8)) as i8;
            //     self.make_move(pos1, pos2);
            // }
            Ok(()) => {
                let recieved_message = str::from_utf8(&buffer[0..128]).unwrap();
                let message_parts = recieved_message.split(":").collect::<Vec<&str>>();
                // let move_message = message_parts[1].chars().collect::<Vec<char>>();
                // let (from, to) = (
                //     ((move_message[0].to_digit(18).unwrap() - 10)
                //         + ((move_message[1].to_digit(18).unwrap() - 1) * 8))
                //         as i8,
                //     ((move_message[2].to_digit(18).unwrap() - 10)
                //         + ((move_message[3].to_digit(18).unwrap() - 1) * 8))
                //         as i8,
                // );
                // self.make_move(from, to);
                let full_fen = message_parts[3];
                if full_fen != &self.get_fen() {
                    let fen_parts = full_fen.split("/").enumerate();
                    for (i, line) in fen_parts {
                        let mut pos = (7 - i) * 8;
                        for k in line.chars() {
                            if k.is_ascii_digit() {
                                for _ in 0..(k.to_digit(10).unwrap()) {
                                    self.set_piece(pos as i8, ' ');
                                    pos += 1;
                                }
                            } else {
                                self.set_piece(pos as i8, k);
                                pos += 1;
                            }
                        }
                    }
                    self.state.side_to_move = self.state.side_to_move.opposite();
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{:?}", e),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb_u32(0xE83D84));

        for (x, y, color) in &self.grid {
            let square = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(*x, *y, X * 50.0, X * 50.0),
                *color,
            )
            .unwrap();
            canvas.draw(&square, graphics::DrawParam::default());
        }
        if let Some((row, col)) = self.selected_square {
            let colorrr: u32;
            if (row + col) % 2 == 1 {
                colorrr = 0x737373;
            } else {
                colorrr = 0xa22b5c;
            }
            let highlight = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(
                    col as f32 * 50.0 * X,
                    row as f32 * 50.0 * X,
                    50.0 * X,
                    50.0 * X,
                ),
                Color::from_rgb_u32(colorrr),
            )
            .unwrap();
            canvas.draw(&highlight, graphics::DrawParam::default());
        }
        for piece_position in &self.available_squares {
            let colorrr: u32;
            let row = piece_position / 8;
            let col = piece_position % 8;

            let x = col as f32 * 50.0 * X;
            let y = (7 - row) as f32 * 50.0 * X;

            if (row + col) % 2 == 0 {
                colorrr = 0x737373;
            } else {
                colorrr = 0xa22b5c;
            }

            let circles = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [x + 25.0 * X, y + 25.0 * X],
                15.0 * X,
                0.1,
                Color::from_rgb_u32(colorrr),
            )
            .unwrap();

            canvas.draw(&circles, graphics::DrawParam::default());
        }
        let piece_bitboards = [
            (self.state.board.white_pawns, 0),
            (self.state.board.white_knights, 1),
            (self.state.board.white_bishops, 2),
            (self.state.board.white_rooks, 3),
            (self.state.board.white_queens, 4),
            (self.state.board.white_king, 5),
            (self.state.board.black_pawns, 6),
            (self.state.board.black_knights, 7),
            (self.state.board.black_bishops, 8),
            (self.state.board.black_rooks, 9),
            (self.state.board.black_queens, 10),
            (self.state.board.black_king, 11),
        ];
        for (piece, symbol) in piece_bitboards.iter() {
            for k in 0..64 {
                if (piece >> k) & 1 == 1 {
                    let row = k / 8;
                    let col = k % 8;

                    let x = col as f32 * 50.0 * X;
                    let y = (7 - row) as f32 * 50.0 * X;
                    canvas.draw(
                        &self.image[*symbol],
                        DrawParam::default().dest([x, y]).scale([0.4 * X, 0.4 * X]),
                    );
                }
            }
        }
        // ctx.gfx
        //     .add_font("FFFnt", graphics::FontData::from_path(ctx, "/font2.ttf")?);
        // let mut text = Text::new("HELLO");
        // text.set_scale(100.0);

        // canvas.draw(
        //     &text,
        //     DrawParam::default()
        //         .dest([500.0, 1000.0])
        //         .color(Color::WHITE),
        // );
        // text.set_font("FFFnt");
        // canvas.draw(
        //     &text,
        //     DrawParam::default()
        //         .dest([100.0, 1000.0])
        //         .color(Color::WHITE),
        // );

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let connection: Connection;
    let stream: TcpStream;
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    if text.trim() == "server" {
        connection = Connection::Server;
        let game_listener = TcpListener::bind("127.0.0.1:7878")?;
        // game_listener.set_nonblocking(false);
        stream = game_listener.accept()?.0;
        println!("Server");
    } else {
        connection = Connection::Client;
        let game_listener = TcpStream::connect("127.0.0.1:7878")?;
        stream = game_listener;
        println!("Client");
    }
    stream.set_nonblocking(true)?;
    println!("{:?}", connection);
    // GAME IS CLIENT
    // let mut text = String::new();
    // let stream = TcpStream::connect("127.0.0.1:7878")?;
    // GAME IS CLIENT

    // GAME IS SERVER
    // GAME IS SERVER
    let (mut ctx, event_loop) = ContextBuilder::new("Chess", "Me")
        .window_setup(conf::WindowSetup::default().title("My First ggez App"))
        .window_mode(conf::WindowMode::default().dimensions(400.0 * X, 400.0 * X))
        .add_resource_path("./resources")
        .build()?;

    // GAME IS CLIENT
    // let state = Mainstate::new(&mut ctx, stream)?;
    // GAME IS CLIENT

    // GAME IS SERVER
    let state = Mainstate::new(&mut ctx, connection, stream)?;
    // GAME IS SERVER

    event::run(ctx, event_loop, state)
}
