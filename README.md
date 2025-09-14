# Color Wars
A 2-player tactical board game of territorial expansion and chain reactions, inspired by the game found in the Android app "1 2 3 4 PLAYER GAMES" by JindoBlu.

# Game Overview
Color Wars is a 2-player strategy game played on a `5×5` grid. Players take turns building up the value of their squares. When a square's value reaches its limit, it "pops," converting adjacent squares to its color and potentially setting off a cascade of chain reactions across the board. The objective is to eliminate your opponent by capturing every square they control.

## Core Components
- **Board**: A `5×5` grid of squares.

- **Squares**: A square can be in one of two states:

  - **Empty**: It has no owner or value.

  - **Occupied**: It is controlled by a player and has their color and a numeric value from `1` to `3`.

## Objective
The goal is to be the last player with squares on the board. You win by completely eliminating your opponent's squares.

## How to Play
1. Initial Setup

    1. Determine which player goes first (`Player 1`).

    2. `Player 1` chooses one empty square on the board and places their starting tile. This square is now marked with `Player 1`'s color and has a value of `3`.

    3. `Player 2` then does the same on any other empty square.

2. Gameplay

    On your turn, you perform one simple action:

    - Choose **one** of the squares you currently occupy and increase its value by `1`.

3. The "Pop" Mechanic

    The central mechanic of the game is the "pop."

    - **Trigger**: If your move causes a square's value to reach `4`, it immediately "pops." A square can never have a resting value of `4`.

    - **Effect**: When a square pops:

      1. The original square that popped becomes **empty**.

      2. It sends a `+1` value to each of the four orthogonally adjacent squares (North, East, South, and West).

      3. Any adjacent square that receives a `+1` value is **immediately converted to your color** before its value is incremented.

            `Example: A Red square pops. It is adjacent to a Blue square that has a value of 2. The Blue square becomes a Red square, and its new value is 3 (2 + 1).`

    - **Board Edges**: If a square on an edge or corner pops, any `+1` value sent off the board is lost.

        `For example, a corner square only has two adjacent neighbors, so it will only send a +1 value to those two squares.`

4. Chain Reactions

    A single move can trigger a cascade of pops.

    - If a pop causes an adjacent square's value to become `4`, that square will also immediately pop.

    - This chain reaction will continue until there are no squares on the board with a value of `4`. The entire chain resolves within the same turn.

    - **Resolution Order**: If a pop causes multiple squares to trigger simultaneously, the active player can resolve them in any order they choose. The final state of the board will be the same regardless of the resolution order.

## Winning the Game
The game ends when your opponent has no more squares left on the board. You are then declared the winner.
