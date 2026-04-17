"""
---
tags:
  - game
  - python
  - terminal
  - snake
parents:
  - Game Development
  - Python Scripts
children: []
---
"""

import curses
import random
import time
import json
import os

# --- Constants & Variables ---
SCORE_FILE = "highscores.json"
SPECIAL_FOODS = ['F', 'R', 'G', '*']

def save_score(name, score):
    """Handles persistent high score saving via JSON."""
    scores = []
    if os.path.exists(SCORE_FILE):
        with open(SCORE_FILE, 'r') as f:
            try:
                scores = json.load(f)
            except json.JSONDecodeError:
                scores = []
    scores.append({"name": name[:3].upper(), "score": score})
    scores = sorted(scores, key=lambda x: x['score'], reverse=True)[:5]
    with open(SCORE_FILE, 'w') as f:
        json.dump(scores, f)

def get_highscores():
    if os.path.exists(SCORE_FILE):
        with open(SCORE_FILE, 'r') as f:
            try:
                return json.load(f)
            except json.JSONDecodeError:
                return []
    return []

def show_intro(window, screen_height, screen_width):
    """Handles the animated start screen, Leaderboard, and Red Team toggle."""
    window.nodelay(1)

    title_frames = [
        " S N A K E ",
        "-S N A K E-",
        "=S N A K E="
    ]

    frame_idx = 0
    red_team_active = False

    highscores = get_highscores()

    while True:
        window.clear()
        window.border(0)

        # Center the title
        title = title_frames[frame_idx % len(title_frames)]
        window.addstr(screen_height // 2 - 6, (screen_width - len(title)) // 2, title, curses.color_pair(1) | curses.A_BOLD)

        # Leaderboard
        window.addstr(screen_height // 2 - 4, (screen_width - 11) // 2, "LEADERBOARD", curses.color_pair(3) | curses.A_BOLD)
        if highscores:
            for i, entry in enumerate(highscores[:3]):
                score_str = f"{i+1}. {entry['name']} - {entry['score']}"
                window.addstr(screen_height // 2 - 3 + i, (screen_width - len(score_str)) // 2, score_str)
        else:
            ns_str = "No scores yet"
            window.addstr(screen_height // 2 - 3, (screen_width - len(ns_str)) // 2, ns_str)

        # Instructions
        window.addstr(screen_height // 2 + 1, (screen_width - 24) // 2, "PRESS [SPACE] TO START")

        # Red Team Toggle Status
        rt_text = "[R] RED TEAM MODE: " + ("ON " if red_team_active else "OFF")
        rt_color = curses.color_pair(2) if red_team_active else curses.color_pair(0)
        window.addstr(screen_height // 2 + 3, (screen_width - len(rt_text)) // 2, rt_text, rt_color)

        window.refresh()

        # Input handling for intro
        key = window.getch()
        if key == ord(' '):
            break
        elif key in [ord('r'), ord('R')]:
            red_team_active = not red_team_active

        frame_idx += 1
        time.sleep(0.15)

    return red_team_active

def generate_obstacles(num_obstacles, screen_height, screen_width, start_y, start_x, buffer_zone=5):
    """Generates obstacles ensuring they do not block the snake's immediate starting path."""
    obstacles = []
    while len(obstacles) < num_obstacles:
        oy = random.randint(1, screen_height-2)
        ox = random.randint(1, screen_width-2)
        # Check if within buffer zone of the snake's start pos (moving right initially)
        if oy == start_y and (start_x <= ox <= start_x + buffer_zone):
            continue # In immediate horizontal path
        # Also avoid generating directly on the start coordinate or very close
        if abs(oy - start_y) <= 1 and abs(ox - start_x) <= buffer_zone:
            continue

        if [oy, ox] not in obstacles:
            obstacles.append([oy, ox])
    return obstacles

def main(stdscr):
    # --- Color Setup ---
    curses.start_color()
    curses.init_pair(1, curses.COLOR_GREEN, curses.COLOR_BLACK)   # Snake / Standard
    curses.init_pair(2, curses.COLOR_RED, curses.COLOR_BLACK)     # Red Food / Red Team / Obstacles
    curses.init_pair(3, curses.COLOR_CYAN, curses.COLOR_BLACK)    # Blue *
    curses.init_pair(4, curses.COLOR_YELLOW, curses.COLOR_BLACK)  # Dropped segment
    curses.init_pair(5, curses.COLOR_MAGENTA, curses.COLOR_BLACK) # Rainbow addition
    curses.curs_set(0)

    screen_height, screen_width = stdscr.getmaxyx()
    window = curses.newwin(screen_height, screen_width, 0, 0)
    window.keypad(1)

    # 1. Trigger the Intro Animation
    is_red_team = show_intro(window, screen_height, screen_width)

    # 2. Apply Core Game & Red Team Modifiers
    base_speed = 100 if is_red_team else 150
    border_color = curses.color_pair(2) if is_red_team else curses.color_pair(0)
    obstacles = []

    start_y, start_x = screen_height//2, screen_width//4
    snake = [[start_y, start_x]]
    key = curses.KEY_RIGHT
    score = 0
    current_speed = base_speed
    last_eat_time = time.time()
    dropped_segments = []
    food = [screen_height//2, screen_width//2, 'F']
    is_rainbow = False # Triggers when tail is eaten

    # Generate initial obstacles for Red Team
    if is_red_team:
        obstacles = generate_obstacles(5, screen_height, screen_width, start_y, start_x, buffer_zone=10)

    # --- MAIN GAME LOOP ---
    while True:
        window.timeout(int(current_speed))
        window.clear()

        # --- PHASE 1: RENDER ---
        window.attron(border_color)
        window.border(0)
        window.attroff(border_color)

        elapsed = time.time() - last_eat_time
        if elapsed > 30:
            food = [random.randint(1, screen_height-2), random.randint(1, screen_width-2), random.choice(['F', 'F', 'G', 'R', '*'])]
            # Make sure food doesn't land on obstacle
            while food[:2] in obstacles:
                food = [random.randint(1, screen_height-2), random.randint(1, screen_width-2), random.choice(['F', 'F', 'G', 'R', '*'])]
            last_eat_time = time.time()

        window.addstr(0, 2, f' Score: {score} | Time: {int(30-elapsed)}s ', border_color | curses.A_BOLD)

        # Render Obstacles (Red Team)
        for obs in obstacles:
            window.addch(obs[0], obs[1], 'X', curses.color_pair(2) | curses.A_REVERSE)

        # Render Dropped Segments (Flashing)
        for ds in dropped_segments[:]:
            time_left = ds[2] + ds[3] - time.time()
            if time_left <= 0:
                dropped_segments.remove(ds)
            elif int(time_left * 10) % 2 == 0:
                window.addch(ds[0], ds[1], 'S', curses.color_pair(4))

        # Render Food
        f_y, f_x, f_t = food
        f_attr = curses.color_pair(0)
        if f_t == 'R': f_attr = curses.color_pair(2)
        if f_t == '*': f_attr = curses.color_pair(3)
        window.addch(f_y, f_x, f_t, f_attr)

        # Render the Snake
        for i, seg in enumerate(snake):
            # Calculate color: cycle 1-5 if rainbow, else default green (1)
            color = curses.color_pair((i % 5) + 1) if is_rainbow else curses.color_pair(1)
            try:
                window.addch(seg[0], seg[1], '0', color)
            except curses.error:
                pass

        # --- PHASE 2: INPUT ---
        next_key = window.getch()

        # --- PHASE 3: UPDATE ---
        if next_key != -1:
            if next_key == ord(' '):
                if len(snake) > 1:
                    tail = snake.pop()
                    dropped_segments.append([tail[0], tail[1], time.time(), 5.0])
            else:
                # Prevent snake from instantly reversing direction
                if not (key == curses.KEY_RIGHT and next_key == curses.KEY_LEFT) and \
                   not (key == curses.KEY_LEFT and next_key == curses.KEY_RIGHT) and \
                   not (key == curses.KEY_UP and next_key == curses.KEY_DOWN) and \
                   not (key == curses.KEY_DOWN and next_key == curses.KEY_UP):
                    key = next_key

        new_head = [snake[0][0], snake[0][1]]
        if key == curses.KEY_DOWN: new_head[0] += 1
        elif key == curses.KEY_UP: new_head[0] -= 1
        elif key == curses.KEY_LEFT: new_head[1] -= 1
        elif key == curses.KEY_RIGHT: new_head[1] += 1

        snake.insert(0, new_head)

        # Collision (Walls, Self, Obstacles)
        if (snake[0][0] in [0, screen_height-1] or
            snake[0][1] in [0, screen_width-1] or
            snake[0] in snake[1:] or
            snake[0] in obstacles):
            break

        # Check Special Items (Eating your own dropped segment)
        for ds in dropped_segments[:]:
            if snake[0] == [ds[0], ds[1]]:
                score += 100
                is_rainbow = True # Activate Rainbow Mode
                dropped_segments.remove(ds)

        # Check Regular Food
        if snake[0][0] == food[0] and snake[0][1] == food[1]:
            last_eat_time = time.time()
            if food[2] == 'F': score += 10
            elif food[2] == 'G': current_speed *= 0.9 # Faster
            elif food[2] == 'R': current_speed *= 1.1 # Slower
            elif food[2] == '*':
                score += 25
                last_eat_time += 5 # Add time

            food = [random.randint(1, screen_height-2), random.randint(1, screen_width-2), random.choice(['F', 'F', 'G', 'R', '*'])]
            while food[:2] in obstacles:
                food = [random.randint(1, screen_height-2), random.randint(1, screen_width-2), random.choice(['F', 'F', 'G', 'R', '*'])]
        else:
            snake.pop()

    # --- End Game Sequence ---
    window.nodelay(0)
    stdscr.clear()
    stdscr.addstr(screen_height//2, screen_width//2 - 10, f"GAME OVER. Score: {score}")
    stdscr.addstr(screen_height//2 + 1, screen_width//2 - 10, "Enter 3 Letters: ")
    curses.echo()
    name = stdscr.getstr(screen_height//2 + 1, screen_width//2 + 7, 3).decode('utf-8')
    save_score(name, score)

if __name__ == "__main__":
    curses.wrapper(main)
