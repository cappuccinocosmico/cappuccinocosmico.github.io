+++
title = "Intro To Quantum Weirdness"
date = 2025-01-01

[extra]
author = "Nicole Venner"
+++

# What is going on with all this "Quantum Stuff"

Initially started when I was doing some research on interpretations of quantum mechanics and found myself struggling to wrap my head around some of the basic math and differences between the theories. But when I tried to learn more, I found that pretty much all resources on QM boil down into:

1. Physics majors needing to learn the basics for other more advanced stuff.

2. Mathemeticans and Computer Scientists wanting to design algorithms for quantum computers.

But not a lot for a third group

3. People with a technical background (Engineers, Chemists, Mathematicians, Logicians) with a passing interest in the philosophical implications of QM.

And this aims to be an easy primer which aims to teach all the math needed to understand the axioms of QM, whats really weird about it, and what all the different interpretations of QM mean.

This does mean that we are glancing over an absolute ton of detail, if you find yourself confused by anything I would suggest looking through this incredible set of lectures on quantum information science and finding the lecture on the particular topic:

# What is QM??


Its possible to decompose the math of how quantum systems work into multiple layers.

<!-- https://q.uiver.app/#q=WzAsNCxbMCwzLCJcXHRleHR7UXVhbnR1bSBNZWNoYW5pY3N9Il0sWzAsMiwiXFx0ZXh0e1F1YW50dW0gRWxlY3Ryb2R5bmFtaWNzfSJdLFswLDEsIlxcdGV4dHtRdWFudHVtIENocm9tb2R5bmFtaWNzfSJdLFswLDAsIlxcdGV4dHtTdGFuZGFyZCBNb2RlbCBvZiBQYXJ0aWNsZSBQaHlzaWNzfSJdLFsxLDJdLFsyLDNdLFswLDFdXQ== -->
<iframe class="quiver-embed" src="https://q.uiver.app/#q=WzAsNCxbMCwzLCJcXHRleHR7UXVhbnR1bSBNZWNoYW5pY3N9Il0sWzAsMiwiXFx0ZXh0e1F1YW50dW0gRWxlY3Ryb2R5bmFtaWNzfSJdLFswLDEsIlxcdGV4dHtRdWFudHVtIENocm9tb2R5bmFtaWNzfSJdLFswLDAsIlxcdGV4dHtTdGFuZGFyZCBNb2RlbCBvZiBQYXJ0aWNsZSBQaHlzaWNzfSJdLFsxLDJdLFsyLDNdLFswLDFdXQ==&embed" width="590" height="560" style="border-radius: 8px; border: none;"></iframe>

and all of the philosophical implications generally exist in the base layer, which doesnt involve any fields, particles, forces or any other interactions. All it discusses is "states". So we can make the simplifying assumption that all our states are built from combinations of arbitrary states labeled $| 0 \rangle$ and $| 1 \rangle$. What exactly the underlying states are is immaterial, all that matters for our purposes is that under a classical view they are mutually exclusive. They could be 
- $| 0 \rangle$ could be a photon thats vertically polarized and $| 1 \rangle$ could be that same photon horizontally polarized
- $| 0 \rangle$ could be a hydrogen atom in the ground state, and $| 1 \rangle$ could be that same atom in the first excited state.
- $| 0 \rangle$ could be an electron that has spin aligned with an external magnetic field, and $| 1 \rangle$ could be the same election pointing opposite the field.

In fact all the setups we talk about have been experimentally validated width all three of these examples.


# Classical Mixed States 


