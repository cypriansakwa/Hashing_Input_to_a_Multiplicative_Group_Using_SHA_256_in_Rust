## Overview
This Rust program demonstrates how to hash an input into a multiplicative group modulo a prime number using SHA-256 as the hash function. The hashed value is reduced to fit within the range of valid elements of the multiplicative group, i.e., between 
$1$ and $p-1$, where $p$ is the prime number.
## How it Works
- **SHA-256 Hashing:**
   - The input is hashed using the SHA-256 hash function to produce a 32-byte (256-bit) output.
- **Modular Reduction:**
   - The resulting hash is translated to a big integer, which is reduced modulo $p-1$, where $p$ is a huge prime. This assures that the output is a legitimate element in the multiplicative group $\mathbb{Z}_n^\*$, which consists of numbers from $1$ to $p-1$ that are coprime with $p$.
- **Offset:**
  - To ensure that the result is not zero, $1$ is added after executing the modulo operation, ensuring that the group element lies in the range $[1,p−1]$.
## Steps:
1. **Input Hashing:**
   - The input data is hashed using SHA-256, and the resulting hash is converted from hexadecimal format into a large integer.
2. **Modular Arithmetic:**
   - The hash is reduced modulo $p−1$, where $p$ is the provided prime number, ensuring the result fits within the range of valid elements in the multiplicative group $\mathbb{Z}_n^\*$.
3. **Result:**
   - The program prints the resulting group element.
