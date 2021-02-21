# rust-arduino-blink

Blink example for Arduino Uno in Rust. Developed on Fedora Linux 33.

Requires rust nightly-2021-01-07, avr-gcc, avr-libc, and avrdude.

ROM size: 294 bytes with "s" optimization level.

Dump:
```rust
$ avr-objdump -S rust-arduino-blink.elf 

rust-arduino-blink.elf:     file format elf32-avr


Disassembly of section .text:

00000000 <__vectors>:
   0:	0c 94 34 00 	jmp	0x68	; 0x68 <__ctors_end>
   4:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
   8:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
   c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  10:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  14:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  18:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  1c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  20:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  24:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  28:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  2c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  30:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  34:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  38:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  3c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  40:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  44:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  48:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  4c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  50:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  54:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  58:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  5c:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  60:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>
  64:	0c 94 51 00 	jmp	0xa2	; 0xa2 <__bad_interrupt>

00000068 <__ctors_end>:
  68:	11 24       	eor	r1, r1
  6a:	1f be       	out	0x3f, r1	; 63
  6c:	cf ef       	ldi	r28, 0xFF	; 255
  6e:	d8 e0       	ldi	r29, 0x08	; 8
  70:	de bf       	out	0x3e, r29	; 62
  72:	cd bf       	out	0x3d, r28	; 61

00000074 <__do_copy_data>:
  74:	11 e0       	ldi	r17, 0x01	; 1
  76:	a0 e0       	ldi	r26, 0x00	; 0
  78:	b1 e0       	ldi	r27, 0x01	; 1
  7a:	e6 e2       	ldi	r30, 0x26	; 38
  7c:	f1 e0       	ldi	r31, 0x01	; 1
  7e:	02 c0       	rjmp	.+4      	; 0x84 <__do_copy_data+0x10>
  80:	05 90       	lpm	r0, Z+
  82:	0d 92       	st	X+, r0
  84:	a0 30       	cpi	r26, 0x00	; 0
  86:	b1 07       	cpc	r27, r17
  88:	d9 f7       	brne	.-10     	; 0x80 <__do_copy_data+0xc>

0000008a <__do_clear_bss>:
  8a:	21 e0       	ldi	r18, 0x01	; 1
  8c:	a0 e0       	ldi	r26, 0x00	; 0
  8e:	b1 e0       	ldi	r27, 0x01	; 1
  90:	01 c0       	rjmp	.+2      	; 0x94 <.do_clear_bss_start>

00000092 <.do_clear_bss_loop>:
  92:	1d 92       	st	X+, r1

00000094 <.do_clear_bss_start>:
  94:	a1 30       	cpi	r26, 0x01	; 1
  96:	b2 07       	cpc	r27, r18
  98:	e1 f7       	brne	.-8      	; 0x92 <.do_clear_bss_loop>
  9a:	0e 94 53 00 	call	0xa6	; 0xa6 <main>
  9e:	0c 94 91 00 	jmp	0x122	; 0x122 <_exit>

000000a2 <__bad_interrupt>:
  a2:	0c 94 00 00 	jmp	0	; 0x0 <__vectors>

000000a6 <main>:

// AVR HAL library. Crate is named "arduino_uno" in Cargo.toml.
use arduino_uno::prelude::*;

// Main function
#[arduino_uno::entry]
  a6:	0f 93       	push	r16
  a8:	1f 93       	push	r17
        if #[cfg(target_arch = "avr")] {
            let sreg: u8;

            // Store current state
            unsafe {
                llvm_asm!("in $0,0x3F" :"=r"(sreg) ::: "volatile");
  aa:	8f b7       	in	r24, 0x3f	; 63
            unsafe { llvm_asm!("cli" :::: "volatile") };
  ac:	f8 94       	cli
impl atmega328p::Peripherals {
    /// Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        crate::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
  ae:	90 91 00 01 	lds	r25, 0x0100	; 0x800100 <DEVICE_PERIPHERALS>
  b2:	90 30       	cpi	r25, 0x00	; 0
  b4:	11 f0       	breq	.+4      	; 0xba <main+0x14>
  b6:	90 e0       	ldi	r25, 0x00	; 0
  b8:	03 c0       	rjmp	.+6      	; 0xc0 <main+0x1a>
  ba:	91 e0       	ldi	r25, 0x01	; 1
}
impl Peripherals {
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
  bc:	90 93 00 01 	sts	0x0100, r25	; 0x800100 <DEVICE_PERIPHERALS>
            disable();

            let r = f(unsafe { &CriticalSection::new() });

            // Restore interrupt state
            if sreg & 0x80 != 0x00 {
  c0:	88 23       	and	r24, r24
  c2:	0a f4       	brpl	.+2      	; 0xc6 <main+0x20>
            llvm_asm!("sei" :::: "volatile");
  c4:	78 94       	sei
    #[track_caller]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "const_option", issue = "67441")]
    pub const fn unwrap(self) -> T {
        match self {
            Some(val) => val,
  c6:	90 30       	cpi	r25, 0x00	; 0
  c8:	09 f4       	brne	.+2      	; 0xcc <main+0x26>
  ca:	22 c0       	rjmp	.+68     	; 0x110 <main+0x6a>
        // Not panicking to keep codegen impact smaller.
        abort();
    }
    // SAFETY: the caller must uphold the safety contract for `volatile_store`.
    unsafe {
        intrinsics::volatile_store(dst, src);
  cc:	25 9a       	sbi	0x04, 5	; 4
  ce:	20 e0       	ldi	r18, 0x00	; 0
  d0:	30 e0       	ldi	r19, 0x00	; 0
  d2:	47 ef       	ldi	r20, 0xF7	; 247
  d4:	5f e3       	ldi	r21, 0x3F	; 63
  d6:	69 e7       	ldi	r22, 0x79	; 121
  d8:	70 e0       	ldi	r23, 0x00	; 0
  da:	eb e7       	ldi	r30, 0x7B	; 123
  dc:	f4 e0       	ldi	r31, 0x04	; 4
  de:	80 e2       	ldi	r24, 0x20	; 32
  e0:	83 b9       	out	0x03, r24	; 3
  e2:	d9 01       	movw	r26, r18
  e4:	89 01       	movw	r16, r18
cfg_if::cfg_if! {
    if #[cfg(target_arch = "avr")] {
        #[allow(unused_assignments)]
        fn busy_loop(mut c: u16) {
            unsafe {
                llvm_asm!("1: sbiw $0,1\n\tbrne 1b"
  e6:	ca 01       	movw	r24, r20
  e8:	01 97       	sbiw	r24, 0x01	; 1
  ea:	f1 f7       	brne	.-4      	; 0xe8 <main+0x42>
  ec:	91 e0       	ldi	r25, 0x01	; 1
        // compile down to fairly poor code. This is slightly better,
        // but still has some overhead and may not lead to cycle-accurate
        // delays.
        let iters = us >> 12;
        let mut i = 0;
        while i < iters {
  ee:	a6 17       	cp	r26, r22
  f0:	b7 07       	cpc	r27, r23
  f2:	02 07       	cpc	r16, r18
  f4:	13 07       	cpc	r17, r19
  f6:	08 f0       	brcs	.+2      	; 0xfa <main+0x54>
  f8:	90 e0       	ldi	r25, 0x00	; 0
  fa:	91 70       	andi	r25, 0x01	; 1
            delay::DelayUs::<u16>::delay_us(self, 0xfff);
            i += 1;
  fc:	af 5f       	subi	r26, 0xFF	; 255
  fe:	bf 4f       	sbci	r27, 0xFF	; 255
 100:	0f 4f       	sbci	r16, 0xFF	; 255
 102:	1f 4f       	sbci	r17, 0xFF	; 255
        while i < iters {
 104:	90 30       	cpi	r25, 0x00	; 0
 106:	79 f7       	brne	.-34     	; 0xe6 <main+0x40>
                llvm_asm!("1: sbiw $0,1\n\tbrne 1b"
 108:	cf 01       	movw	r24, r30
 10a:	01 97       	sbiw	r24, 0x01	; 1
 10c:	f1 f7       	brne	.-4      	; 0x10a <main+0x64>
 10e:	e7 cf       	rjmp	.-50     	; 0xde <main+0x38>
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never))]
#[track_caller]
#[lang = "panic"] // needed by codegen for panic on overflow and other `Assert` MIR terminators
pub fn panic(expr: &'static str) -> ! {
    if cfg!(feature = "panic_immediate_abort") {
        super::intrinsics::abort()
 110:	0e 94 8c 00 	call	0x118	; 0x118 <abort>
 114:	0e 94 8c 00 	call	0x118	; 0x118 <abort>

00000118 <abort>:
 118:	81 e0       	ldi	r24, 0x01	; 1
 11a:	90 e0       	ldi	r25, 0x00	; 0
 11c:	f8 94       	cli
 11e:	0c 94 91 00 	jmp	0x122	; 0x122 <_exit>

00000122 <_exit>:
 122:	f8 94       	cli

00000124 <__stop_program>:
 124:	ff cf       	rjmp	.-2      	; 0x124 <__stop_program>

```
