## Overview
This Rust program demonstrates how to hash an input into a multiplicative group modulo a prime number using SHA-256 as the hash function. The hashed value is reduced to fit within the range of valid elements of the multiplicative group, i.e., between 
$1$ and $p-1$, where $p$ is the prime number.
## How it Works
- **SHA-256 Hashing:**
   - The input is hashed using the SHA-256 hash function to produce a 32-byte (256-bit) output.
- **Modular Reduction:**
   - The resulting hash is converted into a large integer, and this integer is reduced modulo $p-1$, where $p$ is a large prime. This ensures the output is a valid element in the multiplicative group $\mathbb{Z}_n^\*$​ , which consists of numbers from $1$ to $p−1$ that are coprime with $p$.
   - 
