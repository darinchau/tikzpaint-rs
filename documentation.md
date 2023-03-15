Here is a high-level rough sketch of how the app works:

- Every graphics manipulation language/software has objects to be drawn and a canvas where objects can be drawn.
- In the figures module, the Figure struct is an abstraction of said canvas, and Figure Objects are abstractions of the objects.
- A figure object can be said to be composed in 3 parts: Drawables -> Figure Objects -> Plottables

- Drawables are high level abstraction of Figure objects. We insulate them from migration hell and provide easy-to-use combinations
- of things to draw.

- Figure objects handle the core logic of the plot. For instance, they are responsible for handling plot options (e.g. fill color of line) and projections

- Plottables are the final layer which is responsible of generating the tikz code/svg code etc.