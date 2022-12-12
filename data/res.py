#!/usr/bin/env python3
# 2022 Day 12: Hill Climbing Algorithm

from collections import defaultdict


def process_input(filename):
    """Acquire input data"""
    with open(filename) as file:
        input = file.read().splitlines()
        input.append("")

    map = {}
    start = {}

    for y, line in enumerate(input):
        for x, ch in enumerate(line):
            if ch == "S":
                ch = "a"
            elif ch == "E":
                end = (x, y)
                ch = "z"
            map[x, y] = ch
            if ch == "a":
                start[x, y] = 1

    x_len = len(input[0])
    y_len = len(input)

    return map, x_len, y_len, start, end


def climb_hill(map, x_len, y_len, start, end):
    path_steps = defaultdict(lambda: 999999)
    queue = [end]
    x, y = end
    path_steps[x, y] = 0

    while len(queue) > 0:
        x, y = queue.pop()
        if (x, y) in start:
            continue
        steps = path_steps[x, y]
        elevation = map[x, y]
        explore(x - 1, y, steps, elevation, path_steps, queue)
        explore(x, y - 1, steps, elevation, path_steps, queue)
        explore(x + 1, y, steps, elevation, path_steps, queue)
        explore(x, y + 1, steps, elevation, path_steps, queue)

    result = 9999999
    for a, b in start:
        result = min(path_steps[a, b], result)
    return result


def explore(x, y, steps, elevation, path_steps, queue):
    global map
    if (x, y) not in map:
        return
    new_elevation = map[x, y]
    if ord(elevation) - ord(new_elevation) > 1:
        return
    new_steps = steps + 1
    if new_steps < path_steps[x, y]:
        path_steps[x, y] = new_steps
        queue.append((x, y))
    return


# -----------------------------------------------------------------------------------------

filename = "day12.txt"
# filename = 'sample.txt'

map, x_len, y_len, start, end = process_input(filename)

steps = climb_hill(map, x_len, y_len, start, end)

print("")
print("Goal reached in", steps, "steps")
print("")
