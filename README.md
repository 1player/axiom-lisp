Mum said it was my turn to write a Lisp.

This is meant to be a very simple Lisp interpreter I can use as a starting point for more advanced implementations.

## Features

- [ ] language docs
- [ ] test suite
- [x] REPL
- [x] define
- [x] quote
- [x] car/cdr
- [x] basic integer functions
- [ ] equality
- [ ] basic boolean functions
- [ ] if
- [ ] closures
- [ ] quasiquote
- [ ] macros

## Gotchas

Lists are actually array.

I'm still debating whether it makes sense to turn them into linked lists. I like arrays better, but the prepending `cons` would be an unacceptable *O(n)* operation.
