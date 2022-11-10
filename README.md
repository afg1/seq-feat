# Seq-feat
Sequence features in Rust!

## What is it
This is primarily my project to teach myself how to do rust good, but also hopefully useful to enable client-side sequence feature extraction for later search prefiltering in RNAcentral.

The methods I'm implementing are largely taken from the paper *MathFeature: feature extraction package for DNA, RNA and protein sequences based on mathematical descriptors* (https://doi.org/10.1093/bib/bbab434) and the accompanying github repo https://github.com/Bonidia/MathFeature

## Why rust
I like rust, and I want to learn how to use it better, so implementing a reasonably sized project in it is a good way to do that.

Rust has the advantage of being fast and easily targeting wasm, so I can develop some blazing fast tools that can run *client side*. 

Also, coming from a C++ background, and the nightmare of getting CMake to find all my dependencies (other build systems are available), being able to just `cargo build` and have it figure everything out is very refreshing.

