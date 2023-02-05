Mum said it was my turn to implement a Lisp.

This is meant to be a very simple Lisp interpreter I can use as a starting point for more advanced implementations.

## Features

- [x] REPL
- [x] define
- [x] car/cdr
- [x] quote
- [ ] quasiquote
- [ ] basic integer functions
- [ ] if
- [ ] closures
- [ ] macros

## Gotchas

Lists are actually array.

I'm still debating whether it makes sense to turn them into linked lists. I like arrays better, but the prepending `cons` would be an unacceptable *O(n)* operation.
