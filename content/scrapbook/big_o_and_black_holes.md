+++
title = "Big O And Black Holes"
date = 2025-01-01

[extra]
author = "Nicole Venner"
+++

# It is impossible to build a computer that can do array lookups in less then $cal(O)(sqrt(N))$

### Stage 1: Shouldn't an array lookup be $cal(O)(1)$?

BIRD OBJECTION: This doesnt make sense, if I compile this code:

```rs
#[no_mangle]
fn main() {
  // Create a vector with 100_000 instances of the number 42
  let test_vec = vec![42_isize, 100_000];

  // Get the value at the 42069th position
  let value = test_vec[42069];

  // Print the number out
  println!("{}",value);
}
```

and check how it decompiles on my x86 machine:

```bash
main:
        push    rax
        mov     rax, qword ptr [rip + __rust_no_alloc_shim_is_unstable@GOTPCREL]
        movzx   eax, byte ptr [rax]
        lea     rdx, [rip + .L__unnamed_1]
        mov     edi, 42069
        mov     esi, 2
        call    qword ptr [rip + core::panicking::panic_bounds_check::hc5c09b1b32c09393@GOTPCREL]

.L__unnamed_2:
        .ascii  "/app/example.rs"

.L__unnamed_1:
        .quad   .L__unnamed_2
        .asciz  "\017\000\000\000\000\000\000\000\007\000\000\000\027\000\000"
```

We can see that the main operation that indexes into the list just consists of a single assembly operation `mov edi, 42069`, that is going to take constant time. So what gives???

### Stage 2: Definition of Big-$cal(O)$ and where it breaks down.

A lot of Big-$cal(O)$'s weird and confusing behavior comes from the fact that it doesn't really measure "performance of an algorithm as its input grows" so much as it measures "performance of an algorithm at infinite sized inputs"[1]. Mainly because if we can define big $cal(O)$ as an equivalence relation.

<!-- $$ -->
<!-- cal(O)(A) = cal(O)(B) upright(  if and only if  )\lim_{n arrow \infty} \frac{upright(Runtime)(A(n))}{upright(Runtime)(B(n))}= upright(a nonzero real number) -->
<!-- $$ -->

Not only does this mean that any algorithm's performance below any fixed input size, like $100^(100^100)$, or $op("Tree")(3)$ has no impact on the Big-$cal(O)$, but it isnt even possible to define on programs that dont infinitely scale.

This is where our program above runs into its first limitation. Because on a regular 64 bit compilation target you only have access to $2^64$ memory addresses. And in general on any computer accessing a location in a list, will at least require parsing the address. Which should always take $log_2(N)$ bits to store. Thus making our algorithm have a complexity of at least $cal(O)(log(N))$

[1]: Under a nonstandard analysis framework (calculus but with values that are infinitely large $(omega)$ or infinitely small $(epsilon)$, this statement is literally true, where an algorithm $A$ is in $cal(O)(sqrt(n)$ if and only if the runtime of $A$ with an input of size $omega$ is some real multiple of $sqrt(omega)$.

### Stage 3:
