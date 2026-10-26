/**
 * Skin manifests (`skin.json`) describe a folder of PNG sprite strips, one per pet state.
 *
 * Skins are user-replaceable, so parsing is strict about structure (typos in state names
 * are reported, not ignored) but forgiving about omissions: any state left out falls back
 * to `idle` at render time.
 */

export const PET_STATES = [
  "idle",
  "listening",
  "thinking",
  "talking",
  "working",
  "waiting",
  "sleeping",
  "error",
] as const;

export type PetState = (typeof PET_STATES)[number];

export const FALLBACK_STATE: PetState = "idle";

const DEFAULT_FPS = 8;
const MAX_FPS = 60;

export type ScaleMode = "pixelated" | "smooth";

export interface StateAnimation {
  frames: number;
  fps: number;
  loop: boolean;
}

export interface SkinManifest {
  name: string;
  frame: { width: number; height: number };
  scale: ScaleMode;
  states: Partial<Record<PetState, StateAnimation>> & { idle: StateAnimation };
}

export type ParseResult =
  | { ok: true; skin: SkinManifest }
  | { ok: false; errors: string[] };

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isPositiveInteger(value: unknown): value is number {
  return typeof value === "number" && Number.isInteger(value) && value > 0;
}

function isPetState(name: string): name is PetState {
  return (PET_STATES as readonly string[]).includes(name);
}

function parseAnimation(
  stateName: string,
  raw: unknown,
  errors: string[],
): StateAnimation | undefined {
  if (!isRecord(raw)) {
    errors.push(`states.${stateName} must be an object`);
    return undefined;
  }
  const { frames, fps = DEFAULT_FPS, loop = true } = raw;
  if (!isPositiveInteger(frames)) {
    errors.push(`states.${stateName}.frames must be a positive integer`);
  }
  if (typeof fps !== "number" || fps <= 0 || fps > MAX_FPS) {
    errors.push(`states.${stateName}.fps must be a number in (0, ${MAX_FPS}]`);
  }
  if (typeof loop !== "boolean") {
    errors.push(`states.${stateName}.loop must be true or false`);
  }
  return { frames: frames as number, fps: fps as number, loop: loop as boolean };
}

/**
 * Validates parsed `skin.json` content.
 *
 * @param raw - The JSON value read from `skin.json`.
 * @returns The normalized manifest with defaults applied, or every validation error found.
 */
export function parseSkinManifest(raw: unknown): ParseResult {
  const errors: string[] = [];
  if (!isRecord(raw)) {
    return { ok: false, errors: ["skin.json must contain a JSON object"] };
  }

  const { name, frame, scale = "pixelated", states } = raw;
  if (typeof name !== "string" || name.trim() === "") {
    errors.push("name must be a non-empty string");
  }
  if (!isRecord(frame) || !isPositiveInteger(frame.width) || !isPositiveInteger(frame.height)) {
    errors.push("frame must have positive integer width and height");
  }
  if (scale !== "pixelated" && scale !== "smooth") {
    errors.push('scale must be "pixelated" or "smooth"');
  }

  const parsedStates: Partial<Record<PetState, StateAnimation>> = {};
  if (!isRecord(states)) {
    errors.push("states must be an object");
  } else {
    for (const [stateName, animation] of Object.entries(states)) {
      if (!isPetState(stateName)) {
        errors.push(`unknown state "${stateName}" (expected one of: ${PET_STATES.join(", ")})`);
        continue;
      }
      parsedStates[stateName] = parseAnimation(stateName, animation, errors);
    }
    if (!("idle" in states)) {
      errors.push("states.idle is required: it is the fallback for every other state");
    }
  }

  if (errors.length > 0) {
    return { ok: false, errors };
  }
  return {
    ok: true,
    skin: {
      name: name as string,
      frame: frame as SkinManifest["frame"],
      scale: scale as ScaleMode,
      states: parsedStates as SkinManifest["states"],
    },
  };
}

/** Returns the state a skin will actually draw for `requested`, falling back to idle. */
export function resolveState(skin: SkinManifest, requested: PetState): PetState {
  return skin.states[requested] ? requested : FALLBACK_STATE;
}

/** File name of the sprite strip for a state, relative to the skin folder. */
export function spriteFileName(state: PetState): string {
  return `${state}.png`;
}
