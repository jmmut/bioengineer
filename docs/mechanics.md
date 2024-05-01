# The design goal

I think I want to make this game about managing dynamic cycles.

So you would produce some resource, and some underproduction hiccup would produce underproduction in
some other resource(s).

## Simplest mechanic

Build solar panels -> power air cleaners -> clean X Liters of air

Here there's only 2 resources: energy and air. Building too many air cleaners will make you run out
of energy, so all cleaners will stop.

## Storage

Building a machine has a cost of 100 Kg, and each wall of rock weights 10 000 Kg.

You start with 400 Kg, so you can build 4 machines on the surface. You could dig and build a machine
underground, but you have nowhere to put the dirt and rock you dug, because the machine network
can store 1 000 Kg per machine, except for the storage machine, which can store 9 900 Kg.

This means you can build an underground storage, increasing the storage capacity in 9 900 Kg, and
the available resources by the same amount. Building a storage machine on the surface costs 100 Kg
too. Building one underground storage gives you material to build 99 machines, but the main purpose
of storage machines is to be able to dig and put the material somewhere else, probably above the
ground, or in the water.

## Adding more mechanics

For example, I could do that the air cleaners produce toxic waste, which is another power source,
and that solar panels are not enough for other types of mechanics.

I could also allow starting with other mechanic than air cleaners, like water cleaning, that
produces the same toxic waste, so you can choose the order of development.

I could do fluid simulation with air, so you need to pump dirty air into the chambers with air
cleaners and pump the clean air out.
