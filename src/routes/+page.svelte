<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import SpritePlayer from "$lib/skin/SpritePlayer.svelte";
  import { loadSkin, type LoadedSkin } from "$lib/skin/loader";
  import { PET_STATES, type PetState } from "$lib/skin/manifest";

  const DEFAULT_SKIN_URL = "/skins/default";
  const PET_SIZE_PX = 128;
  const LISTENING_AFTER_SHOW_MS = 1200;
  // Must match PET_SHOWN_EVENT in src-tauri/src/pet.rs.
  const PET_SHOWN_EVENT = "pet://shown";

  let skin = $state<LoadedSkin | null>(null);
  let skinError = $state<string | null>(null);
  let petState = $state<PetState>("idle");
  let listeningTimer: ReturnType<typeof setTimeout> | undefined;

  /** Resolves after the browser has painted the current frame. */
  function afterNextPaint(): Promise<void> {
    return new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
  }

  async function handleShown() {
    petState = "listening";
    clearTimeout(listeningTimer);
    listeningTimer = setTimeout(() => (petState = "idle"), LISTENING_AFTER_SHOW_MS);
    await afterNextPaint();
    await invoke("report_pet_painted");
  }

  /** Dev-only: right-click cycles through every state to preview a skin. */
  function cycleState(event: MouseEvent) {
    if (!import.meta.env.DEV) return;
    event.preventDefault();
    clearTimeout(listeningTimer);
    petState = PET_STATES[(PET_STATES.indexOf(petState) + 1) % PET_STATES.length];
  }

  onMount(() => {
    loadSkin(DEFAULT_SKIN_URL)
      .then((loaded) => (skin = loaded))
      .catch((error: Error) => (skinError = error.message));

    const unlisten = listen(PET_SHOWN_EVENT, handleShown);
    return () => {
      clearTimeout(listeningTimer);
      unlisten.then((stop) => stop());
    };
  });
</script>

<main data-tauri-drag-region oncontextmenu={cycleState}>
  {#if skin}
    <SpritePlayer {skin} state={petState} size={PET_SIZE_PX} />
  {:else if skinError}
    <p class="error" title={skinError}>skin error</p>
  {/if}
</main>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    background: transparent;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  main {
    width: 100vw;
    height: 100vh;
    display: grid;
    place-items: center;
    cursor: grab;
  }

  /* Let mouse-downs reach the drag region instead of the canvas. */
  main > :global(*) {
    pointer-events: none;
  }

  .error {
    font: 12px system-ui;
    color: #e5484d;
  }
</style>
