import { Idl, TypesCoder } from "@project-serum/mainstay";

export class SplNameServiceTypesCoder implements TypesCoder {
  constructor(_idl: Idl) {}

  encode<T = any>(_name: string, _type: T): Buffer {
    throw new Error("SplNameService does not have user-defined types");
  }
  decode<T = any>(_name: string, _typeData: Buffer): T {
    throw new Error("SplNameService does not have user-defined types");
  }
}
