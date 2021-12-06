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

**Aliases:** `!w`

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
    - Label to go to (`Label` or `None`, optional, defaults to `None`)
    - Flag that determines whether to show the choice or not (`Bool`, optional, defaults to `true`)
* `text: String`: the text to be shown right before the choice. In some I/O implementations it might have different formatting. *(Default: *`""`*)*

Choices are the only place where players can save or restore their saves - although you can turn this off, too (using `!save false`).

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

### !ending
```none
!ending name

!ending "good"
```
Ends the game with a specific ending.

*Introduced in AdventureScript 0.1*

**Aliases:** `!end`

**Arguments:**

- `name: String`: The name the ending is referred by.

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

*Introduced in AdventureScript 1.0-pre as `saveon` and `saveoff`*

**Aliases:** `!sv`

**Arguments:**

- `state: Bool`: Whether saving should be enabled (`true`) or disabled (`false`).

### !if
```none
!if condition; gotrue; gofalse

!if var == 2; {labelA}; {labelB}
```

Jumps to a different point in the script depending on whether the condition is true or false.

**Arguments:**

- `condition: Bool`: The value to be checked. Usually used with a conditional operator. [TODO: Add link]
- `gotrue: Label`: The label to go to if the condition is true.
- `gofalse: Label`: The label to go to if the condition is false.

### !error
```none
!error message

!error "The balance of the universe has been destroyed or something idk"
```
Shows a custom error message and aborts game execution.

*Introduced in AdventureScript 2.0*

**Arguments:**

- `message: String`: The message to be displayed.

## Flag and variable commands

### !flag
```none
!flag flag; value

!flag flag_one; false
```
Sets the value of the specified flag.

*Introduced in AdventureScript 0.1*

**Arguments:**

- `flag: VarRef`: The name of the flag.
    - In this command, you can omit the `?` prefix (which specifies you're checking inside the flags)
- `value: Bool`: The value to set the flag to. *(Default: `true`.)*

### !set
```none
!set var; value

!set variable1; 18
```
Sets the value of the specified variable.

*Introduced in AdventureScript 1.0-pre as `var`*

**Arguments:**

- `var: VarRef`: The name of the variable.
    - You can optionally also set to flags, since they're essentially boolean variables, but `!flag` is recommended instead.
- `value: Any`: The value to set the variable to.

### !add
```none
!add var; value

!add somevar; -1
```
Adds the specified value to the given variable.

*Introduced in AdventureScript 1.0-pre as `incvar`*

**Arguments:**

- `var: VarRef`: The name of the variable to be added to.
- `value: Any`: The value to add to the variable (which should be compatible for adding in the first place!)