#!/usr/bin/env python3
from PIL import Image, ImageDraw

# Cute pixel eyes like the toast - 2x2 black with white highlight
# With light grey outline

size = 22
img = Image.new('RGBA', (size, size), (0, 0, 0, 0))

# Colors
BLACK = (0, 0, 0, 255)
WHITE = (255, 255, 255, 255)
GREY = (180, 180, 180, 255)

# Draw light grey outline (rounded-ish shape)
outline_pixels = [
    # Top edge
    (7, 4), (8, 4), (9, 4), (10, 4), (11, 4), (12, 4), (13, 4), (14, 4),
    # Bottom edge
    (7, 17), (8, 17), (9, 17), (10, 17), (11, 17), (12, 17), (13, 17), (14, 17),
    # Left edge
    (6, 5), (6, 6), (5, 7), (5, 8), (5, 9), (5, 10), (5, 11), (5, 12), (5, 13), (5, 14), (6, 15), (6, 16),
    # Right edge
    (15, 5), (15, 6), (16, 7), (16, 8), (16, 9), (16, 10), (16, 11), (16, 12), (16, 13), (16, 14), (15, 15), (15, 16),
]

for x, y in outline_pixels:
    img.putpixel((x, y), GREY)

# Left eye (2x2 black with white highlight top-left)
left_eye_x, left_eye_y = 8, 9
img.putpixel((left_eye_x, left_eye_y), WHITE)      # Highlight
img.putpixel((left_eye_x + 1, left_eye_y), BLACK)
img.putpixel((left_eye_x, left_eye_y + 1), BLACK)
img.putpixel((left_eye_x + 1, left_eye_y + 1), BLACK)

# Right eye (2x2 black with white highlight top-left)
right_eye_x, right_eye_y = 12, 9
img.putpixel((right_eye_x, right_eye_y), WHITE)    # Highlight
img.putpixel((right_eye_x + 1, right_eye_y), BLACK)
img.putpixel((right_eye_x, right_eye_y + 1), BLACK)
img.putpixel((right_eye_x + 1, right_eye_y + 1), BLACK)

img.save('/Users/jasontoo/Github/dot/backend/icons/icon.png')
print("Icon created!")
