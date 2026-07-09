https://www.youtube.com/watch?v=rl0jkP9kOMw

# Computer Fundamentals

- **Binary and Decimal Systems:** Computers operate on zeros and ones (binary), which represent the presence or absence of electricity. This is analogous to the decimal system (base 10) humans use, where each digit's place value is a power of 10. Binary uses powers of two.
- **Logic Gates:** Basic building blocks of digital circuits that perform logical operations (AND, OR, NOT, XOR, NAND) on binary inputs to produce a binary output. These gates are deterministic, meaning their output is solely determined by their inputs.
- **Circuits:** Logic gates can be chained together to create circuits that perform more complex operations, such as addition. The video demonstrates building a half-adder and a full-adder to perform binary addition, eventually scaling up to an 8-bit adder.

# Number Representation and Arithmetic

- **Binary Arithmetic:** The video explains how to convert between decimal and binary numbers and how to perform addition in binary, including handling carry-overs.
- **Two's Complement:** A method for representing negative numbers in binary, crucial for subtraction. Negating a number involves inverting all its bits and adding one.
- **Subtraction:** Implemented by adding the two's complement of the subtrahend (the number being subtracted) to the minuend (the number from which another is subtracted).

# Computer Architecture Components

- **ALU (Arithmetic Logic Unit):** The circuit that performs arithmetic and logical operations. It takes inputs from registers and performs operations like addition and subtraction based on control signals.
- **Registers:** Small, fast storage locations within the CPU that hold data temporarily. They are built using flip-flops, which can store a single bit of data and change their state based on clock signals.
- **RAM (Random Access Memory):** Memory that stores data and instructions. It has addresses to read from or write to specific locations. The video demonstrates building a 4-bit RAM unit.
- **ROM (Read-Only Memory):** A type of memory used for storing fixed data or programs, like lookup tables. The video uses ROM to implement the control logic for executing instructions.
- **Bus:** A communication pathway that connects various components of the computer, allowing data transfer. Tri-state buffers and switches are used to manage bus access, ensuring only one component writes to the bus at a time.
- **Control Unit:** The "brain" of the computer, which orchestrates the execution of instructions. It uses the instruction register (holding the current instruction), the program counter (holding the address of the next instruction), and flags (like carry and zero) to determine which operations to perform and in what order, often driven by a clock signal.
- **Clock:** A signal that synchronizes the operations of the computer, dictating when data can be read or written.
- **Seven Segment Display:** A component used to visually display decimal numbers, converting binary input into a readable format.

# Program Execution

- **Instructions:** Defined operations (like Load A, Add, Store A, Jump, Halt) that the computer can perform. Each instruction has an op code (specifying the operation) and potentially an argument (like a RAM address or an immediate value).
- **Fetch-Decode-Execute Cycle:** The fundamental process by which a computer runs a program: fetching the instruction from memory, decoding it to understand what to do, and then executing it.
- **Program Counter (PC):** A register that holds the address of the next instruction to be fetched from memory.
- **Assembly Language:** The video demonstrates writing a simple program (counting down from 15, calculating Fibonacci numbers) in a low-level assembly-like format, which is then conceptually loaded into RAM and executed by the computer.
- **Modern Computers:** The video briefly touches on how modern processors (like ARM in iPhones) scale these concepts with 32-bit architectures, much larger RAM capacities, faster clock speeds, and specialized instructions for tasks like interacting with peripherals (e.g., LEDs, displays) via memory-mapped I/O (MMIO).
