import { $, eventElement } from "./dom.ts";
import {
  escapeHtml as esc,
  human,
  errorMessage,
  savedSketches,
  savedSession,
  parseSketch,
  chordDuration,
} from "./model.ts";
import type {
  Progression,
  GenerationRequest,
  Sketch,
  Session,
  EngineInfo,
  RenderedChord,
  ScoreKey,
} from "./model.ts";
const clone = <T>(value: T): T => structuredClone(value);
const fields = [
  "key",
  "mode",
  "style",
  "region",
  "era",
  "mood",
  "complexity",
  "creativity",
  "length",
  "seed",
  "energy",
  "cadence",
  "harmonic_trait",
  "tempo",
  "beats_per_chord",
] as const;
const numeric = new Set([
  "creativity",
  "length",
  "seed",
  "tempo",
  "beats_per_chord",
]);
const KEYS = {
  session: "noctave.session.v1",
  sketches: "noctave.sketches.v1",
  volume: "noctave.volume.v1",
};
let progression: Progression | null = null,
  selected = 0,
  busy = false,
  history: Session[] = [],
  title = "Untitled exploration",
  exploration = 1;
let toastTimer: ReturnType<typeof setTimeout> | undefined,
  storageWarning = false,
  sketches: Sketch[] = [],
  audio: AudioContext | null = null,
  master: GainNode | null = null,
  nodes = new Set<OscillatorNode>();
let playing = false,
  looping = false,
  nextChord = 0,
  nextTime = 0,
  scheduler: ReturnType<typeof setInterval> | undefined,
  animation = 0,
  playbackStart = 0,
  playDuration = 0;
let scheduled: Array<{ time: number; index: number }> = [],
  volume = 0.55,
  audioStarting = false;

function toast(message: string, error = false) {
  clearTimeout(toastTimer);
  $("toast").textContent = message;
  $("toast").classList.toggle("error", error);
  $("toast").hidden = false;
  toastTimer = setTimeout(
    () => {
      $("toast").hidden = true;
    },
    error ? 7500 : 3800,
  );
}
function readStorage(key: string, fallback: unknown): unknown {
  try {
    const item = localStorage.getItem(key);
    return item === null ? fallback : JSON.parse(item);
  } catch {
    return fallback;
  }
}
function writeStorage(key: string, value: unknown) {
  try {
    localStorage.setItem(key, JSON.stringify(value));
    return true;
  } catch {
    if (!storageWarning) {
      toast(
        "Browser storage is unavailable or full. Export JSON to keep your work.",
        true,
      );
      storageWarning = true;
    }
    return false;
  }
}
async function api(path: string, body: unknown, binary: true): Promise<Blob>;
async function api<T = Progression>(
  path: string,
  body: unknown,
  binary?: false,
): Promise<T>;
async function api(
  path: string,
  body: unknown,
  binary = false,
): Promise<unknown> {
  const response = await fetch(path, {
    method: "POST",
    headers: { "Content-Type": "application/json", "X-Noctave": "1" },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(30000),
  });
  if (!response.ok) {
    let message = `Request failed (${response.status})`;
    try {
      message = (await response.json()).error || message;
    } catch {}
    throw new Error(message);
  }
  return binary ? response.blob() : response.json();
}
function report(error: unknown) {
  console.error(error);
  toast(
    error instanceof Error && error.name === "TimeoutError"
      ? "The local engine took too long. Please try again."
      : errorMessage(error),
    true,
  );
}
function setBusy(value: boolean) {
  busy = value;
  document.body.classList.toggle("busy", value);
  $("chord-grid").setAttribute("aria-busy", String(value));
  document
    .querySelectorAll<HTMLInputElement | HTMLSelectElement | HTMLButtonElement>(
      "#controls input, #controls select, #controls button",
    )
    .forEach((node) => (node.disabled = value));
  for (const id of [
    "save",
    "export-toggle",
    "play",
    "explain",
    "replay",
    "transpose-key",
    "tempo",
    "beats_per_chord",
    "import",
  ] as const)
    $(id).disabled = value || (!progression && id !== "import");
  $("undo").disabled = value || history.length === 0;
  $("unlock-all").disabled =
    value || !progression?.chords.some((c) => c.locked);
  $("seed").disabled = value;
  if (!value) updateAvailability();
}
function updateAvailability() {
  $("region").disabled = $("style").value !== "post-rock";
  if ($("region").disabled) $("region").value = "global";
  $("era").disabled = $("region").value !== "japan";
  if ($("era").disabled) $("era").value = "any";
  $("creativity-value").textContent = `${$("creativity").value}%`;
  $("creativity").style.setProperty("--fill", `${$("creativity").value}%`);
}
function requestFromForm(randomize: boolean): GenerationRequest {
  const result: Record<string, unknown> = {};
  for (const field of fields)
    result[field] = numeric.has(field)
      ? Number($(field).value)
      : $(field).value;
  for (const [field, min, max] of [
    ["seed", 0, 4294967295],
    ["tempo", 40, 220],
    ["creativity", 0, 100],
  ] as const) {
    const value = Number(result[field]);
    if (
      !$(field).value.trim() ||
      !Number.isInteger(value) ||
      value < min ||
      value > max
    )
      throw new Error(
        `${human(field)} must be a whole number from ${min} to ${max}.`,
      );
  }
  if (randomize) result.seed = crypto.getRandomValues(new Uint32Array(1))[0];
  result.locks = Array.from({ length: Number(result.length) }, (_, i) =>
    progression?.chords[i]?.locked ? clone(progression.chords[i].chord) : null,
  );
  return result as unknown as GenerationRequest;
}
function applyForm(request: GenerationRequest) {
  for (const field of fields) {
    if (request[field] === undefined) continue;
    const control = $(field);
    if (
      control instanceof HTMLSelectElement &&
      !Array.from(control.options).some(
        (o) => o.value === String(request[field]),
      )
    ) {
      control.add(new Option(String(request[field]), String(request[field])));
    }
    control.value = String(request[field]);
  }
  $("transpose-key").value = request.key;
  updateAvailability();
}
function pushHistory() {
  if (progression) {
    history.push({ progression: clone(progression), title, exploration });
    if (history.length > 24) history.shift();
  }
}
function persist() {
  if (progression)
    writeStorage(KEYS.session, { progression, title, exploration });
}
async function generate(randomize = true) {
  if (busy) return;
  let request;
  try {
    request = requestFromForm(randomize);
  } catch (error) {
    report(error);
    return;
  }
  if (
    progression &&
    request.locks.length === progression.chords.length &&
    request.locks.every(Boolean)
  ) {
    toast(
      "Every chord is locked. Unlock a chord to explore another possibility.",
    );
    return;
  }
  stopPlayback();
  setBusy(true);
  try {
    const result = await api("/api/generate", request);
    pushHistory();
    progression = result;
    selected = 0;
    exploration++;
    title = "Untitled exploration";
    applyForm(result.request);
    render();
    persist();
    $("generate-hint").textContent =
      `${result.searched} possibilities explored. This one is yours.`;
  } catch (error) {
    report(error);
    if (!progression)
      $("chord-grid").innerHTML =
        '<div class="loading-placeholder">The engine could not start. Try Generate progression again.</div>';
  } finally {
    setBusy(false);
  }
}
function render() {
  if (!progression) return;
  const p = progression;
  selected = Math.min(selected, p.chords.length - 1);
  $("progression-heading").innerHTML =
    `${esc(title)} <span class="small-badge">${String(exploration).padStart(2, "0")}</span>`;
  $("progression-meta").innerHTML =
    `<span class="tag key-tag">${esc(p.request.key)} ${esc(human(p.request.mode))}</span><span class="tag">${esc(human(p.request.style))}</span><span class="tag">${esc(human(p.request.mood))}</span><span class="tag muted-tag">${p.chords.length} chords · ${p.request.creativity}% creativity${p.request.region === "japan" ? " · Japan*" : ""}</span>`;
  $("chord-grid").innerHTML = p.chords
    .map(
      (
        c,
        i,
      ) => `<article class="chord-card${i === selected ? " selected" : ""}${c.locked ? " locked" : ""}" data-index="${i}">
    <button class="chord-select" data-select="${i}" aria-label="Select and audition chord ${i + 1}: ${esc(c.name)}" aria-pressed="${i === selected}"><span class="chord-top">${String(i + 1).padStart(2, "0")}<span class="numeral">${esc(c.roman)}</span></span><span class="chord-name${c.name.length > 10 ? " small" : ""}">${esc(c.name)}</span><span class="chord-notes">${c.notes.map(esc).join(" · ")}</span></button>
    <button class="chord-lock icon-button" data-lock="${i}" aria-label="${c.locked ? "Unlock" : "Lock"} chord ${i + 1}" aria-pressed="${c.locked}" title="${c.locked ? "Unlock" : "Lock"} this chord">${c.locked ? "▣" : "▢"}</button><span class="chord-level" style="--tension:${Math.round(c.tension * 100)}%"></span></article>`,
    )
    .join("");
  const locked = p.chords.filter((c) => c.locked).length;
  $("lock-count").textContent = String(locked);
  $("unlock-all").disabled = locked === 0;
  $("generate").innerHTML =
    `<span aria-hidden="true">✧</span> ${locked ? "Regenerate unlocked" : "Generate progression"} <kbd>G</kbd>`;
  $("energy-label").textContent = human(p.request.energy).toUpperCase();
  $("play-position").textContent =
    `00 / ${String(p.chords.length).padStart(2, "0")}`;
  $("transpose-key").value = p.request.key;
  drawTension();
  renderDetail();
  for (const id of [
    "save",
    "export-toggle",
    "play",
    "explain",
    "replay",
    "transpose-key",
  ] as const)
    $(id).disabled = busy;
  $("undo").disabled = history.length === 0 || busy;
}
function renderSelection() {
  document.querySelectorAll(".chord-card").forEach((node, i) => {
    node.classList.toggle("selected", i === selected);
    node
      .querySelector(".chord-select")
      ?.setAttribute("aria-pressed", String(i === selected));
  });
  renderDetail();
  drawTension();
}
function renderDetail() {
  if (!progression) return;
  const c = progression.chords[selected];
  $("detail-name").textContent = c.name;
  $("detail-tags").innerHTML =
    `<span class="tag">${esc(c.roman)}</span><span class="tag">${esc(c.function)}</span><span class="tag">${c.chord.inversion ? `Inversion ${c.chord.inversion}` : "Root position"}</span>`;
  const pitches = new Set(c.midi.map((n) => n % 12));
  const white = [0, 2, 4, 5, 7, 9, 11, 12, 14, 16, 17, 19, 21, 23];
  const blacks = [
    { pc: 1, x: 1 },
    { pc: 3, x: 2 },
    { pc: 6, x: 4 },
    { pc: 8, x: 5 },
    { pc: 10, x: 6 },
    { pc: 1, x: 8 },
    { pc: 3, x: 9 },
    { pc: 6, x: 11 },
    { pc: 8, x: 12 },
    { pc: 10, x: 13 },
  ];
  $("piano").innerHTML =
    white
      .map(
        (p) =>
          `<span class="white-key${pitches.has(p % 12) ? " active" : ""}"></span>`,
      )
      .join("") +
    blacks
      .map(
        (b) =>
          `<span class="black-key${pitches.has(b.pc) ? " active" : ""}" style="left:${(b.x / 14) * 100}%"></span>`,
      )
      .join("");
  $("piano").setAttribute(
    "aria-label",
    `Pitch classes in ${c.name}: ${c.notes.join(", ")}`,
  );
  $("detail-notes").textContent = c.notes.join("   ");
  $("detail-reason").textContent =
    c.reasons.find((r) => r.startsWith("Shares")) || c.reasons[0];
}
function drawTension() {
  if (!progression) return;
  const chords = progression.chords;
  const w = 420,
    h = 106,
    pad = 9;
  const point = (v: number, i: number): [number, number] => [
    pad + (i * (w - pad * 2)) / (chords.length - 1),
    9 + (1 - v) * 72,
  ];
  const points = chords.map((c, i) => point(c.tension, i));
  const path = points
    .map(([x, y], i) => `${i ? "L" : "M"}${x.toFixed(1)},${y.toFixed(1)}`)
    .join(" ");
  const target = chords
    .map((c, i) => point(c.target_tension, i))
    .map(([x, y], i) => `${i ? "L" : "M"}${x.toFixed(1)},${y.toFixed(1)}`)
    .join(" ");
  $("tension-chart").innerHTML =
    `<svg viewBox="0 0 ${w} ${h}" role="img" aria-label="Contextual tension for ${chords.length} chords, solid line. Target curve, dashed line."><defs><linearGradient id="tension-fill" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stop-color="#bde4a2" stop-opacity=".15"/><stop offset="100%" stop-color="#bde4a2" stop-opacity="0"/></linearGradient></defs>${[25, 53, 81].map((y) => `<line x1="${pad}" x2="${w - pad}" y1="${y}" y2="${y}" stroke="#34442b" stroke-width=".6" stroke-dasharray="2 4"/>`).join("")}<path d="${path} L${w - pad},88 L${pad},88 Z" fill="url(#tension-fill)"/><path d="${target}" fill="none" stroke="#6e865d" stroke-width="1.2" stroke-dasharray="4 5"/><path d="${path}" fill="none" stroke="#c4efac" stroke-width="1.8" stroke-linejoin="round"/>${points.map(([x, y], i) => `<circle cx="${x}" cy="${y}" r="${i === selected ? 4 : 2.8}" fill="${i === selected ? "#d9f3c7" : "#1c2818"}" stroke="#b8d69f" stroke-width="1.2"/><text x="${x}" y="103" text-anchor="middle" fill="#7e956c" font-size="8" font-family="monospace">${String(i + 1).padStart(2, "0")}</text>`).join("")}</svg>`;
}
function syncLocks() {
  if (!progression) return;
  progression.request.locks = progression.chords.map((c) =>
    c.locked ? clone(c.chord) : null,
  );
  persist();
}
function toggleLock(index: number) {
  if (!progression || busy) return;
  progression.chords[index].locked = !progression.chords[index].locked;
  syncLocks();
  render();
}
async function transposeTo(key: string) {
  if (busy || !progression || key === progression.request.key) return;
  stopPlayback();
  setBusy(true);
  try {
    const result = await api("/api/transpose", { progression, key });
    pushHistory();
    progression = result;
    applyForm(result.request);
    render();
    persist();
    toast(`Transposed to ${key}. Chord relationships and locks preserved.`);
  } catch (error) {
    $("transpose-key").value = progression.request.key;
    report(error);
  } finally {
    setBusy(false);
  }
}
function undo() {
  if (busy || !history.length) return;
  stopPlayback();
  const previous = history.pop();
  if (!previous) return;
  progression = previous.progression;
  title = previous.title;
  exploration = previous.exploration;
  applyForm(progression.request);
  selected = 0;
  render();
  persist();
  toast("Previous exploration restored.");
}
function modal(html: string) {
  $("modal-content").innerHTML = html;
  if (!$("modal").open) $("modal").showModal();
}
function explain() {
  if (!progression) return;
  const c = progression.chords[selected];
  const keys: ScoreKey[] = [
    "theory",
    "style",
    "movement",
    "tension",
    "repetition",
    "phrase",
  ];
  modal(
    `<div class="eyebrow">CHORD ${String(selected + 1).padStart(2, "0")} · THE THEORY BEHIND IT</div><h2>${esc(c.name)} <span class="small-badge">${esc(c.roman)}</span></h2><p>${esc(c.notes.join(" · "))} · ${esc(c.function)}</p><ul>${c.reasons.map((r) => `<li>${esc(r)}</li>`).join("")}</ul><div class="modal-scores">${keys.map((key) => `<div class="score-cell"><small>${key}</small>${c.score[key].toFixed(2)}</div>`).join("")}</div><p>These are the engine’s actual score contributions, not probabilities or a measure of musical quality. The tension model and style weights are experimental.</p>`,
  );
}
function profileInfo() {
  const p = progression?.profile;
  modal(
    `<div class="eyebrow">A NOTE ON STYLE</div><h2>Exploration, with context.</h2><p>${p ? esc(p.lineage.join(" → ")) : "Rock family profiles"}</p><p>Every style in this first beta is a hand-authored musical hypothesis. These profiles have not yet been validated against an annotated song corpus or musician listening tests.</p><p>The Japan and 2000s modifiers are exploratory sound-design variants. They should not be treated as verified descriptions of a regional scene.</p><div class="modal-scores"><div class="score-cell"><small>Research status</small>Experimental</div><div class="score-cell"><small>Annotated tracks</small>0</div><div class="score-cell"><small>Confidence</small>Unvalidated</div></div><p>You can inspect every bundled profile in data/styles/profiles.json. Your listening feedback helps decide what comes next.</p>`,
  );
}
function showView(name: "studio" | "sketchbook") {
  $("studio-view").hidden = name !== "studio";
  $("sketchbook-view").hidden = name !== "sketchbook";
  $("studio-tab").classList.toggle("active", name === "studio");
  $("sketchbook-tab").classList.toggle("active", name === "sketchbook");
  $("page-label").textContent =
    name === "studio" ? "Harmonic studio" : "Sketchbook";
  if (name === "sketchbook") renderSketches();
}
function saveSketch() {
  if (!progression || busy) return;
  modal(
    `<div class="eyebrow">KEEP THIS FEELING</div><h2>Save your sketch.</h2><label for="sketch-name">Give this progression a name</label><input id="sketch-name" class="save-input" type="text" maxlength="70" value="${esc(title === "Untitled exploration" ? `${human(progression.request.mood)} in ${progression.request.key}` : title)}"><p>Saved in this browser. Export JSON for a backup you can move to another device.</p><div class="modal-actions"><button id="cancel-save" class="button secondary">Cancel</button><button id="confirm-save" class="button primary">Save sketch</button></div>`,
  );
  $("sketch-name").select();
  $("cancel-save").onclick = () => $("modal").close();
  $("confirm-save").onclick = () => {
    if (!progression) return;
    if (sketches.length >= 60) {
      toast(
        "Your sketchbook has 60 sketches. Export and remove one before saving another.",
        true,
      );
      return;
    }
    const name = $("sketch-name").value.trim() || "Untitled sketch";
    const item = {
      id: crypto.randomUUID(),
      title: name,
      saved: new Date().toISOString(),
      progression: clone(progression),
    };
    const next = [item, ...sketches];
    if (!writeStorage(KEYS.sketches, next)) return;
    sketches = next;
    title = name;
    persist();
    render();
    renderSketches();
    $("modal").close();
    toast("Sketch saved. A good place to come back to.");
  };
  $("sketch-name").onkeydown = (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      $("confirm-save").click();
    }
  };
}
function renderSketches() {
  $("saved-count").textContent = String(sketches.length);
  if (!sketches.length) {
    $("sketches").innerHTML =
      '<div class="empty-state"><span aria-hidden="true">▤</span><h2>Leave a little room for ideas.</h2><p>Your saved progressions will live here.<br>Start with a feeling, then keep what resonates.</p><button id="back-to-studio" class="button primary">Explore the studio →</button></div>';
    $("back-to-studio").onclick = () => showView("studio");
    return;
  }
  $("sketches").innerHTML = sketches
    .map(
      (s) =>
        `<article class="sketch"><div class="eyebrow">${esc(new Date(s.saved).toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" }))}</div><h3>${esc(s.title)}</h3><p>${esc(s.progression.request.key)} ${esc(human(s.progression.request.mode))} · ${esc(human(s.progression.request.style))}</p><p class="sketch-chords">${s.progression.chords.map((c) => esc(c.name)).join(" · ")}</p><div class="sketch-actions"><button class="button secondary" data-load="${esc(s.id)}">Open sketch ↗</button><button class="icon-button" data-download="${esc(s.id)}" aria-label="Export ${esc(s.title)} as JSON">↓</button><button class="icon-button remove-sketch" data-remove="${esc(s.id)}" aria-label="Remove ${esc(s.title)}">×</button></div></article>`,
    )
    .join("");
}
async function loadSketch(item: Pick<Sketch, "title" | "progression">) {
  if (busy) return;
  stopPlayback();
  setBusy(true);
  try {
    const result = await api("/api/restore", item.progression);
    pushHistory();
    progression = result;
    title = item.title || "Imported sketch";
    selected = 0;
    applyForm(result.request);
    render();
    persist();
    showView("studio");
    toast("Sketch restored, including its voicings and locks.");
  } catch (error) {
    report(error);
  } finally {
    setBusy(false);
  }
}
function removeSketch(id: string) {
  const item = sketches.find((s) => s.id === id);
  if (!item) return;
  modal(
    `<div class="eyebrow">YOUR SKETCHBOOK</div><h2>Remove this sketch?</h2><p>“${esc(item.title)}” will be removed from this browser’s sketchbook. Export a JSON copy first if you want to keep it.</p><div class="modal-actions"><button class="button secondary" id="cancel-remove">Keep sketch</button><button class="button primary" id="confirm-remove">Remove sketch</button></div>`,
  );
  $("cancel-remove").onclick = () => $("modal").close();
  $("confirm-remove").onclick = () => {
    const next = sketches.filter((s) => s.id !== id);
    if (writeStorage(KEYS.sketches, next)) {
      sketches = next;
      renderSketches();
      $("modal").close();
      toast("Sketch removed from this browser.");
    }
  };
}
function filename(name = title) {
  return `noctave-${
    name
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-|-$/g, "")
      .slice(0, 60) || "sketch"
  }`;
}
function download(blob: Blob, name: string) {
  const url = URL.createObjectURL(blob),
    link = document.createElement("a");
  link.href = url;
  link.download = name;
  document.body.append(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 3000);
}
async function exportAs(kind: string) {
  if (!progression || busy) return;
  closeExport();
  const p = progression,
    names = p.chords.map((c) => c.name).join(" → "),
    romans = p.chords.map((c) => c.roman).join(" → ");
  try {
    if (kind === "copy" || kind === "roman") {
      await navigator.clipboard.writeText(kind === "copy" ? names : romans);
      toast("Copied to clipboard.");
    } else if (kind === "json")
      download(
        new Blob([JSON.stringify(p, null, 2)], { type: "application/json" }),
        `${filename()}.json`,
      );
    else if (kind === "text")
      download(
        new Blob(
          [
            `${title}\nNoctave ${p.engine_version}\n${p.request.key} ${human(p.request.mode)} · ${human(p.request.style)}\nSeed: ${p.request.seed} · ${p.request.tempo} BPM · ${p.request.beats_per_chord} beats/chord\n\n${names}\n${romans}\n\nExperimental style profile.\n`,
          ],
          { type: "text/plain" },
        ),
        `${filename()}.txt`,
      );
    else if (kind === "midi") {
      const blob = await api("/api/midi", p, true);
      download(blob, `${filename()}.mid`);
      toast("MIDI exported. Ready for your DAW.");
    }
  } catch (error) {
    report(error);
  }
}
function closeExport() {
  $("export-menu").hidden = true;
  $("export-toggle").setAttribute("aria-expanded", "false");
}

async function initAudio() {
  if (!audio) {
    if (!window.AudioContext)
      throw new Error(
        "This browser does not support audio preview. You can still export MIDI.",
      );
    audio = new AudioContext();
    master = audio.createGain();
    master.gain.value = volume * 0.5;
    master.connect(audio.destination);
  }
  if (audio.state === "suspended") await audio.resume();
}
function soundChord(chord: RenderedChord, at: number, duration: number) {
  if (!audio || !master) return;
  const voices = Math.max(1, chord.midi.length);
  for (const note of chord.midi) {
    const frequency = 440 * Math.pow(2, (note - 69) / 12);
    const envelope = audio.createGain();
    envelope.gain.setValueAtTime(0, at);
    envelope.gain.linearRampToValueAtTime(0.7 / Math.sqrt(voices), at + 0.018);
    envelope.gain.exponentialRampToValueAtTime(
      0.2 / Math.sqrt(voices),
      at + Math.min(0.65, duration * 0.6),
    );
    envelope.gain.setTargetAtTime(0, at + duration * 0.82, 0.075);
    envelope.connect(master);
    for (const [multiple, amplitude] of [
      [1, 0.72],
      [2, 0.16],
      [3, 0.05],
    ]) {
      const oscillator = audio.createOscillator(),
        gain = audio.createGain();
      oscillator.type = "sine";
      oscillator.frequency.value = frequency * multiple;
      gain.gain.value = amplitude;
      oscillator.connect(gain);
      gain.connect(envelope);
      nodes.add(oscillator);
      oscillator.onended = () => {
        nodes.delete(oscillator);
        oscillator.disconnect();
        gain.disconnect();
      };
      oscillator.start(at);
      oscillator.stop(at + duration + 0.4);
    }
    setTimeout(
      () => envelope.disconnect(),
      Math.max(0, (at - audio.currentTime + duration + 0.6) * 1000),
    );
  }
}
async function audition(index: number) {
  if (!progression || busy) return;
  stopPlayback();
  selected = index;
  renderSelection();
  try {
    await initAudio();
    if (audio)
      soundChord(progression.chords[index], audio.currentTime + 0.025, 1.6);
  } catch (error) {
    report(error);
  }
}
function schedulerTick() {
  if (!playing || !audio || !progression) return;
  while (nextTime < audio.currentTime + 0.15) {
    if (nextChord >= progression.chords.length) {
      if (looping) nextChord = 0;
      else return;
    }
    const index = nextChord++;
    soundChord(progression.chords[index], nextTime, playDuration * 0.95);
    scheduled.push({ time: nextTime, index });
    nextTime += playDuration;
  }
}
function animatePlayback() {
  if (!playing || !audio || !progression) return;
  let latest = null;
  while (scheduled.length && scheduled[0].time <= audio.currentTime)
    latest = scheduled.shift();
  if (latest) {
    selected = latest.index;
    renderSelection();
    document
      .querySelectorAll(".chord-card")
      .forEach((node, i) => node.classList.toggle("playing", i === selected));
    $("play-position").textContent =
      `${String(selected + 1).padStart(2, "0")} / ${String(progression.chords.length).padStart(2, "0")}`;
  }
  const total = playDuration * progression.chords.length;
  const elapsed = Math.max(0, audio.currentTime - playbackStart);
  $("play-progress-bar").style.width =
    `${((looping ? elapsed % total : Math.min(elapsed, total)) / total) * 100}%`;
  if (
    !looping &&
    nextChord >= progression.chords.length &&
    audio.currentTime >= nextTime
  ) {
    stopPlayback();
    return;
  }
  animation = requestAnimationFrame(animatePlayback);
}
async function play() {
  if (busy || !progression || audioStarting) return;
  if (playing) {
    stopPlayback();
    return;
  }
  audioStarting = true;
  try {
    requestFromForm(false);
    stopPlayback();
    await initAudio();
    if (busy || !audio) return;
    playing = true;
    nextChord = 0;
    scheduled = [];
    playDuration = chordDuration(
      Number($("tempo").value),
      Number($("beats_per_chord").value),
    );
    nextTime = audio.currentTime + 0.05;
    playbackStart = nextTime;
    $("play").textContent = "■";
    $("play").setAttribute("aria-label", "Stop progression");
    $("stop").disabled = false;
    schedulerTick();
    scheduler = setInterval(schedulerTick, 25);
    animatePlayback();
  } catch (error) {
    report(error);
  } finally {
    audioStarting = false;
  }
}
function stopPlayback() {
  playing = false;
  clearInterval(scheduler);
  cancelAnimationFrame(animation);
  scheduled = [];
  for (const node of nodes) {
    try {
      node.stop();
    } catch {}
  }
  nodes.clear();
  $("play").textContent = "▶";
  $("play").setAttribute("aria-label", "Play progression");
  $("stop").disabled = true;
  $("play-progress-bar").style.width = "0%";
  $("play-position").textContent =
    `00 / ${String(progression?.chords.length || 8).padStart(2, "0")}`;
  document
    .querySelectorAll(".chord-card.playing")
    .forEach((node) => node.classList.remove("playing"));
}

function bindEvents() {
  $("controls").addEventListener("submit", (event) => {
    event.preventDefault();
    generate();
  });
  $("controls").addEventListener("change", (event) => {
    updateAvailability();
    $("generate-hint").textContent =
      "Settings ready. Generate to hear the change.";
    if (
      ["mode", "length"].includes(eventElement(event)?.id ?? "") &&
      progression?.chords.some((c) => c.locked)
    ) {
      progression.chords.forEach((c) => (c.locked = false));
      syncLocks();
      render();
      toast("Locks cleared because the mode or length changed.");
    }
  });
  $("creativity").addEventListener("input", updateAvailability);
  $("chord-grid").addEventListener("click", (event) => {
    const target = eventElement(event);
    const lock = target?.closest<HTMLElement>("[data-lock]");
    if (lock) {
      toggleLock(Number(lock.dataset.lock));
      return;
    }
    const chord = target?.closest<HTMLElement>("[data-select]");
    if (chord) audition(Number(chord.dataset.select));
  });
  $("unlock-all").onclick = () => {
    if (busy || !progression) return;
    progression.chords.forEach((c) => (c.locked = false));
    syncLocks();
    render();
    toast("All chords unlocked.");
  };
  $("replay").onclick = () => generate(false);
  $("seed").addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      generate(false);
    }
  });
  $("transpose-key").onchange = () => transposeTo($("transpose-key").value);
  $("undo").onclick = undo;
  $("play").onclick = play;
  $("stop").onclick = stopPlayback;
  $("loop").onclick = () => {
    looping = !looping;
    $("loop").setAttribute("aria-pressed", String(looping));
  };
  for (const id of ["tempo", "beats_per_chord"] as const)
    $(id).addEventListener("change", () => {
      stopPlayback();
      const tempo = Number($("tempo").value);
      if (!Number.isInteger(tempo) || tempo < 40 || tempo > 220) {
        toast("Tempo must be a whole number from 40 to 220.", true);
        return;
      }
      if (progression) {
        progression.request.tempo = tempo;
        progression.request.beats_per_chord = Number(
          $("beats_per_chord").value,
        );
        persist();
      }
    });
  $("volume").oninput = () => {
    volume = Number($("volume").value) / 100;
    $("volume").style.setProperty("--fill", `${volume * 100}%`);
    if (master && audio)
      master.gain.setTargetAtTime(volume * 0.5, audio.currentTime, 0.02);
    writeStorage(KEYS.volume, volume);
  };
  $("explain").onclick = explain;
  $("profile-info").onclick = profileInfo;
  $("save").onclick = saveSketch;
  $("studio-tab").onclick = () => showView("studio");
  $("sketchbook-tab").onclick = () => showView("sketchbook");
  $("export-toggle").onclick = () => {
    const visible = $("export-menu").hidden;
    $("export-menu").hidden = !visible;
    $("export-toggle").setAttribute("aria-expanded", String(visible));
    if (visible) $("export-menu").querySelector("button")?.focus();
  };
  $("export-menu").onclick = (event) => {
    const button = eventElement(event)?.closest<HTMLElement>("[data-export]");
    if (button?.dataset.export) exportAs(button.dataset.export);
  };
  document.addEventListener("click", (event) => {
    if (!eventElement(event)?.closest(".export-wrap")) closeExport();
  });
  $("shortcuts").onclick = () =>
    modal(
      '<div class="eyebrow">STAY IN THE FLOW</div><h2>A few handy shortcuts.</h2><div class="shortcuts-list"><span>Generate a new progression</span><kbd>G</kbd><span>Play / stop</span><kbd>Space</kbd><span>Lock selected chord</span><kbd>L</kbd><span>Save sketch</span><kbd>Ctrl S</kbd><span>Undo generation</span><kbd>Ctrl Z</kbd><span>Select previous / next chord</span><kbd>← →</kbd><span>Close dialog or menu</span><kbd>Esc</kbd></div><p>Shortcuts pause while you type in a control.</p>',
    );
  $("about").onclick = () =>
    modal(
      '<div class="eyebrow">NOCTAVE · 0.1.0-BETA.1</div><h2>Make space for possibility.</h2><p>A small harmonic sketchbook, built around music theory, weighted randomness, and your curiosity. Every progression is generated locally by Rust.</p><p>This free first beta includes six Rock-family styles, seven modes, 2–16 chords, reproducible seeds, chord locks, transposition, and MIDI / JSON / text export. The keyboard preview is a simple synthesizer.</p><p>Style profiles and tension estimates are experimental. MIDI uses 4/4 and one piano track. The studio runs in a local browser window; it has no DAW plugin or cloud sync.</p><p>Save ideas in the sketchbook and export JSON backups. Use the power button to quit the local engine.</p><p><em>Generate possibilities, not songs.</em></p>',
    );
  $("quit").onclick = () =>
    modal(
      '<div class="eyebrow">UNTIL NEXT TIME</div><h2>Close the studio?</h2><p>Your current exploration is saved in this browser automatically. The local engine will stop.</p><div class="modal-actions"><button id="keep-open" class="button secondary">Keep exploring</button><button id="confirm-quit" class="button primary">Quit Noctave</button></div>',
    );
  $("modal").addEventListener("click", async (event) => {
    if (eventElement(event)?.id === "keep-open") $("modal").close();
    if (eventElement(event)?.id === "confirm-quit") {
      stopPlayback();
      persist();
      try {
        await api("/api/quit", {});
        $("modal-content").innerHTML =
          '<div class="eyebrow">SEE YOU SOON</div><h2>Studio closed.</h2><p>Your exploration is saved locally. You can close this window and launch Noctave again whenever inspiration returns.</p>';
        setBusy(true);
      } catch (error) {
        report(error);
      }
    }
    if (event.target === $("modal")) {
      const r = $("modal").getBoundingClientRect();
      if (
        event.clientX < r.left ||
        event.clientX > r.right ||
        event.clientY < r.top ||
        event.clientY > r.bottom
      )
        $("modal").close();
    }
  });
  $("sketches").addEventListener("click", (event) => {
    const target = eventElement(event);
    const load = target?.closest<HTMLElement>("[data-load]"),
      remove = target?.closest<HTMLElement>("[data-remove]"),
      save = target?.closest<HTMLElement>("[data-download]");
    if (load) {
      const item = sketches.find((s) => s.id === load.dataset.load);
      if (item) loadSketch(item);
    }
    if (remove?.dataset.remove) removeSketch(remove.dataset.remove);
    if (save) {
      const item = sketches.find((s) => s.id === save.dataset.download);
      if (item)
        download(
          new Blob([JSON.stringify(item.progression, null, 2)], {
            type: "application/json",
          }),
          `${filename(item.title)}.json`,
        );
    }
  });
  $("import").onclick = () => $("import-file").click();
  $("import-file").onchange = async () => {
    const file = $("import-file").files?.[0];
    $("import-file").value = "";
    if (!file) return;
    try {
      if (file.size > 262144)
        throw new Error("Sketch files must be smaller than 256 KB.");
      const data: unknown = JSON.parse(await file.text());
      await loadSketch({
        title: file.name.replace(/\.json$/i, ""),
        progression: parseSketch(data),
      });
    } catch (error) {
      report(error);
    }
  };
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      closeExport();
      return;
    }
    if (
      $("modal").open ||
      eventElement(event)?.closest("input,select,textarea,[contenteditable]")
    )
      return;
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
      event.preventDefault();
      saveSketch();
      return;
    }
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "z") {
      event.preventDefault();
      undo();
      return;
    }
    if (event.ctrlKey || event.metaKey || event.altKey || busy) return;
    if (event.key.toLowerCase() === "g") {
      event.preventDefault();
      generate();
    }
    if (event.key === " " && !eventElement(event)?.closest("button")) {
      event.preventDefault();
      play();
    }
    if (event.key.toLowerCase() === "l") {
      event.preventDefault();
      toggleLock(selected);
    }
    if (progression && ["ArrowLeft", "ArrowRight"].includes(event.key)) {
      event.preventDefault();
      selected =
        (selected +
          (event.key === "ArrowLeft" ? -1 : 1) +
          progression.chords.length) %
        progression.chords.length;
      renderSelection();
    }
  });
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) stopPlayback();
  });
  window.addEventListener("pagehide", stopPlayback);
}
async function start() {
  $("transpose-key").innerHTML = $("key").innerHTML;
  sketches = savedSketches(readStorage(KEYS.sketches, []));
  const storedVolume = readStorage(KEYS.volume, 0.55);
  volume =
    typeof storedVolume === "number" && Number.isFinite(storedVolume)
      ? Math.max(0, Math.min(1, storedVolume))
      : 0.55;
  $("volume").value = String(Math.round(volume * 100));
  $("volume").style.setProperty("--fill", `${volume * 100}%`);
  bindEvents();
  renderSketches();
  updateAvailability();
  setBusy(true);
  try {
    const response = await fetch("/api/info", {
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok)
      throw new Error("The local engine is unavailable. Relaunch Noctave.");
    const info: EngineInfo = await response.json();
    $("engine-version").textContent = `NOCTAVE · ${info.version.toUpperCase()}`;
    const saved = savedSession(readStorage(KEYS.session, null));
    if (saved) {
      try {
        progression = await api("/api/restore", saved.progression);
        title = saved.title;
        exploration = saved.exploration;
      } catch {
        toast(
          "The previous session could not be restored. Starting a new exploration.",
          true,
        );
      }
    }
    if (!progression)
      progression = await api<Progression>("/api/generate", info.defaults);
    applyForm(progression.request);
    render();
    persist();
  } catch (error) {
    report(error);
    $("chord-grid").innerHTML =
      '<div class="loading-placeholder">Unable to reach the local engine. Relaunch Noctave or try Generate again.</div>';
  } finally {
    setBusy(false);
  }
}
start();
