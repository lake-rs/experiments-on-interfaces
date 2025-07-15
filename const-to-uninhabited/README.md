# Experiment question

Can a type use (const) generics to make an associated type possibly uninhabited?

# Motivation

See <../fallthrough-impls/> --
to gain some optimizations, it'll practical to have a choice of whether or not something is available at the type level.

# Outcome

Yes, it works.

The current implementation needs some very tame unsafe, but that should be fine,
and can still be improved on.
