# Basic Bump Allocator

## Learning Log

### Use of Cell and RefCell

We want to use our Arena multiple time and returns pointers that point to element in the Arena.
Therefore, the borrowchecker, will complain. The solution is to use interior mutability.
We use `RefCell` for `MmapMut` because we need to get references to the object. `MmapMut` is not `Copy`
and `Cell` for `offset` because this one is `Copy`.


### Using Raw Pointer

We allocate memory using the crate `memmap2`(`memmap` is not maintain anymore and `memmap2` is a maintained fork).


### Alignment

In order to optimize the use of our allocator by the CPU, we need to align our data in memory with the 64 bits memory bus. This is achieve by the following line
```rust
let new_offset = (self.offset.get() + align - 1) & !(align - 1);
```
The use of `&` is like a modulo. We want to start in a place where our new allocation won't be cut when the CPU reads the memory. We want to be sure it will bring as much data as possible in the memory bus. Here is a representation of what's happening

```
Memory Layout in Arena Allocator
=================================

Memory addresses (in bytes):
   0    1    2    3    4    5    6    7    8    9   10   11
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│    │    │    │    │    │    │    │    │    │    │    │    │  Initial arena
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘
  ↑
offset = 0


Step 1: Allocate u8 (size=1, align=1)
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│ u8 │    │    │    │    │    │    │    │    │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘
       ↑
offset = 1  (no alignment needed, 0 is already aligned to 1)


Step 2: Allocate another u8 (size=1, align=1)
┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│ u8 │ u8 │    │    │    │    │    │    │    │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘
            ↑
offset = 2  (no alignment needed, 1 is aligned to 1)


Step 3: Allocate u32 (size=4, align=4)
         offset=2, need alignment to 4
         aligned = (2 + 4 - 1) & !(4-1) = 5 & !3 = 4

┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
│ u8 │ u8 │ XX │ XX │ u32────────────────── │    │    │    │
└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘
 0    1    2    3    4    5    6    7    8    9   10   11
             ↑    ↑   ↑───────────────↑ ↑
             PADDING      u32(4bytes)   offset = 8

Legend:
  XX  = Wasted padding bytes (to satisfy alignment)
  u32 = The 4 bytes of the u32 value (must start at multiple of 4)
```
