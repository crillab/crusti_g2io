# crusti_g2io: a Graph Generator following an Inner/Outer pattern

This app is dedicated to the generation of directed graphs where nodes form communities, with links between these communities.

To generate a graph, three parameters must be provided:
  - an _outer graph generator_, which produces a graph used to give the links between the communities;
  - an _inner graph generator_, which produces the communities;
  - a _linker_, which is responsible for the linking of communities, following the edges of the outer graph.

The app begins by generating the outer graph.
For each node in it, an inner graph is generated.
Then, for each (directed) edge in the outer graph, the linker is called to produce the necessary edges.

## Generators and linkers

The available graph generators can be list with the `crusti_g2io generators-undirected` and `crusti_g2io generators-directed` command.
This command writes the generators and a description about them:

```text
me@machine:/home/me$ crusti_g2io generators-undirected
[...]
ba       A generator following the Barabási-Albert model, initialized by a star graph.
         First parameter gives the number of nodes of the graph, while the second one gives the number of nodes of the initial star graph.
[...]
```

Most generators have parameters, described in their description.
In order to associate parameters to a generator, a slash is added after its name, and the parameters are given as a comma separated list after the slash.
For example, to get a Barabási-Albert graph generators to get graphs with 100 nodes, starting with a star graph of 5 nodes, the generator is `ba/100,5`.
If a generator has no parameters, the slash is optional.

Most generators are able to produce both directed and undirected graphs.
If you want more information on how directed graphs are created with generators that normally produce undirected graphs (and vice-versa), take a look at the API documentation.
This documentation can be generated from the sources by a call to `cargo doc`, and automatically opened in your browser by adding the `--open` flag.
If you have not `cargo` (the Rust package manager) installed on your machine, you can follow the instructions of [the Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Just like the generators, the linkers can be listed with `crusti_g2io linkers-undirected` and `crusti_g2io linkers-directed` command.
Linkers that admit parameters must be built the same way than generators.

## Generating graphs

To generate a graph, the user must provide to the `generate-undirected` or `generate-directed` command both inner and outer generators, in addition to the linker:

```text
me@machine:/home/me$ crusti_g2io generate-undirected --inner ba/100,5 --outer chain/5 --linker first
[...]
```

With no other options given in the CLI, the output mixes log messages and the generated graph.
There are two simple ways of getting rid of this behavior:

* add the `-x` (`--export`) option, followed by a file name: the file will be created/truncated and the graph will be stored into it;
* set the `--logging-level` to a restrictive value (eg. `warn`, `error`, `off`).

By default, the graph is formatted using the graphviz's dot format; it can be changed with the `-f` option.
Run `crusti_g2io generate-undirected -h` or `crusti_g2io generate-directed -h` to get the list of available values for this option.

## Reproducibility

By default, a random seed is chosen in a random fashion when a graph is built.
This seed is logged in the app message, allowing its further reuse, and used to fit the main pseudorandom number generator (PRNG). This main seed can also be set by the user using the `-s` option.

Although this app can take advantage of several processor cores, using the same seed multiple times insures the generated graph will be exactly the same. The achieve this, the outer graph generation is made in a sequential way, using the main PRNG. Then, this PRNG is used to sequentially associate a new seed to each node of the outer graph, making each parallel inner graph generation process having its own predictable PRNG. The same applies during the linking process: the main PRNG is sequentially used to generate a seed for each edge of the outer graph, allowing each parallel linking step to have its own predictable PRNG.

The processor cores management is let to the [rayon crate](https://crates.io/crates/rayon).

## Adding generators, linkers and output format

This app and its related library are built with the aim of being easily extended, by adding new generators and linkers. In the API documentation (`cargo doc --open`), see the documentation of the `generators`, `linkers` and `display` modules for more information.

## License

This software is developed at CRIL (Centre de Recherche en Informatique de Lens).
It is made available under the terms of the GNU GPLv3 license.
