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

<h2>What's coming?</h2>
There's a bunch of more features which will be added to this program as its still a major work in progress. Some of these features include:

- **Automatic pairing generator** - At the start of every round, the program will generate and print the pairings for that round 
- **Byes** - If you have a tournament with an odd number of people, one person will be left out without a game. There will be a setting so you can choose how many points this person gets - for example, a bye might get 0 points or half a point (0.5 points)
- **Sorted standings** - Right now the standings aren't sorted, and the winner isn't announced. Hopefully this will be added in a future update!
- **The ability to stop/save tournaments** - The ability to save the tournaments in a type of file which the program will generate and then be able to read so a tournament can be continued another time
- **More options** - Byes (mentioned above), whether or not the results should be saved in a file, what the file should be called, etc
- **GUI** - I have never done any graphical programming in Rust, so there is a high chance this feature won't be added, and it'll remain as a console application. However I'll try my best! If any of you guys would like to write a GUI, you can fork the repo :)


Enjoy! Feel free to contribute and put any feedback on <a href="https://discord.gg/vxvKSvd">my discord server</a>
