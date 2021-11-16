# riddle
Riddle is an interpretted game langauge designed primarily to explore and learn Rust. By "Interpretted Game Language" it is meant that this combines elements of both a programming language and terminal based video games. A source file provided to the riddle interpretter can be executed fully deterministically like other interpretted langauges; however, the player-programmer also can play an active role in the termination of the program. They are, at times, allowed to freely move within the source code files that are being interpretted and can modify the contents of it, thus removing the determinism of the source being only interpretted. This allows for both simple text based games and traditional programs to intermingle.

## A Bird's Eye View of Riddle Language Concepts
Riddle is a 2 dimensional, circular stack based language at its core, not too dissimilar from languages like Befunge. In a depature from other languages (if one wishes to call Riddle a programming language, that is) the player-programmer can, but does not always, control the interpretter's program counter; that is to say, if the player-programmer is in control and they pass on a tile associated with addition, that operation will take place. Some tiles can force behaviors like movement, and this control of the program counter remains deterministic until another tile returns control. Furthermore, the player-programmer has the ability to modify contents of the file being interpretted to some degree.

### Riddle Elements in Detail

#### The Stage File
Since source files are also what the player-programmer plays in, the concept of them is intentionally confounded. Each stage is composed of a 2 dimensional grid where each tile may be populated by an ASCII character or empty, and each row has the same number of columns and vice-versa. Even if the source file does not have the same number of characters per line, they will be padded with spaces by the interpretter so that each stage meets this constraint. Furthermore, the boundaries of the stage wrap such that movement upward at the top of the stage results in the program counter (player controlled element) being put at the bottom and the same for rightmost and leftmost as appropriate.

There are elements that can be used in Stage Files that allow for stacking and moving between stages.

#### The Player-Programmer as Program Counter
In a certain way, each program or game written in Riddle has two programmers, who may or may not be the same person. First, there is a traditionaly programmer who writes the source Stage File. Second, there is the player-programmer who can control where the program counter moves next and modify elements of the Stage File as it is interpretted. If the stack ever completely empties, the player-programmer "dies" and execution abruptly ends.
