#! /usr/bin/env python3

from PIL import Image, ImageDraw, ImageFont
import imageio
import glob

debug = False

background_color = "#000000" if debug else "#181818"

# Step 1: Render ASCII to images
def ascii_to_image(ascii_file, font_path="/System/Library/Fonts/SFNSMono.ttf", image_size=(720, 820), font_size=12):
    with open(ascii_file, 'r') as file:
        content = file.read()
    font = ImageFont.truetype(font_path, font_size)
    img = Image.new("RGB", image_size, background_color)
    draw = ImageDraw.Draw(img)
    draw.text((10, 10), content, fill="white", font=font)
    return img


frame_duration = 25
end_duration = 1000

ascii_files = sorted(glob.glob("part2/*.txt"))  # Adjust the path
if debug:
    ascii_files = ascii_files[:10]
else:
    ascii_files = ascii_files[:1600]
images = [ascii_to_image(f) for f in ascii_files]
last_image = images[-1]

# Repeat the last image in the end of the gif
for _ in range(end_duration // frame_duration):
    images.append(last_image)

# Step 2: Create GIF
images[0].save("output.gif", save_all=True, append_images=images[1:], duration=frame_duration, loop=0)
