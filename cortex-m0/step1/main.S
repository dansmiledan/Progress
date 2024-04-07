  .syntax unified
  .cpu cortex-m0plus
  .fpu softvfp
  .thumb
@ vector table start
.section .text
.word 0x20001000 @ 1
.word _start
@ vector table end

.thumb_func
_start:
loop:
	adds r0, r0, #1
b loop

.global _start
