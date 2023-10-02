use crate::*;
#[derive(Accounts)]
pub struct SnakesAndLaddersGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

impl<'info> SnakesAndLaddersGame<'info> {
    pub fn start_game(&mut self) {
        let Self { game, .. } = self;
        msg!("game:start");
        game.state = GameState::Start;
        game.players[0].state = PlayerState::MyTurn;
    }
    pub fn add_player(&mut self, player_pk: Pubkey) {
        let Self { game, .. } = self;
        msg!("game:add player");
        let new_player = create_player(player_pk);
        game.players.push(new_player);
    }

    pub fn move_player(&mut self, player_pk: Pubkey) -> Result<()> {
        let Self { game, .. } = self;
        msg!("game:move player");
        if game.state == GameState::Start {
            let board = game.board.clone();
            let board_size = game.board_size;
            let mut is_won=false;
            for i in 0..game.players.len() {
                msg!("game:index {}", i);
                msg!("game:public player:{}", game.players[i].pk);
                let player = &mut game.players[i];
                if player.pk == player_pk && player.state == PlayerState::MyTurn {
                    let move_step = rand_range(1, 6, 333)? as u8;
                    msg!("game:move step {}", move_step);
                    if player.position + move_step <= board_size {
                        player.position += (move_step as u8);
                        let board = board[player.position as usize];
                        if board.state_board == BoardState::Ladder {
                            player.position += (board.value_board as u8);
                        } else if board.state_board == BoardState::Snake {
                            player.position = (board.value_board as u8);
                        }
                    }
                    msg!("game:random position {}", player.position);
                    if player.position == board_size {
                        is_won=true;
                        player.state = PlayerState::Won;
                    } else {
                        player.state = PlayerState::Waiting;
                        if i + 1 >= game.players.len() {
                            game.players[0].state = PlayerState::MyTurn;
                        } else {
                            game.players[i + 1].state = PlayerState::MyTurn;
                        }
                    }
                    break;
                }
            }
            if is_won{
                game.state = GameState::End;
            }
        }
        Ok(())
    }
}
