use crate::*;

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init, payer = player, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'_> {
    pub fn process(
        &mut self,
        player_pk: Pubkey,
        board_size: u8,
        number_of_snakes: u8,
        number_of_ladders: u8,
        is_private: bool,
        is_beatable: bool,
    ) -> Result<()> {
        let Self { game, .. } = self;
        msg!("game:init process");
        game.state = GameState::Waiting;
        let new_player=create_player(player_pk);
        game.players.push(new_player);
        game.board_size = board_size;
        game.number_of_ladders = number_of_ladders;
        game.number_of_snakes = number_of_snakes;
        game.is_private = is_private;
        game.is_beatable = is_beatable;

        self.init_board();

        Ok(())
    }
    pub fn init_board(&mut self) -> Result<()> {
        let Self { game, .. } = self;

        self.set_board_item();
        self.set_snakes_in_board();
        self.set_ladders_in_board();

        Ok(())
    }

    pub fn set_board_item(&mut self) {
        let Self { game, .. } = self;
        msg!("game:set board item");
        msg!("game:board size:{}", game.board_size.to_string());

        for i in 0..game.board_size+1 {
            game.board.push(BoardItem::default());
        }
        msg!("game:one item of board {:?}", game.board[2].state_board);
    }

    pub fn set_snakes_in_board(&mut self) -> Result<()> {
        msg!("game:set snakes in board");
        let Self { game, .. } = self;
        for i in 0..game.number_of_snakes {
            let pos_random = rand_range(1, (game.board_size - 1) as u64,(i^5) as u32 )?;
            msg!("game:pos random snake:{}", pos_random);
            let index_random = pos_random as usize;
            game.board[index_random].state_board = BoardState::Snake;
            let val_random = rand_range(1, (pos_random - 1)as u64 ,(i^7) as u32);
            game.board[index_random].value_board = val_random? as u8;
        }
        Ok(())
    }

    pub fn set_ladders_in_board(&mut self) -> Result<()> {
        msg!("game:set ladders in board");
        let Self { game, .. } = self;
        for i in 0..game.number_of_ladders {
            let pos_random = rand_range(1, (game.board_size - 1) as u64,(i^3) as u32)?;
            msg!("game:pos random ladder:{}", pos_random);
            let index_random = pos_random as usize;
            game.board[index_random].state_board = BoardState::Ladder;
            let val_random = rand_range(1, (game.board_size as u64 - pos_random - 1)as u64 ,(i^2) as u32)?;
            msg!("game:val random ladder:{}", val_random);
            game.board[index_random].value_board = val_random as u8;
        }
        Ok(())
    }
}
