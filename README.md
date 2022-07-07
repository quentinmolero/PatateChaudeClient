# Patate chaude client

---

## Groupe 1

- Quentin MOLERO
- Remy MACHAVOINE
- Florian GUISTI
- Cédric LEPROHON

## Who did what ?

- Cédric LEPROHON
  - HashCash challenge with tests
  - Bonus : advanced technique for performance : (Mutli-threading for performance)
- Florian GUISTI
  - Server part with tests 40% (Mob Programming most of the time)
  - Bonus: continuous integration : (realized with Github-action)
- Quentin MOLERO
  - Client part with tests
  - Server part with tests 60% (Mob Programming most of the time)
  - refactoring for better code quality
  - Bonus: Strategy to design next target on the server
- Remy MACHAVOINE
  - RecoverSecret, complexity 0 to 16 with tests
  - Bonus: Monstrous Maze plus tests with and without monsters (despite the server not sending monsters, the algorithm is verified through tests)
  - Bonus: Strategy to design next target on the server

## work organization

we made the choice to split the work from the beginning and that therefore two people would not work on the network part in order to advance as fast as possible on the whole project.
We also created a discord chat to organize the work and meetings which allowed us to follow the work of everyone in real time.

## Specificity

A specificity to our subject is that we decided to do the monstrous maze challenge even if it was not required.

## Completed bonuses

- Strategy to design next target on the server
- monstrous maze with tests
- Continuous integration with github-action
- Advanced technique for performance
- Server part with tests

## Challenges achieved

[x] HashCash :white_check_mark:

[x] RecoverSecret :white_check_mark:

[x] MonstrousMaze :white_check_mark:

# Getting Started

## compile for debug

```bash
cargo build
```

## compile for release (much more performance)


```bash
cargo build --release
```

## Options available

- `-h` : Display help message
- `-i` : Set the ip of the server, default is `127.0.0.1`
- `-p` : Port of the server, default value is `7878`
- `-u` : Choose a player username, default is `Player + actual timestamp`
- `-V` : Version of the client

