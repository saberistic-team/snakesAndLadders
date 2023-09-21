use crate::*;
#[derive(Accounts)]
pub struct SnakesAndLaddersGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

impl<'info> SnakesAndLaddersGame<'info> {
    #[access_control(Self::constraints(&self))]
    pub fn process(&mut self) -> Result<()> {
        let Self { game, .. } = self;
        msg!("program:------------------------------------------------------------");
        for i in 1..6 {
            let rng = Rand::new(8);
            let pos_random = rng?.rand_range(10, 88);//get_random(10, 88, i).unwrap();
            let index_random = pos_random as usize;
            game.board[index_random].type_board = BoardState::Snake;
            let rng2 = Rand::new(8);
            let val_random = rng2?.rand_range(1, pos_random - 1);//get_random(1, pos_random - 1, i).unwrap();
            game.board[index_random].value_board = val_random as u8;
        }
        msg!("program:------------------------------------------------------------");
        // check later that the start and end should not be the same
        for i in 6..12 {
            let rng = Rand::new(8);
            let mut pos_random = rng?.rand_range(10, 88);//get_random(10, 88, i).unwrap();
            let mut index_random = pos_random as usize;
            while game.board[index_random].type_board != BoardState::Empty {
            let rng2 = Rand::new(8);
                pos_random = rng2?.rand_range(10, 88);//get_random(10, 88, i).unwrap();
                index_random = pos_random as usize;
            }
            game.board[index_random].type_board = BoardState::Stair;
            let rng3 = Rand::new(8);
            let val_random = rng3?.rand_range(1, 88 - pos_random);//get_random(1, 88 - pos_random, i).unwrap();
            game.board[index_random].value_board = val_random as u8;
        }
        msg!("program:------------------------------------------------------------");
       

        Ok(())
    }

    pub fn constraints(&self) -> Result<()> {
        msg!("constraints join");

        Ok(())
    }
    pub fn move_player(&mut self) -> Result<()> { 
        let mut rng = Rand::new(32);
        // let v: Vec<u32> = (0..100).map(|i| rng.rand_range(1, 6)).collect();
        // msg!("program:{:?}", v);
        let Self { game, .. } = self;
        if game.state != GameState::PlayerOneWon
            && game.state != GameState::PlayerTwoWon
            && game.state == GameState::PlayerOne
        {
            // let mut dice=get_random(1,6,5).unwrap();
            let mut dice = rng?.rand_range(1, 6);
            msg!("program: dice for player one:{}!", dice);
            let move_step = game.pos_player_one + (dice as u8);
            game.pos_player_one = move_step;
            if move_step <= 89 {
                game.pos_player_one = move_step;
                if game.board[game.pos_player_one as usize].type_board == BoardState::Stair {
                    game.pos_player_one =
                        game.pos_player_one + game.board[game.pos_player_one as usize].value_board;
                } else if game.board[game.pos_player_one as usize].type_board == BoardState::Snake {
                    game.pos_player_one =
                        game.pos_player_one - game.board[game.pos_player_one as usize].value_board;
                }
                if game.pos_player_one == 89 {
                    game.state = GameState::PlayerOneWon;
                } else {
                    game.state = GameState::PlayerTwo;
                }
            } else {
                game.state = GameState::PlayerTwo;
            }
        } else if game.state != GameState::PlayerOneWon && game.state != GameState::PlayerTwoWon {
            // let mut dice=get_random(1,6,1).unwrap();
            let mut dice = rng?.rand_range(1, 6);
            msg!("program: dice for player two:{}!", dice);
            let move_step = game.pos_player_two + (dice as u8);

            if move_step <= 89 {
                game.pos_player_two = move_step;
                if game.board[game.pos_player_two as usize].type_board == BoardState::Stair {
                    game.pos_player_two =
                        game.pos_player_two + game.board[game.pos_player_two as usize].value_board;
                } else if game.board[game.pos_player_two as usize].type_board == BoardState::Snake {
                    game.pos_player_two =
                        game.pos_player_two - game.board[game.pos_player_two as usize].value_board;
                }
                if game.pos_player_two == 89 {
                    game.state = GameState::PlayerTwoWon;
                } else {
                    game.state = GameState::PlayerOne;
                }
            } else {
                game.state = GameState::PlayerOne;
            }
        }

       Ok(())
   }
}

pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rand {
    pub fn new(seed: u32) -> Result<Rand> {
        let slot = Clock::get()?.slot;
        let KX: u32 = slot as u32;
        let KY: u32 = (slot ^ 2) as u32;
        let KZ: u32 = (slot ^ 22) as u32;
        let KW: u32 = (slot ^ 23) as u32;
        Ok(Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        })
    }

    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }
    pub fn rand_range(&mut self, a: u32, b: u32) -> u32 {
        let m = (b - a + 1) as u32;
        let result = a + (self.rand() % m) as u32;
        return result;
    }
}