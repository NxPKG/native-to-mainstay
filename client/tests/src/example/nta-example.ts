import assert from "assert";
import { BN } from "@project-serum/mainstay";
import { Keypair, PublicKey } from "@solana/web3.js";

import { getProvider, loadKp, test, confirmTx } from "../utils";

export async function ntaExampleTests() {
  const provider = await getProvider();
  const kp = await loadKp();
}
