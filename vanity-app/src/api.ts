import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type { UnlistenFn };

export interface ChainInfo {
  id: string;
  display_name: string;
  menu_label: string;
  supports_exact_case: boolean;
  pattern_hint: string;
}

export interface SystemInfo {
  cpu_description: string;
  worker_description: string;
  total_memory_gb: number;
  available_memory_gb: number;
  memory_pressure: string;
  summary_line: string;
  estimated_keys_per_sec: number;
}

export interface EstimateResult {
  pattern_description: string;
  case_mode: string;
  attempts_label: string;
  time_label: string;
  difficulty: string;
  difficulty_bars: string;
  risk: "None" | "Caution" | "Long" | "Impractical";
  pattern_chars: number;
  warning: string | null;
  length_guide: string | null;
  prefix_match: string;
  suffix_match: string;
  ignore_case: boolean;
}

export interface KeyExport {
  label: string;
  value: string;
  hint: string | null;
}

export interface ProgressPayload {
  attempts: number;
  rate: number;
  eta_min: number;
}

export interface SpeedPayload {
  rate: number;
  time_label: string;
}

export interface DonePayload {
  address: string;
  exports: KeyExport[];
  attempts: number;
  elapsed_secs: number;
  chain_display_name: string;
  pattern_description: string;
  case_mode: string;
  prefix_match: string;
  suffix_match: string;
  ignore_case: boolean;
}

export interface ErrorPayload {
  message: string;
}

export function listChains(): Promise<ChainInfo[]> {
  return invoke("list_chains");
}

export function getSystemProfile(chain: string): Promise<SystemInfo> {
  return invoke("get_system_profile", { chain });
}

export function estimate(
  chain: string,
  prefix: string,
  suffix: string,
  exact: boolean,
): Promise<EstimateResult> {
  return invoke("estimate", { chain, prefix, suffix, exact });
}

export function startGrind(
  chain: string,
  prefix: string,
  suffix: string,
  exact: boolean,
  force = false,
): Promise<void> {
  return invoke("start_grind", { chain, prefix, suffix, exact, force });
}

export function stopGrind(): Promise<void> {
  return invoke("stop_grind");
}

export function saveResult(payload: {
  chain_display_name: string;
  pattern_description: string;
  case_mode: string;
  address: string;
  exports: { label: string; value: string }[];
  attempts: number;
  elapsed_secs: number;
}): Promise<string | null> {
  return invoke("save_result", { payload });
}

export interface GrindEvents {
  onCalibrating: () => void;
  onSpeed: (p: SpeedPayload) => void;
  onProgress: (p: ProgressPayload) => void;
  onDone: (p: DonePayload) => void;
  onCancelled: () => void;
  onError: (p: ErrorPayload) => void;
}

export async function subscribeGrindEvents(handlers: GrindEvents): Promise<UnlistenFn> {
  const unlisteners = await Promise.all([
    listen("grind-calibrating", () => handlers.onCalibrating()),
    listen<SpeedPayload>("grind-speed", (e) => handlers.onSpeed(e.payload)),
    listen<ProgressPayload>("grind-progress", (e) => handlers.onProgress(e.payload)),
    listen<DonePayload>("grind-done", (e) => handlers.onDone(e.payload)),
    listen("grind-cancelled", () => handlers.onCancelled()),
    listen<ErrorPayload>("grind-error", (e) => handlers.onError(e.payload)),
  ]);
  return () => unlisteners.forEach((fn) => fn());
}
