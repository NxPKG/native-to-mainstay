import { Idl, Coder } from "@project-serum/mainstay";

import { SplTokenLendingAccountsCoder } from "./accounts";
import { SplTokenLendingEventsCoder } from "./events";
import { SplTokenLendingInstructionCoder } from "./instructions";
import { SplTokenLendingStateCoder } from "./state";
import { SplTokenLendingTypesCoder } from "./types";

/**
 * Coder for SplTokenLending
 */
export class SplTokenLendingCoder implements Coder {
  readonly accounts: SplTokenLendingAccountsCoder;
  readonly events: SplTokenLendingEventsCoder;
  readonly instruction: SplTokenLendingInstructionCoder;
  readonly state: SplTokenLendingStateCoder;
  readonly types: SplTokenLendingTypesCoder;

  constructor(idl: Idl) {
    this.accounts = new SplTokenLendingAccountsCoder(idl);
    this.events = new SplTokenLendingEventsCoder(idl);
    this.instruction = new SplTokenLendingInstructionCoder(idl);
    this.state = new SplTokenLendingStateCoder(idl);
    this.types = new SplTokenLendingTypesCoder(idl);
  }
}
