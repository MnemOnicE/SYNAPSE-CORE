# Snake Game

A classic terminal-based Snake game built in Python using the `curses` library, complete with an intro screen, leaderboards, and multiple game modes.

## Features

- **Classic Snake Mechanics**: Eat food to grow and increase your score.
- **Red Team Mode**: A harder version where random obstacles (`X`) are placed on the field, and the base game speed is faster.
- **Leaderboards**: The top 5 scores are automatically saved and displayed on the intro screen.
- **Special Foods**:
  - `F`: Normal Food (+10 score)
  - `G`: Speed Up / Green Food (decreases update interval)
  - `R`: Slow Down / Red Food (increases update interval)
  - `*`: Star Food (+25 score, adds +5 seconds before food despawn/respawn)
- **Rainbow Snake**: Drop a segment with `[SPACE]`. If you eat your own dropped segment, your snake turns into a rainbow and you gain 100 points!

## Requirements

This game runs in the terminal using the Python standard library.

- **Linux / macOS**: Runs out of the box (the `curses` module is included by default).
- **Windows**: Requires the `windows-curses` package.

You can install the requirements with:
```bash
pip install -r requirements.txt
```

## How to Play

1. Run the script:
   ```bash
   python snake.py
   ```
2. On the start screen:
   - Press **`[SPACE]`** to start the game.
   - Press **`[R]`** to toggle Red Team mode on or off.
3. In-Game:
   - Use the **Arrow Keys** to steer the snake.
   - Press **`[SPACE]`** to drop your tail segment as a consumable item (costs 1 length).
