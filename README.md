## About This Repository

This repository is based on the **Nand2Tetris Part One** course. It contains my full implementation of the "Hack" computer - starting from the NAND gate all the way up to a fully functioning machine.

### Why Am I Doing This?

I've always been extremely curious about how computers actually work on the physical level. This question has been on my mind for a long time, even before I started studying computer science.

To find the answer, I studied Computer Architecture and Organization from different sources. And when the chance finally came to find the answer, it was a truly remarkable moment for me - it made me elated to understand how people managed to convert some small vacuum tubes into a machine you can program. That was really fascinating.

After gaining a solid understanding of this interesting topic and reaching a point where I could start building a computer myself, I decided to take the Nand2Tetris course to learn even more and apply my knowledge in a practical way.

### How?

The computer is all about evaluating a set of Boolean functions, implemented in an organized way. These functions are evaluated inside the computer according to the organization chosen by the designers. Boolean functions are computed using combinational and sequential circuits, where combinational circuits evaluate functions based purely on the current inputs. Every Boolean function can be expressed using at least one Boolean expression called the canonical representation (The Elements of Computing Systems, p.9). In other words, any Boolean function can be represented using the basic gates: And, Or, and Not. In fact, the number of Boolean functions that can be defined over n binary variables is 2^(2^n). From this fact, the NAND gate can be used to construct the basic gates (And, Or, and Not) This is why we call the NAND gate a universal gate: a gate that can implement any other gate so any boolean function in the world. Since every Boolean function has its canonical form, the NAND gate can be used to implement any Boolean function. The same goes for the NOR gate. So, actually we could shift all of our work in this repository to implement a computer using only NOR gates and this is possible. As long as we use a universal gate, we can implement anything. And in our case, that “anything” is the computer **a set of Boolean functions organized together**.

