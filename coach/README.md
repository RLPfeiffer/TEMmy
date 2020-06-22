TEMmy Coach
===========

An interactive guide to running TEM captures for the MarcLab.

This file aims to explain all there is to know for modifying the Coach.

# Build

Run build.sh. It will generate a static site in a folder called `temmy`, which you can upload to any hosting service.

Building on Windows is supported only with a Bash emulator like msys2.

## Dependencies included

Most of the dependencies are included in this repository to keep their versions static.

### Inkjs

Inkjs is a re-implementation of [Ink](http:www.inklestudios.com/ink/), a scripting language for making choice-based games. The important parts of TEMmy are written in Ink because it is a simple language, and anyone should be able to update TEMmy's Ink logic by following this guide: [[Writing With Ink](http:github.com/inkle/ink/blob/master/Documentation/WritingWithInk.md).

The Inkjs source code is governed by the MIT License, and its license file is included in the boilerplate directory.

### Ink

TEMmy is built using Inklecate, the official Ink compiler. Ink is governed by the MIT License, and its license file is included in the inklecate directory.

## Separate Dependencies

On non-windows platforms, you need to install Mono for running inklecate.exe.

# Source

## HTML

Inkjs can run on a static HTML website. The HTML for TEMmy lives in <index.html> (which is a modified version of an Inkjs template file).

## JavaScript

TEMmy uses some JavaScript functions to manipulate the DOM (for instance, to display images for the tutorial. These functions are defined in <temmy.js>

## Ink

The code in <protocol.ink> contains *all* of TEMmy's internal logic for guiding a new electron microscope user. You should almost never have to edit anything outside of that file.