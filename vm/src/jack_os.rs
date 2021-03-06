// The supplied "Operating System" source from the original Jack Virtual Machine.
// The Java implementation goes a step further and implements the OS interfaces
// inline in native Java. These impelemtantions are reasonably fast enough though.
pub const SOURCE: &str = "
function Array.new 0
push argument 0
push constant 0
gt
not
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 2
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 0
call Memory.alloc 1
return
function Array.dispose 0
push argument 0
pop pointer 0
push pointer 0
call Memory.deAlloc 1
pop temp 0
push constant 0
return
function Keyboard.init 0
push constant 0
return
function Keyboard.keyPressed 0
push constant 24576
call Memory.peek 1
return
function Keyboard.readChar 2
push constant 0
call Output.printChar 1
pop temp 0
label WHILE_EXP0
push local 1
push constant 0
eq
push local 0
push constant 0
gt
or
not
if-goto WHILE_END0
call Keyboard.keyPressed 0
pop local 0
push local 0
push constant 0
gt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push local 0
pop local 1
label IF_FALSE0
goto WHILE_EXP0
label WHILE_END0
call String.backSpace 0
call Output.printChar 1
pop temp 0
push local 1
call Output.printChar 1
pop temp 0
push local 1
return
function Keyboard.readLine 5
push constant 80
call String.new 1
pop local 3
push argument 0
call Output.printString 1
pop temp 0
call String.newLine 0
pop local 1
call String.backSpace 0
pop local 2
label WHILE_EXP0
push local 4
not
not
if-goto WHILE_END0
call Keyboard.readChar 0
pop local 0
push local 0
push local 1
eq
pop local 4
push local 4
not
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push local 0
push local 2
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push local 3
call String.eraseLastChar 1
pop temp 0
goto IF_END1
label IF_FALSE1
push local 3
push local 0
call String.appendChar 2
pop local 3
label IF_END1
label IF_FALSE0
goto WHILE_EXP0
label WHILE_END0
push local 3
return
function Keyboard.readInt 2
push argument 0
call Keyboard.readLine 1
pop local 0
push local 0
call String.intValue 1
pop local 1
push local 0
call String.dispose 1
pop temp 0
push local 1
return
function Math.init 1
push constant 16
call Array.new 1
pop static 1
push constant 16
call Array.new 1
pop static 0
push constant 0
push static 0
add
push constant 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
label WHILE_EXP0
push local 0
push constant 15
lt
not
if-goto WHILE_END0
push local 0
push constant 1
add
pop local 0
push local 0
push static 0
add
push local 0
push constant 1
sub
push static 0
add
pop pointer 1
push that 0
push local 0
push constant 1
sub
push static 0
add
pop pointer 1
push that 0
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Math.abs 0
push argument 0
push constant 0
lt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push argument 0
neg
pop argument 0
label IF_FALSE0
push argument 0
return
function Math.multiply 5
push argument 0
push constant 0
lt
push argument 1
push constant 0
gt
and
push argument 0
push constant 0
gt
push argument 1
push constant 0
lt
and
or
pop local 4
push argument 0
call Math.abs 1
pop argument 0
push argument 1
call Math.abs 1
pop argument 1
push argument 0
push argument 1
lt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push argument 0
pop local 1
push argument 1
pop argument 0
push local 1
pop argument 1
label IF_FALSE0
label WHILE_EXP0
push local 2
push constant 1
sub
push argument 1
push constant 1
sub
lt
not
if-goto WHILE_END0
push local 3
push static 0
add
pop pointer 1
push that 0
push argument 1
and
push constant 0
eq
not
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push local 0
push argument 0
add
pop local 0
push local 2
push local 3
push static 0
add
pop pointer 1
push that 0
add
pop local 2
label IF_FALSE1
push argument 0
push argument 0
add
pop argument 0
push local 3
push constant 1
add
pop local 3
goto WHILE_EXP0
label WHILE_END0
push local 4
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push local 0
neg
pop local 0
label IF_FALSE2
push local 0
return
function Math.divide 4
push argument 1
push constant 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 3
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 0
push constant 0
lt
push argument 1
push constant 0
gt
and
push argument 0
push constant 0
gt
push argument 1
push constant 0
lt
and
or
pop local 2
push constant 0
push static 1
add
push argument 1
call Math.abs 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push argument 0
call Math.abs 1
pop argument 0
label WHILE_EXP0
push local 0
push constant 15
lt
push local 3
not
and
not
if-goto WHILE_END0
push constant 32767
push local 0
push static 1
add
pop pointer 1
push that 0
push constant 1
sub
sub
push local 0
push static 1
add
pop pointer 1
push that 0
push constant 1
sub
lt
pop local 3
push local 3
not
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push local 0
push constant 1
add
push static 1
add
push local 0
push static 1
add
pop pointer 1
push that 0
push local 0
push static 1
add
pop pointer 1
push that 0
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 1
add
push static 1
add
pop pointer 1
push that 0
push constant 1
sub
push argument 0
push constant 1
sub
gt
pop local 3
push local 3
not
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push local 0
push constant 1
add
pop local 0
label IF_FALSE2
label IF_FALSE1
goto WHILE_EXP0
label WHILE_END0
label WHILE_EXP1
push local 0
push constant 1
neg
gt
not
if-goto WHILE_END1
push local 0
push static 1
add
pop pointer 1
push that 0
push constant 1
sub
push argument 0
push constant 1
sub
gt
not
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
push local 1
push local 0
push static 0
add
pop pointer 1
push that 0
add
pop local 1
push argument 0
push local 0
push static 1
add
pop pointer 1
push that 0
sub
pop argument 0
label IF_FALSE3
push local 0
push constant 1
sub
pop local 0
goto WHILE_EXP1
label WHILE_END1
push local 2
if-goto IF_TRUE4
goto IF_FALSE4
label IF_TRUE4
push local 1
neg
pop local 1
label IF_FALSE4
push local 1
return
function Math.sqrt 4
push argument 0
push constant 0
lt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 4
call Sys.error 1
pop temp 0
label IF_FALSE0
push constant 7
pop local 0
label WHILE_EXP0
push local 0
push constant 1
neg
gt
not
if-goto WHILE_END0
push local 3
push local 0
push static 0
add
pop pointer 1
push that 0
add
pop local 1
push local 1
push local 1
call Math.multiply 2
pop local 2
push local 2
push argument 0
gt
not
push local 2
push constant 0
lt
not
and
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push local 1
pop local 3
label IF_FALSE1
push local 0
push constant 1
sub
pop local 0
goto WHILE_EXP0
label WHILE_END0
push local 3
return
function Math.max 0
push argument 0
push argument 1
gt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push argument 0
pop argument 1
label IF_FALSE0
push argument 1
return
function Math.min 0
push argument 0
push argument 1
lt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push argument 0
pop argument 1
label IF_FALSE0
push argument 1
return
function Memory.init 0
push constant 0
pop static 0
push constant 2048
push static 0
add
push constant 14334
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 2049
push static 0
add
push constant 2050
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 0
return
function Memory.peek 0
push argument 0
push static 0
add
pop pointer 1
push that 0
return
function Memory.poke 0
push argument 0
push static 0
add
push argument 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 0
return
function Memory.alloc 1
push argument 0
push constant 1
lt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 5
call Sys.error 1
pop temp 0
label IF_FALSE0
push constant 2048
pop local 0
label WHILE_EXP0
push constant 0
push local 0
add
pop pointer 1
push that 0
push argument 0
lt
not
if-goto WHILE_END0
push constant 1
push local 0
add
pop pointer 1
push that 0
pop local 0
goto WHILE_EXP0
label WHILE_END0
push local 0
push argument 0
add
push constant 16379
gt
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push constant 6
call Sys.error 1
pop temp 0
label IF_FALSE1
push constant 0
push local 0
add
pop pointer 1
push that 0
push argument 0
push constant 2
add
gt
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push argument 0
push constant 2
add
push local 0
add
push constant 0
push local 0
add
pop pointer 1
push that 0
push argument 0
sub
push constant 2
sub
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 1
push local 0
add
pop pointer 1
push that 0
push local 0
push constant 2
add
eq
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
push argument 0
push constant 3
add
push local 0
add
push local 0
push argument 0
add
push constant 4
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto IF_END3
label IF_FALSE3
push argument 0
push constant 3
add
push local 0
add
push constant 1
push local 0
add
pop pointer 1
push that 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
label IF_END3
push constant 1
push local 0
add
push local 0
push argument 0
add
push constant 2
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
label IF_FALSE2
push constant 0
push local 0
add
push constant 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 2
add
return
function Memory.deAlloc 2
push argument 0
push constant 2
sub
pop local 0
push constant 1
push local 0
add
pop pointer 1
push that 0
pop local 1
push constant 0
push local 1
add
pop pointer 1
push that 0
push constant 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 0
push local 0
add
push constant 1
push local 0
add
pop pointer 1
push that 0
push local 0
sub
push constant 2
sub
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto IF_END0
label IF_FALSE0
push constant 0
push local 0
add
push constant 1
push local 0
add
pop pointer 1
push that 0
push local 0
sub
push constant 0
push local 1
add
pop pointer 1
push that 0
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 1
push local 1
add
pop pointer 1
push that 0
push local 1
push constant 2
add
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push constant 1
push local 0
add
push local 0
push constant 2
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto IF_END1
label IF_FALSE1
push constant 1
push local 0
add
push constant 1
push local 1
add
pop pointer 1
push that 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
label IF_END1
label IF_END0
push constant 0
return
function Output.init 0
push constant 16384
pop static 4
push constant 0
not
pop static 2
push constant 32
pop static 1
push constant 0
pop static 0
push constant 6
call String.new 1
pop static 3
call Output.initMap 0
pop temp 0
call Output.createShiftedMap 0
pop temp 0
push constant 0
return
function Output.initMap 0
push constant 127
call Array.new 1
pop static 5
push constant 0
push constant 63
push constant 63
push constant 63
push constant 63
push constant 63
push constant 63
push constant 63
push constant 63
push constant 63
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 32
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 33
push constant 12
push constant 30
push constant 30
push constant 30
push constant 12
push constant 12
push constant 0
push constant 12
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 34
push constant 54
push constant 54
push constant 20
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 35
push constant 0
push constant 18
push constant 18
push constant 63
push constant 18
push constant 18
push constant 63
push constant 18
push constant 18
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 36
push constant 12
push constant 30
push constant 51
push constant 3
push constant 30
push constant 48
push constant 51
push constant 30
push constant 12
push constant 12
push constant 0
call Output.create 12
pop temp 0
push constant 37
push constant 0
push constant 0
push constant 35
push constant 51
push constant 24
push constant 12
push constant 6
push constant 51
push constant 49
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 38
push constant 12
push constant 30
push constant 30
push constant 12
push constant 54
push constant 27
push constant 27
push constant 27
push constant 54
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 39
push constant 12
push constant 12
push constant 6
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 40
push constant 24
push constant 12
push constant 6
push constant 6
push constant 6
push constant 6
push constant 6
push constant 12
push constant 24
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 41
push constant 6
push constant 12
push constant 24
push constant 24
push constant 24
push constant 24
push constant 24
push constant 12
push constant 6
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 42
push constant 0
push constant 0
push constant 0
push constant 51
push constant 30
push constant 63
push constant 30
push constant 51
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 43
push constant 0
push constant 0
push constant 0
push constant 12
push constant 12
push constant 63
push constant 12
push constant 12
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 44
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 12
push constant 12
push constant 6
push constant 0
call Output.create 12
pop temp 0
push constant 45
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 63
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 46
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 12
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 47
push constant 0
push constant 0
push constant 32
push constant 48
push constant 24
push constant 12
push constant 6
push constant 3
push constant 1
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 48
push constant 12
push constant 30
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 30
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 49
push constant 12
push constant 14
push constant 15
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 63
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 50
push constant 30
push constant 51
push constant 48
push constant 24
push constant 12
push constant 6
push constant 3
push constant 51
push constant 63
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 51
push constant 30
push constant 51
push constant 48
push constant 48
push constant 28
push constant 48
push constant 48
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 52
push constant 16
push constant 24
push constant 28
push constant 26
push constant 25
push constant 63
push constant 24
push constant 24
push constant 60
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 53
push constant 63
push constant 3
push constant 3
push constant 31
push constant 48
push constant 48
push constant 48
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 54
push constant 28
push constant 6
push constant 3
push constant 3
push constant 31
push constant 51
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 55
push constant 63
push constant 49
push constant 48
push constant 48
push constant 24
push constant 12
push constant 12
push constant 12
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 56
push constant 30
push constant 51
push constant 51
push constant 51
push constant 30
push constant 51
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 57
push constant 30
push constant 51
push constant 51
push constant 51
push constant 62
push constant 48
push constant 48
push constant 24
push constant 14
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 58
push constant 0
push constant 0
push constant 12
push constant 12
push constant 0
push constant 0
push constant 12
push constant 12
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 59
push constant 0
push constant 0
push constant 12
push constant 12
push constant 0
push constant 0
push constant 12
push constant 12
push constant 6
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 60
push constant 0
push constant 0
push constant 24
push constant 12
push constant 6
push constant 3
push constant 6
push constant 12
push constant 24
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 61
push constant 0
push constant 0
push constant 0
push constant 63
push constant 0
push constant 0
push constant 63
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 62
push constant 0
push constant 0
push constant 3
push constant 6
push constant 12
push constant 24
push constant 12
push constant 6
push constant 3
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 64
push constant 30
push constant 51
push constant 51
push constant 59
push constant 59
push constant 59
push constant 27
push constant 3
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 63
push constant 30
push constant 51
push constant 51
push constant 24
push constant 12
push constant 12
push constant 0
push constant 12
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 65
push constant 12
push constant 30
push constant 51
push constant 51
push constant 63
push constant 51
push constant 51
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 66
push constant 31
push constant 51
push constant 51
push constant 51
push constant 31
push constant 51
push constant 51
push constant 51
push constant 31
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 67
push constant 28
push constant 54
push constant 35
push constant 3
push constant 3
push constant 3
push constant 35
push constant 54
push constant 28
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 68
push constant 15
push constant 27
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 27
push constant 15
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 69
push constant 63
push constant 51
push constant 35
push constant 11
push constant 15
push constant 11
push constant 35
push constant 51
push constant 63
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 70
push constant 63
push constant 51
push constant 35
push constant 11
push constant 15
push constant 11
push constant 3
push constant 3
push constant 3
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 71
push constant 28
push constant 54
push constant 35
push constant 3
push constant 59
push constant 51
push constant 51
push constant 54
push constant 44
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 72
push constant 51
push constant 51
push constant 51
push constant 51
push constant 63
push constant 51
push constant 51
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 73
push constant 30
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 74
push constant 60
push constant 24
push constant 24
push constant 24
push constant 24
push constant 24
push constant 27
push constant 27
push constant 14
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 75
push constant 51
push constant 51
push constant 51
push constant 27
push constant 15
push constant 27
push constant 51
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 76
push constant 3
push constant 3
push constant 3
push constant 3
push constant 3
push constant 3
push constant 35
push constant 51
push constant 63
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 77
push constant 33
push constant 51
push constant 63
push constant 63
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 78
push constant 51
push constant 51
push constant 55
push constant 55
push constant 63
push constant 59
push constant 59
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 79
push constant 30
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 80
push constant 31
push constant 51
push constant 51
push constant 51
push constant 31
push constant 3
push constant 3
push constant 3
push constant 3
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 81
push constant 30
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 63
push constant 59
push constant 30
push constant 48
push constant 0
call Output.create 12
pop temp 0
push constant 82
push constant 31
push constant 51
push constant 51
push constant 51
push constant 31
push constant 27
push constant 51
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 83
push constant 30
push constant 51
push constant 51
push constant 6
push constant 28
push constant 48
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 84
push constant 63
push constant 63
push constant 45
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 85
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 86
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 30
push constant 30
push constant 12
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 87
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 63
push constant 63
push constant 63
push constant 18
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 88
push constant 51
push constant 51
push constant 30
push constant 30
push constant 12
push constant 30
push constant 30
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 89
push constant 51
push constant 51
push constant 51
push constant 51
push constant 30
push constant 12
push constant 12
push constant 12
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 90
push constant 63
push constant 51
push constant 49
push constant 24
push constant 12
push constant 6
push constant 35
push constant 51
push constant 63
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 91
push constant 30
push constant 6
push constant 6
push constant 6
push constant 6
push constant 6
push constant 6
push constant 6
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 92
push constant 0
push constant 0
push constant 1
push constant 3
push constant 6
push constant 12
push constant 24
push constant 48
push constant 32
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 93
push constant 30
push constant 24
push constant 24
push constant 24
push constant 24
push constant 24
push constant 24
push constant 24
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 94
push constant 8
push constant 28
push constant 54
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 95
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 63
push constant 0
call Output.create 12
pop temp 0
push constant 96
push constant 6
push constant 12
push constant 24
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 97
push constant 0
push constant 0
push constant 0
push constant 14
push constant 24
push constant 30
push constant 27
push constant 27
push constant 54
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 98
push constant 3
push constant 3
push constant 3
push constant 15
push constant 27
push constant 51
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 99
push constant 0
push constant 0
push constant 0
push constant 30
push constant 51
push constant 3
push constant 3
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 100
push constant 48
push constant 48
push constant 48
push constant 60
push constant 54
push constant 51
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 101
push constant 0
push constant 0
push constant 0
push constant 30
push constant 51
push constant 63
push constant 3
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 102
push constant 28
push constant 54
push constant 38
push constant 6
push constant 15
push constant 6
push constant 6
push constant 6
push constant 15
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 103
push constant 0
push constant 0
push constant 30
push constant 51
push constant 51
push constant 51
push constant 62
push constant 48
push constant 51
push constant 30
push constant 0
call Output.create 12
pop temp 0
push constant 104
push constant 3
push constant 3
push constant 3
push constant 27
push constant 55
push constant 51
push constant 51
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 105
push constant 12
push constant 12
push constant 0
push constant 14
push constant 12
push constant 12
push constant 12
push constant 12
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 106
push constant 48
push constant 48
push constant 0
push constant 56
push constant 48
push constant 48
push constant 48
push constant 48
push constant 51
push constant 30
push constant 0
call Output.create 12
pop temp 0
push constant 107
push constant 3
push constant 3
push constant 3
push constant 51
push constant 27
push constant 15
push constant 15
push constant 27
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 108
push constant 14
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 109
push constant 0
push constant 0
push constant 0
push constant 29
push constant 63
push constant 43
push constant 43
push constant 43
push constant 43
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 110
push constant 0
push constant 0
push constant 0
push constant 29
push constant 51
push constant 51
push constant 51
push constant 51
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 111
push constant 0
push constant 0
push constant 0
push constant 30
push constant 51
push constant 51
push constant 51
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 112
push constant 0
push constant 0
push constant 0
push constant 30
push constant 51
push constant 51
push constant 51
push constant 31
push constant 3
push constant 3
push constant 0
call Output.create 12
pop temp 0
push constant 113
push constant 0
push constant 0
push constant 0
push constant 30
push constant 51
push constant 51
push constant 51
push constant 62
push constant 48
push constant 48
push constant 0
call Output.create 12
pop temp 0
push constant 114
push constant 0
push constant 0
push constant 0
push constant 29
push constant 55
push constant 51
push constant 3
push constant 3
push constant 7
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 115
push constant 0
push constant 0
push constant 0
push constant 30
push constant 51
push constant 6
push constant 24
push constant 51
push constant 30
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 116
push constant 4
push constant 6
push constant 6
push constant 15
push constant 6
push constant 6
push constant 6
push constant 54
push constant 28
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 117
push constant 0
push constant 0
push constant 0
push constant 27
push constant 27
push constant 27
push constant 27
push constant 27
push constant 54
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 118
push constant 0
push constant 0
push constant 0
push constant 51
push constant 51
push constant 51
push constant 51
push constant 30
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 119
push constant 0
push constant 0
push constant 0
push constant 51
push constant 51
push constant 51
push constant 63
push constant 63
push constant 18
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 120
push constant 0
push constant 0
push constant 0
push constant 51
push constant 30
push constant 12
push constant 12
push constant 30
push constant 51
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 121
push constant 0
push constant 0
push constant 0
push constant 51
push constant 51
push constant 51
push constant 62
push constant 48
push constant 24
push constant 15
push constant 0
call Output.create 12
pop temp 0
push constant 122
push constant 0
push constant 0
push constant 0
push constant 63
push constant 27
push constant 12
push constant 6
push constant 51
push constant 63
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 123
push constant 56
push constant 12
push constant 12
push constant 12
push constant 7
push constant 12
push constant 12
push constant 12
push constant 56
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 124
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 12
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 125
push constant 7
push constant 12
push constant 12
push constant 12
push constant 56
push constant 12
push constant 12
push constant 12
push constant 7
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 126
push constant 38
push constant 45
push constant 25
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
push constant 0
call Output.create 12
pop temp 0
push constant 0
return
function Output.create 1
push constant 11
call Array.new 1
pop local 0
push argument 0
push static 5
add
push local 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 0
push local 0
add
push argument 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 1
push local 0
add
push argument 2
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 2
push local 0
add
push argument 3
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 3
push local 0
add
push argument 4
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 4
push local 0
add
push argument 5
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 5
push local 0
add
push argument 6
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 6
push local 0
add
push argument 7
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 7
push local 0
add
push argument 8
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 8
push local 0
add
push argument 9
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 9
push local 0
add
push argument 10
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 10
push local 0
add
push argument 11
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 0
return
function Output.createShiftedMap 4
push constant 127
call Array.new 1
pop static 6
push constant 0
pop local 2
label WHILE_EXP0
push local 2
push constant 127
lt
not
if-goto WHILE_END0
push local 2
push static 5
add
pop pointer 1
push that 0
pop local 0
push constant 11
call Array.new 1
pop local 1
push local 2
push static 6
add
push local 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 0
pop local 3
label WHILE_EXP1
push local 3
push constant 11
lt
not
if-goto WHILE_END1
push local 3
push local 1
add
push local 3
push local 0
add
pop pointer 1
push that 0
push constant 256
call Math.multiply 2
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 3
push constant 1
add
pop local 3
goto WHILE_EXP1
label WHILE_END1
push local 2
push constant 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 32
pop local 2
goto IF_END0
label IF_FALSE0
push local 2
push constant 1
add
pop local 2
label IF_END0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Output.getMap 1
push argument 0
push constant 32
lt
push argument 0
push constant 126
gt
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 0
pop argument 0
label IF_FALSE0
push static 2
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push argument 0
push static 5
add
pop pointer 1
push that 0
pop local 0
goto IF_END1
label IF_FALSE1
push argument 0
push static 6
add
pop pointer 1
push that 0
pop local 0
label IF_END1
push local 0
return
function Output.drawChar 4
push argument 0
call Output.getMap 1
pop local 2
push static 1
pop local 0
label WHILE_EXP0
push local 1
push constant 11
lt
not
if-goto WHILE_END0
push static 2
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push local 0
push static 4
add
pop pointer 1
push that 0
push constant 256
neg
and
pop local 3
goto IF_END0
label IF_FALSE0
push local 0
push static 4
add
pop pointer 1
push that 0
push constant 255
and
pop local 3
label IF_END0
push local 0
push static 4
add
push local 1
push local 2
add
pop pointer 1
push that 0
push local 3
or
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 32
add
pop local 0
push local 1
push constant 1
add
pop local 1
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Output.moveCursor 0
push argument 0
push constant 0
lt
push argument 0
push constant 22
gt
or
push argument 1
push constant 0
lt
or
push argument 1
push constant 63
gt
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 20
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 1
push constant 2
call Math.divide 2
pop static 0
push constant 32
push argument 0
push constant 352
call Math.multiply 2
add
push static 0
add
pop static 1
push argument 1
push static 0
push constant 2
call Math.multiply 2
eq
pop static 2
push constant 32
call Output.drawChar 1
pop temp 0
push constant 0
return
function Output.printChar 0
push argument 0
call String.newLine 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
call Output.println 0
pop temp 0
goto IF_END0
label IF_FALSE0
push argument 0
call String.backSpace 0
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
call Output.backSpace 0
pop temp 0
goto IF_END1
label IF_FALSE1
push argument 0
call Output.drawChar 1
pop temp 0
push static 2
not
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push static 0
push constant 1
add
pop static 0
push static 1
push constant 1
add
pop static 1
label IF_FALSE2
push static 0
push constant 32
eq
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
call Output.println 0
pop temp 0
goto IF_END3
label IF_FALSE3
push static 2
not
pop static 2
label IF_END3
label IF_END1
label IF_END0
push constant 0
return
function Output.printString 2
push argument 0
call String.length 1
pop local 1
label WHILE_EXP0
push local 0
push local 1
lt
not
if-goto WHILE_END0
push argument 0
push local 0
call String.charAt 2
call Output.printChar 1
pop temp 0
push local 0
push constant 1
add
pop local 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Output.printInt 0
push static 3
push argument 0
call String.setInt 2
pop temp 0
push static 3
call Output.printString 1
pop temp 0
push constant 0
return
function Output.println 0
push static 1
push constant 352
add
push static 0
sub
pop static 1
push constant 0
pop static 0
push constant 0
not
pop static 2
push static 1
push constant 8128
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 32
pop static 1
label IF_FALSE0
push constant 0
return
function Output.backSpace 0
push static 2
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push static 0
push constant 0
gt
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push static 0
push constant 1
sub
pop static 0
push static 1
push constant 1
sub
pop static 1
goto IF_END1
label IF_FALSE1
push constant 31
pop static 0
push static 1
push constant 32
eq
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push constant 8128
pop static 1
label IF_FALSE2
push static 1
push constant 321
sub
pop static 1
label IF_END1
push constant 0
pop static 2
goto IF_END0
label IF_FALSE0
push constant 0
not
pop static 2
label IF_END0
push constant 32
call Output.drawChar 1
pop temp 0
push constant 0
return
function Screen.init 1
push constant 16384
pop static 1
push constant 0
not
pop static 2
push constant 17
call Array.new 1
pop static 0
push constant 0
push static 0
add
push constant 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
label WHILE_EXP0
push local 0
push constant 16
lt
not
if-goto WHILE_END0
push local 0
push constant 1
add
pop local 0
push local 0
push static 0
add
push local 0
push constant 1
sub
push static 0
add
pop pointer 1
push that 0
push local 0
push constant 1
sub
push static 0
add
pop pointer 1
push that 0
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Screen.clearScreen 1
label WHILE_EXP0
push local 0
push constant 8192
lt
not
if-goto WHILE_END0
push local 0
push static 1
add
push constant 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 1
add
pop local 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Screen.updateLocation 0
push static 2
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push argument 0
push static 1
add
push argument 0
push static 1
add
pop pointer 1
push that 0
push argument 1
or
pop temp 0
pop pointer 1
push temp 0
pop that 0
goto IF_END0
label IF_FALSE0
push argument 0
push static 1
add
push argument 0
push static 1
add
pop pointer 1
push that 0
push argument 1
not
and
pop temp 0
pop pointer 1
push temp 0
pop that 0
label IF_END0
push constant 0
return
function Screen.setColor 0
push argument 0
pop static 2
push constant 0
return
function Screen.drawPixel 3
push argument 0
push constant 0
lt
push argument 0
push constant 511
gt
or
push argument 1
push constant 0
lt
or
push argument 1
push constant 255
gt
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 7
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 0
push constant 16
call Math.divide 2
pop local 0
push argument 0
push local 0
push constant 16
call Math.multiply 2
sub
pop local 1
push argument 1
push constant 32
call Math.multiply 2
push local 0
add
pop local 2
push local 2
push local 1
push static 0
add
pop pointer 1
push that 0
call Screen.updateLocation 2
pop temp 0
push constant 0
return
function Screen.drawConditional 0
push argument 2
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push argument 1
push argument 0
call Screen.drawPixel 2
pop temp 0
goto IF_END0
label IF_FALSE0
push argument 0
push argument 1
call Screen.drawPixel 2
pop temp 0
label IF_END0
push constant 0
return
function Screen.drawLine 11
push argument 0
push constant 0
lt
push argument 2
push constant 511
gt
or
push argument 1
push constant 0
lt
or
push argument 3
push constant 255
gt
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 8
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 2
push argument 0
sub
call Math.abs 1
pop local 3
push argument 3
push argument 1
sub
call Math.abs 1
pop local 2
push local 3
push local 2
lt
pop local 6
push local 6
push argument 3
push argument 1
lt
and
push local 6
not
push argument 2
push argument 0
lt
and
or
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push argument 0
pop local 4
push argument 2
pop argument 0
push local 4
pop argument 2
push argument 1
pop local 4
push argument 3
pop argument 1
push local 4
pop argument 3
label IF_FALSE1
push local 6
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push local 3
pop local 4
push local 2
pop local 3
push local 4
pop local 2
push argument 1
pop local 1
push argument 0
pop local 0
push argument 3
pop local 8
push argument 0
push argument 2
gt
pop local 7
goto IF_END2
label IF_FALSE2
push argument 0
pop local 1
push argument 1
pop local 0
push argument 2
pop local 8
push argument 1
push argument 3
gt
pop local 7
label IF_END2
push constant 2
push local 2
call Math.multiply 2
push local 3
sub
pop local 5
push constant 2
push local 2
call Math.multiply 2
pop local 9
push constant 2
push local 2
push local 3
sub
call Math.multiply 2
pop local 10
push local 1
push local 0
push local 6
call Screen.drawConditional 3
pop temp 0
label WHILE_EXP0
push local 1
push local 8
lt
not
if-goto WHILE_END0
push local 5
push constant 0
lt
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
push local 5
push local 9
add
pop local 5
goto IF_END3
label IF_FALSE3
push local 5
push local 10
add
pop local 5
push local 7
if-goto IF_TRUE4
goto IF_FALSE4
label IF_TRUE4
push local 0
push constant 1
sub
pop local 0
goto IF_END4
label IF_FALSE4
push local 0
push constant 1
add
pop local 0
label IF_END4
label IF_END3
push local 1
push constant 1
add
pop local 1
push local 1
push local 0
push local 6
call Screen.drawConditional 3
pop temp 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Screen.drawRectangle 9
push argument 0
push argument 2
gt
push argument 1
push argument 3
gt
or
push argument 0
push constant 0
lt
or
push argument 2
push constant 511
gt
or
push argument 1
push constant 0
lt
or
push argument 3
push constant 255
gt
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 9
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 0
push constant 16
call Math.divide 2
pop local 3
push argument 0
push local 3
push constant 16
call Math.multiply 2
sub
pop local 7
push argument 2
push constant 16
call Math.divide 2
pop local 4
push argument 2
push local 4
push constant 16
call Math.multiply 2
sub
pop local 8
push local 7
push static 0
add
pop pointer 1
push that 0
push constant 1
sub
not
pop local 6
push local 8
push constant 1
add
push static 0
add
pop pointer 1
push that 0
push constant 1
sub
pop local 5
push argument 1
push constant 32
call Math.multiply 2
push local 3
add
pop local 0
push local 4
push local 3
sub
pop local 2
label WHILE_EXP0
push argument 1
push argument 3
gt
not
not
if-goto WHILE_END0
push local 0
push local 2
add
pop local 1
push local 2
push constant 0
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push local 0
push local 5
push local 6
and
call Screen.updateLocation 2
pop temp 0
goto IF_END1
label IF_FALSE1
push local 0
push local 6
call Screen.updateLocation 2
pop temp 0
push local 0
push constant 1
add
pop local 0
label WHILE_EXP1
push local 0
push local 1
lt
not
if-goto WHILE_END1
push local 0
push constant 1
neg
call Screen.updateLocation 2
pop temp 0
push local 0
push constant 1
add
pop local 0
goto WHILE_EXP1
label WHILE_END1
push local 1
push local 5
call Screen.updateLocation 2
pop temp 0
label IF_END1
push argument 1
push constant 1
add
pop argument 1
push local 1
push constant 32
add
push local 2
sub
pop local 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Screen.drawHorizontal 11
push argument 1
push argument 2
call Math.min 2
pop local 7
push argument 1
push argument 2
call Math.max 2
pop local 8
push argument 0
push constant 1
neg
gt
push argument 0
push constant 256
lt
and
push local 7
push constant 512
lt
and
push local 8
push constant 1
neg
gt
and
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push local 7
push constant 0
call Math.max 2
pop local 7
push local 8
push constant 511
call Math.min 2
pop local 8
push local 7
push constant 16
call Math.divide 2
pop local 1
push local 7
push local 1
push constant 16
call Math.multiply 2
sub
pop local 9
push local 8
push constant 16
call Math.divide 2
pop local 2
push local 8
push local 2
push constant 16
call Math.multiply 2
sub
pop local 10
push local 9
push static 0
add
pop pointer 1
push that 0
push constant 1
sub
not
pop local 5
push local 10
push constant 1
add
push static 0
add
pop pointer 1
push that 0
push constant 1
sub
pop local 4
push argument 0
push constant 32
call Math.multiply 2
push local 1
add
pop local 0
push local 2
push local 1
sub
pop local 6
push local 0
push local 6
add
pop local 3
push local 6
push constant 0
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push local 0
push local 4
push local 5
and
call Screen.updateLocation 2
pop temp 0
goto IF_END1
label IF_FALSE1
push local 0
push local 5
call Screen.updateLocation 2
pop temp 0
push local 0
push constant 1
add
pop local 0
label WHILE_EXP0
push local 0
push local 3
lt
not
if-goto WHILE_END0
push local 0
push constant 1
neg
call Screen.updateLocation 2
pop temp 0
push local 0
push constant 1
add
pop local 0
goto WHILE_EXP0
label WHILE_END0
push local 3
push local 4
call Screen.updateLocation 2
pop temp 0
label IF_END1
label IF_FALSE0
push constant 0
return
function Screen.drawSymetric 0
push argument 1
push argument 3
sub
push argument 0
push argument 2
add
push argument 0
push argument 2
sub
call Screen.drawHorizontal 3
pop temp 0
push argument 1
push argument 3
add
push argument 0
push argument 2
add
push argument 0
push argument 2
sub
call Screen.drawHorizontal 3
pop temp 0
push argument 1
push argument 2
sub
push argument 0
push argument 3
sub
push argument 0
push argument 3
add
call Screen.drawHorizontal 3
pop temp 0
push argument 1
push argument 2
add
push argument 0
push argument 3
sub
push argument 0
push argument 3
add
call Screen.drawHorizontal 3
pop temp 0
push constant 0
return
function Screen.drawCircle 3
push argument 0
push constant 0
lt
push argument 0
push constant 511
gt
or
push argument 1
push constant 0
lt
or
push argument 1
push constant 255
gt
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 12
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 0
push argument 2
sub
push constant 0
lt
push argument 0
push argument 2
add
push constant 511
gt
or
push argument 1
push argument 2
sub
push constant 0
lt
or
push argument 1
push argument 2
add
push constant 255
gt
or
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push constant 13
call Sys.error 1
pop temp 0
label IF_FALSE1
push argument 2
pop local 1
push constant 1
push argument 2
sub
pop local 2
push argument 0
push argument 1
push local 0
push local 1
call Screen.drawSymetric 4
pop temp 0
label WHILE_EXP0
push local 1
push local 0
gt
not
if-goto WHILE_END0
push local 2
push constant 0
lt
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push local 2
push constant 2
push local 0
call Math.multiply 2
add
push constant 3
add
pop local 2
goto IF_END2
label IF_FALSE2
push local 2
push constant 2
push local 0
push local 1
sub
call Math.multiply 2
add
push constant 5
add
pop local 2
push local 1
push constant 1
sub
pop local 1
label IF_END2
push local 0
push constant 1
add
pop local 0
push argument 0
push argument 1
push local 0
push local 1
call Screen.drawSymetric 4
pop temp 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function String.new 0
push constant 3
call Memory.alloc 1
pop pointer 0
push argument 0
push constant 0
lt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 14
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 0
push constant 0
gt
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push argument 0
call Array.new 1
pop this 1
label IF_FALSE1
push argument 0
pop this 0
push constant 0
pop this 2
push pointer 0
return
function String.dispose 0
push argument 0
pop pointer 0
push this 0
push constant 0
gt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push this 1
call Array.dispose 1
pop temp 0
label IF_FALSE0
push pointer 0
call Memory.deAlloc 1
pop temp 0
push constant 0
return
function String.length 0
push argument 0
pop pointer 0
push this 2
return
function String.charAt 0
push argument 0
pop pointer 0
push argument 1
push constant 0
lt
push argument 1
push this 2
gt
or
push argument 1
push this 2
eq
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 15
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 1
push this 1
add
pop pointer 1
push that 0
return
function String.setCharAt 0
push argument 0
pop pointer 0
push argument 1
push constant 0
lt
push argument 1
push this 2
gt
or
push argument 1
push this 2
eq
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 16
call Sys.error 1
pop temp 0
label IF_FALSE0
push argument 1
push this 1
add
push argument 2
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 0
return
function String.appendChar 0
push argument 0
pop pointer 0
push this 2
push this 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 17
call Sys.error 1
pop temp 0
label IF_FALSE0
push this 2
push this 1
add
push argument 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push this 2
push constant 1
add
pop this 2
push pointer 0
return
function String.eraseLastChar 0
push argument 0
pop pointer 0
push this 2
push constant 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 18
call Sys.error 1
pop temp 0
label IF_FALSE0
push this 2
push constant 1
sub
pop this 2
push constant 0
return
function String.intValue 5
push argument 0
pop pointer 0
push this 2
push constant 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 0
return
label IF_FALSE0
push constant 0
not
pop local 3
push constant 0
push this 1
add
pop pointer 1
push that 0
push constant 45
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push constant 0
not
pop local 4
push constant 1
pop local 0
label IF_FALSE1
label WHILE_EXP0
push local 0
push this 2
lt
push local 3
and
not
if-goto WHILE_END0
push local 0
push this 1
add
pop pointer 1
push that 0
push constant 48
sub
pop local 2
push local 2
push constant 0
lt
push local 2
push constant 9
gt
or
not
pop local 3
push local 3
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push local 1
push constant 10
call Math.multiply 2
push local 2
add
pop local 1
push local 0
push constant 1
add
pop local 0
label IF_FALSE2
goto WHILE_EXP0
label WHILE_END0
push local 4
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
push local 1
neg
pop local 1
label IF_FALSE3
push local 1
return
function String.setInt 4
push argument 0
pop pointer 0
push this 0
push constant 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 19
call Sys.error 1
pop temp 0
label IF_FALSE0
push constant 6
call Array.new 1
pop local 2
push argument 1
push constant 0
lt
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push constant 0
not
pop local 3
push argument 1
neg
pop argument 1
label IF_FALSE1
push argument 1
pop local 1
label WHILE_EXP0
push local 1
push constant 0
gt
not
if-goto WHILE_END0
push argument 1
push constant 10
call Math.divide 2
pop local 1
push local 0
push local 2
add
push constant 48
push argument 1
push local 1
push constant 10
call Math.multiply 2
sub
add
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 1
add
pop local 0
push local 1
pop argument 1
goto WHILE_EXP0
label WHILE_END0
push local 3
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push local 0
push local 2
add
push constant 45
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 0
push constant 1
add
pop local 0
label IF_FALSE2
push this 0
push local 0
lt
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
push constant 19
call Sys.error 1
pop temp 0
label IF_FALSE3
push local 0
push constant 0
eq
if-goto IF_TRUE4
goto IF_FALSE4
label IF_TRUE4
push constant 0
push this 1
add
push constant 48
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 1
pop this 2
goto IF_END4
label IF_FALSE4
push constant 0
pop this 2
label WHILE_EXP1
push this 2
push local 0
lt
not
if-goto WHILE_END1
push this 2
push this 1
add
push local 0
push this 2
push constant 1
add
sub
push local 2
add
pop pointer 1
push that 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
push this 2
push constant 1
add
pop this 2
goto WHILE_EXP1
label WHILE_END1
label IF_END4
push local 2
call Array.dispose 1
pop temp 0
push constant 0
return
function String.newLine 0
push constant 128
return
function String.backSpace 0
push constant 129
return
function String.doubleQuote 0
push constant 34
return
function Sys.init 0
call Memory.init 0
pop temp 0
call Math.init 0
pop temp 0
call Screen.init 0
pop temp 0
call Output.init 0
pop temp 0
call Keyboard.init 0
pop temp 0
call Main.main 0
pop temp 0
call Sys.halt 0
pop temp 0
push constant 0
return
function Sys.halt 0
label WHILE_EXP0
push constant 0
not
not
if-goto WHILE_END0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Sys.wait 1
push argument 0
push constant 0
lt
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 1
call Sys.error 1
pop temp 0
label IF_FALSE0
label WHILE_EXP0
push argument 0
push constant 0
gt
not
if-goto WHILE_END0
push constant 50
pop local 0
label WHILE_EXP1
push local 0
push constant 0
gt
not
if-goto WHILE_END1
push local 0
push constant 1
sub
pop local 0
goto WHILE_EXP1
label WHILE_END1
push argument 0
push constant 1
sub
pop argument 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function Sys.error 0
push constant 3
call String.new 1
push constant 69
call String.appendChar 2
push constant 82
call String.appendChar 2
push constant 82
call String.appendChar 2
call Output.printString 1
pop temp 0
push argument 0
call Output.printInt 1
pop temp 0
call Sys.halt 0
pop temp 0
push constant 0
return
";
