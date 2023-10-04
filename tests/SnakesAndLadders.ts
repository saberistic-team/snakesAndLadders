import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SnakesAndLadders } from "../target/types/snakes_and_ladders";
import { Keypair } from '@solana/web3.js'
import fs from 'fs'
import { expect } from "chai";
function wait(seconds) {
  return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

describe("SnakesAndLadders", () => {

  const bytes = JSON.parse(fs.readFileSync('player.json'))
  const player = Keypair.fromSecretKey(new Uint8Array(bytes));
  const bytes_mehdi = JSON.parse(fs.readFileSync('mehdi.json'))
  const mehdi = Keypair.fromSecretKey(new Uint8Array(bytes_mehdi));
  const bytes_ash = JSON.parse(fs.readFileSync('ash.json'))
  const ash = Keypair.fromSecretKey(new Uint8Array(bytes_ash));
  console.log(player.publicKey.toBase58())



  const game = anchor.web3.Keypair.generate();
  // const playerOne = anchor.web3.Keypair.generate();
  // const playerTwo = anchor.web3.Keypair.generate();
  console.log('creating game: ', game.publicKey.toString());
  // console.log('player one game: ', playerOne.publicKey.toString());
  // console.log('player two game: ', playerTwo.publicKey.toString());
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SnakesAndLadders as Program<SnakesAndLadders>;


  // it("create user!", async () => {
  //   const tx = await program.methods.createUser("main player")
  //     .accounts({ user: player.publicKey })
  //     .signers([player])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  //   const user =await program.account.user.fetch(player.publicKey);
  //   console.log("test user:",user);
  // });

  // it("create user!", async () => {
  //   const tx = await program.methods.createUser("mehdi")
  //     .accounts({ user: mehdi.publicKey })
  //     .signers([mehdi])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  //   const user =await program.account.user.fetch(mehdi.publicKey);
  //   console.log("test user:",user);
  // });


  it("Is initialized!", async () => {
    const tx = await program.methods.initializeGame(player.publicKey, 70, 5, 5, false, false)
      .accounts({ game: game.publicKey })
      .signers([game])
      .rpc();
    console.log("Your transaction signature", tx);
    const data = await program.account.game.fetch(game.publicKey);
    console.log('board:', data.board);
  });

  const invite = anchor.web3.Keypair.generate();

  it("invite player", async () => {
    const tx = await program.methods.invitePlayer(game.publicKey,player.publicKey,mehdi.publicKey)
      .accounts({ invite: invite.publicKey,game:game.publicKey,invitee:mehdi.publicKey ,inviter:player.publicKey})
      .signers([invite])
      .rpc();
    console.log("Your transaction signature", tx);
    const data = await program.account.invite.fetch(invite.publicKey);
    console.log('board:', data);
  });



  // it("add player!", async () => {

  //   const tx = await program.methods.addPlayer(mehdi.publicKey).accounts({ game: game.publicKey }).rpc();
  // });

  // it("add player!", async () => {

  //   const tx = await program.methods.addPlayer(ash.publicKey).accounts({ game: game.publicKey }).rpc();
  // });

  // it("start game!", async () => {

  //   const tx = await program.methods.startGame().accounts({ game: game.publicKey }).rpc();
  // });


  // it("move player!", async () => {

  //   const tx = await program.methods.movePlayer(player.publicKey).accounts({ game: game.publicKey }).rpc();
  // });

  // it("move player!", async () => {

  //   const tx = await program.methods.movePlayer(mehdi.publicKey).accounts({ game: game.publicKey }).rpc();
  // });

  // it("move player!", async () => {

  //   const tx = await program.methods.movePlayer(ash.publicKey).accounts({ game: game.publicKey }).rpc();
  //   const data = await program.account.game.fetch(game.publicKey);
  //   console.log('players:', data.players);
  //   console.log('players:', data.state.end);
  // });






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