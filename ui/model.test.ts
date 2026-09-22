import { describe, expect, test } from "bun:test";
import {
  escapeHtml,
  parseSketch,
  savedSketches,
  savedSession,
  chordDuration,
} from "./model.ts";
import type { Progression } from "./model.ts";

const fixture: Progression = {
  engine_version: "0.1.0-beta.1",
  request: {
    key: "D",
    mode: "major",
    style: "post-rock",
    length: 2,
    region: "global",
    era: "any",
    mood: "dreamy",
    complexity: "simple",
    creativity: 35,
    seed: 42,
    energy: "steady",
    cadence: "open",
    harmonic_trait: "none",
    tempo: 88,
    beats_per_chord: 4,
    locks: [],
  },
  profile: {
    id: "post-rock",
    name: "Post-Rock",
    lineage: ["Rock", "Post-Rock"],
    weights: {},
    research_status: "experimental",
    confidence: 0,
    sample_tracks: 0,
    sample_artists: 0,
  },
  scale: ["D", "E", "F#", "G", "A", "B", "C#"],
  searched: 96,
  score: 1,
  chords: [
    ["D", "I", 0],
    ["G", "IV", 3],
  ].map(([name, roman, degree]) => ({
    name: String(name),
    roman: String(roman),
    chord: {
      degree: Number(degree),
      alteration: 0,
      third: 4,
      fifth: 7,
      seventh: null,
      extensions: [],
      inversion: 0,
      source: "diatonic",
    },
    notes: [],
    midi: [],
    function: "Tonic",
    tension: 0.2,
    target_tension: 0.3,
    reasons: [],
    score: {
      theory: 0,
      style: 0,
      movement: 0,
      tension: 0,
      repetition: 0,
      phrase: 0,
      total: 0,
    },
    locked: false,
  })),
};
describe("Untrusted sketches and browser storage", () => {
  test("Rejects malformed files before sending to the engine", () => {
    for (const data of [
      null,
      [],
      {},
      "text",
      { ...fixture, chords: [] },
      { ...fixture, request: { ...fixture.request, length: 999 } },
      { ...fixture, chords: [null, null] },
    ])
      expect(() => parseSketch(data)).toThrow();
  });
  test("Accepts portable sketches and wrapped saved sketches for backend validation", () => {
    expect(parseSketch(fixture)).toEqual(fixture);
    expect(parseSketch({ progression: fixture })).toEqual(fixture);
  });
  test("Corrupted storage cannot populate the sketchbook", () => {
    expect(savedSketches("broken")).toEqual([]);
    expect(
      savedSketches([
        {},
        null,
        { id: "x", title: "x", saved: "bad", progression: fixture },
      ]),
    ).toEqual([]);
  });
  test("Keeps valid storage entries when another entry is corrupted", () => {
    const item = {
      id: "abc",
      title: "<script>untrusted</script>",
      saved: "2026-09-22T00:00:00Z",
      progression: fixture,
    };
    expect(savedSketches([null, item, {}])).toEqual([item]);
  });
  test("Session metadata recovers without trusting invalid title or counter", () => {
    expect(savedSession(null)).toBeNull();
    expect(
      savedSession({ progression: fixture, title: 14, exploration: -1 }),
    ).toMatchObject({ title: "Untitled exploration", exploration: 1 });
  });
  test("Escapes user names and attribute-breaking characters", () => {
    expect(escapeHtml('<img src=x onerror="alert(1)"> & \'')).toBe(
      "&lt;img src=x onerror=&quot;alert(1)&quot;&gt; &amp; &#39;",
    );
  });
});
describe("Playback timing", () => {
  test("Tempo and duration match a MIDI quarter-note clock", () => {
    expect(chordDuration(120, 4)).toBe(2);
    expect(chordDuration(60, 8)).toBe(8);
    expect(chordDuration(220, 1)).toBeCloseTo(60 / 220, 10);
  });
  test("Rejects invalid, fractional, and non-finite timing", () => {
    for (const [tempo, beats] of [
      [0, 4],
      [NaN, 4],
      [Infinity, 1],
      [39, 4],
      [221, 4],
      [88.5, 4],
      [88, 3],
    ])
      expect(() => chordDuration(tempo, beats)).toThrow();
  });
});
