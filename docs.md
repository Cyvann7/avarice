<!-- omit in toc -->
# **Avarice**

<!-- omit in toc -->
## **Contents**
- [**Introduction**](#introduction)
- [**Order of Operations**](#order-of-operations)
- [**Loops**](#loops)
- [**Operations**](#operations)
		- [**Mode: STD**](#mode-std)
		- [**Mode: Stack**](#mode-stack)
		- [**Mode: Arithmetic**](#mode-arithmetic)
- [**Return Codes**](#return-codes)

<br>

___
## **Introduction**

***Avarice: Extreme greed for wealth or material gain.***

Welcome to Avarice. Avarice is a simple stack-based esolang I made for fun. It works in a 2d plane, starting in the top left then moving in all available directions in a breadth first manner, pushing each character to a queue, then popping it later.

All programs are interpreted by [Rust](rust-lang.org/). They are interpreted by a function that returns an integer. See the [Return Codes](#return-codes) section for more.


**Lets go over some examples**

```Avarice
S---T
```

The top left of any Avarice program is denoted with an 'S' for 'Start'. It is computationally the **exact** same as a '+'. It simply connects to the chars below it and to it's right.

The 'T' stands for 'Terminate'. 'T' will end the program and return 1.

It can help if you imagine the program like water. It flows out from S and outwards in all directions.

The '-' ("Dash") just lets the "water" flow through it. It's Vertical counterpart is '|' ("Pipe")

This program does nothing. The water flows from S to T in a straight line. Rather boring, really. So let's do something more interesting - printing a letter!

```Avarice
S--#65&#--P--T
```

Scary new operators! But first, let's talk about "Modes". Avarice operations are divided into *"modes"* that change what a characters does based on which mode you are in. Here, we use the '#' to enter **Stack** mode. In stack mode, each digit you type is pushed into a number, then when a '&' ("Ampersand") symbol is encountered, it pushes the complete number into the stack.

Note that since the program can progress in any order, the number itself can also be in any [order](#order-of-operations), that is:

```Avarice
S        +-#
#        | 1
12    == S 2
 3&#      &3# <- note that the ampersand is hit first here as there is an order that the items are popped from the opqueue
```

In this case, we first see S going into a #. We are now in **Stack** mode.We then see a 6. now we have the number, "6". Then theres a 5. We haven't seen an ampersand yet, so the 5 is added to the end of the 6. We now have "65". Now here's the &. So we take that "65", make it the *number* 65 and throw it onto the stack. Finally we hit the 'P'. This is the PrintChar operator. It pops the stack and prints it as an ascii character. If we wanted to just print the number 65, we would use the small 'p' instead.

Just to end off this little introduction, lets take a look at the classical "Hello World!" program. It get's a little bit large, but don't fret!
```Avarice
S---+-------+-------+-------+-------+-------+-------+-------+-------+-------+-------+-------+
    |       |       |       |       |       |       |       |       |       |       |       |
    #       #       #       #       #       #       #       #       #       #       #       #
    7       6       7       7       7       3       8       7       8       7       6       3
    2       9       6       6       9       2       7       9       2       6       8       3
    &       &       &       &       &       &       &       &       &       &       &       &
    #       #       #       #       #       #       #       #       #       #       #       #
    P       P       P       P       P       P       P       P       P       P       P       P
                                                                                            T
```
Again, imagine the water flow. We start at S, then go right along until we hit a Cross. where the water splits into 2 paths, down and right. Since the water flows at the same rate both ways, and there are more operations along that top path than in the bottom path, we reach the P before we get to the next cross. This means that we sequentially do each bottom section, slowly moving along the right untill we hit that last path that has a T on the end. The numbers along it are just the ascii codes for "HELLO WORLD!". This way of laying out a program is the simplest to read, but definitely not the most efficient or compact. The most compact Hello World 	program is available in the examples directory.

___
## **Order of Operations**

The order that items are interpreted by within the flow is determined by the following order:

```Avarice
 1
2+3
 4
```

Proof of this order can be found [here](./src/examples/1234.avrc). Using this order to form programs can be exceptionally useful in shaping functions e.g.:
```Avarice
              1  
#1234&# ---> 2#3&#  
              4
```
This simple change goes from a 7x1 program to a 3x5 program. Here, it's likely not worth it, but for a better example of how much the order can compact a program, look at the [semi-compact Hello World!](./src/examples/hello_world_semi_compact.avrc) program against the [standard Hello World!](./src/examples/hello_world_readable.avrc) program.

___
## **Loops**
Using the R, Y, and ><^v operators, we can create conditional loops.
First, lets look at an infinte loop, then build up to a conditional loop.
```Avarice
SRR
```
Thats it. That's an infinite loop. The R operator clears the Visited HashSet and acts like a +, as all operators do. feeding back into the other R and vice versa. But what does it mean to clear the Visited HashSet?

Well, when a character is encountered in a program, it stores every character in what is essentially just a big list. Then whenever a character is encountered, we just check if it's in that list, and if it is, we ignore it. This is what prevents infinite loops from something as simple as "S++". So when we clear that list, we essentially "forget" where we've been before. It's important to note that 'R' does not forget itself, but does forget other 'R's.

So thats great. We can make an infinite loop that does nothing. Let's make it do something. How about simply printing "1" out.

```Avarice
S>R---#1&#--p-->R>--#1&0&#--&
```
Here it is, the whole program to infinitely print "1". We start at S, move right into R, which does nothing now as the only thing it forgets that it can access is the '>', which points back into 'R', which won't reset itself.

Then we use "#1&#" to push 1 into the stack, before immediately popping and printing using 'p'. Next we see a structure that shows up in most loops: ">R>". This is a way to have a reset without moving direction, as the R wont feed back into itself, as mentioned above. We use this reset to forget that first 'R' right at the begining.

Then we push 2 numbers into the stack. 1 and 0 go into the stack using **stack** mode. Finally, we encounter a new operator. The Normal Mode '&'. This pops the top 2 values off the stack (here, thats 1 and 0) then acts as an arrow to that location. This '&' could be replaced by a line of arrows, but this is much faster both to write, read and compute - and you wont always be able to reach that point directly.

So we go back to the coordinates (1,0). Since the program is 0-indexed, that means the first row, second character. The '>'. We immediately hit that R, and now we have forgotten that we've done this before, and are in the same state we were in at the start of this, now with one more "1" in stdout.

But we don't always want out program to go infinitely - sometimes we want to stop it.

___
## **Operations**

Note that some operations are available in multiple modes.

#### **Mode: STD**

|Char|Name|Operation|
|:-:|:-:|:-|
| S| *"Start"*| Same as **Cross**. Used to denote the start of the program.|
| T| *"Terminate"*| Ends the program, returning 1.|
| P| *"PrintChar"* | Pops the stack and prints the current value as an [ASCII](https://www.asciitable.com/asciifull.gif) character|
| p | *"Print"*| Pops the stack and prints as a number.|
| \|| *"Pipe"* | Pushes the character above and below onto the opqueue|
| - | *"Dash"* | Pushes the character to it's left and right onto the opqueue|
| + |*"Cross"* | Pushes all surrounding characters onto the opqueue|
| Y |*"Y-Gate"*| Acts like a **Dash**, but pops the stack then checks if it is equal to 0 or None. If it is, acts like a  **v Arrow** as well. This can be used to generate loops.|
| R | *"Reset"*| Sets the Visited HashSet to a blank HashSet. See the [Loops](#loops) section for more details.|
| i | *"Input"*| Asks for input in stdout, then pushes to the stack|
| & | *"GoTo"* | Pops the stack twice, then pushes the operation at that 2D Index to the opqueue|
| ^ <br><......><br>v |"Arrow"|Acts like a **Pipe** or **Dash**, but only works in one direction. This is only useful in situations where the **Reset** operation is involved, as it can prevent infinite loops. <br>Also useful as a way to force a set of operations to happen in a certain order.|
| M | *"Math"*| Enters [**Arithmetic**](#mode-arithmetic) mode. |
| # | *"Hash"* | Enters [**Stack**](#mode-stack) mode. |
___

#### **Mode: Stack**
Available from STD: +, -, |
|Char|Name|Operation|Representation|
|:-:|:-:|:-|:-:|
| D | *"Dupe"* | Duplicates the top value of the stack| [X] -> [X, X] |
| 0..9 | *"Numbers"* | Begins forming a number out of digits. | n" " -> n"X"
| & | *"Ampersand"*  | Finishes forming a number, and pushes the result onto the stack. This means that numbers can be pushed to the stack using `#[digits]&#`| n"X" + [ ]-> n"" + [X]|
| C | *"Clear"*| Removes everything from the stack, cancels the current forming number, then enters [**STD**](#mode-std) mode. |n"12" + [X,Y,Z] -> n" " + [ ] |
| # | *"Hash"* | Enters [**STD**](#mode-std) mode. | - |
___

#### **Mode: Arithmetic**
|Char|Name|Operation|Equation|
|:-:|:-:|:-|:-:|
| + | *"Plus"*  | Pops the stack twice and pushes the sum. |[5,15] -> [20]|
| - | *"Minus"* | Pops the stack twice and pushes the difference.|[15,5] -> [10]|
| * | *"Times"* | Pops the stack twice and pushes the product.| [5,15] -> [75]|
| / | *"Divide"*| Pops the stack twice and pushes the integer floored division.|[15,5] -> [03]|
| M | *"Math"*| Enters [**STD**](#mode-std) mode.| - |


## **Return Codes**
These are the codes returned by the interpret_program() function in Avarice's `main.rs`.
 
| Code  |  Reason |
| :---: | :------ |
| **0** | The program's queue was empty - no operation could be found. This means the program ended without hitting a 'T' |
| **1** | Program terminated by 'T'.|
| **2** | The Maximum number of operations was reached. The maximum number can be set in the config in `main.rs` and is by default 65536. This exists to stop infinite loops. |