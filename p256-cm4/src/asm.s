/*
Pre-processed output of p256-cortex-m4-asm-gcc.S

arm-none-eabi-gcc \
  -O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -mthumb \
  -march=armv7e-m -Wall -Wextra -std=c11 -march=armv7e-m \
  -c P256-Cortex-M4/p256-cortex-m4-asm-gcc.S -E > asm.s


*/
# 1 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
# 1 "/home/alex/git/rmme//"
# 1 "<built-in>"
# 1 "<command-line>"
# 1 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .syntax unified
 .thumb
# 26 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
# 1 "P256-Cortex-M4/p256-cortex-m4-config.h" 1
# 27 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S" 2






 .text

# 2371 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .align 2
P256_order_local:
 .type P256_order, %object
P256_order:
 .global P256_order
 .word 0xFC632551
 .word 0xF3B9CAC2
 .word 0xA7179E84
 .word 0xBCE6FAAD
 .word 0xFFFFFFFF
 .word 0xFFFFFFFF
 .word 0
 .word 0xFFFFFFFF
 .word 0
 .size P256_order, .-P256_order


 .align 2
b_mont:
 .word 0x29c4bddf
 .word 0xd89cdf62
 .word 0x78843090
 .word 0xacf005cd
 .word 0xf7212ed6
 .word 0xe5a220ab
 .word 0x04874834
 .word 0xdc30061d
three_mont:
 .word 0x3
 .word 0x0
 .word 0x0
 .word 0xfffffffd
 .word 0xffffffff
 .word 0xffffffff
 .word 0xfffffffc
 .word 0x2

 .align 2
P256_p:
 .word 0xffffffff
 .word 0xffffffff
 .word 0xffffffff
 .word 0
 .word 0
 .word 0
 .word 1
 .word 0xffffffff

 .type P256_negate_mod_n_if, %function
P256_negate_mod_n_if:
 .global P256_negate_mod_n_if
 ldr r3,=P256_order
 b P256_negate_mod_m_if
 .size P256_negate_mod_n_if, .-P256_negate_mod_n_if

 .type P256_negate_mod_p_if, %function
P256_negate_mod_p_if:
 .global P256_negate_mod_p_if
 adr r3,P256_p
 b P256_negate_mod_m_if
 .size P256_negate_mod_p_if, .-P256_negate_mod_p_if


 .align 2
 .end
