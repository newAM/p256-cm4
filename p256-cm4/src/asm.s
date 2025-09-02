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
 .align 2
# 113 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type mul288x288, %function
mul288x288:
 push {r4-r11,lr}


 mov r4,r0
 mov r5,r2
 mov r6,r1

 movs r1,#72
 bl setzero

 ldm r5,{r0-r2,r8-r12,lr}

 movs r7,#9
0:
 ldm r6!,{r5}
 push {r6,r7}

 movs r3,#0
 ldm r4,{r6,r7}
 umaal r6,r3,r5,r0
 umaal r7,r3,r5,r1
 stm r4!,{r6,r7}
 ldm r4,{r6,r7}
 umaal r6,r3,r5,r2
 umaal r7,r3,r5,r8
 stm r4!,{r6,r7}
 ldm r4,{r6,r7}
 umaal r6,r3,r5,r9
 umaal r7,r3,r5,r10
 stm r4!,{r6,r7}
 ldm r4,{r6,r7}
 umaal r6,r3,r5,r11
 umaal r7,r3,r5,r12
 stm r4!,{r6,r7}
 ldm r4,{r6}
 umaal r3,r6,r5,lr
 stm r4!,{r3,r6}

 subs r4,r4,#36
 pop {r6,r7}

 subs r7,r7,#1
 bne 0b

 pop {r4-r11,pc}
 .size mul288x288, .-mul288x288

 .type setzero, %function
setzero:
 movs r2,#0
 movs r3,#0
0:
 stm r0!,{r2,r3}
 subs r1,r1,#8
 bne 0b
 bx lr
 .size setzero, .-setzero

 .type P256_modinv_sqrt, %function
P256_modinv_sqrt:
 push {r0-r8,lr}


 mov r8,#1
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod
 push {r0-r7}


 bl P256_sqrmod
 bl P256_sqrmod
 push {r0-r7}


 mov r1,sp
 add r2,sp,#32
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{r0-r7}


 mov r8,#8-4
 add r9,sp,#32
 bl P256_sqrmod_many_and_mulmod
 push {r0-r7}


 mov r8,#16-8
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod
 push {r0-r7}


 mov r8,#16
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod
 push {r0-r7}


 mov r8,#32
 add r9,sp,#5*32
 bl P256_sqrmod_many_and_mulmod

 ldr r8,[sp,#6*32]
 cmp r8,#0
 bne 0f


 mov r8,#192-64
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod


 mov r8,#224-192
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod


 mov r8,#240-224
 add r9,sp,#32
 bl P256_sqrmod_many_and_mulmod


 mov r8,#248-240
 add r9,sp,#64
 bl P256_sqrmod_many_and_mulmod


 mov r8,#252-248
 add r9,sp,#128
 bl P256_sqrmod_many_and_mulmod


 mov r8,#256-252
 add r9,sp,#96
 bl P256_sqrmod_many_and_mulmod
 stm sp,{r0-r7}


 mov r1,sp
 add r2,sp,#5*32
 bl P256_mulmod
 b 1f

0:

 mov r8,#160-64
 add r9,sp,#5*32
 bl P256_sqrmod_many_and_mulmod


 mov r8,#254-160
 bl P256_sqrmod_many
1:

 add sp,#6*32+4

 pop {pc}

 .size P256_modinv_sqrt, .-P256_modinv_sqrt





 .type P256_times2, %function
P256_times2:
 adds r0,r0
 adcs r1,r1,r1
 adcs r2,r2,r2
 adcs r3,r3,r3
 adcs r4,r4,r4
 adcs r5,r5,r5
 adcs r6,r6,r6
 adcs r7,r7,r7
 mov r8,#0
 adcs r8,r8,r8

 subs r0,#0xffffffff
 sbcs r1,r1,#0xffffffff
 sbcs r2,r2,#0xffffffff
 sbcs r3,r3,#0
 sbcs r4,r4,#0
 sbcs r5,r5,#0
 sbcs r6,r6,#1
 sbcs r7,r7,#0xffffffff
 sbcs r8,r8,#0

 adds r0,r8
 adcs r1,r1,r8
 adcs r2,r2,r8
 adcs r3,r3,#0
 adcs r4,r4,#0
 adcs r5,r5,#0
 adcs r6,r6,r8, lsr #31
 adcs r7,r7,r8

 bx lr
 .size P256_times2, .-P256_times2

 .type P256_from_montgomery, %function
P256_from_montgomery:
 .global P256_from_montgomery
 push {r0,r4-r11,lr}


 movs r2,#0
 movs r3,#0
 push {r2-r3}

 push {r2-r3}

 push {r2-r3}

 movs r2,#1
 push {r2-r3}

 mov r2,sp
 bl P256_mulmod
 add sp,#32

 pop {r8}

 stm r8,{r0-r7}
 pop {r4-r11,pc}
 .size P256_from_montgomery, .-P256_from_montgomery






 .type P256_check_range_p, %function
P256_check_range_p:
 .global P256_check_range_p
 push {r4-r8,lr}


 ldm r0,{r1-r8}

 movs r0,#0xffffffff

 subs r1,r0
 sbcs r2,r2,r0
 sbcs r3,r3,r0
 sbcs r4,r4,#0
 sbcs r5,r5,#0
 sbcs r6,r6,#0
 sbcs r7,r7,#1
 sbcs r8,r8,r0

 sbcs r0,r0,r0
 lsrs r0,#31

 pop {r4-r8,pc}

 .size P256_check_range_p, .-P256_check_range_p







 .align 2
P256_order_mu:
 .word 0xeedf9bfe
 .word 0x012ffd85
 .word 0xdf1a6c21
 .word 0x43190552
 .word 0xffffffff
 .word 0xfffffffe
 .word 0xffffffff
 .word 0x0
 .word 0x1





 .type P256_reduce_mod_n_once, %function
P256_reduce_mod_n_once:
 push {lr}


 adr r10,P256_order
 ldm r10,{r10,r11,r12,lr}
 subs r0,r10
 sbcs r1,r1,r11
 sbcs r2,r2,r12
 sbcs r3,r3,lr
 sbcs r4,r4,#0xffffffff
 sbcs r5,r5,#0xffffffff
 sbcs r6,r6,#0
 sbcs r7,r7,#0xffffffff
 sbcs r8,r8,#0

 sbc r9,r9,r9
 and r10,r9
 and r11,r9
 and r12,r9
 and lr,r9

 adds r0,r10
 adcs r1,r1,r11
 adcs r2,r2,r12
 adcs r3,r3,lr
 adcs r4,r4,r9
 adcs r5,r5,r9
 adcs r6,r6,#0
 adcs r7,r7,r9
 adcs r8,r8,#0

 pop {pc}
 .size P256_reduce_mod_n_once, .-P256_reduce_mod_n_once



 .type P256_reduce_mod_n_64bytes, %function
P256_reduce_mod_n_64bytes:
 push {r0,r4-r11,lr}

 sub sp,sp,#108


 mov r10,r1

 add r0,sp,#36
 adds r1,r1,#28
 adr r2,P256_order_mu
 bl mul288x288

 mov r0,sp
 add r1,sp,#72
 adr r2,P256_order
 bl mul288x288

 ldm r10,{r0-r8}
 pop {r9-r12}

 subs r0,r0,r9
 sbcs r1,r1,r10
 sbcs r2,r2,r11
 sbcs r3,r3,r12
 pop {r9-r12,lr}

 sbcs r4,r4,r9
 sbcs r5,r5,r10
 sbcs r6,r6,r11
 sbcs r7,r7,r12
 sbcs r8,r8,lr

 bl P256_reduce_mod_n_once
 bl P256_reduce_mod_n_once
 add sp,sp,#72

 pop {r9}


 stm r9,{r0-r7}

 pop {r4-r11,pc}
 .size P256_reduce_mod_n_64bytes, .-P256_reduce_mod_n_64bytes




 .type P256_reduce_mod_n_32bytes, %function
P256_reduce_mod_n_32bytes:
 .global P256_reduce_mod_n_32bytes
 push {r0,r4-r11,lr}


 ldm r1,{r0-r7}
 mov r8,#0
 bl P256_reduce_mod_n_once
 pop {r8}

 stm r8,{r0-r7}
 pop {r4-r11,pc}
 .size P256_reduce_mod_n_32bytes, .-P256_reduce_mod_n_32bytes






 .type P256_add_mod_n, %function
P256_add_mod_n:
 .global P256_add_mod_n
 push {r0,r4-r11,lr}



 mov r12,r1

 ldm r2,{r4-r11}
 ldm r12!,{r0-r3}
 adds r0,r4
 adcs r1,r1,r5
 adcs r2,r2,r6
 adcs r3,r3,r7
 ldm r12,{r4-r7}
 adcs r4,r4,r8
 adcs r5,r5,r9
 adcs r6,r6,r10
 adcs r7,r7,r11
 movs r8,#0
 adcs r8,r8,r8

 bl P256_reduce_mod_n_once
 bl P256_reduce_mod_n_once
 pop {r8}

 stm r8,{r0-r7}

 pop {r4-r11,pc}

 .size P256_add_mod_n, .-P256_add_mod_n







 .type P256_mul_mod_n, %function
P256_mul_mod_n:
 .global P256_mul_mod_n
 movs r3,#0
 push {r3-r10,lr}



 mov r4,r0

 ldm r1,{r1,r3,r5-r10}
 push {r1,r3,r5-r10}


 movs r1,#0
 push {r1}

 ldm r2,{r1,r3,r5-r10}
 push {r1,r3,r5-r10}


 sub sp,#72


 mov r0,sp
 add r1,sp,#72
 add r2,sp,#108
 bl mul288x288

 mov r0,r4
 mov r1,sp
 bl P256_reduce_mod_n_64bytes

 add sp,#144

 pop {r4-r10,pc}

 .size P256_mul_mod_n, .-P256_mul_mod_n






 .type P256_divsteps2_31, %function
P256_divsteps2_31:
 .global P256_divsteps2_31
 push {r3,r4-r8,lr}




 movs r4,#1
 movs r5,#0
 movs r6,#0
 movs r7,#1


 mov lr,#31

0:
 subs r3,r0,#1
 lsl r12,r2,#31
 bic r3,r12,r3
 asrs r3,r3,#31
 lsr r8,r3,#31


 eors r0,r0,r3
 subs r0,r0,r3

 mul r12,r1,r3
 bics r1,r1,r3
 umlal r1,r12,r2,r8
 umaal r2,r12,r2,r3

 mul r12,r4,r3
 bics r4,r4,r3
 umlal r4,r12,r6,r8
 umaal r6,r12,r6,r3

 mul r12,r5,r3
 bics r5,r5,r3
 umlal r5,r12,r7,r8
 umaal r7,r12,r7,r3

 ands r12,r2,#1
 adds r0,r0,#1


 mul r3,r12,r1
 adds r2,r2,r3
 lsrs r2,r2,#1

 umlal r6,r8,r12,r4
 umlal r7,r8,r12,r5

 adds r4,r4,r4
 adds r5,r5,r5

 subs lr,lr,#1
 bne 0b

 pop {r3}
 stm r3!,{r4-r7}

 pop {r4-r8,pc}
 .size P256_divsteps2_31, .-P256_divsteps2_31





 .type P256_matrix_mul_fg_9, %function
P256_matrix_mul_fg_9:
 .global P256_matrix_mul_fg_9
 push {r4-r11,lr}






 and r4,r0,r0,lsl #1
 asrs r4,r4,#31
 eors r0,r0,r4
 subs r0,r0,r4

 and r5,r1,r1,lsl #1
 asrs r5,r5,#31
 eors r1,r1,r5
 subs r1,r1,r5

 ldm r2!,{r6}
 ldr r7,[r2,#36]


 eors r4,r4,r6
 eors r5,r5,r7
 eors r4,r4,r5
 stm r3!,{r5}
 push {r1,r2,r3}




 ldm r2!,{r1,r3,r5-r11}
 eors r1,r1,r4
 eors r3,r3,r4
 eors r5,r5,r4
 eors r6,r6,r4
 eors r7,r7,r4
 eor r8,r8,r4
 eor r9,r9,r4
 eor r10,r10,r4

 subs r1,r1,r4
 sbcs r3,r3,r4
 sbcs r5,r5,r4
 sbcs r6,r6,r4
 sbcs r7,r7,r4
 sbcs r8,r8,r4
 sbcs r9,r9,r4
 sbcs r10,r10,r4

 eor r4,r4,r11


 umull r1,lr,r0,r1
 movs r2,#0
 umull r11,r12,r2,r2
 umaal r2,lr,r0,r3
 umaal r11,lr,r0,r5
 umull r3,r5,r12,r12
 umaal r3,lr,r0,r6
 umaal r5,lr,r0,r7
 umull r6,r7,r12,r12
 umaal r6,lr,r0,r8
 umaal r7,lr,r0,r9
 umaal r12,lr,r0,r10
 mla lr,r0,r4,lr



 pop {r0,r4}

 adds r4,r4,#40
 ldm r4!,{r8,r9}
 mov r10,#0
 umaal r1,r10,r0,r8
 umaal r2,r10,r0,r9
 adds r1,r1,r1
 adcs r2,r2,r2
 ldm r4!,{r1,r8,r9}
 umaal r10,r11,r0,r1
 umaal r11,r3,r0,r8
 umaal r3,r5,r0,r9
 adcs r10,r10,r10
 adcs r11,r11,r11
 adcs r3,r3,r3
 ldm r4,{r1,r4,r8,r9}
 umaal r5,r6,r0,r1
 umaal r6,r7,r0,r4
 umaal r7,r12,r0,r8
 umaal r12,lr,r0,r9
 adcs r5,r5,r5
 adcs r6,r6,r6
 adcs r7,r7,r7
 adcs r12,r12,r12
 sbcs lr,lr,lr
 mvn lr,lr
 pop {r1}

 stm r1!,{r2,r10,r11}
 stm r1!,{r3,r5,r6,r7,r12,lr}

 pop {r4-r11,pc}
 .size P256_matrix_mul_fg_9, .-P256_matrix_mul_fg_9





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






 .type P256_check_range_n, %function
P256_check_range_n:
 .global P256_check_range_n
 push {r4-r11,lr}

 ldm r0,{r1-r8}
 orrs r0,r1,r2
 orrs r0,r3
 orrs r0,r4
 orrs r0,r5
 orrs r0,r6
 orrs r0,r7
 orrs r0,r8
 beq 0f

 adr r0,P256_order
 ldm r0!,{r9-r12}
 subs r1,r9
 sbcs r2,r2,r10
 sbcs r3,r3,r11
 sbcs r4,r4,r12
 ldm r0,{r0-r3}
 sbcs r5,r5,r0
 sbcs r6,r6,r1
 sbcs r7,r7,r2
 sbcs r8,r8,r3

 sbcs r0,r0,r0
 lsrs r0,#31
0:
 pop {r4-r11,pc}

 .size P256_check_range_n, .-P256_check_range_n






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

 .type P256_jacobian_to_affine, %function
P256_jacobian_to_affine:
 .global P256_jacobian_to_affine
 push {r0,r1,r2,r4-r11,lr}



 adds r2,#64
 ldm r2,{r0-r7}
 mov r8,#0
 bl P256_modinv_sqrt
 push {r0-r7}


 bl P256_sqrmod
 push {r0-r7}


 add r1,sp,#32
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{r0-r7}

 mov r1,sp
 ldr r2,[sp,#72]
 bl P256_mulmod
 ldr r8,[sp,#64]
 stm r8,{r0-r7}

 ldr r2,[sp,#72]
 add r1,sp,#32
 adds r2,r2,#32
 bl P256_mulmod
 ldr r8,[sp,#68]
 stm r8,{r0-r7}

 add sp,#76


 pop {r4-r11,pc}
 .size P256_jacobian_to_affine, .-P256_jacobian_to_affine





 .type P256_double_j, %function
P256_double_j:
 .global P256_double_j
 push {r0,r1,r4-r11,lr}






 adds r1,#64
 ldm r1,{r0-r7}
 bl P256_sqrmod
 push {r0-r7}



 ldr r1,[sp,#36]
 adds r1,#32
 add r2,r1,#32
 bl P256_mulmod
 ldr r8,[sp,#32]
 add r8,#64
 stm r8,{r0-r7}


 ldr r1,[sp,#36]
 mov r2,sp
 bl P256_addmod
 push {r0-r7}



 ldr r1,[sp,#68]
 add r2,sp,#32
 bl P256_submod
 add r8,sp,#32
 stm r8,{r0-r7}


 add r1,sp,#32
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{r0-r7}


 lsl r8,r0,#31
 adds r0,r0,r8, asr #31
 adcs r1,r1,r8, asr #31
 adcs r2,r2,r8, asr #31
 adcs r3,r3,#0
 adcs r4,r4,#0
 adcs r5,r5,#0
 adcs r6,r6,r8, lsr #31
 adcs r7,r7,r8, asr #31
 rrxs r7,r7
 rrxs r6,r6
 rrxs r5,r5
 rrxs r4,r4
 rrxs r3,r3
 rrxs r2,r2
 rrxs r1,r1
 rrx r0,r0
 stm sp,{r0-r7}


 add r1,sp,#32
 mov r2,sp
 bl P256_addmod
 add r8,sp,#32
 stm r8,{r0-r7}


 bl P256_sqrmod
 stm sp,{r0-r7}


 ldr r0,[sp,#68]
 adds r0,#32
 ldm r0,{r0-r7}
 bl P256_sqrmod
 ldr r8,[sp,#64]
 add r8,#32
 stm r8,{r0-r7}


 bl P256_sqrmod
 push {r0-r7}



 ldrd r0,r1,[sp,#96]
 add r2,r0,#32
 bl P256_mulmod
 ldr r8,[sp,#96]
 add r8,#32
 stm r8,{r0-r7}


 bl P256_times2
 ldr r8,[sp,#96]
 stm r8,{r0-r7}


 add r1,sp,#32
 mov r2,r8
 bl P256_submod
 ldr r8,[sp,#96]
 stm r8,{r0-r7}


 mov r2,r8
 add r1,r2,#32
 bl P256_submod
 add r8,sp,#32
 stm r8,{r0-r7}


 add r1,sp,#64
 add r2,sp,#32
 bl P256_mulmod
 add r8,sp,#64
 stm r8,{r0-r7}


 add r1,sp,#64
 mov r2,sp
 bl P256_submod
 ldr r8,[sp,#96]
 add r8,#32
 stm r8,{r0-r7}

 add sp,#104


 pop {r4-r11,pc}
 .size P256_double_j, .-P256_double_j





 .type add_sub_helper, %function
add_sub_helper:
 push {lr}

 ldm r1!,{r5-r12}
 stm r0!,{r5-r12}
 ldm r1!,{r5-r12}
 cbz r2,0f

 mov lr,#0
 rsbs r4,r2,#0
 subs r5,r4,r5
 sbcs r6,r4,r6
 sbcs r7,r4,r7
 sbcs r8,lr,r8
 sbcs r9,lr,r9
 sbcs r10,lr,r10
 sbcs r11,r2,r11
 sbcs r12,r4,r12
0:
 stm r0!,{r5-r12}
 cbnz r3,1f
 ldm r1,{r5-r12}
 stm r0,{r5-r12}
 b 2f
1:

 movs r4,#0
 umull r5,r10,r4,r4
 mvns r6,r4
 mvns r7,r4
 mov r8,#0xffffffff
 mov r9,#0xfffffffe

 stm r0,{r3-r10}
2:
 pop {pc}

 .size add_sub_helper, .-add_sub_helper
# 2857 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type P256_add_sub_j, %function
P256_add_sub_j:
 .global P256_add_sub_j
 push {r0-r11,lr}





 add r4,r0,#64
 ldm r4,{r4-r11}
 orrs r4,r5
 orrs r4,r6
 orrs r4,r7
 orrs r4,r8
 orrs r4,r9
 orrs r4,r10
 orrs r4,r11
 bne 2f


 bl add_sub_helper
 add sp,#16

 pop {r4-r11,pc}
2:







 cbnz r3,100f


 adds r1,#64
 ldm r1,{r0-r7}
 bl P256_sqrmod
 push {r0-r7}



 ldr r1,[sp,#32]
 mov r2,sp
 bl P256_mulmod
 ldr r8,[sp,#32]
 stm r8,{r0-r7}


 ldr r1,[sp,#36]
 adds r1,#64
 mov r2,sp
 bl P256_mulmod
 stm sp,{r0-r7}


 ldr r1,[sp,#32]
 adds r1,#32
 mov r2,sp
 bl P256_mulmod
 ldr r8,[sp,#32]
 add r8,#32
 stm r8,{r0-r7}
 b 101f
100:
 sub sp,#32

101:


 ldr r1,[sp,#32]
 adds r1,#64
 ldm r1,{r0-r7}
 bl P256_sqrmod
 push {r0-r7}



 ldr r1,[sp,#68]
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{r0-r7}


 ldr r1,[sp,#64]
 adds r1,#64
 mov r2,sp
 bl P256_mulmod
 stm sp,{r0-r7}


 ldr r1,[sp,#68]
 adds r1,#32
 mov r2,sp
 bl P256_mulmod
 stm sp,{r0-r7}



 ldr r1,[sp,#64]
 add r2,sp,#32
 bl P256_submod
 ldr r8,[sp,#64]
 stm r8,{r0-r7}


 bl P256_sqrmod
 push {r0-r7}



 ldr r2,[sp,#96]
 add r1,r2,#64
 bl P256_mulmod
 ldr r8,[sp,#96]
 add r8,#64
 stm r8,{r0-r7}


 ldr r1,[sp,#108]
 cbnz r1,102f
 ldr r1,[sp,#100]
 adds r1,#64
 mov r2,r8
 bl P256_mulmod
 ldr r8,[sp,#96]
 add r8,#64
 stm r8,{r0-r7}
102:


 ldr r1,[sp,#96]
 mov r2,sp
 bl P256_mulmod
 ldr r8,[sp,#96]
 stm r8,{r0-r7}


 orrs r1,r0
 orrs r1,r2
 orrs r1,r3
 orrs r1,r4
 orrs r1,r5
 orrs r1,r6
 orrs r0,r1,r7
3:
 push {r0}



 ldr r1,[sp,#100]
 adds r1,#32
 add r2,sp,#36
 ldr r3,[sp,#108]
 cbz r3,4f
 bl P256_addmod
 b 5f
4:
 bl P256_submod
5:
 ldr r8,[sp,#100]
 add r8,#32
 stm r8,{r0-r7}


 pop {r8}


 orrs r1,r0
 orrs r1,r2
 orrs r1,r3
 orrs r1,r4
 orrs r1,r5
 orrs r1,r6
 orrs r1,r7
 orrs r1,r8
 bne 6f



 add sp,#96

 ldm sp,{r0-r3}
 bl add_sub_helper

 ldr r0,[sp,#0]
 mov r1,r0
 add sp,#16

 bl P256_double_j
 pop {r4-r11,pc}
6:



 add r1,sp,#64
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#64
 stm r8,{r0-r7}


 ldr r0,[sp,#96]
 adds r0,#32
 ldm r0,{r0-r7}
 bl P256_sqrmod
 stm sp,{r0-r7}


 add r1,sp,#32
 ldr r2,[sp,#96]
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{r0-r7}


 mov r1,sp
 ldr r2,[sp,#96]
 bl P256_submod
 ldr r8,[sp,#96]
 stm r8,{r0-r7}


 add r0,sp,#64
 ldm r0,{r0-r7}
 bl P256_times2
 stm sp,{r0-r7}


 ldr r1,[sp,#96]
 mov r2,sp
 bl P256_submod
 ldr r8,[sp,#96]
 stm r8,{r0-r7}


 add r1,sp,#64
 ldr r2,[sp,#96]
 bl P256_submod
 stm sp,{r0-r7}


 ldr r1,[sp,#96]
 adds r1,#32
 mov r2,sp
 bl P256_mulmod
 stm sp,{r0-r7}


 ldr r0,[sp,#104]
 mov r1,sp
 add r2,sp,#32
 cbz r0,7f
 bl P256_addmod
 b 8f
7:
 bl P256_submod
8:
 ldr r8,[sp,#96]
 add r8,#32
 stm r8,{r0-r7}

 add sp,#112


 pop {r4-r11,pc}
 .size P256_add_sub_j, .-P256_add_sub_j

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
