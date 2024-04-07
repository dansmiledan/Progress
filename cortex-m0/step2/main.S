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
_setup:
	ldr r0, = 0x40021000
	ldr r1, = 0x2
	str r1, [r0, 0x34]
    ldr r0, = 0x50000400
	ldr r1, = 0x01
	@ common output
	str r1, [r0, 0x0]
	@ str r1, [r0, 0x18]
loop:
open_pb0:
    ldr r1, = 0x01
	str r1, [r0, 0x14]
	ldr r2, = 0xfffff
wait_0:
	subs r2, r2, #1
	bne wait_0

close_pb0:	
	ldr r1, = 0x0
	str r1, [r0, 0x14]
	ldr r2, = 0xfffff

wait_1:
    subs r2, r2, #1
	bne wait_1
    b loop

.global _start
