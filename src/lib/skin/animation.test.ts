import { describe, expect, it } from "vitest";
import { frameAt } from "./animation";

describe("frameAt", () => {
  const looping = { frames: 4, fps: 10, loop: true };
  const oneShot = { frames: 4, fps: 10, loop: false };

  it("starts on the first frame", () => {
    expect(frameAt(looping, 0)).toBe(0);
  });

  it("advances one frame per 1/fps seconds", () => {
    expect(frameAt(looping, 99)).toBe(0);
    expect(frameAt(looping, 100)).toBe(1);
    expect(frameAt(looping, 250)).toBe(2);
  });

  it("wraps around for looping animations", () => {
    expect(frameAt(looping, 400)).toBe(0);
    expect(frameAt(looping, 1_000)).toBe(2);
  });

  it("holds the last frame for one-shot animations", () => {
    expect(frameAt(oneShot, 300)).toBe(3);
    expect(frameAt(oneShot, 10_000)).toBe(3);
  });

  it("always shows frame 0 for single-frame strips", () => {
    expect(frameAt({ frames: 1, fps: 8, loop: true }, 12_345)).toBe(0);
  });

  it("treats negative elapsed time as zero", () => {
    expect(frameAt(looping, -500)).toBe(0);
  });
});
