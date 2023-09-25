import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SnakesAndLadders } from "../target/types/snakes_and_ladders";
function wait(seconds) {
  return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

describe("SnakesAndLadders", () => {
  // 1 - Generate a new Keypair for the Game Account
  const game = anchor.web3.Keypair.generate();
  const playerOne = anchor.web3.Keypair.generate();
  const playerTwo = anchor.web3.Keypair.generate();
  console.log('creating game: ', game.publicKey.toString());
  console.log('player one game: ', playerOne.publicKey.toString());
  console.log('player two game: ', playerTwo.publicKey.toString());
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SnakesAndLadders as Program<SnakesAndLadders>;


  console.log(program);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeGame(playerOne.publicKey, playerTwo.publicKey)
      // 3a - Pass the game public key into our accounts context
      .accounts({ game: game.publicKey })
      // 3b - Append the game keypair's signature to transfer authority to our program
      .signers([game])
      .rpc();
    console.log("Your transaction signature", tx);
    //4. Query the chain for our data account and log its value
    const data = await program.account.game.fetch(game.publicKey);
    console.log('player one:', data.playerOne.toString());
    console.log('player two:', data.playerTwo.toString());
    // console.log('rand', data.rand.toNumber());
  });






  it("init board!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeBoard().accounts({ game: game.publicKey }).rpc();
    console.log("Your transaction signature", tx);
    //4. Query the chain for our data account and log its value
    const data = await program.account.game.fetch(game.publicKey);
    console.log('greeted', data.board);
  });

  let flag = true;
  it("move player!", async () => {
    while (flag) {
      await wait(2);
      // Add your test here.
      const tx = await program.methods.playerMoves().accounts({ game: game.publicKey }).rpc();
      const data = await program.account.game.fetch(game.publicKey);
      console.log('pos player one:', data.posPlayerOne);
      console.log('pos player two:', data.posPlayerTwo);
      console.log('state :', data.state);
      if (data.state.toString() == "PlayerOneWon" || data.state.toString() == "PlayerTwoWon") {
        flag = false;
      }
    }
  });


  // it("move player!", async () => {
  // await wait(2);
  //   // Add your test here.
  //   const tx = await program.methods.movePlayer().accounts({ game: game.publicKey }).rpc();
  //   const data = await program.account.game.fetch(game.publicKey);
  //   console.log('pos player one:', data.posPlayerOne);
  //   console.log('pos player two:', data.posPlayerTwo);
  // });


  // it("move player!", async () => {
  // await wait(2);
  //   // Add your test here.
  //   const tx = await program.methods.movePlayer().accounts({ game: game.publicKey }).rpc();
  //   const data = await program.account.game.fetch(game.publicKey);
  //   console.log('pos player one:', data.posPlayerOne);
  //   console.log('pos player two:', data.posPlayerTwo);
  // });


  // it("move player!", async () => {
  //   await wait(2);
  //   // Add your test here.
  //   const tx = await program.methods.movePlayer().accounts({ game: game.publicKey }).rpc();
  //   const data = await program.account.game.fetch(game.publicKey);
  //   console.log('pos player one:', data.posPlayerOne);
  //   console.log('pos player two:', data.posPlayerTwo);
  // });

});