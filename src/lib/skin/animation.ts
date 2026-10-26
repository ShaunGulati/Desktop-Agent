import type { StateAnimation } from "./manifest";

const MS_PER_SECOND = 1000;

/**
 * Frame of a sprite strip to show after `elapsedMs` in a state.
 *
 * Looping animations wrap; one-shot animations hold their last frame.
 */
export function frameAt(animation: StateAnimation, elapsedMs: number): number {
  const ticks = Math.floor((Math.max(0, elapsedMs) * animation.fps) / MS_PER_SECOND);
  if (animation.loop) {
    return ticks % animation.frames;
  }
  return Math.min(ticks, animation.frames - 1);
}
