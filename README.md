# Patate chaude client

---

- 

## Groupe 1

- Quentin MOLERO
- Remy MACHAVOINE
- Florian GUISTI
- Cédric LEPROHON

## Who do what ?

- Cédric LEPROHON
  - HashCash challenge with tests
  - Bonus : advanced technique for performance : (Mutli-threading for performance)
- Florian GUISTI
  - Server part with tests 40% (Mob Programming most of the time)
  - Bonus: continuous integration : (realised with Github-action)
- Quentin MOLERO
  - Client part with tests
  - Server part with tests 60% (Mob Programming most of the time)
  - refacto to pay attention to the quality of the code
- Remy MACHAVOINE
  - RecoverSecret, complexity 0 to 16 with tests
  - Bonus: MonstrousMaze with tests with monsters or not even if the server does not produce monsters (verify monsters scenario with tests)
  - Bonus: game strategy

## work organization

we made the choice to split the work from the beginning and that therefore two people would not work on the network part in order to advance as fast as possible on the whole project.
We also created a discord to organize the work and the meetings which allowed us to advance in this one

## Specificity

a specificity to our project was not asked in the subject but we decided to do it anyway, it was the monstrous maze challenge


## Completed bonuses

- game strategy
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

## compile for release (much more performatn)

```bash
cargo build --release
```

## Options available

- `-h` : Display help message
- `-i` : Set the ip of the server, default is `127.0.0.1`
- `-p` : Port of the server, default value is `7878`
- `-u` : Choose a player username, default is `Player + actual timestamp`
- `-V` : Version of the client

