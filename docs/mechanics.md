# The design goal

I think I want to make this game about managing dynamic cycles.

So you would produce some resource, and some underproduction hiccup would produce underproduction in
some other resource(s).

## Simplest mechanic

Build solar panels -> power air cleaners -> clean X Liters of air

Here there's only 2 resources: energy and air. Building too many air cleaners will make you run out
of energy, so all cleaners will stop.

## Adding more mechanics

For example, I could do that the air cleaners produce toxic waste, which is another power source,
and that solar panels are not enough for other types of mechanics.

I could also allow starting with other mechanic than air cleaners, like water cleaning, that
produces the same toxic waste, so you can choose the order of development.

I could do fluid simulation with air, so you need to pump dirty air into the chambers with air
cleaners and pump the clean air out.
