pub mod accounts {
    pub const FILENAME: &str = "accounts.ts";
    pub const CODER_TYPE: &str = "AccountsCoder";
    pub const CONTENT: &str = r#"// @ts-nocheck
import * as B from "@native-to-mainstay/buffer-layout";
import { AccountsCoder, Idl } from "@project-serum/mainstay";
import { IdlTypeDef } from "@project-serum/mainstay/dist/cjs/idl";

export class <ProgramName>AccountsCoder<A extends string = string>
  implements AccountsCoder
{
  constructor(_idl: Idl) {}

  public async encode<T = any>(accountName: A, account: T): Promise<Buffer> {
    switch (accountName) {
      <EncodeCases>
      default: {
        throw new Error(`Invalid account name: ${accountName}`);
      }
    }
  }

  public decode<T = any>(accountName: A, ix: Buffer): T {
    return this.decodeUnchecked(accountName, ix);
  }

  public decodeUnchecked<T = any>(accountName: A, ix: Buffer): T {
    switch (accountName) {
      <DecodeCases>
      default: {
        throw new Error(`Invalid account name: ${accountName}`);
      }
    }
  }

  public memcmp(
    accountName: A,
    _appendData?: Buffer
  ): { dataSize?: number, offset?: number, bytes?: string } {
    switch (accountName) {
      <MemCmpCases>
      default: {
        throw new Error(`Invalid account name: ${accountName}`);
      }
    }
  }

  public size(idlAccount: IdlTypeDef): number {
    switch (idlAccount.name) {
      <SizeCases>
      default: {
        throw new Error(`Invalid account name: ${idlAccount.name}`);
      }
    }
  }
}

<DecodeFunctions>

<AccountLayouts>"#;
}

pub mod events {
    pub const FILENAME: &str = "events.ts";
    pub const CODER_TYPE: &str = "EventsCoder";
    pub const CONTENT: &str = r#"import { Idl, Event, EventCoder } from "@project-serum/mainstay";
import { IdlEvent } from "@project-serum/mainstay/dist/cjs/idl";

export class <ProgramName>EventsCoder implements EventCoder {
  constructor(_idl: Idl) {}

  decode<E extends IdlEvent = IdlEvent, T = Record<string, string>>(
    _log: string
  ): Event<E, T> | null {
    throw new Error("<ProgramName> program does not have events");
  }
}"#;
}

pub mod index {
    pub const FILENAME: &str = "index.ts";
    pub const CODER_TYPE: &str = "Coder";
    pub const CONTENT: &str = r#"import { Idl, Coder } from "@project-serum/mainstay"

import { <ProgramName>AccountsCoder } from "./accounts";
import { <ProgramName>EventsCoder } from "./events";
import { <ProgramName>InstructionCoder } from "./instructions";
import { <ProgramName>StateCoder } from "./state";
import { <ProgramName>TypesCoder } from "./types";

/**
 * Coder for <ProgramName>
 */
export class <ProgramName>Coder implements Coder {
  readonly accounts: <ProgramName>AccountsCoder;
  readonly events: <ProgramName>EventsCoder;
  readonly instruction: <ProgramName>InstructionCoder;
  readonly state: <ProgramName>StateCoder;
  readonly types: <ProgramName>TypesCoder;

  constructor(idl: Idl) {
    this.accounts = new <ProgramName>AccountsCoder(idl);
    this.events = new <ProgramName>EventsCoder(idl);
    this.instruction = new <ProgramName>InstructionCoder(idl);
    this.state = new <ProgramName>StateCoder(idl);
    this.types = new <ProgramName>TypesCoder(idl);
  }
}
"#;
}

pub mod instructions {
    pub const FILENAME: &str = "instructions.ts";
    pub const CODER_TYPE: &str = "InstructionCoder";
    pub const CONTENT: &str = r#"// @ts-nocheck
import * as B from "@native-to-mainstay/buffer-layout";
import { Idl, InstructionCoder } from "@project-serum/mainstay";

export class <ProgramName>InstructionCoder implements InstructionCoder {
  constructor(_idl: Idl) {}

  encode(ixName: string, ix: any): Buffer {
    switch (ixName) {
      <Cases>
      default: {
        throw new Error(`Invalid instruction: ${ixName}`);
      }
    }
  }

  encodeState(_ixName: string, _ix: any): Buffer {
    throw new Error("<ProgramName> does not have state");
  }
}

<Functions>

const LAYOUT = B.union(B.u8("instruction"));
<Layouts>

function encodeData(ix: any, span: number): Buffer {
  const b = Buffer.alloc(span);
  LAYOUT.encode(ix, b);
  return b;
}
"#;
}

pub mod state {
    pub const FILENAME: &str = "state.ts";
    pub const CODER_TYPE: &str = "StateCoder";
    pub const CONTENT: &str = r#"import { Idl, StateCoder } from "@project-serum/mainstay";

export class <ProgramName>StateCoder implements StateCoder {
  constructor(_idl: Idl) {}

  encode<T = any>(_name: string, _account: T): Promise<Buffer> {
    throw new Error("<ProgramName> does not have state");
  }
  decode<T = any>(_ix: Buffer): T {
    throw new Error("<ProgramName> does not have state");
  }
}"#;
}

pub mod types {
    pub const FILENAME: &str = "types.ts";
    pub const CODER_TYPE: &str = "TypesCoder";
    pub const CONTENT: &str = r#"import { Idl, TypesCoder } from "@project-serum/mainstay";

export class <ProgramName>TypesCoder implements TypesCoder {
  constructor(_idl: Idl) {}
  
  encode<T = any>(_name: string, _type: T): Buffer {
    throw new Error("<ProgramName> does not have user-defined types");
  }
  decode<T = any>(_name: string, _typeData: Buffer): T {
    throw new Error("<ProgramName> does not have user-defined types");
  }
}"#;
}

pub mod src {
    pub const INDEX_FILENAME: &str = "index.ts";
    pub const INDEX_CONTENT: &str = r#"export * from "./program""#;

    pub const PROGRAM_FILENAME: &str = "program.ts";
    pub const PROGRAM_CONTENT: &str = r#"import { PublicKey } from "@solana/web3.js";
import { Program, MainstayProvider } from "@project-serum/mainstay";

import { <ProgramName>Coder } from "./coder";

<ProgramId>

interface GetProgramParams {
  programId?: PublicKey;
  provider?: MainstayProvider;
}

export function <ProgramNameCamel>Program(
  params?: GetProgramParams
): Program<<ProgramName>> {
  return new Program<<ProgramName>>(
    IDL,
    params?.programId ?? <ProgramIdName>,
    params?.provider,
    new <ProgramName>Coder(IDL)
  );
}

<Type>

<Idl>
"#;
}

pub mod package {
    pub const PACKAGE_JSON_FILENAME: &str = "package.json";
    pub const PACKAGE_JSON_CONTENT: &str = r#"{
  "name": "<Name>",
  "description": "<Description>",
  "version": "<Version>",
  "author": "<Author>",
  "license": "<License>",
  "repository": {
    "type": "git",
    "url": "<Repository>"
  },
  "files": [
    "dist"
  ],
  "module": "./dist/esm/index.js",
  "main": "./dist/cjs/index.js",
  "browser": "./dist/browser/index.js",
  "types": "./dist/cjs/index.d.ts",
  "scripts": {
    "init:yarn": "yarn && yarn lint:fix && yarn build:yarn",
    "init:npm": "npm i && npm run lint:fix && npm run build:npm",
    "build:yarn": "rimraf dist/ && yarn build:node && yarn build:browser",
    "build:npm": "rimraf dist/ && npm run build:node && npm run build:browser",
    "build:node": "tsc && tsc -p tsconfig.cjs.json",
    "build:browser": "rollup --config",
    "lint:fix": "prettier src/** -w",
    "lint": "prettier src/** --check",
    "watch": "tsc -p tsconfig.cjs.json --watch"
  },
  "dependencies": {
    "@project-serum/mainstay": "=0.25.0",
    "@native-to-mainstay/buffer-layout": "=0.1.0"
  },
  "devDependencies": {
    "@rollup/plugin-commonjs": "=21.0.2",
    "@rollup/plugin-node-resolve": "=13.1.3",
    "@rollup/plugin-replace": "=3.1.0",
    "@rollup/plugin-typescript": "=8.3.1",
    "@types/node": "=17.0.21",
    "prettier": "=2.7.1",
    "rimraf": "=3.0.2",
    "rollup": "=2.70.1",
    "rollup-plugin-terser": "=7.0.2",
    "tslib": "=2.3.1",
    "typescript": "=4.6.2"
  }
}
"#;

    pub const TSCONFIG_JSON_FILENAME: &str = "tsconfig.json";
    pub const TSCONFIG_JSON_CONTENT: &str = r#"{
  "extends": "./tsconfig.base.json",
  "compilerOptions": {
    "moduleResolution": "node",
    "module": "es2022",
    "target": "es2019",
    "outDir": "dist/esm/",
    "rootDir": "./src",
  }
}
"#;

    pub const TSCONFIG_BASE_JSON_FILENAME: &str = "tsconfig.base.json";
    pub const TSCONFIG_BASE_JSON_CONTENT: &str = r#"{
  "include": [
    "./src/**/*"
  ],
  "compilerOptions": {
    "sourceMap": true,
    "declaration": true,
    "declarationMap": true,
    "allowSyntheticDefaultImports": true,
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    "noImplicitAny": false,
    "strictNullChecks": true,
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "composite": true,
    "baseUrl": ".",
    "typeRoots": [
      "node_modules/@types"
    ],
  }
}
"#;

    pub const TSCONFIG_CJS_JSON_FILENAME: &str = "tsconfig.cjs.json";
    pub const TSCONFIG_CJS_JSON_CONTENT: &str = r#"{
  "extends": "./tsconfig.base.json",
  "compilerOptions": {
    "module": "commonjs",
    "target": "es2019",
    "outDir": "dist/cjs/",
    "rootDir": "./src"
  }
}
"#;

    pub const ROLLUP_CONFIG_FILENAME: &str = "rollup.config.ts";
    pub const ROLLUP_CONFIG_CONTENT: &str = r#"import nodeResolve from "@rollup/plugin-node-resolve";
import typescript from "@rollup/plugin-typescript";
import replace from "@rollup/plugin-replace";
import commonjs from "@rollup/plugin-commonjs";
import { terser } from "rollup-plugin-terser";

const env = process.env.NODE_ENV;

export default {
  input: "src/index.ts",
  plugins: [
    commonjs(),
    nodeResolve({
      browser: true,
      extensions: [".js", ".ts"],
      dedupe: ["bn.js", "buffer"],
      preferBuiltins: false,
    }),
    typescript({
      tsconfig: "./tsconfig.base.json",
      moduleResolution: "node",
      outDir: "types",
      target: "es2019",
      outputToFilesystem: false,
    }),
    replace({
      preventAssignment: true,
      values: {
        "process.env.NODE_ENV": JSON.stringify(env),
        "process.env.ANCHOR_BROWSER": JSON.stringify(true),
      },
    }),
    terser(),
  ],
  external: [
    "@project-serum/borsh",
    "@solana/web3.js",
    "assert",
    "base64-js",
    "bn.js",
    "bs58",
    "buffer",
    "camelcase",
    "eventemitter3",
    "js-sha256",
    "pako",
    "toml",
  ],
  output: {
    file: "dist/browser/index.js",
    format: "es",
    sourcemap: true,
  },
};
"#;

    pub const YARN_LOCK_FILENAME: &str = "yarn.lock";
    pub const YARN_LOCK_CONTENT: &str = r#"# THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.
# yarn lockfile v1


"@babel/code-frame@^7.10.4":
  version "7.18.6"
  resolved "https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.18.6.tgz#3b25d38c89600baa2dcc219edfa88a74eb2c427a"
  integrity sha512-TDCmlK5eOvH+eH7cdAFlNXeVJqWIQ7gW9tY1GJIpUtFb6CmjVyq2VM3u71bOyR8CRihcCgMUYoDNyLXao3+70Q==
  dependencies:
    "@babel/highlight" "^7.18.6"

"@babel/helper-validator-identifier@^7.18.6":
  version "7.18.6"
  resolved "https://registry.yarnpkg.com/@babel/helper-validator-identifier/-/helper-validator-identifier-7.18.6.tgz#9c97e30d31b2b8c72a1d08984f2ca9b574d7a076"
  integrity sha512-MmetCkz9ej86nJQV+sFCxoGGrUbU3q02kgLciwkrt9QqEB7cP39oKEY0PakknEO0Gu20SskMRi+AYZ3b1TpN9g==

"@babel/highlight@^7.18.6":
  version "7.18.6"
  resolved "https://registry.yarnpkg.com/@babel/highlight/-/highlight-7.18.6.tgz#81158601e93e2563795adcbfbdf5d64be3f2ecdf"
  integrity sha512-u7stbOuYjaPezCuLj29hNW1v64M2Md2qupEKP1fHc7WdOA3DgLh37suiSrZYY7haUB7iBeQZ9P1uiRF359do3g==
  dependencies:
    "@babel/helper-validator-identifier" "^7.18.6"
    chalk "^2.0.0"
    js-tokens "^4.0.0"

"@babel/runtime@^7.12.5", "@babel/runtime@^7.17.2":
  version "7.18.9"
  resolved "https://registry.yarnpkg.com/@babel/runtime/-/runtime-7.18.9.tgz#b4fcfce55db3d2e5e080d2490f608a3b9f407f4a"
  integrity sha512-lkqXDcvlFT5rvEjiu6+QYO+1GXrEHRo2LOtS7E4GtX5ESIZOgepqsZBVIj6Pv+a6zqsya9VCgiK1KAK4BvJDAw==
  dependencies:
    regenerator-runtime "^0.13.4"

"@ethersproject/bytes@^5.6.1":
  version "5.6.1"
  resolved "https://registry.yarnpkg.com/@ethersproject/bytes/-/bytes-5.6.1.tgz#24f916e411f82a8a60412344bf4a813b917eefe7"
  integrity sha512-NwQt7cKn5+ZE4uDn+X5RAXLp46E1chXoaMmrxAyA0rblpxz8t58lVkrHXoRIn0lz1joQElQ8410GqhTqMOwc6g==
  dependencies:
    "@ethersproject/logger" "^5.6.0"

"@ethersproject/logger@^5.6.0":
  version "5.6.0"
  resolved "https://registry.yarnpkg.com/@ethersproject/logger/-/logger-5.6.0.tgz#d7db1bfcc22fd2e4ab574cba0bb6ad779a9a3e7a"
  integrity sha512-BiBWllUROH9w+P21RzoxJKzqoqpkyM1pRnEKG69bulE9TSQD8SAIvTQqIMZmmCO8pUNkgLP1wndX1gKghSpBmg==

"@ethersproject/sha2@^5.5.0":
  version "5.6.1"
  resolved "https://registry.yarnpkg.com/@ethersproject/sha2/-/sha2-5.6.1.tgz#211f14d3f5da5301c8972a8827770b6fd3e51656"
  integrity sha512-5K2GyqcW7G4Yo3uenHegbXRPDgARpWUiXc6RiF7b6i/HXUoWlb7uCARh7BAHg7/qT/Q5ydofNwiZcim9qpjB6g==
  dependencies:
    "@ethersproject/bytes" "^5.6.1"
    "@ethersproject/logger" "^5.6.0"
    hash.js "1.1.7"

"@jridgewell/gen-mapping@^0.3.0":
  version "0.3.2"
  resolved "https://registry.yarnpkg.com/@jridgewell/gen-mapping/-/gen-mapping-0.3.2.tgz#c1aedc61e853f2bb9f5dfe6d4442d3b565b253b9"
  integrity sha512-mh65xKQAzI6iBcFzwv28KVWSmCkdRBWoOh+bYQGW3+6OZvbbN3TqMGo5hqYxQniRcH9F2VZIoJCm4pa3BPDK/A==
  dependencies:
    "@jridgewell/set-array" "^1.0.1"
    "@jridgewell/sourcemap-codec" "^1.4.10"
    "@jridgewell/trace-mapping" "^0.3.9"

"@jridgewell/resolve-uri@^3.0.3":
  version "3.1.0"
  resolved "https://registry.yarnpkg.com/@jridgewell/resolve-uri/-/resolve-uri-3.1.0.tgz#2203b118c157721addfe69d47b70465463066d78"
  integrity sha512-F2msla3tad+Mfht5cJq7LSXcdudKTWCVYUgw6pLFOOHSTtZlj6SWNYAp+AhuqLmWdBO2X5hPrLcu8cVP8fy28w==

"@jridgewell/set-array@^1.0.1":
  version "1.1.2"
  resolved "https://registry.yarnpkg.com/@jridgewell/set-array/-/set-array-1.1.2.tgz#7c6cf998d6d20b914c0a55a91ae928ff25965e72"
  integrity sha512-xnkseuNADM0gt2bs+BvhO0p78Mk762YnZdsuzFV018NoG1Sj1SCQvpSqa7XUaTam5vAGasABV9qXASMKnFMwMw==

"@jridgewell/source-map@^0.3.2":
  version "0.3.2"
  resolved "https://registry.yarnpkg.com/@jridgewell/source-map/-/source-map-0.3.2.tgz#f45351aaed4527a298512ec72f81040c998580fb"
  integrity sha512-m7O9o2uR8k2ObDysZYzdfhb08VuEml5oWGiosa1VdaPZ/A6QyPkAJuwN0Q1lhULOf6B7MtQmHENS743hWtCrgw==
  dependencies:
    "@jridgewell/gen-mapping" "^0.3.0"
    "@jridgewell/trace-mapping" "^0.3.9"

"@jridgewell/sourcemap-codec@^1.4.10":
  version "1.4.14"
  resolved "https://registry.yarnpkg.com/@jridgewell/sourcemap-codec/-/sourcemap-codec-1.4.14.tgz#add4c98d341472a289190b424efbdb096991bb24"
  integrity sha512-XPSJHWmi394fuUuzDnGz1wiKqWfo1yXecHQMRf2l6hztTO+nPru658AyDngaBe7isIxEkRsPR3FZh+s7iVa4Uw==

"@jridgewell/trace-mapping@^0.3.9":
  version "0.3.15"
  resolved "https://registry.yarnpkg.com/@jridgewell/trace-mapping/-/trace-mapping-0.3.15.tgz#aba35c48a38d3fd84b37e66c9c0423f9744f9774"
  integrity sha512-oWZNOULl+UbhsgB51uuZzglikfIKSUBO/M9W2OfEjn7cmqoAiCgmv9lyACTUacZwBz0ITnJ2NqjU8Tx0DHL88g==
  dependencies:
    "@jridgewell/resolve-uri" "^3.0.3"
    "@jridgewell/sourcemap-codec" "^1.4.10"

"@native-to-mainstay/buffer-layout@=0.1.0":
  version "0.1.0"
  resolved "https://registry.yarnpkg.com/@native-to-mainstay/buffer-layout/-/buffer-layout-0.1.0.tgz#ff0cb66341bc820b8ee73bb1d1d43bae7e3554b0"
  integrity sha512-7Ykz9KRAm53XqHj5blDUKPX+OXAPO4GZBW4zJhfHGIAbzmqsUFh9kMqR66Bak3mp6wyv1OVTwSr8ZGHKswPxDg==
  dependencies:
    "@solana/buffer-layout" "=4.0.0"
    "@solana/buffer-layout-utils" "=0.2.0"

"@project-serum/mainstay@=0.25.0":
  version "0.25.0"
  resolved "https://registry.yarnpkg.com/@project-serum/mainstay/-/mainstay-0.25.0.tgz#88ee4843336005cf5a64c80636ce626f0996f503"
  integrity sha512-E6A5Y/ijqpfMJ5psJvbw0kVTzLZFUcOFgs6eSM2M2iWE1lVRF18T6hWZVNl6zqZsoz98jgnNHtVGJMs+ds9A7A==
  dependencies:
    "@project-serum/borsh" "^0.2.5"
    "@solana/web3.js" "^1.36.0"
    base64-js "^1.5.1"
    bn.js "^5.1.2"
    bs58 "^4.0.1"
    buffer-layout "^1.2.2"
    camelcase "^5.3.1"
    cross-fetch "^3.1.5"
    crypto-hash "^1.3.0"
    eventemitter3 "^4.0.7"
    js-sha256 "^0.9.0"
    pako "^2.0.3"
    snake-case "^3.0.4"
    superstruct "^0.15.4"
    toml "^3.0.0"

"@project-serum/borsh@^0.2.5":
  version "0.2.5"
  resolved "https://registry.yarnpkg.com/@project-serum/borsh/-/borsh-0.2.5.tgz#6059287aa624ecebbfc0edd35e4c28ff987d8663"
  integrity sha512-UmeUkUoKdQ7rhx6Leve1SssMR/Ghv8qrEiyywyxSWg7ooV7StdpPBhciiy5eB3T0qU1BXvdRNC8TdrkxK7WC5Q==
  dependencies:
    bn.js "^5.1.2"
    buffer-layout "^1.2.0"

"@rollup/plugin-commonjs@=21.0.2":
  version "21.0.2"
  resolved "https://registry.yarnpkg.com/@rollup/plugin-commonjs/-/plugin-commonjs-21.0.2.tgz#0b9c539aa1837c94abfaf87945838b0fc8564891"
  integrity sha512-d/OmjaLVO4j/aQX69bwpWPpbvI3TJkQuxoAk7BH8ew1PyoMBLTOuvJTjzG8oEoW7drIIqB0KCJtfFLu/2GClWg==
  dependencies:
    "@rollup/pluginutils" "^3.1.0"
    commondir "^1.0.1"
    estree-walker "^2.0.1"
    glob "^7.1.6"
    is-reference "^1.2.1"
    magic-string "^0.25.7"
    resolve "^1.17.0"

"@rollup/plugin-node-resolve@=13.1.3":
  version "13.1.3"
  resolved "https://registry.yarnpkg.com/@rollup/plugin-node-resolve/-/plugin-node-resolve-13.1.3.tgz#2ed277fb3ad98745424c1d2ba152484508a92d79"
  integrity sha512-BdxNk+LtmElRo5d06MGY4zoepyrXX1tkzX2hrnPEZ53k78GuOMWLqmJDGIIOPwVRIFZrLQOo+Yr6KtCuLIA0AQ==
  dependencies:
    "@rollup/pluginutils" "^3.1.0"
    "@types/resolve" "1.17.1"
    builtin-modules "^3.1.0"
    deepmerge "^4.2.2"
    is-module "^1.0.0"
    resolve "^1.19.0"

"@rollup/plugin-replace@=3.1.0":
  version "3.1.0"
  resolved "https://registry.yarnpkg.com/@rollup/plugin-replace/-/plugin-replace-3.1.0.tgz#d31e3a90c6b47064f3c9f2ce0ded5bcf0d3b82f6"
  integrity sha512-pA3XRUrSKybVYqmH5TqWNZpGxF+VV+1GrYchKgCNIj2vsSOX7CVm2RCtx8p2nrC7xvkziYyK+lSi74T93MU3YA==
  dependencies:
    "@rollup/pluginutils" "^3.1.0"
    magic-string "^0.25.7"

"@rollup/plugin-typescript@=8.3.1":
  version "8.3.1"
  resolved "https://registry.yarnpkg.com/@rollup/plugin-typescript/-/plugin-typescript-8.3.1.tgz#b7dc75ed6b4876e260b9e80624fab23bc98e4ac1"
  integrity sha512-84rExe3ICUBXzqNX48WZV2Jp3OddjTMX97O2Py6D1KJaGSwWp0mDHXj+bCGNJqWHIEKDIT2U0sDjhP4czKi6cA==
  dependencies:
    "@rollup/pluginutils" "^3.1.0"
    resolve "^1.17.0"

"@rollup/pluginutils@^3.1.0":
  version "3.1.0"
  resolved "https://registry.yarnpkg.com/@rollup/pluginutils/-/pluginutils-3.1.0.tgz#706b4524ee6dc8b103b3c995533e5ad680c02b9b"
  integrity sha512-GksZ6pr6TpIjHm8h9lSQ8pi8BE9VeubNT0OMJ3B5uZJ8pz73NPiqOtCog/x2/QzM1ENChPKxMDhiQuRHsqc+lg==
  dependencies:
    "@types/estree" "0.0.39"
    estree-walker "^1.0.1"
    picomatch "^2.2.2"

"@solana/buffer-layout-utils@=0.2.0":
  version "0.2.0"
  resolved "https://registry.yarnpkg.com/@solana/buffer-layout-utils/-/buffer-layout-utils-0.2.0.tgz#b45a6cab3293a2eb7597cceb474f229889d875ca"
  integrity sha512-szG4sxgJGktbuZYDg2FfNmkMi0DYQoVjN2h7ta1W1hPrwzarcFLBq9UpX1UjNXsNpT9dn+chgprtWGioUAr4/g==
  dependencies:
    "@solana/buffer-layout" "^4.0.0"
    "@solana/web3.js" "^1.32.0"
    bigint-buffer "^1.1.5"
    bignumber.js "^9.0.1"

"@solana/buffer-layout@=4.0.0", "@solana/buffer-layout@^4.0.0":
  version "4.0.0"
  resolved "https://registry.yarnpkg.com/@solana/buffer-layout/-/buffer-layout-4.0.0.tgz#75b1b11adc487234821c81dfae3119b73a5fd734"
  integrity sha512-lR0EMP2HC3+Mxwd4YcnZb0smnaDw7Bl2IQWZiTevRH5ZZBZn6VRWn3/92E3qdU4SSImJkA6IDHawOHAnx/qUvQ==
  dependencies:
    buffer "~6.0.3"

"@solana/web3.js@^1.32.0", "@solana/web3.js@^1.36.0":
  version "1.53.0"
  resolved "https://registry.yarnpkg.com/@solana/web3.js/-/web3.js-1.53.0.tgz#24a6341e4026fc2b993656141361c54bec917c07"
  integrity sha512-QyQDA9U5b+AiTo1ANsj9WihWWECeLv6VRpiTE7xPe5hLYANXZYecnlLglNiEzVgRg/jLvR5DrCISXhHx/mAEJw==
  dependencies:
    "@babel/runtime" "^7.12.5"
    "@ethersproject/sha2" "^5.5.0"
    "@solana/buffer-layout" "^4.0.0"
    bigint-buffer "^1.1.5"
    bn.js "^5.0.0"
    borsh "^0.7.0"
    bs58 "^4.0.1"
    buffer "6.0.1"
    fast-stable-stringify "^1.0.0"
    jayson "^3.4.4"
    js-sha3 "^0.8.0"
    node-fetch "2"
    react-native-url-polyfill "^1.3.0"
    rpc-websockets "^7.5.0"
    secp256k1 "^4.0.2"
    superstruct "^0.14.2"
    tweetnacl "^1.0.3"

"@types/connect@^3.4.33":
  version "3.4.35"
  resolved "https://registry.yarnpkg.com/@types/connect/-/connect-3.4.35.tgz#5fcf6ae445e4021d1fc2219a4873cc73a3bb2ad1"
  integrity sha512-cdeYyv4KWoEgpBISTxWvqYsVy444DOqehiF3fM3ne10AmJ62RSyNkUnxMJXHQWRQQX2eR94m5y1IZyDwBjV9FQ==
  dependencies:
    "@types/node" "*"

"@types/estree@*":
  version "1.0.0"
  resolved "https://registry.yarnpkg.com/@types/estree/-/estree-1.0.0.tgz#5fb2e536c1ae9bf35366eed879e827fa59ca41c2"
  integrity sha512-WulqXMDUTYAXCjZnk6JtIHPigp55cVtDgDrO2gHRwhyJto21+1zbVCtOYB2L1F9w4qCQ0rOGWBnBe0FNTiEJIQ==

"@types/estree@0.0.39":
  version "0.0.39"
  resolved "https://registry.yarnpkg.com/@types/estree/-/estree-0.0.39.tgz#e177e699ee1b8c22d23174caaa7422644389509f"
  integrity sha512-EYNwp3bU+98cpU4lAWYYL7Zz+2gryWH1qbdDTidVd6hkiR6weksdbMadyXKXNPEkQFhXM+hVO9ZygomHXp+AIw==

"@types/node@*":
  version "18.7.6"
  resolved "https://registry.yarnpkg.com/@types/node/-/node-18.7.6.tgz#31743bc5772b6ac223845e18c3fc26f042713c83"
  integrity sha512-EdxgKRXgYsNITy5mjjXjVE/CS8YENSdhiagGrLqjG0pvA2owgJ6i4l7wy/PFZGC0B1/H20lWKN7ONVDNYDZm7A==

"@types/node@=17.0.21":
  version "17.0.21"
  resolved "https://registry.yarnpkg.com/@types/node/-/node-17.0.21.tgz#864b987c0c68d07b4345845c3e63b75edd143644"
  integrity sha512-DBZCJbhII3r90XbQxI8Y9IjjiiOGlZ0Hr32omXIZvwwZ7p4DMMXGrKXVyPfuoBOri9XNtL0UK69jYIBIsRX3QQ==

"@types/node@^12.12.54":
  version "12.20.55"
  resolved "https://registry.yarnpkg.com/@types/node/-/node-12.20.55.tgz#c329cbd434c42164f846b909bd6f85b5537f6240"
  integrity sha512-J8xLz7q2OFulZ2cyGTLE1TbbZcjpno7FaN6zdJNrgAdrJ+DZzh/uFR6YrTb4C+nXakvud8Q4+rbhoIWlYQbUFQ==

"@types/resolve@1.17.1":
  version "1.17.1"
  resolved "https://registry.yarnpkg.com/@types/resolve/-/resolve-1.17.1.tgz#3afd6ad8967c77e4376c598a82ddd58f46ec45d6"
  integrity sha512-yy7HuzQhj0dhGpD8RLXSZWEkLsV9ibvxvi6EiJ3bkqLAO1RGo0WbkWQiwpRlSFymTJRz0d3k5LM3kkx8ArDbLw==
  dependencies:
    "@types/node" "*"

"@types/ws@^7.4.4":
  version "7.4.7"
  resolved "https://registry.yarnpkg.com/@types/ws/-/ws-7.4.7.tgz#f7c390a36f7a0679aa69de2d501319f4f8d9b702"
  integrity sha512-JQbbmxZTZehdc2iszGKs5oC3NFnjeay7mtAWrdt7qNtAVK0g19muApzAy4bm9byz79xa2ZnO/BOBC2R8RC5Lww==
  dependencies:
    "@types/node" "*"

JSONStream@^1.3.5:
  version "1.3.5"
  resolved "https://registry.yarnpkg.com/JSONStream/-/JSONStream-1.3.5.tgz#3208c1f08d3a4d99261ab64f92302bc15e111ca0"
  integrity sha512-E+iruNOY8VV9s4JEbe1aNEm6MiszPRr/UfcHMz0TQh1BXSxHK+ASV1R6W4HpjBhSeS+54PIsAMCBmwD06LLsqQ==
  dependencies:
    jsonparse "^1.2.0"
    through ">=2.2.7 <3"

acorn@^8.5.0:
  version "8.8.0"
  resolved "https://registry.yarnpkg.com/acorn/-/acorn-8.8.0.tgz#88c0187620435c7f6015803f5539dae05a9dbea8"
  integrity sha512-QOxyigPVrpZ2GXT+PFyZTl6TtOFc5egxHIP9IlQ+RbupQuX4RkT/Bee4/kQuC02Xkzg84JcT7oLYtDIQxp+v7w==

ansi-styles@^3.2.1:
  version "3.2.1"
  resolved "https://registry.yarnpkg.com/ansi-styles/-/ansi-styles-3.2.1.tgz#41fbb20243e50b12be0f04b8dedbf07520ce841d"
  integrity sha512-VT0ZI6kZRdTh8YyJw3SMbYm/u+NqfsAxEpWO0Pf9sq8/e94WxxOpPKx9FR1FlyCtOVDNOQ+8ntlqFxiRc+r5qA==
  dependencies:
    color-convert "^1.9.0"

balanced-match@^1.0.0:
  version "1.0.2"
  resolved "https://registry.yarnpkg.com/balanced-match/-/balanced-match-1.0.2.tgz#e83e3a7e3f300b34cb9d87f615fa0cbf357690ee"
  integrity sha512-3oSeUO0TMV67hN1AmbXsK4yaqU7tjiHlbxRDZOpH0KW9+CeX4bRAaX0Anxt0tx2MrpRpWwQaPwIlISEJhYU5Pw==

base-x@^3.0.2:
  version "3.0.9"
  resolved "https://registry.yarnpkg.com/base-x/-/base-x-3.0.9.tgz#6349aaabb58526332de9f60995e548a53fe21320"
  integrity sha512-H7JU6iBHTal1gp56aKoaa//YUxEaAOUiydvrV/pILqIHXTtqxSkATOnDA2u+jZ/61sD+L/412+7kzXRtWukhpQ==
  dependencies:
    safe-buffer "^5.0.1"

base64-js@^1.3.1, base64-js@^1.5.1:
  version "1.5.1"
  resolved "https://registry.yarnpkg.com/base64-js/-/base64-js-1.5.1.tgz#1b1b440160a5bf7ad40b650f095963481903930a"
  integrity sha512-AKpaYlHn8t4SVbOHCy+b5+KKgvR4vrsD8vbvrbiQJps7fKDTkjkDry6ji0rUJjC0kzbNePLwzxq8iypo41qeWA==

bigint-buffer@^1.1.5:
  version "1.1.5"
  resolved "https://registry.yarnpkg.com/bigint-buffer/-/bigint-buffer-1.1.5.tgz#d038f31c8e4534c1f8d0015209bf34b4fa6dd442"
  integrity sha512-trfYco6AoZ+rKhKnxA0hgX0HAbVP/s808/EuDSe2JDzUnCp/xAsli35Orvk67UrTEcwuxZqYZDmfA2RXJgxVvA==
  dependencies:
    bindings "^1.3.0"

bignumber.js@^9.0.1:
  version "9.1.0"
  resolved "https://registry.yarnpkg.com/bignumber.js/-/bignumber.js-9.1.0.tgz#8d340146107fe3a6cb8d40699643c302e8773b62"
  integrity sha512-4LwHK4nfDOraBCtst+wOWIHbu1vhvAPJK8g8nROd4iuc3PSEjWif/qwbkh8jwCJz6yDBvtU4KPynETgrfh7y3A==

bindings@^1.3.0:
  version "1.5.0"
  resolved "https://registry.yarnpkg.com/bindings/-/bindings-1.5.0.tgz#10353c9e945334bc0511a6d90b38fbc7c9c504df"
  integrity sha512-p2q/t/mhvuOj/UeLlV6566GD/guowlr0hHxClI0W9m7MWYkL1F0hLo+0Aexs9HSPCtR1SXQ0TD3MMKrXZajbiQ==
  dependencies:
    file-uri-to-path "1.0.0"

bn.js@^4.11.9:
  version "4.12.0"
  resolved "https://registry.yarnpkg.com/bn.js/-/bn.js-4.12.0.tgz#775b3f278efbb9718eec7361f483fb36fbbfea88"
  integrity sha512-c98Bf3tPniI+scsdk237ku1Dc3ujXQTSgyiPUDEOe7tRkhrqridvh8klBv0HCEso1OLOYcHuCv/cS6DNxKH+ZA==

bn.js@^5.0.0, bn.js@^5.1.2, bn.js@^5.2.0:
  version "5.2.1"
  resolved "https://registry.yarnpkg.com/bn.js/-/bn.js-5.2.1.tgz#0bc527a6a0d18d0aa8d5b0538ce4a77dccfa7b70"
  integrity sha512-eXRvHzWyYPBuB4NBy0cmYQjGitUrtqwbvlzP3G6VFnNRbsZQIxQ10PbKKHt8gZ/HW/D/747aDl+QkDqg3KQLMQ==

borsh@^0.7.0:
  version "0.7.0"
  resolved "https://registry.yarnpkg.com/borsh/-/borsh-0.7.0.tgz#6e9560d719d86d90dc589bca60ffc8a6c51fec2a"
  integrity sha512-CLCsZGIBCFnPtkNnieW/a8wmreDmfUtjU2m9yHrzPXIlNbqVs0AQrSatSG6vdNYUqdc83tkQi2eHfF98ubzQLA==
  dependencies:
    bn.js "^5.2.0"
    bs58 "^4.0.0"
    text-encoding-utf-8 "^1.0.2"

brace-expansion@^1.1.7:
  version "1.1.11"
  resolved "https://registry.yarnpkg.com/brace-expansion/-/brace-expansion-1.1.11.tgz#3c7fcbf529d87226f3d2f52b966ff5271eb441dd"
  integrity sha512-iCuPHDFgrHX7H2vEI/5xpz07zSHB00TpugqhmYtVmMO6518mCuRMoOYFldEBl0g187ufozdaHgWKcYFb61qGiA==
  dependencies:
    balanced-match "^1.0.0"
    concat-map "0.0.1"

brorand@^1.1.0:
  version "1.1.0"
  resolved "https://registry.yarnpkg.com/brorand/-/brorand-1.1.0.tgz#12c25efe40a45e3c323eb8675a0a0ce57b22371f"
  integrity sha512-cKV8tMCEpQs4hK/ik71d6LrPOnpkpGBR0wzxqr68g2m/LB2GxVYQroAjMJZRVM1Y4BCjCKc3vAamxSzOY2RP+w==

bs58@^4.0.0, bs58@^4.0.1:
  version "4.0.1"
  resolved "https://registry.yarnpkg.com/bs58/-/bs58-4.0.1.tgz#be161e76c354f6f788ae4071f63f34e8c4f0a42a"
  integrity sha512-Ok3Wdf5vOIlBrgCvTq96gBkJw+JUEzdBgyaza5HLtPm7yTHkjRy8+JzNyHF7BHa0bNWOQIp3m5YF0nnFcOIKLw==
  dependencies:
    base-x "^3.0.2"

buffer-from@^1.0.0:
  version "1.1.2"
  resolved "https://registry.yarnpkg.com/buffer-from/-/buffer-from-1.1.2.tgz#2b146a6fd72e80b4f55d255f35ed59a3a9a41bd5"
  integrity sha512-E+XQCRwSbaaiChtv6k6Dwgc+bx+Bs6vuKJHHl5kox/BaKbhiXzqQOwK4cO22yElGp2OCmjwVhT3HmxgyPGnJfQ==

buffer-layout@^1.2.0, buffer-layout@^1.2.2:
  version "1.2.2"
  resolved "https://registry.yarnpkg.com/buffer-layout/-/buffer-layout-1.2.2.tgz#b9814e7c7235783085f9ca4966a0cfff112259d5"
  integrity sha512-kWSuLN694+KTk8SrYvCqwP2WcgQjoRCiF5b4QDvkkz8EmgD+aWAIceGFKMIAdmF/pH+vpgNV3d3kAKorcdAmWA==

buffer@6.0.1:
  version "6.0.1"
  resolved "https://registry.yarnpkg.com/buffer/-/buffer-6.0.1.tgz#3cbea8c1463e5a0779e30b66d4c88c6ffa182ac2"
  integrity sha512-rVAXBwEcEoYtxnHSO5iWyhzV/O1WMtkUYWlfdLS7FjU4PnSJJHEfHXi/uHPI5EwltmOA794gN3bm3/pzuctWjQ==
  dependencies:
    base64-js "^1.3.1"
    ieee754 "^1.2.1"

buffer@^5.4.3:
  version "5.7.1"
  resolved "https://registry.yarnpkg.com/buffer/-/buffer-5.7.1.tgz#ba62e7c13133053582197160851a8f648e99eed0"
  integrity sha512-EHcyIPBQ4BSGlvjB16k5KgAJ27CIsHY/2JBmCRReo48y9rQ3MaUzWX3KVlBa4U7MyX02HdVj0K7C3WaB3ju7FQ==
  dependencies:
    base64-js "^1.3.1"
    ieee754 "^1.1.13"

buffer@~6.0.3:
  version "6.0.3"
  resolved "https://registry.yarnpkg.com/buffer/-/buffer-6.0.3.tgz#2ace578459cc8fbe2a70aaa8f52ee63b6a74c6c6"
  integrity sha512-FTiCpNxtwiZZHEZbcbTIcZjERVICn9yq/pDFkTl95/AxzD1naBctN7YO68riM/gLSDY7sdrMby8hofADYuuqOA==
  dependencies:
    base64-js "^1.3.1"
    ieee754 "^1.2.1"

bufferutil@^4.0.1:
  version "4.0.6"
  resolved "https://registry.yarnpkg.com/bufferutil/-/bufferutil-4.0.6.tgz#ebd6c67c7922a0e902f053e5d8be5ec850e48433"
  integrity sha512-jduaYOYtnio4aIAyc6UbvPCVcgq7nYpVnucyxr6eCYg/Woad9Hf/oxxBRDnGGjPfjUm6j5O/uBWhIu4iLebFaw==
  dependencies:
    node-gyp-build "^4.3.0"

builtin-modules@^3.1.0:
  version "3.3.0"
  resolved "https://registry.yarnpkg.com/builtin-modules/-/builtin-modules-3.3.0.tgz#cae62812b89801e9656336e46223e030386be7b6"
  integrity sha512-zhaCDicdLuWN5UbN5IMnFqNMhNfo919sH85y2/ea+5Yg9TsTkeZxpL+JLbp6cgYFS4sRLp3YV4S6yDuqVWHYOw==

camelcase@^5.3.1:
  version "5.3.1"
  resolved "https://registry.yarnpkg.com/camelcase/-/camelcase-5.3.1.tgz#e3c9b31569e106811df242f715725a1f4c494320"
  integrity sha512-L28STB170nwWS63UjtlEOE3dldQApaJXZkOI1uMFfzf3rRuPegHaHesyee+YxQ+W6SvRDQV6UrdOdRiR153wJg==

chalk@^2.0.0:
  version "2.4.2"
  resolved "https://registry.yarnpkg.com/chalk/-/chalk-2.4.2.tgz#cd42541677a54333cf541a49108c1432b44c9424"
  integrity sha512-Mti+f9lpJNcwF4tWV8/OrTTtF1gZi+f8FqlyAdouralcFWFQWF2+NgCHShjkCb+IFBLq9buZwE1xckQU4peSuQ==
  dependencies:
    ansi-styles "^3.2.1"
    escape-string-regexp "^1.0.5"
    supports-color "^5.3.0"

color-convert@^1.9.0:
  version "1.9.3"
  resolved "https://registry.yarnpkg.com/color-convert/-/color-convert-1.9.3.tgz#bb71850690e1f136567de629d2d5471deda4c1e8"
  integrity sha512-QfAUtd+vFdAtFQcC8CCyYt1fYWxSqAiK2cSD6zDB8N3cpsEBAvRxp9zOGg6G/SHHJYAT88/az/IuDGALsNVbGg==
  dependencies:
    color-name "1.1.3"

color-name@1.1.3:
  version "1.1.3"
  resolved "https://registry.yarnpkg.com/color-name/-/color-name-1.1.3.tgz#a7d0558bd89c42f795dd42328f740831ca53bc25"
  integrity sha512-72fSenhMw2HZMTVHeCA9KCmpEIbzWiQsjN+BHcBbS9vr1mtt+vJjPdksIBNUmKAW8TFUDPJK5SUU3QhE9NEXDw==

commander@^2.20.0, commander@^2.20.3:
  version "2.20.3"
  resolved "https://registry.yarnpkg.com/commander/-/commander-2.20.3.tgz#fd485e84c03eb4881c20722ba48035e8531aeb33"
  integrity sha512-GpVkmM8vF2vQUkj2LvZmD35JxeJOLCwJ9cUkugyk2nuhbv3+mJvpLYYt+0+USMxE+oj+ey/lJEnhZw75x/OMcQ==

commondir@^1.0.1:
  version "1.0.1"
  resolved "https://registry.yarnpkg.com/commondir/-/commondir-1.0.1.tgz#ddd800da0c66127393cca5950ea968a3aaf1253b"
  integrity sha512-W9pAhw0ja1Edb5GVdIF1mjZw/ASI0AlShXM83UUGe2DVr5TdAPEA1OA8m/g8zWp9x6On7gqufY+FatDbC3MDQg==

concat-map@0.0.1:
  version "0.0.1"
  resolved "https://registry.yarnpkg.com/concat-map/-/concat-map-0.0.1.tgz#d8a96bd77fd68df7793a73036a3ba0d5405d477b"
  integrity sha512-/Srv4dswyQNBfohGpz9o6Yb3Gz3SrUDqBH5rTuhGR7ahtlbYKnVxw2bCFMRljaA7EXHaXZ8wsHdodFvbkhKmqg==

cross-fetch@^3.1.5:
  version "3.1.5"
  resolved "https://registry.yarnpkg.com/cross-fetch/-/cross-fetch-3.1.5.tgz#e1389f44d9e7ba767907f7af8454787952ab534f"
  integrity sha512-lvb1SBsI0Z7GDwmuid+mU3kWVBwTVUbe7S0H52yaaAdQOXq2YktTCZdlAcNKFzE6QtRz0snpw9bNiPeOIkkQvw==
  dependencies:
    node-fetch "2.6.7"

crypto-hash@^1.3.0:
  version "1.3.0"
  resolved "https://registry.yarnpkg.com/crypto-hash/-/crypto-hash-1.3.0.tgz#b402cb08f4529e9f4f09346c3e275942f845e247"
  integrity sha512-lyAZ0EMyjDkVvz8WOeVnuCPvKVBXcMv1l5SVqO1yC7PzTwrD/pPje/BIRbWhMoPe436U+Y2nD7f5bFx0kt+Sbg==

deepmerge@^4.2.2:
  version "4.2.2"
  resolved "https://registry.yarnpkg.com/deepmerge/-/deepmerge-4.2.2.tgz#44d2ea3679b8f4d4ffba33f03d865fc1e7bf4955"
  integrity sha512-FJ3UgI4gIl+PHZm53knsuSFpE+nESMr7M4v9QcgB7S63Kj/6WqMiFQJpBBYz1Pt+66bZpP3Q7Lye0Oo9MPKEdg==

delay@^5.0.0:
  version "5.0.0"
  resolved "https://registry.yarnpkg.com/delay/-/delay-5.0.0.tgz#137045ef1b96e5071060dd5be60bf9334436bd1d"
  integrity sha512-ReEBKkIfe4ya47wlPYf/gu5ib6yUG0/Aez0JQZQz94kiWtRQvZIQbTiehsnwHvLSWJnQdhVeqYue7Id1dKr0qw==

dot-case@^3.0.4:
  version "3.0.4"
  resolved "https://registry.yarnpkg.com/dot-case/-/dot-case-3.0.4.tgz#9b2b670d00a431667a8a75ba29cd1b98809ce751"
  integrity sha512-Kv5nKlh6yRrdrGvxeJ2e5y2eRUpkUosIW4A2AS38zwSz27zu7ufDwQPi5Jhs3XAlGNetl3bmnGhQsMtkKJnj3w==
  dependencies:
    no-case "^3.0.4"
    tslib "^2.0.3"

elliptic@^6.5.4:
  version "6.5.4"
  resolved "https://registry.yarnpkg.com/elliptic/-/elliptic-6.5.4.tgz#da37cebd31e79a1367e941b592ed1fbebd58abbb"
  integrity sha512-iLhC6ULemrljPZb+QutR5TQGB+pdW6KGD5RSegS+8sorOZT+rdQFbsQFJgvN3eRqNALqJer4oQ16YvJHlU8hzQ==
  dependencies:
    bn.js "^4.11.9"
    brorand "^1.1.0"
    hash.js "^1.0.0"
    hmac-drbg "^1.0.1"
    inherits "^2.0.4"
    minimalistic-assert "^1.0.1"
    minimalistic-crypto-utils "^1.0.1"

es6-promise@^4.0.3:
  version "4.2.8"
  resolved "https://registry.yarnpkg.com/es6-promise/-/es6-promise-4.2.8.tgz#4eb21594c972bc40553d276e510539143db53e0a"
  integrity sha512-HJDGx5daxeIvxdBxvG2cb9g4tEvwIk3i8+nhX0yGrYmZUzbkdg8QbDevheDB8gd0//uPj4c1EQua8Q+MViT0/w==

es6-promisify@^5.0.0:
  version "5.0.0"
  resolved "https://registry.yarnpkg.com/es6-promisify/-/es6-promisify-5.0.0.tgz#5109d62f3e56ea967c4b63505aef08291c8a5203"
  integrity sha512-C+d6UdsYDk0lMebHNR4S2NybQMMngAOnOwYBQjTOiv0MkoJMP0Myw2mgpDLBcpfCmRLxyFqYhS/CfOENq4SJhQ==
  dependencies:
    es6-promise "^4.0.3"

escape-string-regexp@^1.0.5:
  version "1.0.5"
  resolved "https://registry.yarnpkg.com/escape-string-regexp/-/escape-string-regexp-1.0.5.tgz#1b61c0562190a8dff6ae3bb2cf0200ca130b86d4"
  integrity sha512-vbRorB5FUQWvla16U8R/qgaFIya2qGzwDrNmCZuYKrbdSUMG6I1ZCGQRefkRVhuOkIGVne7BQ35DSfo1qvJqFg==

estree-walker@^1.0.1:
  version "1.0.1"
  resolved "https://registry.yarnpkg.com/estree-walker/-/estree-walker-1.0.1.tgz#31bc5d612c96b704106b477e6dd5d8aa138cb700"
  integrity sha512-1fMXF3YP4pZZVozF8j/ZLfvnR8NSIljt56UhbZ5PeeDmmGHpgpdwQt7ITlGvYaQukCvuBRMLEiKiYC+oeIg4cg==

estree-walker@^2.0.1:
  version "2.0.2"
  resolved "https://registry.yarnpkg.com/estree-walker/-/estree-walker-2.0.2.tgz#52f010178c2a4c117a7757cfe942adb7d2da4cac"
  integrity sha512-Rfkk/Mp/DL7JVje3u18FxFujQlTNR2q6QfMSMB7AvCBx91NGj/ba3kCfza0f6dVDbw7YlRf/nDrn7pQrCCyQ/w==

eventemitter3@^4.0.7:
  version "4.0.7"
  resolved "https://registry.yarnpkg.com/eventemitter3/-/eventemitter3-4.0.7.tgz#2de9b68f6528d5644ef5c59526a1b4a07306169f"
  integrity sha512-8guHBZCwKnFhYdHr2ysuRWErTwhoN2X8XELRlrRwpmfeY2jjuUN4taQMsULKUVo1K4DvZl+0pgfyoysHxvmvEw==

eyes@^0.1.8:
  version "0.1.8"
  resolved "https://registry.yarnpkg.com/eyes/-/eyes-0.1.8.tgz#62cf120234c683785d902348a800ef3e0cc20bc0"
  integrity sha512-GipyPsXO1anza0AOZdy69Im7hGFCNB7Y/NGjDlZGJ3GJJLtwNSb2vrzYrTYJRrRloVx7pl+bhUaTB8yiccPvFQ==

fast-stable-stringify@^1.0.0:
  version "1.0.0"
  resolved "https://registry.yarnpkg.com/fast-stable-stringify/-/fast-stable-stringify-1.0.0.tgz#5c5543462b22aeeefd36d05b34e51c78cb86d313"
  integrity sha512-wpYMUmFu5f00Sm0cj2pfivpmawLZ0NKdviQ4w9zJeR8JVtOpOxHmLaJuj0vxvGqMJQWyP/COUkF75/57OKyRag==

file-uri-to-path@1.0.0:
  version "1.0.0"
  resolved "https://registry.yarnpkg.com/file-uri-to-path/-/file-uri-to-path-1.0.0.tgz#553a7b8446ff6f684359c445f1e37a05dacc33dd"
  integrity sha512-0Zt+s3L7Vf1biwWZ29aARiVYLx7iMGnEUl9x33fbB/j3jR81u/O2LbqK+Bm1CDSNDKVtJ/YjwY7TUd5SkeLQLw==

fs.realpath@^1.0.0:
  version "1.0.0"
  resolved "https://registry.yarnpkg.com/fs.realpath/-/fs.realpath-1.0.0.tgz#1504ad2523158caa40db4a2787cb01411994ea4f"
  integrity sha512-OO0pH2lK6a0hZnAdau5ItzHPI6pUlvI7jMVnxUQRtw4owF2wk8lOSabtGDCTP4Ggrg2MbGnWO9X8K1t4+fGMDw==

fsevents@~2.3.2:
  version "2.3.2"
  resolved "https://registry.yarnpkg.com/fsevents/-/fsevents-2.3.2.tgz#8a526f78b8fdf4623b709e0b975c52c24c02fd1a"
  integrity sha512-xiqMQR4xAeHTuB9uWm+fFRcIOgKBMiOBP+eXiyT7jsgVCq1bkVygt00oASowB7EdtpOHaaPgKt812P9ab+DDKA==

function-bind@^1.1.1:
  version "1.1.1"
  resolved "https://registry.yarnpkg.com/function-bind/-/function-bind-1.1.1.tgz#a56899d3ea3c9bab874bb9773b7c5ede92f4895d"
  integrity sha512-yIovAzMX49sF8Yl58fSCWJ5svSLuaibPxXQJFLmBObTuCr0Mf1KiPopGM9NiFjiYBCbfaa2Fh6breQ6ANVTI0A==

glob@^7.1.3, glob@^7.1.6:
  version "7.2.3"
  resolved "https://registry.yarnpkg.com/glob/-/glob-7.2.3.tgz#b8df0fb802bbfa8e89bd1d938b4e16578ed44f2b"
  integrity sha512-nFR0zLpU2YCaRxwoCJvL6UvCH2JFyFVIvwTLsIf21AuHlMskA1hhTdk+LlYJtOlYt9v6dvszD2BGRqBL+iQK9Q==
  dependencies:
    fs.realpath "^1.0.0"
    inflight "^1.0.4"
    inherits "2"
    minimatch "^3.1.1"
    once "^1.3.0"
    path-is-absolute "^1.0.0"

has-flag@^3.0.0:
  version "3.0.0"
  resolved "https://registry.yarnpkg.com/has-flag/-/has-flag-3.0.0.tgz#b5d454dc2199ae225699f3467e5a07f3b955bafd"
  integrity sha512-sKJf1+ceQBr4SMkvQnBDNDtf4TXpVhVGateu0t918bl30FnbE2m4vNLX+VWe/dpjlb+HugGYzW7uQXH98HPEYw==

has-flag@^4.0.0:
  version "4.0.0"
  resolved "https://registry.yarnpkg.com/has-flag/-/has-flag-4.0.0.tgz#944771fd9c81c81265c4d6941860da06bb59479b"
  integrity sha512-EykJT/Q1KjTWctppgIAgfSO0tKVuZUjhgMr17kqTumMl6Afv3EISleU7qZUzoXDFTAHTDC4NOoG/ZxU3EvlMPQ==

has@^1.0.3:
  version "1.0.3"
  resolved "https://registry.yarnpkg.com/has/-/has-1.0.3.tgz#722d7cbfc1f6aa8241f16dd814e011e1f41e8796"
  integrity sha512-f2dvO0VU6Oej7RkWJGrehjbzMAjFp5/VKPp5tTpWIV4JHHZK1/BxbFRtf/siA2SWTe09caDmVtYYzWEIbBS4zw==
  dependencies:
    function-bind "^1.1.1"

hash.js@1.1.7, hash.js@^1.0.0, hash.js@^1.0.3:
  version "1.1.7"
  resolved "https://registry.yarnpkg.com/hash.js/-/hash.js-1.1.7.tgz#0babca538e8d4ee4a0f8988d68866537a003cf42"
  integrity sha512-taOaskGt4z4SOANNseOviYDvjEJinIkRgmp7LbKP2YTTmVxWBl87s/uzK9r+44BclBSp2X7K1hqeNfz9JbBeXA==
  dependencies:
    inherits "^2.0.3"
    minimalistic-assert "^1.0.1"

hmac-drbg@^1.0.1:
  version "1.0.1"
  resolved "https://registry.yarnpkg.com/hmac-drbg/-/hmac-drbg-1.0.1.tgz#d2745701025a6c775a6c545793ed502fc0c649a1"
  integrity sha512-Tti3gMqLdZfhOQY1Mzf/AanLiqh1WTiJgEj26ZuYQ9fbkLomzGchCws4FyrSd4VkpBfiNhaE1On+lOz894jvXg==
  dependencies:
    hash.js "^1.0.3"
    minimalistic-assert "^1.0.0"
    minimalistic-crypto-utils "^1.0.1"

ieee754@^1.1.13, ieee754@^1.2.1:
  version "1.2.1"
  resolved "https://registry.yarnpkg.com/ieee754/-/ieee754-1.2.1.tgz#8eb7a10a63fff25d15a57b001586d177d1b0d352"
  integrity sha512-dcyqhDvX1C46lXZcVqCpK+FtMRQVdIMN6/Df5js2zouUsqG7I6sFxitIC+7KYK29KdXOLHdu9zL4sFnoVQnqaA==

inflight@^1.0.4:
  version "1.0.6"
  resolved "https://registry.yarnpkg.com/inflight/-/inflight-1.0.6.tgz#49bd6331d7d02d0c09bc910a1075ba8165b56df9"
  integrity sha512-k92I/b08q4wvFscXCLvqfsHCrjrF7yiXsQuIVvVE7N82W3+aqpzuUdBbfhWcy/FZR3/4IgflMgKLOsvPDrGCJA==
  dependencies:
    once "^1.3.0"
    wrappy "1"

inherits@2, inherits@^2.0.3, inherits@^2.0.4:
  version "2.0.4"
  resolved "https://registry.yarnpkg.com/inherits/-/inherits-2.0.4.tgz#0fa2c64f932917c3433a0ded55363aae37416b7c"
  integrity sha512-k/vGaX4/Yla3WzyMCvTQOXYeIHvqOKtnqBduzTHpzpQZzAskKMhZ2K+EnBiSM9zGSoIFeMpXKxa4dYeZIQqewQ==

is-core-module@^2.9.0:
  version "2.10.0"
  resolved "https://registry.yarnpkg.com/is-core-module/-/is-core-module-2.10.0.tgz#9012ede0a91c69587e647514e1d5277019e728ed"
  integrity sha512-Erxj2n/LDAZ7H8WNJXd9tw38GYM3dv8rk8Zcs+jJuxYTW7sozH+SS8NtrSjVL1/vpLvWi1hxy96IzjJ3EHTJJg==
  dependencies:
    has "^1.0.3"

is-module@^1.0.0:
  version "1.0.0"
  resolved "https://registry.yarnpkg.com/is-module/-/is-module-1.0.0.tgz#3258fb69f78c14d5b815d664336b4cffb6441591"
  integrity sha512-51ypPSPCoTEIN9dy5Oy+h4pShgJmPCygKfyRCISBI+JoWT/2oJvK8QPxmwv7b/p239jXrm9M1mlQbyKJ5A152g==

is-reference@^1.2.1:
  version "1.2.1"
  resolved "https://registry.yarnpkg.com/is-reference/-/is-reference-1.2.1.tgz#8b2dac0b371f4bc994fdeaba9eb542d03002d0b7"
  integrity sha512-U82MsXXiFIrjCK4otLT+o2NA2Cd2g5MLoOVXUZjIOhLurrRxpEXzI8O0KZHr3IjLvlAH1kTPYSuqer5T9ZVBKQ==
  dependencies:
    "@types/estree" "*"

isomorphic-ws@^4.0.1:
  version "4.0.1"
  resolved "https://registry.yarnpkg.com/isomorphic-ws/-/isomorphic-ws-4.0.1.tgz#55fd4cd6c5e6491e76dc125938dd863f5cd4f2dc"
  integrity sha512-BhBvN2MBpWTaSHdWRb/bwdZJ1WaehQ2L1KngkCkfLUGF0mAWAT1sQUQacEmQ0jXkFw/czDXPNQSL5u2/Krsz1w==

jayson@^3.4.4:
  version "3.7.0"
  resolved "https://registry.yarnpkg.com/jayson/-/jayson-3.7.0.tgz#b735b12d06d348639ae8230d7a1e2916cb078f25"
  integrity sha512-tfy39KJMrrXJ+mFcMpxwBvFDetS8LAID93+rycFglIQM4kl3uNR3W4lBLE/FFhsoUCEox5Dt2adVpDm/XtebbQ==
  dependencies:
    "@types/connect" "^3.4.33"
    "@types/node" "^12.12.54"
    "@types/ws" "^7.4.4"
    JSONStream "^1.3.5"
    commander "^2.20.3"
    delay "^5.0.0"
    es6-promisify "^5.0.0"
    eyes "^0.1.8"
    isomorphic-ws "^4.0.1"
    json-stringify-safe "^5.0.1"
    lodash "^4.17.20"
    uuid "^8.3.2"
    ws "^7.4.5"

jest-worker@^26.2.1:
  version "26.6.2"
  resolved "https://registry.yarnpkg.com/jest-worker/-/jest-worker-26.6.2.tgz#7f72cbc4d643c365e27b9fd775f9d0eaa9c7a8ed"
  integrity sha512-KWYVV1c4i+jbMpaBC+U++4Va0cp8OisU185o73T1vo99hqi7w8tSJfUXYswwqqrjzwxa6KpRK54WhPvwf5w6PQ==
  dependencies:
    "@types/node" "*"
    merge-stream "^2.0.0"
    supports-color "^7.0.0"

js-sha256@^0.9.0:
  version "0.9.0"
  resolved "https://registry.yarnpkg.com/js-sha256/-/js-sha256-0.9.0.tgz#0b89ac166583e91ef9123644bd3c5334ce9d0966"
  integrity sha512-sga3MHh9sgQN2+pJ9VYZ+1LPwXOxuBJBA5nrR5/ofPfuiJBE2hnjsaN8se8JznOmGLN2p49Pe5U/ttafcs/apA==

js-sha3@^0.8.0:
  version "0.8.0"
  resolved "https://registry.yarnpkg.com/js-sha3/-/js-sha3-0.8.0.tgz#b9b7a5da73afad7dedd0f8c463954cbde6818840"
  integrity sha512-gF1cRrHhIzNfToc802P800N8PpXS+evLLXfsVpowqmAFR9uwbi89WvXg2QspOmXL8QL86J4T1EpFu+yUkwJY3Q==

js-tokens@^4.0.0:
  version "4.0.0"
  resolved "https://registry.yarnpkg.com/js-tokens/-/js-tokens-4.0.0.tgz#19203fb59991df98e3a287050d4647cdeaf32499"
  integrity sha512-RdJUflcE3cUzKiMqQgsCu06FPu9UdIJO0beYbPhHN4k6apgJtifcoCtT9bcxOpYBtpD2kCM6Sbzg4CausW/PKQ==

json-stringify-safe@^5.0.1:
  version "5.0.1"
  resolved "https://registry.yarnpkg.com/json-stringify-safe/-/json-stringify-safe-5.0.1.tgz#1296a2d58fd45f19a0f6ce01d65701e2c735b6eb"
  integrity sha512-ZClg6AaYvamvYEE82d3Iyd3vSSIjQ+odgjaTzRuO3s7toCdFKczob2i0zCh7JE8kWn17yvAWhUVxvqGwUalsRA==

jsonparse@^1.2.0:
  version "1.3.1"
  resolved "https://registry.yarnpkg.com/jsonparse/-/jsonparse-1.3.1.tgz#3f4dae4a91fac315f71062f8521cc239f1366280"
  integrity sha512-POQXvpdL69+CluYsillJ7SUhKvytYjW9vG/GKpnf+xP8UWgYEM/RaMzHHofbALDiKbbP1W8UEYmgGl39WkPZsg==

lodash@^4.17.20:
  version "4.17.21"
  resolved "https://registry.yarnpkg.com/lodash/-/lodash-4.17.21.tgz#679591c564c3bffaae8454cf0b3df370c3d6911c"
  integrity sha512-v2kDEe57lecTulaDIuNTPy3Ry4gLGJ6Z1O3vE1krgXZNrsQ+LFTGHVxVjcXPs17LhbZVGedAJv8XZ1tvj5FvSg==

lower-case@^2.0.2:
  version "2.0.2"
  resolved "https://registry.yarnpkg.com/lower-case/-/lower-case-2.0.2.tgz#6fa237c63dbdc4a82ca0fd882e4722dc5e634e28"
  integrity sha512-7fm3l3NAF9WfN6W3JOmf5drwpVqX78JtoGJ3A6W0a6ZnldM41w2fV5D490psKFTpMds8TJse/eHLFFsNHHjHgg==
  dependencies:
    tslib "^2.0.3"

magic-string@^0.25.7:
  version "0.25.9"
  resolved "https://registry.yarnpkg.com/magic-string/-/magic-string-0.25.9.tgz#de7f9faf91ef8a1c91d02c2e5314c8277dbcdd1c"
  integrity sha512-RmF0AsMzgt25qzqqLc1+MbHmhdx0ojF2Fvs4XnOqz2ZOBXzzkEwc/dJQZCYHAn7v1jbVOjAZfK8msRn4BxO4VQ==
  dependencies:
    sourcemap-codec "^1.4.8"

merge-stream@^2.0.0:
  version "2.0.0"
  resolved "https://registry.yarnpkg.com/merge-stream/-/merge-stream-2.0.0.tgz#52823629a14dd00c9770fb6ad47dc6310f2c1f60"
  integrity sha512-abv/qOcuPfk3URPfDzmZU1LKmuw8kT+0nIHvKrKgFrwifol/doWcdA4ZqsWQ8ENrFKkd67Mfpo/LovbIUsbt3w==

minimalistic-assert@^1.0.0, minimalistic-assert@^1.0.1:
  version "1.0.1"
  resolved "https://registry.yarnpkg.com/minimalistic-assert/-/minimalistic-assert-1.0.1.tgz#2e194de044626d4a10e7f7fbc00ce73e83e4d5c7"
  integrity sha512-UtJcAD4yEaGtjPezWuO9wC4nwUnVH/8/Im3yEHQP4b67cXlD/Qr9hdITCU1xDbSEXg2XKNaP8jsReV7vQd00/A==

minimalistic-crypto-utils@^1.0.1:
  version "1.0.1"
  resolved "https://registry.yarnpkg.com/minimalistic-crypto-utils/-/minimalistic-crypto-utils-1.0.1.tgz#f6c00c1c0b082246e5c4d99dfb8c7c083b2b582a"
  integrity sha512-JIYlbt6g8i5jKfJ3xz7rF0LXmv2TkDxBLUkiBeZ7bAx4GnnNMr8xFpGnOxn6GhTEHx3SjRrZEoU+j04prX1ktg==

minimatch@^3.1.1:
  version "3.1.2"
  resolved "https://registry.yarnpkg.com/minimatch/-/minimatch-3.1.2.tgz#19cd194bfd3e428f049a70817c038d89ab4be35b"
  integrity sha512-J7p63hRiAjw1NDEww1W7i37+ByIrOWO5XQQAzZ3VOcL0PNybwpfmV/N05zFAzwQ9USyEcX6t3UO+K5aqBQOIHw==
  dependencies:
    brace-expansion "^1.1.7"

no-case@^3.0.4:
  version "3.0.4"
  resolved "https://registry.yarnpkg.com/no-case/-/no-case-3.0.4.tgz#d361fd5c9800f558551a8369fc0dcd4662b6124d"
  integrity sha512-fgAN3jGAh+RoxUGZHTSOLJIqUc2wmoBwGR4tbpNAKmmovFoWq0OdRkb0VkldReO2a2iBT/OEulG9XSUc10r3zg==
  dependencies:
    lower-case "^2.0.2"
    tslib "^2.0.3"

node-addon-api@^2.0.0:
  version "2.0.2"
  resolved "https://registry.yarnpkg.com/node-addon-api/-/node-addon-api-2.0.2.tgz#432cfa82962ce494b132e9d72a15b29f71ff5d32"
  integrity sha512-Ntyt4AIXyaLIuMHF6IOoTakB3K+RWxwtsHNRxllEoA6vPwP9o4866g6YWDLUdnucilZhmkxiHwHr11gAENw+QA==

node-fetch@2, node-fetch@2.6.7:
  version "2.6.7"
  resolved "https://registry.yarnpkg.com/node-fetch/-/node-fetch-2.6.7.tgz#24de9fba827e3b4ae44dc8b20256a379160052ad"
  integrity sha512-ZjMPFEfVx5j+y2yF35Kzx5sF7kDzxuDj6ziH4FFbOp87zKDZNx8yExJIb05OGF4Nlt9IHFIMBkRl41VdvcNdbQ==
  dependencies:
    whatwg-url "^5.0.0"

node-gyp-build@^4.2.0, node-gyp-build@^4.3.0:
  version "4.5.0"
  resolved "https://registry.yarnpkg.com/node-gyp-build/-/node-gyp-build-4.5.0.tgz#7a64eefa0b21112f89f58379da128ac177f20e40"
  integrity sha512-2iGbaQBV+ITgCz76ZEjmhUKAKVf7xfY1sRl4UiKQspfZMH2h06SyhNsnSVy50cwkFQDGLyif6m/6uFXHkOZ6rg==

once@^1.3.0:
  version "1.4.0"
  resolved "https://registry.yarnpkg.com/once/-/once-1.4.0.tgz#583b1aa775961d4b113ac17d9c50baef9dd76bd1"
  integrity sha512-lNaJgI+2Q5URQBkccEKHTQOPaXdUxnZZElQTZY0MFUAuaEqe1E+Nyvgdz/aIyNi6Z9MzO5dv1H8n58/GELp3+w==
  dependencies:
    wrappy "1"

pako@^2.0.3:
  version "2.0.4"
  resolved "https://registry.yarnpkg.com/pako/-/pako-2.0.4.tgz#6cebc4bbb0b6c73b0d5b8d7e8476e2b2fbea576d"
  integrity sha512-v8tweI900AUkZN6heMU/4Uy4cXRc2AYNRggVmTR+dEncawDJgCdLMximOVA2p4qO57WMynangsfGRb5WD6L1Bg==

path-is-absolute@^1.0.0:
  version "1.0.1"
  resolved "https://registry.yarnpkg.com/path-is-absolute/-/path-is-absolute-1.0.1.tgz#174b9268735534ffbc7ace6bf53a5a9e1b5c5f5f"
  integrity sha512-AVbw3UJ2e9bq64vSaS9Am0fje1Pa8pbGqTTsmXfaIiMpnr5DlDhfJOuLj9Sf95ZPVDAUerDfEk88MPmPe7UCQg==

path-parse@^1.0.7:
  version "1.0.7"
  resolved "https://registry.yarnpkg.com/path-parse/-/path-parse-1.0.7.tgz#fbc114b60ca42b30d9daf5858e4bd68bbedb6735"
  integrity sha512-LDJzPVEEEPR+y48z93A0Ed0yXb8pAByGWo/k5YYdYgpY2/2EsOsksJrq7lOHxryrVOn1ejG6oAp8ahvOIQD8sw==

picomatch@^2.2.2:
  version "2.3.1"
  resolved "https://registry.yarnpkg.com/picomatch/-/picomatch-2.3.1.tgz#3ba3833733646d9d3e4995946c1365a67fb07a42"
  integrity sha512-JU3teHTNjmE2VCGFzuY8EXzCDVwEqB2a8fsIvwaStHhAWJEeVd1o1QD80CU6+ZdEXXSLbSsuLwJjkCBWqRQUVA==

prettier@=2.7.1:
  version "2.7.1"
  resolved "https://registry.yarnpkg.com/prettier/-/prettier-2.7.1.tgz#e235806850d057f97bb08368a4f7d899f7760c64"
  integrity sha512-ujppO+MkdPqoVINuDFDRLClm7D78qbDt0/NR+wp5FqEZOoTNAjPHWj17QRhu7geIHJfcNhRk1XVQmF8Bp3ye+g==

punycode@^2.1.1:
  version "2.1.1"
  resolved "https://registry.yarnpkg.com/punycode/-/punycode-2.1.1.tgz#b58b010ac40c22c5657616c8d2c2c02c7bf479ec"
  integrity sha512-XRsRjdf+j5ml+y/6GKHPZbrF/8p2Yga0JPtdqTIY2Xe5ohJPD9saDJJLPvp9+NSBprVvevdXZybnj2cv8OEd0A==

randombytes@^2.1.0:
  version "2.1.0"
  resolved "https://registry.yarnpkg.com/randombytes/-/randombytes-2.1.0.tgz#df6f84372f0270dc65cdf6291349ab7a473d4f2a"
  integrity sha512-vYl3iOX+4CKUWuxGi9Ukhie6fsqXqS9FE2Zaic4tNFD2N2QQaXOMFbuKK4QmDHC0JO6B1Zp41J0LpT0oR68amQ==
  dependencies:
    safe-buffer "^5.1.0"

react-native-url-polyfill@^1.3.0:
  version "1.3.0"
  resolved "https://registry.yarnpkg.com/react-native-url-polyfill/-/react-native-url-polyfill-1.3.0.tgz#c1763de0f2a8c22cc3e959b654c8790622b6ef6a"
  integrity sha512-w9JfSkvpqqlix9UjDvJjm1EjSt652zVQ6iwCIj1cVVkwXf4jQhQgTNXY6EVTwuAmUjg6BC6k9RHCBynoLFo3IQ==
  dependencies:
    whatwg-url-without-unicode "8.0.0-3"

regenerator-runtime@^0.13.4:
  version "0.13.9"
  resolved "https://registry.yarnpkg.com/regenerator-runtime/-/regenerator-runtime-0.13.9.tgz#8925742a98ffd90814988d7566ad30ca3b263b52"
  integrity sha512-p3VT+cOEgxFsRRA9X4lkI1E+k2/CtnKtU4gcxyaCUreilL/vqI6CdZ3wxVUx3UOUg+gnUOQQcRI7BmSI656MYA==

resolve@^1.17.0, resolve@^1.19.0:
  version "1.22.1"
  resolved "https://registry.yarnpkg.com/resolve/-/resolve-1.22.1.tgz#27cb2ebb53f91abb49470a928bba7558066ac177"
  integrity sha512-nBpuuYuY5jFsli/JIs1oldw6fOQCBioohqWZg/2hiaOybXOft4lonv85uDOKXdf8rhyK159cxU5cDcK/NKk8zw==
  dependencies:
    is-core-module "^2.9.0"
    path-parse "^1.0.7"
    supports-preserve-symlinks-flag "^1.0.0"

rimraf@=3.0.2:
  version "3.0.2"
  resolved "https://registry.yarnpkg.com/rimraf/-/rimraf-3.0.2.tgz#f1a5402ba6220ad52cc1282bac1ae3aa49fd061a"
  integrity sha512-JZkJMZkAGFFPP2YqXZXPbMlMBgsxzE8ILs4lMIX/2o0L9UBw9O/Y3o6wFw/i9YLapcUJWwqbi3kdxIPdC62TIA==
  dependencies:
    glob "^7.1.3"

rollup-plugin-terser@=7.0.2:
  version "7.0.2"
  resolved "https://registry.yarnpkg.com/rollup-plugin-terser/-/rollup-plugin-terser-7.0.2.tgz#e8fbba4869981b2dc35ae7e8a502d5c6c04d324d"
  integrity sha512-w3iIaU4OxcF52UUXiZNsNeuXIMDvFrr+ZXK6bFZ0Q60qyVfq4uLptoS4bbq3paG3x216eQllFZX7zt6TIImguQ==
  dependencies:
    "@babel/code-frame" "^7.10.4"
    jest-worker "^26.2.1"
    serialize-javascript "^4.0.0"
    terser "^5.0.0"

rollup@=2.70.1:
  version "2.70.1"
  resolved "https://registry.yarnpkg.com/rollup/-/rollup-2.70.1.tgz#824b1f1f879ea396db30b0fc3ae8d2fead93523e"
  integrity sha512-CRYsI5EuzLbXdxC6RnYhOuRdtz4bhejPMSWjsFLfVM/7w/85n2szZv6yExqUXsBdz5KT8eoubeyDUDjhLHEslA==
  optionalDependencies:
    fsevents "~2.3.2"

rpc-websockets@^7.5.0:
  version "7.5.0"
  resolved "https://registry.yarnpkg.com/rpc-websockets/-/rpc-websockets-7.5.0.tgz#bbeb87572e66703ff151e50af1658f98098e2748"
  integrity sha512-9tIRi1uZGy7YmDjErf1Ax3wtqdSSLIlnmL5OtOzgd5eqPKbsPpwDP5whUDO2LQay3Xp0CcHlcNSGzacNRluBaQ==
  dependencies:
    "@babel/runtime" "^7.17.2"
    eventemitter3 "^4.0.7"
    uuid "^8.3.2"
    ws "^8.5.0"
  optionalDependencies:
    bufferutil "^4.0.1"
    utf-8-validate "^5.0.2"

safe-buffer@^5.0.1, safe-buffer@^5.1.0:
  version "5.2.1"
  resolved "https://registry.yarnpkg.com/safe-buffer/-/safe-buffer-5.2.1.tgz#1eaf9fa9bdb1fdd4ec75f58f9cdb4e6b7827eec6"
  integrity sha512-rp3So07KcdmmKbGvgaNxQSJr7bGVSVk5S9Eq1F+ppbRo70+YeaDxkw5Dd8NPN+GD6bjnYm2VuPuCXmpuYvmCXQ==

secp256k1@^4.0.2:
  version "4.0.3"
  resolved "https://registry.yarnpkg.com/secp256k1/-/secp256k1-4.0.3.tgz#c4559ecd1b8d3c1827ed2d1b94190d69ce267303"
  integrity sha512-NLZVf+ROMxwtEj3Xa562qgv2BK5e2WNmXPiOdVIPLgs6lyTzMvBq0aWTYMI5XCP9jZMVKOcqZLw/Wc4vDkuxhA==
  dependencies:
    elliptic "^6.5.4"
    node-addon-api "^2.0.0"
    node-gyp-build "^4.2.0"

serialize-javascript@^4.0.0:
  version "4.0.0"
  resolved "https://registry.yarnpkg.com/serialize-javascript/-/serialize-javascript-4.0.0.tgz#b525e1238489a5ecfc42afacc3fe99e666f4b1aa"
  integrity sha512-GaNA54380uFefWghODBWEGisLZFj00nS5ACs6yHa9nLqlLpVLO8ChDGeKRjZnV4Nh4n0Qi7nhYZD/9fCPzEqkw==
  dependencies:
    randombytes "^2.1.0"

snake-case@^3.0.4:
  version "3.0.4"
  resolved "https://registry.yarnpkg.com/snake-case/-/snake-case-3.0.4.tgz#4f2bbd568e9935abdfd593f34c691dadb49c452c"
  integrity sha512-LAOh4z89bGQvl9pFfNF8V146i7o7/CqFPbqzYgP+yYzDIDeS9HaNFtXABamRW+AQzEVODcvE79ljJ+8a9YSdMg==
  dependencies:
    dot-case "^3.0.4"
    tslib "^2.0.3"

source-map-support@~0.5.20:
  version "0.5.21"
  resolved "https://registry.yarnpkg.com/source-map-support/-/source-map-support-0.5.21.tgz#04fe7c7f9e1ed2d662233c28cb2b35b9f63f6e4f"
  integrity sha512-uBHU3L3czsIyYXKX88fdrGovxdSCoTGDRZ6SYXtSRxLZUzHg5P/66Ht6uoUlHu9EZod+inXhKo3qQgwXUT/y1w==
  dependencies:
    buffer-from "^1.0.0"
    source-map "^0.6.0"

source-map@^0.6.0:
  version "0.6.1"
  resolved "https://registry.yarnpkg.com/source-map/-/source-map-0.6.1.tgz#74722af32e9614e9c287a8d0bbde48b5e2f1a263"
  integrity sha512-UjgapumWlbMhkBgzT7Ykc5YXUT46F0iKu8SGXq0bcwP5dz/h0Plj6enJqjz1Zbq2l5WaqYnrVbwWOWMyF3F47g==

sourcemap-codec@^1.4.8:
  version "1.4.8"
  resolved "https://registry.yarnpkg.com/sourcemap-codec/-/sourcemap-codec-1.4.8.tgz#ea804bd94857402e6992d05a38ef1ae35a9ab4c4"
  integrity sha512-9NykojV5Uih4lgo5So5dtw+f0JgJX30KCNI8gwhz2J9A15wD0Ml6tjHKwf6fTSa6fAdVBdZeNOs9eJ71qCk8vA==

superstruct@^0.14.2:
  version "0.14.2"
  resolved "https://registry.yarnpkg.com/superstruct/-/superstruct-0.14.2.tgz#0dbcdf3d83676588828f1cf5ed35cda02f59025b"
  integrity sha512-nPewA6m9mR3d6k7WkZ8N8zpTWfenFH3q9pA2PkuiZxINr9DKB2+40wEQf0ixn8VaGuJ78AB6iWOtStI+/4FKZQ==

superstruct@^0.15.4:
  version "0.15.5"
  resolved "https://registry.yarnpkg.com/superstruct/-/superstruct-0.15.5.tgz#0f0a8d3ce31313f0d84c6096cd4fa1bfdedc9dab"
  integrity sha512-4AOeU+P5UuE/4nOUkmcQdW5y7i9ndt1cQd/3iUe+LTz3RxESf/W/5lg4B74HbDMMv8PHnPnGCQFH45kBcrQYoQ==

supports-color@^5.3.0:
  version "5.5.0"
  resolved "https://registry.yarnpkg.com/supports-color/-/supports-color-5.5.0.tgz#e2e69a44ac8772f78a1ec0b35b689df6530efc8f"
  integrity sha512-QjVjwdXIt408MIiAqCX4oUKsgU2EqAGzs2Ppkm4aQYbjm+ZEWEcW4SfFNTr4uMNZma0ey4f5lgLrkB0aX0QMow==
  dependencies:
    has-flag "^3.0.0"

supports-color@^7.0.0:
  version "7.2.0"
  resolved "https://registry.yarnpkg.com/supports-color/-/supports-color-7.2.0.tgz#1b7dcdcb32b8138801b3e478ba6a51caa89648da"
  integrity sha512-qpCAvRl9stuOHveKsn7HncJRvv501qIacKzQlO/+Lwxc9+0q2wLyv4Dfvt80/DPn2pqOBsJdDiogXGR9+OvwRw==
  dependencies:
    has-flag "^4.0.0"

supports-preserve-symlinks-flag@^1.0.0:
  version "1.0.0"
  resolved "https://registry.yarnpkg.com/supports-preserve-symlinks-flag/-/supports-preserve-symlinks-flag-1.0.0.tgz#6eda4bd344a3c94aea376d4cc31bc77311039e09"
  integrity sha512-ot0WnXS9fgdkgIcePe6RHNk1WA8+muPa6cSjeR3V8K27q9BB1rTE3R1p7Hv0z1ZyAc8s6Vvv8DIyWf681MAt0w==

terser@^5.0.0:
  version "5.14.2"
  resolved "https://registry.yarnpkg.com/terser/-/terser-5.14.2.tgz#9ac9f22b06994d736174f4091aa368db896f1c10"
  integrity sha512-oL0rGeM/WFQCUd0y2QrWxYnq7tfSuKBiqTjRPWrRgB46WD/kiwHwF8T23z78H6Q6kGCuuHcPB+KULHRdxvVGQA==
  dependencies:
    "@jridgewell/source-map" "^0.3.2"
    acorn "^8.5.0"
    commander "^2.20.0"
    source-map-support "~0.5.20"

text-encoding-utf-8@^1.0.2:
  version "1.0.2"
  resolved "https://registry.yarnpkg.com/text-encoding-utf-8/-/text-encoding-utf-8-1.0.2.tgz#585b62197b0ae437e3c7b5d0af27ac1021e10d13"
  integrity sha512-8bw4MY9WjdsD2aMtO0OzOCY3pXGYNx2d2FfHRVUKkiCPDWjKuOlhLVASS+pD7VkLTVjW268LYJHwsnPFlBpbAg==

"through@>=2.2.7 <3":
  version "2.3.8"
  resolved "https://registry.yarnpkg.com/through/-/through-2.3.8.tgz#0dd4c9ffaabc357960b1b724115d7e0e86a2e1f5"
  integrity sha512-w89qg7PI8wAdvX60bMDP+bFoD5Dvhm9oLheFp5O4a2QF0cSBGsBX4qZmadPMvVqlLJBBci+WqGGOAPvcDeNSVg==

toml@^3.0.0:
  version "3.0.0"
  resolved "https://registry.yarnpkg.com/toml/-/toml-3.0.0.tgz#342160f1af1904ec9d204d03a5d61222d762c5ee"
  integrity sha512-y/mWCZinnvxjTKYhJ+pYxwD0mRLVvOtdS2Awbgxln6iEnt4rk0yBxeSBHkGJcPucRiG0e55mwWp+g/05rsrd6w==

tr46@~0.0.3:
  version "0.0.3"
  resolved "https://registry.yarnpkg.com/tr46/-/tr46-0.0.3.tgz#8184fd347dac9cdc185992f3a6622e14b9d9ab6a"
  integrity sha512-N3WMsuqV66lT30CrXNbEjx4GEwlow3v6rr4mCcv6prnfwhS01rkgyFdjPNBYd9br7LpXV1+Emh01fHnq2Gdgrw==

tslib@=2.3.1:
  version "2.3.1"
  resolved "https://registry.yarnpkg.com/tslib/-/tslib-2.3.1.tgz#e8a335add5ceae51aa261d32a490158ef042ef01"
  integrity sha512-77EbyPPpMz+FRFRuAFlWMtmgUWGe9UOG2Z25NqCwiIjRhOf5iKGuzSe5P2w1laq+FkRy4p+PCuVkJSGkzTEKVw==

tslib@^2.0.3:
  version "2.4.0"
  resolved "https://registry.yarnpkg.com/tslib/-/tslib-2.4.0.tgz#7cecaa7f073ce680a05847aa77be941098f36dc3"
  integrity sha512-d6xOpEDfsi2CZVlPQzGeux8XMwLT9hssAsaPYExaQMuYskwb+x1x7J371tWlbBdWHroy99KnVB6qIkUbs5X3UQ==

tweetnacl@^1.0.3:
  version "1.0.3"
  resolved "https://registry.yarnpkg.com/tweetnacl/-/tweetnacl-1.0.3.tgz#ac0af71680458d8a6378d0d0d050ab1407d35596"
  integrity sha512-6rt+RN7aOi1nGMyC4Xa5DdYiukl2UWCbcJft7YhxReBGQD7OAM8Pbxw6YMo4r2diNEA8FEmu32YOn9rhaiE5yw==

typescript@=4.6.2:
  version "4.6.2"
  resolved "https://registry.yarnpkg.com/typescript/-/typescript-4.6.2.tgz#fe12d2727b708f4eef40f51598b3398baa9611d4"
  integrity sha512-HM/hFigTBHZhLXshn9sN37H085+hQGeJHJ/X7LpBWLID/fbc2acUMfU+lGD98X81sKP+pFa9f0DZmCwB9GnbAg==

utf-8-validate@^5.0.2:
  version "5.0.9"
  resolved "https://registry.yarnpkg.com/utf-8-validate/-/utf-8-validate-5.0.9.tgz#ba16a822fbeedff1a58918f2a6a6b36387493ea3"
  integrity sha512-Yek7dAy0v3Kl0orwMlvi7TPtiCNrdfHNd7Gcc/pLq4BLXqfAmd0J7OWMizUQnTTJsyjKn02mU7anqwfmUP4J8Q==
  dependencies:
    node-gyp-build "^4.3.0"

uuid@^8.3.2:
  version "8.3.2"
  resolved "https://registry.yarnpkg.com/uuid/-/uuid-8.3.2.tgz#80d5b5ced271bb9af6c445f21a1a04c606cefbe2"
  integrity sha512-+NYs2QeMWy+GWFOEm9xnn6HCDp0l7QBD7ml8zLUmJ+93Q5NF0NocErnwkTkXVFNiX3/fpC6afS8Dhb/gz7R7eg==

webidl-conversions@^3.0.0:
  version "3.0.1"
  resolved "https://registry.yarnpkg.com/webidl-conversions/-/webidl-conversions-3.0.1.tgz#24534275e2a7bc6be7bc86611cc16ae0a5654871"
  integrity sha512-2JAn3z8AR6rjK8Sm8orRC0h/bcl/DqL7tRPdGZ4I1CjdF+EaMLmYxBHyXuKL849eucPFhvBoxMsflfOb8kxaeQ==

webidl-conversions@^5.0.0:
  version "5.0.0"
  resolved "https://registry.yarnpkg.com/webidl-conversions/-/webidl-conversions-5.0.0.tgz#ae59c8a00b121543a2acc65c0434f57b0fc11aff"
  integrity sha512-VlZwKPCkYKxQgeSbH5EyngOmRp7Ww7I9rQLERETtf5ofd9pGeswWiOtogpEO850jziPRarreGxn5QIiTqpb2wA==

whatwg-url-without-unicode@8.0.0-3:
  version "8.0.0-3"
  resolved "https://registry.yarnpkg.com/whatwg-url-without-unicode/-/whatwg-url-without-unicode-8.0.0-3.tgz#ab6df4bf6caaa6c85a59f6e82c026151d4bb376b"
  integrity sha512-HoKuzZrUlgpz35YO27XgD28uh/WJH4B0+3ttFqRo//lmq+9T/mIOJ6kqmINI9HpUpz1imRC/nR/lxKpJiv0uig==
  dependencies:
    buffer "^5.4.3"
    punycode "^2.1.1"
    webidl-conversions "^5.0.0"

whatwg-url@^5.0.0:
  version "5.0.0"
  resolved "https://registry.yarnpkg.com/whatwg-url/-/whatwg-url-5.0.0.tgz#966454e8765462e37644d3626f6742ce8b70965d"
  integrity sha512-saE57nupxk6v3HY35+jzBwYa0rKSy0XR8JSxZPwgLr7ys0IBzhGviA1/TUGJLmSVqs8pb9AnvICXEuOHLprYTw==
  dependencies:
    tr46 "~0.0.3"
    webidl-conversions "^3.0.0"

wrappy@1:
  version "1.0.2"
  resolved "https://registry.yarnpkg.com/wrappy/-/wrappy-1.0.2.tgz#b5243d8f3ec1aa35f1364605bc0d1036e30ab69f"
  integrity sha512-l4Sp/DRseor9wL6EvV2+TuQn63dMkPjZ/sp9XkghTEbV9KlPS1xUsZ3u7/IQO4wxtcFB4bgpQPRcR3QCvezPcQ==

ws@^7.4.5:
  version "7.5.9"
  resolved "https://registry.yarnpkg.com/ws/-/ws-7.5.9.tgz#54fa7db29f4c7cec68b1ddd3a89de099942bb591"
  integrity sha512-F+P9Jil7UiSKSkppIiD94dN07AwvFixvLIj1Og1Rl9GGMuNipJnV9JzjD6XuqmAeiswGvUmNLjr5cFuXwNS77Q==

ws@^8.5.0:
  version "8.8.1"
  resolved "https://registry.yarnpkg.com/ws/-/ws-8.8.1.tgz#5dbad0feb7ade8ecc99b830c1d77c913d4955ff0"
  integrity sha512-bGy2JzvzkPowEJV++hF07hAD6niYSr0JzBNo/J29WsB57A2r7Wlc1UFcTR9IzrPvuNVO4B8LGqF8qcpsVOhJCA==
"#;
}
