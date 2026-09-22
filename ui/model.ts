/** The serializable boundary shared with the Rust engine. */
export type Mode =
  | "major"
  | "minor"
  | "dorian"
  | "phrygian"
  | "lydian"
  | "mixolydian"
  | "locrian";
export type Style =
  | "rock"
  | "post-rock"
  | "math-rock"
  | "shoegaze"
  | "indie-rock"
  | "midwest-emo";
export interface Chord {
  degree: number;
  alteration: number;
  third: number;
  fifth: number;
  seventh: number | null;
  extensions: Array<{ degree: number; semitones: number }>;
  inversion: number;
  source: "diatonic" | "borrowed" | "secondary-dominant";
}
export interface GenerationRequest {
  key: string;
  mode: Mode;
  style: Style;
  region: "global" | "japan";
  era: "any" | "2000s";
  mood: "neutral" | "melancholic" | "hopeful" | "dreamy" | "restless";
  complexity: "simple" | "balanced" | "rich";
  creativity: number;
  length: number;
  seed: number;
  energy: "steady" | "slow-build" | "wave" | "resolve";
  cadence: "open" | "resolved" | "loop";
  harmonic_trait:
    "none" | "atmospheric" | "cinematic" | "math-influenced" | "heavy";
  tempo: number;
  beats_per_chord: number;
  locks: Array<Chord | null>;
}
export type ScoreKey =
  "theory" | "style" | "movement" | "tension" | "repetition" | "phrase";
export interface RenderedChord {
  chord: Chord;
  name: string;
  roman: string;
  notes: string[];
  midi: number[];
  function: string;
  tension: number;
  target_tension: number;
  reasons: string[];
  score: Record<ScoreKey | "total", number>;
  locked: boolean;
}
export interface Profile {
  id: string;
  name: string;
  lineage: string[];
  weights: Record<string, number>;
  research_status: string;
  confidence: number;
  sample_tracks: number;
  sample_artists: number;
}
export interface Progression {
  engine_version: string;
  request: GenerationRequest;
  profile: Profile;
  scale: string[];
  chords: RenderedChord[];
  searched: number;
  score: number;
}
export interface Sketch {
  id: string;
  title: string;
  saved: string;
  progression: Progression;
}
export interface Session {
  progression: Progression;
  title: string;
  exploration: number;
}
export interface EngineInfo {
  version: string;
  defaults: GenerationRequest;
}

export const escapeHtml = (value: unknown): string =>
  String(value).replace(
    /[&<>"']/g,
    (char) =>
      ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[
        char
      ] ?? char,
  );
export const human = (value: string): string =>
  value
    .split("-")
    .map((part) => (part ? part[0].toUpperCase() + part.slice(1) : ""))
    .join(" ");
export const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);
export function errorMessage(error: unknown): string {
  return error instanceof Error
    ? error.message
    : "Something went wrong. Please try again.";
}

/** Basic browser-storage shape gate; the Rust restore endpoint validates musical content. */
export function isProgression(value: unknown): value is Progression {
  if (
    !isRecord(value) ||
    typeof value.engine_version !== "string" ||
    !isRecord(value.request) ||
    !Array.isArray(value.chords)
  )
    return false;
  const request = value.request;
  if (
    typeof request.key !== "string" ||
    typeof request.mode !== "string" ||
    typeof request.style !== "string" ||
    typeof request.length !== "number"
  )
    return false;
  if (
    value.chords.length !== request.length ||
    request.length < 2 ||
    request.length > 16
  )
    return false;
  return value.chords.every(
    (chord) =>
      isRecord(chord) &&
      typeof chord.name === "string" &&
      typeof chord.roman === "string" &&
      isRecord(chord.chord),
  );
}
export function savedSketches(value: unknown): Sketch[] {
  if (!Array.isArray(value)) return [];
  return value
    .filter(
      (item): item is Sketch =>
        isRecord(item) &&
        typeof item.id === "string" &&
        typeof item.title === "string" &&
        typeof item.saved === "string" &&
        Number.isFinite(Date.parse(item.saved)) &&
        isProgression(item.progression),
    )
    .slice(0, 60);
}
export function savedSession(value: unknown): Session | null {
  if (!isRecord(value) || !isProgression(value.progression)) return null;
  return {
    progression: value.progression,
    title:
      typeof value.title === "string" ? value.title : "Untitled exploration",
    exploration:
      typeof value.exploration === "number" &&
      Number.isSafeInteger(value.exploration) &&
      value.exploration > 0
        ? value.exploration
        : 1,
  };
}
export function parseSketch(value: unknown): Progression {
  const data =
    isRecord(value) && "progression" in value ? value.progression : value;
  if (!isProgression(data))
    throw new Error(
      "This file is not a Noctave JSON sketch. Export a JSON sketch from Noctave first.",
    );
  return data;
}
export function chordDuration(tempo: number, beats: number): number {
  if (
    !Number.isInteger(tempo) ||
    tempo < 40 ||
    tempo > 220 ||
    ![1, 2, 4, 8].includes(beats)
  )
    throw new Error("Choose 40–220 BPM and 1, 2, 4, or 8 beats per chord.");
  return (60 / tempo) * beats;
}
