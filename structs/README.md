### Structs:

A *struct*, or *structure*, is a custom data type that lets you pacakge together and name multiple related values that make up a meaningful group.
If you'sre familiar with an object-oriented language, a *struct* is like an object's data atttribute.

### Methods:

*Methods* are similar to functions: we declare them with teh `fn` keyword and a name, they can have parameters and a return value,
and they contain some code that's run when the method is called from somewhere else.
Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always `self`,
which represents the instance of the struct the method is beign called on.
