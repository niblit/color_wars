# Color Wars
Found this game in the android app "1 2 3 4 PLAYER GAMES" by JindoBlu (com.JindoBlu.FourPlayers)

## Setting
Board game on 5x5 squares, each square can either be empty or occupied by a team color and a numeric value (0 <= value <= 4)

## Win Condition
Conquer all squares occupied by the opponent (eliminate all of the opponent squares)

## Initial Setup
After determining move order, each player sets a square with their color and a value of 3

## Rules
- In your turn, add +1 of value to one of your occupied squares, this means you can only make moves on your occupied squares
- If any square's value equals 4, said square will "pop", sending a +1 value to the 4 adyacent squares, that is, squares in the north, east, south and west positions
- The original square will become empty
- The pop mechanic means the four adyacent squares will always be set to your team's color, and the previous value of the adyacent squares will be added with the +1 you sent
- This pop mechanic will repeat as many times as necessary, such as there isn't any square with a value of 4, getting to four means inmediate pop
