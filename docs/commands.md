# Commands

## Basic commands
These commands control the main flow of an AdventureScript game.

### !disp
```none
!disp text

!disp "Hello!"
```
This command acts as an equivalent to normally displaying text by using no command at all.

*Introduced on AdventureScript 2.0*

**Aliases:** `!show`, `!text`

**Arguments:**

* `text: String`: the text to be displayed.
### !input
```none
!input
```
Waits until the user's input. On the standard console I/O, this waits until the user presses a key.

*Introduced on AdventureScript 0.1*

**Aliases:** `!i`, `!n` (deprecated)

The `\i` escape code can be used as an equivalent, either on an empty line or at the end of a line with text in it.

### !choice
```none
!choice text; choices; ...

!choice "choice text"; {"Do this": {label1}, "Do that": {label2}}
```

Ask the player a choice and lead to a different label depending on their answer.

*Introduced on AdventureScript 2.0*

**Aliases:** `!ch`

**Arguments:**

* `text: String`: the text to be shown right before the choice. In some I/O implementations it might have different formatting. *(Default: *`""`*)*
* `choices: Map`: a map containing each choice's text and labels.
    * This map must follow the pattern `{choice1: goto1, choice2: goto2, ...}`, where `choiceX` are `String`s and `gotoX` are `Label`s, up to a maximum of 9 choices.

Choices are the only place where players can save or restore their saves - although you can turn this off, too (using `!save false`).

### !oldchoice
```none
!oldchoice text; ...
```
**This command is deprecated.**

Old variant of the `!choice` command, taking the choices' data as individual arguments.

*Introduced in AdventureScript 0.1*

**Parameters:**

- `text: String`: 

### !goto
```none
!goto pos

!goto {here}
```
Go to the label indicated.

*Introduced in AdventureScript 0.1*

**Aliases:** `!go`

**Arguments:**

- `pos: Label`: the label to go to.

### !label
```none
!label name

!label "here"
```
Creates a label at that position.

*Introduced in AdventureScript 2.0*

**Arguments:**

- `name: String`: the name the label will be referred by.

Equivalent to `{name}` or `\l[name]`.

### !ending
```none
!ending name

!ending "good"
```
Ends the game with a specific ending.

*Introduced in AdventureScript 0.1*

**Aliases:** `!end`

**Arguments:**

- `name: String`: The name the ending is referred by. Must be defined in the `info.json` file. 

### !loadscript
```none
!loadscript name

!loadscript "script"
```
Starts reading from the beginning of the indicated script.

*Introduced in AdventureScript 0.1*

**Aliases:** `!load`, `!ld`

**Arguments:**

- `name: String`: The filename of the script, excluding the '.as2' extension.

### !save
```none
!save state

!save true
```
Turns saving on or off.

*Introduced in AdventureScript 2.0*

**Aliases:** `!sv`

**Arguments:**

- `state: Bool`: Whether saving should be enabled (`true`) or disabled (`false`).

You can also use the commands `!saveon` and `!saveoff`. *(Introduced in AdventureScript 1.0-pre)*