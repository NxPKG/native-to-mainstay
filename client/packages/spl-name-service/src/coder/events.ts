import { Idl, Event, EventCoder } from "@project-serum/mainstay";
import { IdlEvent } from "@project-serum/mainstay/dist/cjs/idl";

export class SplNameServiceEventsCoder implements EventCoder {
  constructor(_idl: Idl) {}

  decode<E extends IdlEvent = IdlEvent, T = Record<string, string>>(
    _log: string
  ): Event<E, T> | null {
    throw new Error("SplNameService program does not have events");
  }
}
