RaumEn
======

An OpenGL engine written in Rust.

I started with glium but it's no longer being updated and at this point I have a better understanding of OpenGL than of glium's current state.  I tried vulkano but didn't make it very far.  I tried GFX but couldn't figure the basics out well enough and they're upcoming version looks to be very different so learning the old version then having to relearn the new version doesn't sound like fun.  So straight OpenGL it is! And I've made a lot of progress.  I'm not as far as I was in the version I was writing in Kotlin, but I hit a road block with reading and writing frame buffers.  I didn't want to go through the entire codebase to find out what I was doing wrong and I wanted to have a binary executable when all was said and done, so I started again in Rust.

Status
------

The "engine" is currently able to:
- load an obj file as a mesh
- load glsl from files instead of being hard coded
- load and use a texture
- ~~draw the mesh with or without a texture~~
- accept keyboard and mouse input
- move a Mob object with WASD keys
- Terrain! (A work in progress)

Near future ToDo:
- determine textures and objects to load from a data file
- improve Mob movement
- implement rudimentary animation

Future Plans:
RaumEn should be able to use config files and binary libraries to create the game experience.  It should be modular enough for FPS, RPG, and adventure type games, but who knows?  I have vague plans for a space shooter with side to side movement in which you're constantly moving forward, based on an old TI-99/4A game I used to play but can't remember the name of, as my first working game.

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
