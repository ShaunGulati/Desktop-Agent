import { describe, expect, it } from "vitest";
import { parseSkinManifest, resolveState, spriteFileName } from "./manifest";

const minimalSkin = {
  name: "Test",
  frame: { width: 64, height: 64 },
  states: { idle: { frames: 4 } },
};

function parseOk(raw: unknown) {
  const result = parseSkinManifest(raw);
  if (!result.ok) throw new Error(result.errors.join("; "));
  return result.skin;
}

function parseErrors(raw: unknown): string[] {
  const result = parseSkinManifest(raw);
  if (result.ok) throw new Error("expected manifest to be rejected");
  return result.errors;
}

describe("parseSkinManifest", () => {
  it("accepts a skin with only an idle state and applies defaults", () => {
    const skin = parseOk(minimalSkin);
    expect(skin.scale).toBe("pixelated");
    expect(skin.states.idle).toEqual({ frames: 4, fps: 8, loop: true });
  });

  it("keeps explicit fps, loop and scale values", () => {
    const skin = parseOk({
      ...minimalSkin,
      scale: "smooth",
      states: { idle: { frames: 2, fps: 12, loop: false } },
    });
    expect(skin.scale).toBe("smooth");
    expect(skin.states.idle).toEqual({ frames: 2, fps: 12, loop: false });
  });

  it("rejects a skin without an idle state", () => {
    const errors = parseErrors({ ...minimalSkin, states: { thinking: { frames: 2 } } });
    expect(errors.some((e) => e.includes("states.idle is required"))).toBe(true);
  });

  it("reports misspelled state names instead of silently ignoring them", () => {
    const errors = parseErrors({
      ...minimalSkin,
      states: { idle: { frames: 1 }, thinkng: { frames: 2 } },
    });
    expect(errors).toEqual([expect.stringContaining('unknown state "thinkng"')]);
  });

  it.each([0, -1, 1.5, "4"])("rejects frames = %s", (frames) => {
    const errors = parseErrors({ ...minimalSkin, states: { idle: { frames } } });
    expect(errors).toEqual(["states.idle.frames must be a positive integer"]);
  });

  it.each([0, 61, "8"])("rejects fps = %s", (fps) => {
    const errors = parseErrors({ ...minimalSkin, states: { idle: { frames: 1, fps } } });
    expect(errors[0]).toContain("states.idle.fps");
  });

  it("collects every error at once", () => {
    const errors = parseErrors({ name: "", frame: { width: 0 }, scale: "blurry", states: [] });
    expect(errors).toHaveLength(4);
  });

  it.each([null, [], "skin", 3])("rejects non-object input %s", (raw) => {
    expect(parseErrors(raw)).toEqual(["skin.json must contain a JSON object"]);
  });
});

describe("resolveState", () => {
  const skin = parseOk({
    ...minimalSkin,
    states: { idle: { frames: 1 }, thinking: { frames: 3 } },
  });

  it("uses the requested state when the skin defines it", () => {
    expect(resolveState(skin, "thinking")).toBe("thinking");
  });

  it("falls back to idle when the skin omits the state", () => {
    expect(resolveState(skin, "waiting")).toBe("idle");
  });
});

describe("spriteFileName", () => {
  it("names each strip after its state", () => {
    expect(spriteFileName("thinking")).toBe("thinking.png");
  });
});
