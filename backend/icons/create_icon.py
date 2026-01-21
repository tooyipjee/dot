#!/usr/bin/env python3
from PIL import Image, ImageDraw

# Create a 32x32 image (will be scaled by macOS)
size = 32
img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Pixel size for blocky look
pixel_size = 2

# Simple kawaii face using pixels
# Eyes (^_^)
pixels = [
    # Left eye
    (6, 12), (8, 12), (10, 12),
    (6, 14), (10, 14),
    # Right eye
    (22, 12), (24, 12), (26, 12),
    (22, 14), (26, 14),
    # Mouth (w shape)
    (12, 20), (14, 22), (16, 22), (18, 22), (20, 20),
]

# Draw each pixel
for x, y in pixels:
    draw.rectangle(
        [x, y, x + pixel_size - 1, y + pixel_size - 1],
        fill=(0, 0, 0, 255)
    )

# Save as PNG
img.save('/Users/jasontoo/Github/dot/backend/icons/icon.png')
print("Icon created!")
