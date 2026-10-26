<script lang="ts">
  import { frameAt } from "./animation";
  import type { LoadedSkin } from "./loader";
  import { resolveState, type PetState } from "./manifest";

  interface Props {
    skin: LoadedSkin;
    state: PetState;
    /** On-screen size of one frame, in CSS pixels. */
    size: number;
  }

  let { skin, state, size }: Props = $props();
  let canvas: HTMLCanvasElement;

  $effect(() => {
    const context = canvas.getContext("2d");
    if (!context) return;

    const { manifest, strips } = skin;
    const drawnState = resolveState(manifest, state);
    const animation = manifest.states[drawnState]!;
    const strip = strips[drawnState]!;
    const { width, height } = manifest.frame;

    const pixelRatio = window.devicePixelRatio || 1;
    canvas.width = Math.round(size * pixelRatio);
    canvas.height = Math.round(size * pixelRatio);
    context.imageSmoothingEnabled = manifest.scale === "smooth";

    const startedAt = performance.now();
    let lastFrame = -1;
    let handle = 0;

    // Redraw only when the frame index changes, so an idle pet costs almost nothing.
    const tick = (now: number) => {
      const frame = frameAt(animation, now - startedAt);
      if (frame !== lastFrame) {
        lastFrame = frame;
        context.clearRect(0, 0, canvas.width, canvas.height);
        context.drawImage(strip, frame * width, 0, width, height, 0, 0, canvas.width, canvas.height);
      }
      const holdsLastFrame = !animation.loop && frame === animation.frames - 1;
      if (animation.frames > 1 && !holdsLastFrame) {
        handle = requestAnimationFrame(tick);
      }
    };
    handle = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(handle);
  });
</script>

<canvas bind:this={canvas} style:width="{size}px" style:height="{size}px" aria-label="CAPRI pet ({state})"
></canvas>

<style>
  canvas {
    display: block;
  }
</style>
