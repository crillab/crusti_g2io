# crusti_g2io: a Graph Generator following an Inner/Outer pattern

This app is dedicated to the generation of directed graphs where nodes form communities, with links between these communities.

To generate a graph, three parameters must be provided:
  - an _outer graph generator_, which produces a graph used to give the links between the communities;
  - an _inner graph generator_, which produces the communities;
  - a _linker_, which is responsible for the linking of communities, following the edges of the outer graph.

The app begins by generating the outer graph.
For each node in it, an inner graph is generated.
Then, for each (directed) edge in the outer graph, the linker is called to produce the necessary edges.

## Generators

The available graph generators can be list with the `crusti_g2io generators` command.
This command writes the generators and writes a description about them:

```text
me@machine:/home/me$ crusti_g2io generators
[...]
ba       A generator following the Barabási-Albert model, initialized by a star graph.
         First parameter gives the number of nodes of the graph, while the second one gives the number of nodes of the initial star graph.
[...]
```

Most generators have parameters.
In order to associate parameters to a generator, a slash is added after its name, and the parameters are given as a comma separated list after the slash.
For example, to get a Barabási-Albert graph generators to get graphs with 100 nodes, starting with a star graph of 5 nodes, the generator is `ba/100,5`.
If a generator has no parameters, the slash is optional.

## Linkers

Just like the generators, the linkers can be listed with `crusti_g2io linkers` command.
Linkers that admit parameters must be built the same way than generators.

## Generating graphs

To generate a graph, the user must provide to the `generate` both inner and outer generators, in addition to the linker:

```text
me@machine:/home/me$ crusti_g2io generate --inner ba/100,5 --outer chain/5 --linker f2f
[...]
```

By default, the output is formatted using the graphviz's dot format.
The format can be changed with the `-f` option.
Run `crusti_g2io generate -h` to get the list of available values for this option.
