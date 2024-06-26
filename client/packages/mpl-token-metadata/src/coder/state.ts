import { Idl, StateCoder } from "@project-serum/mainstay";

export class MplTokenMetadataStateCoder implements StateCoder {
  constructor(_idl: Idl) {}

  encode<T = any>(_name: string, _account: T): Promise<Buffer> {
    throw new Error("MplTokenMetadata does not have state");
  }
  decode<T = any>(_ix: Buffer): T {
    throw new Error("MplTokenMetadata does not have state");
  }
}
