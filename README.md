# chip-8-emulator
A simple chip-8 emulator written in Rust

## Usage
```
git clone https://github.com/Njord0/chip-8-emulator
cd chip-8-emulator
cargo run test/test.ch8
```
`test/test.ch8` can be replaced by any valid ch8 ROM
This is a simple rom that display "0123" to screen and then wait for a keypress

Chip-8-keypad become: (azerty layout)
|1|2|3|C|  |&|é|"|'|
|-|-|-|-|  |-|-|-|-|
|4|5|6|D|  |a|z|e|r|
|7|8|9|E|  |q|s|d|f|
|A|0|B|F|  |w|x|c|v|


## Pong game
[](images/pong.png)


## Tetris
[](images/tetris.png)


## TO-DO
* Implement audio
* ...

## Credits
I used this repo to download games:
https://github.com/loktar00/chip8
