RaumEn
======

An OpenGL engine written in Rust.

Status
------

Still learning.  Opens a window ~~and draws a triangle that moves.
Working on an implementation of Vector and Matrix classes from LWJGL with my own modifications.~~

..and draws a shape from a .obj file!  ~~Next, shading!~~  Camera and specular lighting working!

Mob support started.

Experience writing it in Rust
-----------------------------

Initially I was having trouble translating my understanding of OpenGL from the way it's done in LWJGL to the way it's done in Glium.  I feel like I've made it over the first hurdle, there.  Glium, while there's a lot to learn, still, does the hard work for you so that you're not manually managing states of things.  You don't have to turn something on before a draw call and turn it back off when you're done, you just tell each draw call what settings it needs to use.  That is my current understanding, anyway.

And then there's rust itself.  The compiler demands pristine code.  It can't enforce good logic, but it can force you to write your bad logic safely.  The design of the language forces you to write memory safe code (provided you don't specifically tell it you want a chunk of code to do something unsafe) in which the lifetime/scope of variables is so well defined that there isn't a need for garbage collection, manual or otherwise.  You have to work harder to get the compiler to let you do things with variables that leave data in an ambiguous state.  The compiler will complain and complain until it knows exactly how long each variable will live, and a part of that is the ownership and borrowing concept which keeps you from using what are more or less the equivalent to pointers in C/C++ in an irresponsible way.  It makes you put your toys away or you just don't get to play with them.  My grasp of Rust isn't perfect, so if what I've said here is wrong or misguided, that's why.

And now for my interpretation of various languages :)

Ruby says "Do whatever you want, however you want, it'll probably work.  You're not writing anything too serious, are you?"

Python says "You can do anything as long as you do it the one right way.  Go Science!"

Java says "Whatever you want to do, describe it in great detail.  Seriously, I do not want to have to guess about anything."

C says "I'm going to go ahead and compile this, but you'd better know what you're doing or this isn't going to end well."

C++ says what C says, but it adds "Let's get dangerous :)"

Haskell says "You can't just leave this information lying around! You either pass it to another function or you get rid of it.  If you want to do something, you're going to have to figure out how to do it without storing anything in variables.  Variables are how mistakes are made!  Just do things in the right order and you won't need them!"

Rust says "Fix that, ok now that, this too, because until this code is neat and tidy we are not going to Disney Land.  You are going to love Disney Land, but seriously, fix this and this.  See, doesn't it feel better to have a clean house?  *You're welcome.*"
