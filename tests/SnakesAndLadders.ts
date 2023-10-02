import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SnakesAndLadders } from "../target/types/snakes_and_ladders";
import { Keypair } from '@solana/web3.js'
import fs from 'fs'
function wait(seconds) {
  return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

describe("SnakesAndLadders", () => {

  const bytes = JSON.parse(fs.readFileSync('player.json'))
 const player = Keypair.fromSecretKey(new Uint8Array(bytes));
  console.log(player.publicKey.toBase58())



  const game = anchor.web3.Keypair.generate();
  // const playerOne = anchor.web3.Keypair.generate();
  // const playerTwo = anchor.web3.Keypair.generate();
  console.log('creating game: ', game.publicKey.toString());
  // console.log('player one game: ', playerOne.publicKey.toString());
  // console.log('player two game: ', playerTwo.publicKey.toString());
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SnakesAndLadders as Program<SnakesAndLadders>;



  it("Is initialized!", async () => {
    const tx = await program.methods.initializeGame(player.publicKey, 70,5,5,false,false)
      .accounts({ game: game.publicKey })
      .signers([game])
      .rpc();
    console.log("Your transaction signature", tx);
    const data = await program.account.game.fetch(game.publicKey);
    console.log('board:', data.board);
  });


  it("add player!", async () => {
    
      const tx = await program.methods.addPlayer(player.publicKey).accounts({ game: game.publicKey }).rpc();
      const data = await program.account.game.fetch(game.publicKey);
      console.log('players:', data.players);
  });





  // let flag = true;
  // it("move player!", async () => {
  //   while (flag) {
  //     await wait(2);
  //     // Add your test here.
  //     const tx = await program.methods.playerMoves().accounts({ game: game.publicKey }).rpc();
  //     const data = await program.account.game.fetch(game.publicKey);
  //     console.log('pos player one:', data.posPlayerOne);
  //     console.log('pos player two:', data.posPlayerTwo);
  //     console.log('state :', data.state);
  //     if (data.state.toString() == "PlayerOneWon" || data.state.toString() == "PlayerTwoWon") {
  //       flag = false;
  //     }
  //   }
  // });



});