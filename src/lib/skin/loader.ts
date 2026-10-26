import { parseSkinManifest, PET_STATES, spriteFileName, type PetState, type SkinManifest } from "./manifest";

/** A parsed skin with its sprite strips decoded and ready to draw. */
export interface LoadedSkin {
  manifest: SkinManifest;
  strips: Partial<Record<PetState, HTMLImageElement>>;
}

function loadImage(url: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => resolve(image);
    image.onerror = () => reject(new Error(`could not load sprite strip ${url}`));
    image.src = url;
  });
}

/**
 * Fetches `skin.json` from `baseUrl` and decodes a strip for every state it declares.
 *
 * @throws if the manifest is missing or invalid, or the idle strip fails to load.
 *   Other strips that fail to load are dropped, so those states fall back to idle.
 */
export async function loadSkin(baseUrl: string): Promise<LoadedSkin> {
  const response = await fetch(`${baseUrl}/skin.json`);
  if (!response.ok) {
    throw new Error(`could not read ${baseUrl}/skin.json (HTTP ${response.status})`);
  }
  const parsed = parseSkinManifest(await response.json());
  if (!parsed.ok) {
    throw new Error(`invalid skin.json:\n- ${parsed.errors.join("\n- ")}`);
  }

  const manifest = parsed.skin;
  const declared = PET_STATES.filter((state) => manifest.states[state]);
  const results = await Promise.allSettled(
    declared.map((state) => loadImage(`${baseUrl}/${spriteFileName(state)}`)),
  );

  const strips: LoadedSkin["strips"] = {};
  results.forEach((result, index) => {
    const state = declared[index];
    if (result.status === "fulfilled") {
      strips[state] = result.value;
    } else {
      console.warn(`[skin] ${result.reason}; "${state}" will fall back to idle`);
      delete manifest.states[state];
    }
  });
  if (!strips.idle) {
    throw new Error(`skin "${manifest.name}" is missing a loadable idle.png`);
  }
  return { manifest, strips };
}
