# Kicad Generator

> [!WARNING]
> This project is still in development.\
> It is intended for my personal needs and may not be useful for anyone else.\
> It might also not work properly with your installation or the kicad library you might be using.\
> Still I will happily accept any pull requests and or comments about it to make\
> it better and/or usable to needs I didn't consider.

A kicad file generator written in rust.

the goal of this library is to be able to programmatically represent circuits and then generate the kicad files for them

## TodoList

- [x] Read symbols library files
- [x] Place a symbol on a schematic
- [x] Write schematic to a file
- [ ] Be able to connect wire to specific pins of a symbolInstance
- [ ] Be able to manipulate higher level building blocks (e.g. Monostable/Astable 555 timer with parametric delays)
- [ ] Generate a starting PCB layout for that schematic
- [ ] Write footprint to a file
- [ ] Given more high-level building blocks determinate ideal$^*$ values of resistors and capacitors to use

\* "ideal" given my needs at the moment and/or parametric needs like minimizing the number of different unique components used.