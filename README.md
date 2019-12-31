# vec(tor)-rac(ing)

This is a simulation of racers on a grid containing a racetrack with walls
beside it. Each turn, a racer can accelerate one unit in one of the cardinal
directions or can do nothing. There is no friction. I myself did not come up
with the game, which is traditionally played by hand with graph paper. My
version features a randomly generated, infinite track and racers driven by
artificial intelligence (a neural network). Racers improve through natural
selection. A racer is scored primarily on how far upward it gets and secondarily
on how long its race took.

## Installation

You can clone the repository and build the program with cargo, or you can
install the binary like this:

```
cargo install vec-rac
```

I suggest the latter method.

## Running the Simulation

The program makes use of all available cores to find better racers, but doing so
can still take a while; I suggest you run the program with optimizations on.
When a new best racer is found, a movie is played of its accomplishment. I have
not implemented saving of racers, so to record your progress, you should run the
program something like this:

```
asciinema rec -i 1 -c 'vec-rac ...'
```

Asciinema is a separate program. The above command runs the command after `-c`
and squashes pause times (when new racers are being generated) to at most one
second. You can rewatch the progression this way.
