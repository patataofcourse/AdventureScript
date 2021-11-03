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
### !wait
```none
!wait
```
Waits until the user's input. On the default I/O, this input is a keypress.

*Introduced on AdventureScript 0.1*

**Aliases:** `!w`, `!n` (deprecated)

The `\w` escape code can be used as an equivalent, either on an empty line or at the end of a line with text in it.

### !choice
```none
!choice choice1; choice2; choice3; ...; choice9; ...

!choice ["Do this": {label1}]; ["Do that": {label2}]; text="Choose a thing"
```

Ask the player a choice and lead to a different label depending on their answer.

*Introduced on AdventureScript 2.0*

**Aliases:** `!ch`

**Arguments:**

* `choice1 - choice9: List`: Lists containing the information of the choices to give:
    - Choice text (`String`)
    - Label to go to (`Label`, optional, defaults to `None`)
    - Flag that determines whether to show the choice or not (`Bool`, optional, defaults to `true`)
* `text: String`: the text to be shown right before the choice. In some I/O implementations it might have different formatting. *(Default: *`""`*)*

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