#!/bin/python3
from PIL import Image
import sys

def load_colors(colors):
    result = []
    for color in colors.split(','):
        rgb = color.split(':')
        assert len(rgb) == 3
        result.append([int(x) for x in rgb])

    assert len(result) > 0
    return result


def main():
    if len(sys.argv) != 3:
        print("Error: expected three arguments.")
        print("Usage: ", sys.argv[0], " FILENAME.png R:B:B,R:G:B,R:G:B,R:G:B")
        print()
        print("where FILENAME.png is name of the file followed with")
        print("triplets of RGB values, delimited by comma. Each color in triplet shall")
        print("be separated by a colon. This will return index of closest color for each column.")
        return

    filename = sys.argv[1]
    colors = load_colors(sys.argv[2])

    im = Image.open(filename)
    pixels = im.load()
    width, height = im.size

    wRawResults = []
    for w in range(width):
        subRes = []
        for h in range(height):
            pixel = pixels[w, h]
            if type(pixel) is int:
                pixel = (255, 255, 255)
            subRes.append(pixel)
        wRawResults.append(averagePixelTriplet(subRes))
    
    l = len(wRawResults)
    i = 0
    for r in wRawResults:
        closestIdx = getClosestColor(r, colors)
        sys.stdout.write(str(closestIdx))
        i += 1
        if i != l:
            sys.stdout.write(',')
    sys.stdout.write('\n')


def averagePixelTriplet(rgbs):
    r, g, b = (0, 0, 0)
    rgbsCount = len(rgbs)
    for idx in range(rgbsCount):
        r += rgbs[idx][0]
        g += rgbs[idx][1]
        b += rgbs[idx][2]
    return [r/rgbsCount, g/rgbsCount, b/rgbsCount]

def getClosestColor(pixel, colorsToMatch):
    matchIdx = 0
    matchDistance = squaredDistanceToPixel(pixel, colorsToMatch[0])
    for idx in range(1, len(colorsToMatch)):
        newMatchDistance = squaredDistanceToPixel(pixel, colorsToMatch[idx])
        if newMatchDistance < matchDistance:
            matchIdx = idx
            matchDistance = newMatchDistance
    return matchIdx

def squaredDistanceToPixel(pix1, pix2):
    return (pix1[0]-pix2[0])**2 + (pix1[1]-pix2[1])**2 + (pix1[2]-pix2[2])**2

if __name__ == "__main__":
    main()

