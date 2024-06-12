### Packages

You can import these native Solana programs to your JS project and use it like any other [Mainstay](https://mainstay-lang.com) program.

**Example:**

```ts
import { splTokenProgram } from "@native-to-mainstay/spl-token";
import { BN } from "@project-serum/mainstay";

const program = splTokenProgram();
await program.methods
  .transfer(new BN(<TransferAmount>))
  .accounts({
    authority: <Authority>,
    destination: <DestinationTokenAddress>,
    source: <SourceTokenAddress>,
  })
  .rpc();
```

You can see more examples in the [tests](https://github.com/nxpkg/native-to-mainstay/tree/master/client/tests) directory.
