const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("testprogram", () => {
  // Use a local provider.
  const provider = anchor.AnchorProvider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  it("Is initialized!", async () => {

    const program = anchor.workspace.Testprogram;

    // The Account to create.
    const myAccount = anchor.web3.Keypair.generate();

    await program.rpc.initialize({
      accounts: {
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [myAccount],
    });

    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    assert.ok(account.data, "Empty");
    _myAccount = myAccount;
  });


  it("Wrote to the account", async () => {
    const myAccount = _myAccount;

    const program = anchor.workspace.Testprogram;

    await program.rpc.write("hello world", {
      accounts: {
        myAccount: myAccount.publicKey,
      }
    })

    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    assert.equal(account.data, "hello world");

  });
});