import {
  listChains,
  estimate,
  getSystemProfile,
  startGrind,
  stopGrind,
  saveResult,
  subscribeGrindEvents,
  type ChainInfo,
  type DonePayload,
  type EstimateResult,
  type UnlistenFn,
} from "./api";

type ScreenId = "home" | "help" | "chain" | "pattern" | "summary" | "grind" | "result";
type MatchKind = "suffix" | "prefix" | "both";

const screens: Record<ScreenId, HTMLElement> = {
  home: el("#screen-home"),
  help: el("#screen-help"),
  chain: el("#screen-chain"),
  pattern: el("#screen-pattern"),
  summary: el("#screen-summary"),
  grind: el("#screen-grind"),
  result: el("#screen-result"),
};

function el<T extends HTMLElement = HTMLElement>(selector: string): T {
  const found = document.querySelector<T>(selector);
  if (!found) throw new Error(`missing element: ${selector}`);
  return found;
}

function showScreen(id: ScreenId) {
  for (const [key, node] of Object.entries(screens)) {
    node.classList.toggle("is-active", key === id);
  }
}

function toast(message: string) {
  const node = el("#toast");
  node.textContent = message;
  node.hidden = false;
  window.setTimeout(() => {
    node.hidden = true;
  }, 2600);
}

// ── App state ──────────────────────────────────────────────────────

let selectedChain: ChainInfo | null = null;
let matchKind: MatchKind = "suffix";
let lastEstimate: EstimateResult | null = null;
let impracticalConfirmShown = false;
let grindUnlisten: UnlistenFn | null = null;
let estimateTimer: number | undefined;
let lastResult: DonePayload | null = null;

// ── Home & Help ────────────────────────────────────────────────────

async function initHelpScreen() {
  const chains = await listChains();
  el("#help-chains").textContent = `Supported: ${chains.map((c) => c.id).join(", ")} · Cardano, TON coming soon`;
}

function goHome() {
  impracticalConfirmShown = false;
  showScreen("home");
}

// ── Chain screen ───────────────────────────────────────────────────

async function initChainScreen() {
  const grid = el("#chain-grid");
  const chains = await listChains();
  grid.innerHTML = "";
  for (const chain of chains) {
    const card = document.createElement("button");
    card.className = "chain-card";
    card.innerHTML = `<span class="chain-id">${chain.id}</span><span class="chain-label">${chain.menu_label}</span>`;
    card.addEventListener("click", () => selectChain(chain));
    grid.appendChild(card);
  }
}

function selectChain(chain: ChainInfo) {
  selectedChain = chain;
  matchKind = "suffix";
  el("#pattern-chain-name").textContent = chain.display_name;
  el("#pattern-hint").textContent = chain.pattern_hint;
  el<HTMLInputElement>("#input-prefix").value = "";
  el<HTMLInputElement>("#input-suffix").value = "";
  el<HTMLInputElement>("#input-exact").checked = false;

  const exactRow = el("#exact-row");
  const exactInput = el<HTMLInputElement>("#input-exact");
  exactInput.disabled = !chain.supports_exact_case;
  exactRow.classList.toggle("disabled", !chain.supports_exact_case);

  setMatchKind("suffix");
  hideEstimate();
  showScreen("pattern");
}

// ── Pattern screen ─────────────────────────────────────────────────

function setMatchKind(kind: MatchKind) {
  matchKind = kind;
  document.querySelectorAll<HTMLButtonElement>(".match-tabs .tab").forEach((tab) => {
    tab.classList.toggle("active", tab.dataset.kind === kind);
  });

  const prefixField = el("#field-prefix");
  const suffixField = el("#field-suffix");
  const prefixInput = el<HTMLInputElement>("#input-prefix");
  const suffixInput = el<HTMLInputElement>("#input-suffix");

  prefixField.classList.toggle("inactive", kind === "suffix");
  suffixField.classList.toggle("inactive", kind === "prefix");
  prefixInput.disabled = kind === "suffix";
  suffixInput.disabled = kind === "prefix";

  if (kind === "suffix") prefixInput.value = "";
  if (kind === "prefix") suffixInput.value = "";

  scheduleEstimate();
}

function hideEstimate() {
  el("#estimate-box").hidden = true;
  el("#pattern-error").hidden = true;
  lastEstimate = null;
  el<HTMLButtonElement>("#btn-continue").disabled = true;
}

function scheduleEstimate() {
  window.clearTimeout(estimateTimer);
  estimateTimer = window.setTimeout(runEstimate, 250);
}

function currentPatternValues() {
  const prefix = el<HTMLInputElement>("#input-prefix").value.trim();
  const suffix = el<HTMLInputElement>("#input-suffix").value.trim();
  const exact = el<HTMLInputElement>("#input-exact").checked;

  if (matchKind === "suffix") return { prefix: "", suffix, exact };
  if (matchKind === "prefix") return { prefix, suffix: "", exact };
  return { prefix, suffix, exact };
}

async function runEstimate() {
  if (!selectedChain) return;
  const { prefix, suffix, exact } = currentPatternValues();

  if (!prefix && !suffix) {
    hideEstimate();
    return;
  }

  try {
    const result = await estimate(selectedChain.id, prefix, suffix, exact);
    lastEstimate = result;
    el("#pattern-error").hidden = true;
    el("#estimate-box").hidden = false;
    el("#estimate-target").textContent = `${result.pattern_description} · ${result.case_mode}`;
    el("#estimate-difficulty").textContent = `${result.difficulty}  ${result.difficulty_bars}`;
    el("#estimate-attempts").textContent = `~${result.attempts_label}`;
    el("#estimate-time").textContent = result.time_label;

    const guide = el("#estimate-length-guide");
    if (result.length_guide) {
      guide.hidden = false;
      guide.textContent = result.length_guide;
    } else {
      guide.hidden = true;
    }

    const riskNote = el("#estimate-risk-note");
    if (result.warning) {
      riskNote.hidden = false;
      riskNote.classList.toggle("impractical", result.risk === "Impractical");
      riskNote.textContent = result.warning;
    } else {
      riskNote.hidden = true;
    }

    el<HTMLButtonElement>("#btn-continue").disabled = false;
  } catch (e) {
    el("#estimate-box").hidden = true;
    const errBox = el("#pattern-error");
    errBox.hidden = false;
    errBox.textContent = String(e);
    lastEstimate = null;
    el<HTMLButtonElement>("#btn-continue").disabled = true;
  }
}

async function showSummary() {
  if (!selectedChain || !lastEstimate) return;
  impracticalConfirmShown = false;
  el("#confirm-impractical").hidden = true;

  const { prefix, suffix, exact } = currentPatternValues();
  const sys = await getSystemProfile(selectedChain.id);

  el("#sum-chain").textContent = selectedChain.display_name;
  el("#sum-target").textContent = lastEstimate.pattern_description;
  el("#sum-mode").textContent = lastEstimate.case_mode;
  el("#sum-attempts").textContent = `~${lastEstimate.attempts_label} (avg)`;
  el("#sum-time").textContent = lastEstimate.time_label;
  el("#sum-time").className = riskClass(lastEstimate.risk);
  el("#sum-difficulty").textContent = `${lastEstimate.difficulty_bars} ${lastEstimate.difficulty}`;

  el("#sum-cpu").textContent = sys.cpu_description;
  el("#sum-workers").textContent = sys.worker_description;
  el("#sum-ram").textContent = `${sys.total_memory_gb.toFixed(1)} GB total · ${sys.available_memory_gb.toFixed(1)} GB free`;
  el("#sum-memory").textContent = sys.memory_pressure;
  el("#sum-speed").textContent = `~${formatSpeed(sys.estimated_keys_per_sec)} keys/sec (estimated, calibrated at grind)`;

  const warn = el("#sum-warning");
  if (lastEstimate.warning) {
    warn.hidden = false;
    warn.classList.toggle("impractical", lastEstimate.risk === "Impractical");
    warn.textContent = lastEstimate.warning;
  } else {
    warn.hidden = true;
  }

  const guide = el("#sum-length-guide");
  if (lastEstimate.length_guide) {
    guide.hidden = false;
    guide.textContent = lastEstimate.length_guide;
  } else {
    guide.hidden = true;
  }

  const grindBtn = el<HTMLButtonElement>("#btn-grind");
  if (lastEstimate.risk === "Impractical") {
    grindBtn.textContent = "Start anyway (not recommended)";
    grindBtn.classList.add("danger-fill");
  } else {
    grindBtn.textContent = "Start grinding";
    grindBtn.classList.remove("danger-fill");
  }

  // stash for grind
  el("#btn-grind").dataset.prefix = prefix;
  el("#btn-grind").dataset.suffix = suffix;
  el("#btn-grind").dataset.exact = String(exact);

  showScreen("summary");
}

function riskClass(risk: EstimateResult["risk"]): string {
  if (risk === "Impractical") return "risk-impractical";
  if (risk === "Long") return "risk-long";
  return "risk-ok";
}

async function beginGrind() {
  if (!selectedChain || !lastEstimate) return;

  const btn = el<HTMLButtonElement>("#btn-grind");
  const prefix = btn.dataset.prefix ?? "";
  const suffix = btn.dataset.suffix ?? "";
  const exact = btn.dataset.exact === "true";
  const force = lastEstimate.risk === "Impractical";

  if (force && !impracticalConfirmShown) {
    impracticalConfirmShown = true;
    el("#confirm-impractical").hidden = false;
    btn.textContent = "Really start?";
    return;
  }

  el("#grind-target").textContent = `${selectedChain.display_name} · ${lastEstimate.pattern_description}`;
  el("#grind-phase").textContent = "Calibrating…";
  el("#grind-line").hidden = true;
  showScreen("grind");

  grindUnlisten?.();
  grindUnlisten = await subscribeGrindEvents({
    onCalibrating: () => {
      el("#grind-phase").textContent = "Calibrating…";
    },
    onSpeed: (p) => {
      el("#grind-phase").textContent = `Grinding · est. ${p.time_label} at ~${formatSpeed(p.rate)} keys/s`;
    },
    onProgress: (p) => {
      el("#grind-line").hidden = false;
      el("#grind-attempts").textContent = formatCount(p.attempts);
      el("#grind-rate").textContent = formatSpeed(p.rate);
      el("#grind-eta").textContent = `${Math.max(0, Math.round(p.eta_min))} min`;
    },
    onDone: (p) => {
      grindUnlisten?.();
      grindUnlisten = null;
      showResult(p);
    },
    onCancelled: () => {
      grindUnlisten?.();
      grindUnlisten = null;
      toast("Grind stopped");
      showScreen("summary");
    },
    onError: (p) => {
      grindUnlisten?.();
      grindUnlisten = null;
      toast(`Error: ${p.message}`);
      showScreen("summary");
    },
  });

  try {
    await startGrind(selectedChain.id, prefix, suffix, exact, force);
  } catch (e) {
    grindUnlisten?.();
    grindUnlisten = null;
    toast(`Error: ${String(e)}`);
    showScreen("summary");
  }
}

function formatSpeed(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `${(n / 1_000).toFixed(0)}K`;
  return n.toFixed(0);
}

function formatCount(n: number): string {
  return n.toLocaleString("en-US");
}

// ── Grind screen ───────────────────────────────────────────────────

async function handleStop() {
  el<HTMLButtonElement>("#btn-stop").disabled = true;
  try {
    await stopGrind();
  } catch (e) {
    toast(String(e));
  } finally {
    el<HTMLButtonElement>("#btn-stop").disabled = false;
  }
}

// ── Result screen ──────────────────────────────────────────────────

function highlightAddress(
  address: string,
  prefixMatch: string,
  suffixMatch: string,
  ignoreCase: boolean,
): string {
  const starts =
    prefixMatch.length > 0 &&
    address.length >= prefixMatch.length &&
    (ignoreCase
      ? address.slice(0, prefixMatch.length).toLowerCase() === prefixMatch.toLowerCase()
      : address.startsWith(prefixMatch));
  const ends =
    suffixMatch.length > 0 &&
    address.length >= suffixMatch.length &&
    (ignoreCase
      ? address.slice(-suffixMatch.length).toLowerCase() === suffixMatch.toLowerCase()
      : address.endsWith(suffixMatch));

  if (starts && ends) {
    const mid = address.slice(prefixMatch.length, address.length - suffixMatch.length);
    return `<span class="match-hl">${escapeHtml(address.slice(0, prefixMatch.length))}</span>${escapeHtml(mid)}<span class="match-hl">${escapeHtml(address.slice(-suffixMatch.length))}</span>`;
  }
  if (starts) {
    return `<span class="match-hl">${escapeHtml(address.slice(0, prefixMatch.length))}</span>${escapeHtml(address.slice(prefixMatch.length))}`;
  }
  if (ends) {
    return `${escapeHtml(address.slice(0, -suffixMatch.length))}<span class="match-hl">${escapeHtml(address.slice(-suffixMatch.length))}</span>`;
  }
  return escapeHtml(address);
}

function escapeHtml(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function showResult(payload: DonePayload) {
  lastResult = payload;
  const rate = payload.elapsed_secs > 0 ? payload.attempts / payload.elapsed_secs : 0;

  el("#result-address").innerHTML = highlightAddress(
    payload.address,
    payload.prefix_match,
    payload.suffix_match,
    payload.ignore_case,
  );
  el("#result-stats").textContent = `${formatCount(payload.attempts)} attempts in ${payload.elapsed_secs.toFixed(2)}s (${formatSpeed(rate)} keys/s avg)`;

  const container = el("#result-exports");
  container.innerHTML = "";
  payload.exports.forEach((exp, i) => {
    const row = document.createElement("div");
    row.className = "export-row";
    const valueId = `export-value-${i}`;
    row.innerHTML = `
      <span class="export-label">${escapeHtml(exp.label)}</span>
      <div class="export-value-row">
        <span class="export-value" id="${valueId}">${escapeHtml(exp.value)}</span>
        <button class="icon-btn" data-reveal="${valueId}">Reveal</button>
        <button class="copy-btn" data-copy-text="${encodeURIComponent(exp.value)}">Copy</button>
      </div>
      ${exp.hint ? `<div class="export-hint">${escapeHtml(exp.hint)}</div>` : ""}
    `;
    container.appendChild(row);
  });

  showScreen("result");
}

function buildCopyAllBlock(): string {
  if (!lastResult) return "";
  const lines = [`Address: ${lastResult.address}`];
  for (const exp of lastResult.exports) {
    lines.push(`${exp.label}: ${exp.value}`);
  }
  return lines.join("\n");
}

async function handleSave() {
  if (!lastResult) return;
  try {
    const path = await saveResult({
      chain_display_name: lastResult.chain_display_name,
      pattern_description: lastResult.pattern_description,
      case_mode: lastResult.case_mode,
      address: lastResult.address,
      exports: lastResult.exports.map((e) => ({ label: e.label, value: e.value })),
      attempts: lastResult.attempts,
      elapsed_secs: lastResult.elapsed_secs,
    });
    if (path) toast(`Saved to ${path}`);
  } catch (e) {
    toast(`Save failed: ${String(e)}`);
  }
}

async function copyText(text: string) {
  await navigator.clipboard.writeText(text);
  toast("Copied to clipboard");
}

// ── Wiring ─────────────────────────────────────────────────────────

function init() {
  showScreen("home");
  initChainScreen();
  initHelpScreen();

  el("#btn-start").addEventListener("click", () => showScreen("chain"));
  el("#btn-help").addEventListener("click", () => showScreen("help"));
  el("#btn-home").addEventListener("click", goHome);

  document.querySelectorAll<HTMLElement>("[data-back]").forEach((btn) => {
    btn.addEventListener("click", () => showScreen(btn.dataset.back as ScreenId));
  });

  document.querySelectorAll<HTMLButtonElement>(".match-tabs .tab").forEach((tab) => {
    tab.addEventListener("click", () => setMatchKind(tab.dataset.kind as MatchKind));
  });

  el<HTMLInputElement>("#input-prefix").addEventListener("input", scheduleEstimate);
  el<HTMLInputElement>("#input-suffix").addEventListener("input", scheduleEstimate);
  el<HTMLInputElement>("#input-exact").addEventListener("change", scheduleEstimate);
  el("#btn-continue").addEventListener("click", showSummary);
  el("#btn-grind").addEventListener("click", beginGrind);
  el("#btn-stop").addEventListener("click", handleStop);
  el("#btn-save").addEventListener("click", handleSave);
  el("#btn-again").addEventListener("click", () => {
    lastResult = null;
    impracticalConfirmShown = false;
    showScreen("pattern");
  });
  el("#btn-copy-all").addEventListener("click", () => copyText(buildCopyAllBlock()));

  document.body.addEventListener("click", (e) => {
    const target = e.target as HTMLElement;

    const revealId = target.dataset.reveal;
    if (revealId) {
      el(`#${revealId}`).classList.toggle("revealed");
      target.textContent = el(`#${revealId}`).classList.contains("revealed") ? "Hide" : "Reveal";
      return;
    }

    const copyTarget = target.dataset.copyTarget;
    if (copyTarget) {
      copyText(el(`#${copyTarget}`).textContent ?? "");
      return;
    }

    const copyText_ = target.dataset.copyText;
    if (copyText_ !== undefined) {
      copyText(decodeURIComponent(copyText_));
    }
  });
}

window.addEventListener("DOMContentLoaded", init);
