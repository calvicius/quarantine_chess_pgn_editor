# Abandoned

see new (rewritten from scratch) : https://github.com/calvicius/rsChessPgnEditor

# quarantine_chess_pgn_editor

An editor of chess games in pgn format; written in Rustc and GTK3.

The code is a bit tangled, but it is the result of one month locked up at home by the corona-virus quarantine, and therefore this code it is not clean, nor am I going to clean it; It is the moment (tomorrow) when I must return to my usual work.

It consists of two windows:

In the left window: The board with drag and drop movement of pieces. The movement is reflected in the text window on the right. Buttons forward, etc...

In the right window: The text of the game is presented in SAN format. When a new move is added on the board, it is reflected here in form of a new variation (or not, depends...). Promotion options, save as pgn, help, uci engine interface, and exit


Variations can be added and deleted; also comments and NAGs.

You can also edit the pgn header data, save the modified game in pgn format. It also has an interface for a UCI engine. 
