# Experiment question

Given two implementations of an `embedded-cal` trait
that each only have some subset of the supported algorithms,
what could a wrapper look like that allows using an algorithm from either?

And how would that work with methods
that call other methods through the trait,
given that `self.` will always go to the own type
and not through some kind of inheritance style dispatch?

# Outcome

There's a sketch in <./src/lib.rs> that looks like it could work.

For composite methods,
it can not use the "composer" parts from the individual impl and then pass it components from another,
but the combinator can have a composer,
and that should be good enough.
(And it's not ruling out we can do even better still.)
