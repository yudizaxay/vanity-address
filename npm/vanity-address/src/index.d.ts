export interface GenerateAddressOptions {
  chain: string;
  prefix?: string;
  suffix?: string;
  caseSensitive?: boolean;
  onProgress?: (attempts: number) => void;
  signal?: AbortSignal;
}

export interface KeyExport {
  label: string;
  value: string;
  hint?: string;
}

export interface Wallet {
  address: string;
  exports: KeyExport[];
}

export class VanityAddressError extends Error {
  code: "INVALID_CHAIN" | "INVALID_PATTERN" | "INTERNAL";
}

export function generateAddress(options: GenerateAddressOptions): Promise<Wallet>;
