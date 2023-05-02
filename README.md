# ccelm (Concurrent Candidate ELiMination)

An implementation of the candidate elimination algorithm which can be run across multiple cores. Written in rust for speed and fearless concurrency

## What is Candidate Elimination?

Candidate Elimination is a type of version-space learning - an older approach to machine learning, originally introduced in 1977 by Tom Mitchell. It involves finding the most specific and most general hypotheses that satisfy all of the training examples that the algorithm has been shown, where each hypothesis is a [logical sentence](https://en.wikipedia.org/wiki/Sentence_(logic)).

The algorithm is rarely seen nowadays, primarily because of its lack of noise resistance. Just one incorrectly labeled training example will cause the algorithm to converge incorrectly - a problem that more modern machine learning methods such as [Neural Networks](https://en.wikipedia.org/wiki/Neural_network).

### Why Use Candidate Elimination Over Other Methods (Such as Neural Networks)

The main advantage candidate elimination has over other more modern approaches is that the output is easily interpretable. It's near impossible to figure out what concepts a neural network (or another black-box approach) has learned through looking at just its weights. However, looking at the hypotheses in the specific and general boundary produced by the algorithm, it is quite easy to see what constraints the target concept has.

<a title="Dfass, Public domain, via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File:Version_space.png"><img alt="Version space" src="https://upload.wikimedia.org/wikipedia/commons/3/33/Version_space.png"></a>

## Installing the Tool

The tool can be installed using [cargo](https://doc.rust-lang.org/cargo/), the package manager installed as part of the rust toolchain.

Since the tool isn't currently published to [crates.io](https://crates.io), the easiest way to get it running is to `cd` into a cloned version of the repository and run:

```bash
cargo install --path .
```

Pre-built binaries are also currently unavailable.

## Included Data

This repository includes data adapted from the paper [At the Boundaries of Syntactic Prehistory](https://royalsocietypublishing.org/doi/10.1098/rstb.2020.0197). The original data can be found [on GitHub](https://github.com/AndreaCeolin/Boundaries).
