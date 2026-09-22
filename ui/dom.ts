type InputId =
  "seed" | "tempo" | "creativity" | "volume" | "sketch-name" | "import-file";
type SelectId =
  | "key"
  | "mode"
  | "style"
  | "region"
  | "era"
  | "mood"
  | "complexity"
  | "length"
  | "energy"
  | "cadence"
  | "harmonic_trait"
  | "beats_per_chord"
  | "transpose-key";
type ButtonId =
  | "generate"
  | "save"
  | "export-toggle"
  | "undo"
  | "unlock-all"
  | "play"
  | "stop"
  | "loop"
  | "explain"
  | "replay"
  | "profile-info"
  | "studio-tab"
  | "sketchbook-tab"
  | "cancel-save"
  | "confirm-save"
  | "back-to-studio"
  | "cancel-remove"
  | "confirm-remove"
  | "shortcuts"
  | "about"
  | "quit"
  | "import";
type OtherId =
  | "toast"
  | "chord-grid"
  | "creativity-value"
  | "generate-hint"
  | "progression-heading"
  | "progression-meta"
  | "lock-count"
  | "energy-label"
  | "play-position"
  | "detail-name"
  | "detail-tags"
  | "piano"
  | "detail-notes"
  | "detail-reason"
  | "tension-chart"
  | "modal-content"
  | "studio-view"
  | "sketchbook-view"
  | "page-label"
  | "saved-count"
  | "sketches"
  | "export-menu"
  | "play-progress-bar"
  | "engine-version";
type Elements = { [K in InputId]: HTMLInputElement } & {
  [K in SelectId]: HTMLSelectElement;
} & { [K in ButtonId]: HTMLButtonElement } & { [K in OtherId]: HTMLElement } & {
  controls: HTMLFormElement;
  modal: HTMLDialogElement;
};
export function $<K extends keyof Elements>(id: K): Elements[K] {
  const element = document.getElementById(id);
  if (!element) throw new Error(`Missing studio element: ${id}`);
  return element as Elements[K];
}
export function eventElement(event: Event): Element | null {
  return event.target instanceof Element ? event.target : null;
}
