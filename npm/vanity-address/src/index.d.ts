export type ChainId =
  | "algo"
  | "aptos"
  | "btc"
  | "btc-segwit"
  | "btc-taproot"
  | "ada"
  | "tia"
  | "cosmos"
  | "dash"
  | "doge"
  | "dydx"
  | "evm"
  | "fil"
  | "hedera"
  | "inj"
  | "icp"
  | "kaspa"
  | "ksm"
  | "ltc"
  | "erd"
  | "near"
  | "osmo"
  | "dot"
  | "xrp"
  | "sei"
  | "sol"
  | "xlm"
  | "sui"
  | "xtz"
  | "ton"
  | "trx";

export interface GenerateAddressOptions {
  chain: ChainId | string;
  prefix?: string;
  suffix?: string;
  /** Substring anywhere; `*` wildcards OK. Comma = OR alternatives. */
  contains?: string;
  caseSensitive?: boolean;
  /** Called as (attempts, info) — first arg stays a number for backward compatibility. */
  onProgress?: (attempts: number, info: ProgressInfo) => void;
  signal?: AbortSignal;
  /** Abort with TimeoutError after this many milliseconds. */
  timeoutMs?: number;
  /** Node only: worker thread count (default = CPU count, use 1 to disable). */
  workers?: number;
  /** Override keys/sec used for ETA estimates. */
  keysPerSec?: number;
}

export interface GenerateAddressesOptions extends GenerateAddressOptions {
  count: number;
}

export interface ProgressInfo {
  attempts: number;
  keysPerSec: number;
  etaSeconds?: number;
}

export interface KeyExport {
  label: string;
  value: string;
  hint?: string;
}

export interface Wallet {
  address: string;
  exports: KeyExport[];
  /** Convenience: first export value (same as exports[0].value when present). */
  privateKey?: string;
}

export interface ChainInfo {
  id: ChainId | string;
  name: string;
}

export type PatternRisk = "none" | "caution" | "long" | "impractical";

export interface DifficultyEstimate {
  attempts: number;
  attemptsLabel: string;
  avgSecs: number;
  timeLabel: string;
  difficulty: string;
  difficultyBars: string;
  risk: PatternRisk | string;
  patternChars: number;
  keysPerSec: number;
  workers: number;
}

export interface ValidatePatternResult {
  ok: boolean;
  error?: string;
  risk?: PatternRisk | string;
  attempts?: number;
  attemptsLabel?: string;
  timeLabel?: string;
}

export class VanityAddressError extends Error {
  code: "INVALID_CHAIN" | "INVALID_PATTERN" | "INTERNAL";
}

export function generateAddress(options: GenerateAddressOptions): Promise<Wallet>;
export function generateAddresses(options: GenerateAddressesOptions): Promise<Wallet[]>;
export function estimateDifficulty(options: GenerateAddressOptions): DifficultyEstimate;
export function validatePattern(options: GenerateAddressOptions): ValidatePatternResult;
export function listChains(): ChainInfo[];
export function isValidChain(id: string): boolean;
