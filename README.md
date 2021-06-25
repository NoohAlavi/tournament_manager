# Nooh Alavi's Tournament Manager

I decided to throw together this small project to help with automatically managing small casual tournaments in a plethora of games

<h2>Dependencies</h2>

- <a href="https://git-scm.com/">Git</a>
- <a href="https://www.rust-lang.org/">Cargo (Rust's Package Manager; comes installed with Rust)</a>

<h2>How To Use?</h2>

1. Clone the repository - `git clone https://github.com/NoohAlavi/tournament_manager`
2. Run `cd tournament_manager` and `cargo r` to run the program. You can also build it with `cargo b`.
3. It will ask for a tournament name and a number of rounds. Enter in the information and click enter.
4. You can now enter names of players to add to the tournament. **There is currently no way to remove players, but it will be added in the future.**
5. When you wish to start the tournament, type `:start`. You can abbreviate it as `:s`.
6. The first round has started! Now, you should be able to see the current round number, the leaderboard (which is currently blank, as the tourney has just started!), and the pairings for that round. (work in progress).
7. Once the game is finished, you can enter how many points each player has received in the round. The amount is up to you to decide! For example, a win could be worth 1 point, a tie 0.5 points, and a loss 0 points.
8. Once the tournament is over, the final pairings will be shown, and the results will be saved in a `.txt` file with the name `<tournament's name>_scores.txt`.

Enjoy! Feel free to contribute and put any feedback on <a href="https://discord.gg/vxvKSvd">my discord server</a>
