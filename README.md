**rustOS** is a small bare-metal operating system kernel that runs **directly on hardware**, without any underlying operating system.  
It is implemented entirely in **Rust**.

This targets **x86_64 systems** and is designed to run inside **QEMU** for development and testing.

---

##  Features

- 🖥️ **VGA Text Mode Driver**
  - Custom VGA buffer implementation
  - 80×25 text mode
  - Full 16-color foreground/background support

- 📡 **Serial Communication**
  - UART 16550 (COM1 @ `0x3F8`)
  - Used for debugging and test output

- ⚡ **Volatile Memory Access**
  - handling of memory-mapped I/O

---

## 🧠 System Architecture

```mermaid
graph TB
    subgraph Kernel["rustOS Kernel"]
        A[main.rs<br/>Entry Point]
        B[VGA Driver<br/>vga_buffer.rs]
        C[Serial Driver<br/>serial.rs]
    end

    subgraph HAL["Hardware Abstraction"]
        D[Volatile Memory]
        E[Port I/O]
    end

    subgraph HW["Hardware"]
        F[VGA Buffer<br/>0xb8000]
        G[Serial COM1<br/>0x3F8]
        H[QEMU Exit Device<br/>0xf4]
    end

    A --> B
    A --> C
    B --> D --> F
    C --> E --> G
    E --> H
````

---

## Details about project

### 1️⃣ VGA Text Mode Driver

* **Memory-Mapped I/O** at `0xb8000`
* **Resolution:** 80×25 characters
* **Color Palette:** 16 foreground + 16 background colors
* **Capabilities**

  * Colored text output
  * Line wrapping
  * Scrolling
  * Global thread-safe writer (`Mutex<Writer>`)

---

### 2️⃣ Serial Communication (`serial.rs`)

* **UART 16550 compatible**
* **Port:** COM1 (`0x3F8`)
* Used for:

  * Debug logs
  * Test output
* Macros:

  * `serial_print!`
  * `serial_println!`

---

### 3️⃣ Kernel Entry Point (`main.rs`)

* `#![no_std]` & `#![no_main]`
* Custom `_start` function
* Custom panic handler
* Built-in test runner
* QEMU exit support for automated testing

---

## ⚙️ Build Configuration

### Custom Target (`x86_64-blog_os.json`)

```json
{
  "llvm-target": "x86_64-unknown-none",
  "os": "none",
  "panic-strategy": "abort",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float"
}
```

## 📚 Dependencies

| Crate       | Version | Purpose                     |
| ----------- | ------- | --------------------------- |
| bootloader  | 0.9     | Kernel booting              |
| volatile    | 0.2.6   | Volatile memory access      |
| spin        | 0.5.2   | Spinlock synchronization    |
| x86_64      | 0.14.2  | CPU & hardware abstractions |
| uart_16550  | 0.2.0   | Serial driver               |
| lazy_static | 1.0     | Static initialization       |

---

## 🎓 Learning Resources

Inspired by
***Writing an OS in Rust***
by **Philipp Oppermann**

Highly recommended if you want to understand *why* things work, not just *how*.
