import { Idl, Event, EventCoder } from "@project-serum/anchor";
import { IdlEvent } from "@project-serum/anchor/dist/cjs/idl";

export class MplTokenMetadataEventsCoder implements EventCoder {
  constructor(_idl: Idl) {}

  decode<E extends IdlEvent = IdlEvent, T = Record<string, string>>(
    _log: string
  ): Event<E, T> | null {
    throw new Error("MplTokenMetadata program does not have events");
  }
}
