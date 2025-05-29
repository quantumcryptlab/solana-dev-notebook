import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { HelloWorld } from "../target/types/hello_world";
import { expect } from "chai";

describe("hello_world", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;

  it("Sends a hello world message", async () => {
    // Call the initialize function via RPC
    const tx = await program.methods.initialize().rpc();
    console.log("Transaction signature:", tx);
    
    // You can find the message in the transaction logs
    // The message will be: "Program log: Hello, World!"
  });
}); 