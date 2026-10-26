"""Generate the placeholder "default" skin in static/skins/default.

The art is deliberately simple (a round blob with a face) so the sprite-sheet pipeline can
be exercised before real art exists. Each state is a horizontal PNG strip of equal frames,
matching the format documented in README.md, so a user skin can replace any file 1:1.

Usage: python3 scripts/generate_placeholder_skin.py  (requires Pillow)
"""

import json
import math
from pathlib import Path

from PIL import Image, ImageDraw

FRAME_SIZE = 64
OUTPUT_DIR = Path(__file__).resolve().parent.parent / "static" / "skins" / "default"

BODY = (124, 92, 255, 255)
BODY_ERROR = (230, 80, 90, 255)
OUTLINE = (40, 28, 90, 255)
EYE = (255, 255, 255, 255)
PUPIL = (20, 20, 30, 255)
ACCENT = (255, 214, 102, 255)

# state -> (frame count, fps, loop)
ANIMATIONS = {
    "idle": (8, 6, True),
    "listening": (4, 8, True),
    "thinking": (6, 8, True),
    "talking": (4, 10, True),
    "working": (8, 12, True),
    "waiting": (4, 4, True),
    "sleeping": (4, 3, True),
    "error": (2, 4, True),
}


def draw_body(draw, dy, color=BODY):
    draw.ellipse((12, 18 + dy, 52, 58 + dy), fill=color, outline=OUTLINE, width=2)


def draw_eyes(draw, dy, style="open"):
    for x in (24, 40):
        if style == "closed":
            draw.line((x - 4, 34 + dy, x + 4, 34 + dy), fill=OUTLINE, width=2)
        elif style == "cross":
            draw.line((x - 3, 31 + dy, x + 3, 37 + dy), fill=OUTLINE, width=2)
            draw.line((x - 3, 37 + dy, x + 3, 31 + dy), fill=OUTLINE, width=2)
        else:
            draw.ellipse((x - 4, 30 + dy, x + 4, 38 + dy), fill=EYE)
            draw.ellipse((x - 2, 32 + dy, x + 2, 36 + dy), fill=PUPIL)


def draw_mouth(draw, dy, open_amount=0):
    if open_amount:
        draw.ellipse((28, 43 + dy, 36, 43 + dy + open_amount), fill=OUTLINE)
    else:
        draw.arc((27, 40 + dy, 37, 47 + dy), start=20, end=160, fill=OUTLINE, width=2)


def bob(frame, frames, amplitude=2):
    return round(amplitude * math.sin(2 * math.pi * frame / frames))


def render_frame(state, frame, frames):
    image = Image.new("RGBA", (FRAME_SIZE, FRAME_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    dy = bob(frame, frames) if state in ("idle", "waiting", "thinking") else 0

    body_color = BODY_ERROR if state == "error" else BODY
    draw_body(draw, dy, body_color)

    eye_style = {"sleeping": "closed", "error": "cross"}.get(state, "open")
    if state == "idle" and frame == frames - 2:
        eye_style = "closed"  # blink
    draw_eyes(draw, dy, eye_style)

    mouth_open = (frame % 2) * 5 + 2 if state == "talking" else 0
    draw_mouth(draw, dy, mouth_open)

    if state == "listening":
        radius = 4 + frame * 2
        draw.arc((50 - radius, 14 - radius, 50 + radius, 14 + radius), 200, 340, ACCENT, 2)
    elif state == "thinking":
        for i in range(3):
            size = 3 if i == frame % 3 else 2
            draw.ellipse((22 + i * 10 - size, 8 - size, 22 + i * 10 + size, 8 + size), fill=ACCENT)
    elif state == "working":
        angle = frame * 360 / frames
        draw.arc((46, 4, 60, 18), angle, angle + 270, ACCENT, 3)
    elif state == "waiting":
        draw.text((48, 2 + dy), "?", fill=ACCENT)
    elif state == "sleeping":
        draw.text((46, 12 - frame * 2), "z", fill=ACCENT)
    return image


def render_strip(state, frames):
    strip = Image.new("RGBA", (FRAME_SIZE * frames, FRAME_SIZE), (0, 0, 0, 0))
    for frame in range(frames):
        strip.paste(render_frame(state, frame, frames), (frame * FRAME_SIZE, 0))
    return strip


def main():
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    manifest = {
        "name": "Default blob",
        "frame": {"width": FRAME_SIZE, "height": FRAME_SIZE},
        "scale": "pixelated",
        "states": {},
    }
    for state, (frames, fps, loop) in ANIMATIONS.items():
        render_strip(state, frames).save(OUTPUT_DIR / f"{state}.png", optimize=True)
        manifest["states"][state] = {"frames": frames, "fps": fps, "loop": loop}
    (OUTPUT_DIR / "skin.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"wrote {len(ANIMATIONS)} strips to {OUTPUT_DIR}")


if __name__ == "__main__":
    main()
