# Experiment question

Can we sensibly have concrete algorithms (eg. 10 = AES-CCM-16-64-128) expressed on the type level,
such that high-level functions such as key generation take an algorithm as a generic parameter?

And then, can we still also support runtime chosen algorithms still?
(Because in many protocols, the choice of algorithms is negotiated).

(The alternative is to only have algorithms in values.)

# Outcome

At least with the approach taken here,
this would rely strongly on the highly unstable `generic_const_exprs`,
and even then it's a bit unwieldy in the signatures.

Working with the more crypto-agile example implementation in there
has shown that the runtime-only approach is not too bad:
If there's an impl that really can only do a single algorithm,
then its "which algorithm" type becomes zero-sized.
