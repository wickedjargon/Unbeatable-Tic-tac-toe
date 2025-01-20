// Tic-tac-toe
// Computer Tic-tac-toe written in Rust.
// Uses bitboards and the minimax algorithm.

// by Farzin Firouzi <farzineff@gmail.com>.
// License: GNU GPLv3 (or any later version).

use raylib::prelude::*;
use std::i8;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
#[derive(Clone)]
enum Player {
    X, // human player
    O, // computer player
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
enum Outcome {
    XWins(u16),
    OWins(u16),
    Tie,
    NotTerminal,
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
struct Board {
    x_board: u16,
    o_board: u16,
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
struct GameState {
    board: Board,
    turn: Player,
    outcome: Outcome,
}

#[allow(dead_code)]
impl GameState {
    fn new() -> GameState {
        GameState {
            board: Board {
                x_board: 0b000_000_000,
                o_board: 0b000_000_000,
            },
            outcome: Outcome::NotTerminal,
            turn: Player::X,
        }
    }

    fn get_available_cells(&self) -> Vec<u8> {
        (0..9)
            .filter(|&cell| {
                let bit_mask = 1 << cell;
                (&self.board.x_board & bit_mask == 0) && (&self.board.o_board & bit_mask == 0)
            })
            .collect()
    }

    fn print_board(&self) {
        for i in 0..3 {
            for j in 0..3 {
                let cell_number = i * 3 + j;
                let bit_mask = 1 << cell_number;

                if self.board.x_board & bit_mask != 0 {
                    print!("X");
                } else if self.board.o_board & bit_mask != 0 {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn get_computer_move(&self) -> u8 {
        let available_cells = self.get_available_cells();
        let mut final_eval = i8::MIN;
        let mut final_move = 0;
        let mut temp_eval;
        for available_cell in available_cells {
            let mut temp_game_state = self.clone();
            temp_game_state.make_move(available_cell);
            temp_game_state.update_outcome();
            temp_game_state.switch_player();
            temp_eval = temp_game_state.evaluate(false);
            if temp_eval > final_eval {
                final_eval = temp_eval;
                final_move = available_cell;
            }
        }
        return final_move;
    }

    fn evaluate(&self, is_maximizig: bool) -> i8 {
        if let Outcome::OWins(_) = self.outcome {
            return 1;
        }
        if let Outcome::XWins(_) = self.outcome {
            return -1;
        }
        if Outcome::Tie == self.outcome {
            return 0;
        }
        if is_maximizig == false {
            // this is the X player. minimizing player
            let mut final_eval = i8::MAX;
            let mut temp_eval;
            let available_cells = self.get_available_cells();
            for available_cell in available_cells {
                let mut temp_game_state = self.clone();
                temp_game_state.make_move(available_cell);
                temp_game_state.update_outcome();
                temp_game_state.switch_player();
                temp_eval = temp_game_state.evaluate(true);
                if temp_eval < final_eval {
                    final_eval = temp_eval;
                }
            }
            return final_eval;
        }
        if is_maximizig == true {
            let mut final_eval = i8::MIN;
            let mut temp_eval;
            let available_cells = self.get_available_cells();
            for available_cell in available_cells {
                let mut temp_game_state = self.clone();
                temp_game_state.make_move(available_cell);
                temp_game_state.update_outcome();
                temp_game_state.switch_player();
                temp_eval = temp_game_state.evaluate(false);
                if temp_eval > final_eval {
                    final_eval = temp_eval;
                }
            }
            return final_eval;
        }
        unreachable!();
    }

    fn computer_move(&mut self) {
        let bit_mask = 1 << self.get_computer_move();

        if self.board.x_board & bit_mask == 0 && self.board.o_board & bit_mask == 0 {
            self.board.o_board |= bit_mask;
        }
    }

    fn make_move(&mut self, cell_number: u8) -> bool {
        let bit_mask = 1 << cell_number;
        if self.board.x_board & bit_mask == 0
            && self.board.o_board & bit_mask == 0
            && self.turn == Player::X
        {
            self.board.x_board |= bit_mask;
            return true;
        } else if self.board.x_board & bit_mask == 0
            && self.board.o_board & bit_mask == 0
            && self.turn == Player::O
        {
            self.board.o_board |= bit_mask;
            return true;
        }
        return false;
    }

    fn update_outcome(&mut self) {
        const WINNING_COMBINATIONS: [u16; 8] = [
            0b111_000_000, // Top row
            0b000_111_000, // Middle row
            0b000_000_111, // Bottom row
            0b100_100_100, // Left column
            0b010_010_010, // Middle column
            0b001_001_001, // Right column
            0b100_010_001, // Diagonal \
            0b001_010_100, // Diagonal /
        ];

        for &combo in &WINNING_COMBINATIONS {
            if self.board.x_board & combo == combo {
                self.outcome = Outcome::XWins(combo);
                return;
            }
            if self.board.o_board & combo == combo {
                self.outcome = Outcome::OWins(combo);
                return;
            }
        }

        // If all cells are filled, and no winner
        if (self.board.x_board | self.board.o_board) == 0b111_111_111 {
            self.outcome = Outcome::Tie;
        }
    }

    fn switch_player(&mut self) {
        match self.turn {
            Player::O => self.turn = Player::X,
            Player::X => self.turn = Player::O,
        }
    }
}

fn get_cell_number(mouse_position: Vector2) -> Option<u8> {
    if mouse_position.x < 0.0
        || mouse_position.x > BOARD_SIZE as f32
        || mouse_position.y < 0.0
        || mouse_position.y > BOARD_SIZE as f32
    {
        return Option::None;
    }

    let column = (mouse_position.x / CELL_SIZE as f32).floor() as u8;
    let row = (mouse_position.y / CELL_SIZE as f32).floor() as u8;
    Option::Some(row * 3 + column)
}

const BOARD_SIZE: i32 = 300;
const CELL_SIZE: i32 = BOARD_SIZE / 3;
const LINE_COLOR: Color = Color::GRAY;
const TITLE: &str = "Tic Tac Toe";
const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(BOARD_SIZE, BOARD_SIZE)
        .title(TITLE)
        .build();
    let mut game_state = GameState::new();

    while !rl.window_should_close() {
        if game_state.outcome == Outcome::NotTerminal
            && game_state.turn == Player::X
            && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let mouse_position = rl.get_mouse_position();
            if let Some(cell_number) = get_cell_number(mouse_position) {
                let is_valid = game_state.make_move(cell_number);
                if is_valid == true {
                    game_state.update_outcome();
                    if game_state.outcome == Outcome::NotTerminal {
                        game_state.switch_player();
                        game_state.computer_move();
                        game_state.update_outcome();
                        if game_state.outcome == Outcome::NotTerminal {
                            game_state.switch_player(); // Only switch if game is still ongoing
                        }
                    }
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        // First vertical line
        d.draw_line(CELL_SIZE, 0, CELL_SIZE, CELL_SIZE * 3, LINE_COLOR);

        // Second vertical line
        d.draw_line(CELL_SIZE * 2, 0, CELL_SIZE * 2, CELL_SIZE * 3, LINE_COLOR);

        // First horizontal line
        d.draw_line(0, CELL_SIZE, CELL_SIZE * 3, CELL_SIZE, LINE_COLOR);

        // Second horizontal line
        d.draw_line(0, CELL_SIZE * 2, CELL_SIZE * 3, CELL_SIZE * 2, LINE_COLOR);

        for i in 0..3 {
            for j in 0..3 {
                let pos_x = j * CELL_SIZE + CELL_SIZE / 2;
                let pos_y = i * CELL_SIZE + CELL_SIZE / 2;
                let cell_number = i * 3 + j;
                let bit_mask = 1 << cell_number;

                let mut color_x = Color::RED;
                let mut color_o = Color::BLUE;

                // Check for a winner and update colors
                match game_state.outcome {
                    Outcome::XWins(combo) if combo & bit_mask != 0 => {
                        color_x = Color::PINK;
                    }
                    Outcome::OWins(combo) if combo & bit_mask != 0 => {
                        color_o = Color::LIGHTBLUE;
                    }
                    _ => {}
                }

                if game_state.board.x_board & bit_mask != 0 {
                    d.draw_text("X", pos_x - 10, pos_y - 10, 40, color_x);
                } else if game_state.board.o_board & bit_mask != 0 {
                    d.draw_text("O", pos_x - 10, pos_y - 10, 40, color_o);
                }
            }
        }
        match game_state.outcome {
            Outcome::OWins(_) => {
                let text = "Computer Wins!";
                let font_size = 30;
                let char_width = font_size / 2; // Approximation for text centering
                let text_width = text.len() as i32 * char_width;
                let text_x = (BOARD_SIZE - text_width) / 2;
                let text_y = (BOARD_SIZE - font_size) / 2;
                let padding = 10; // Padding around text inside the box

                // Draw a semi-transparent box around the text
                d.draw_rectangle(
                    text_x - padding,
                    text_y - padding,
                    text_width + 2 * padding,
                    font_size + 2 * padding,
                    Color::BLACK.fade(0.8),
                );

                // Draw the main text
                d.draw_text(text, text_x, text_y, font_size, Color::CYAN);
            }
            Outcome::XWins(_) => {
                let text = "User Wins!";
                let font_size = 30;
                let char_width = font_size / 2;
                let text_width = text.len() as i32 * char_width;
                let text_x = (BOARD_SIZE - text_width) / 2;
                let text_y = (BOARD_SIZE - font_size) / 2;
                let padding = 10;

                // Draw a semi-transparent box around the text
                d.draw_rectangle(
                    text_x - padding,
                    text_y - padding,
                    text_width + 2 * padding,
                    font_size + 2 * padding,
                    Color::BLACK.fade(0.8),
                );

                // Draw the main text
                d.draw_text(text, text_x, text_y, font_size, Color::CYAN);
            }
            Outcome::Tie => {
                let text = "Tie Game!";
                let font_size = 30;
                let char_width = font_size / 2;
                let text_width = text.len() as i32 * char_width;
                let text_x = (BOARD_SIZE - text_width) / 2;
                let text_y = (BOARD_SIZE - font_size) / 2;
                let padding = 10;

                // Draw a semi-transparent box around the text
                d.draw_rectangle(
                    text_x - padding,
                    text_y - padding,
                    text_width + 2 * padding,
                    font_size + 2 * padding,
                    Color::BLACK.fade(0.8),
                );

                // Draw the main text
                d.draw_text(text, text_x, text_y, font_size, Color::CYAN);
            }
            Outcome::NotTerminal => {}
        }
    }
}
