.global exception
.global exception_handler
.align 4
.set REG_SIZE, 8
exception:
        // Make room to save registers.
        addi sp, sp, -256

        // Save the registers.
        // Infos on register https://en.wikichip.org/wiki/risc-v/registers
        sd ra, 0(sp)
        sd sp, 1*REG_SIZE(sp)
        sd gp, 2*REG_SIZE(sp)
        sd tp, 3*REG_SIZE(sp)
        sd t0, 4*REG_SIZE(sp)
        sd t1, 5*REG_SIZE(sp)
        sd t2, 6*REG_SIZE(sp)
        sd s0, 7*REG_SIZE(sp)
        sd s1, 8*REG_SIZE(sp)
        sd a0, 9*REG_SIZE(sp)
        sd a1, 10*REG_SIZE(sp)
        sd a2, 11*REG_SIZE(sp)
        sd a3, 12*REG_SIZE(sp)
        sd a4, 13*REG_SIZE(sp)
        sd a5, 14*REG_SIZE(sp)
        sd a6, 15*REG_SIZE(sp)
        sd a7, 16*REG_SIZE(sp)
        sd s2, 17*REG_SIZE(sp)
        sd s3, 18*REG_SIZE(sp)
        sd s4, 19*REG_SIZE(sp)
        sd s5, 20*REG_SIZE(sp)
        sd s6, 21*REG_SIZE(sp)
        sd s7, 22*REG_SIZE(sp)
        sd s8, 23*REG_SIZE(sp)
        sd s9, 24*REG_SIZE(sp)
        sd s10, 25*REG_SIZE(sp)
        sd s11, 26*REG_SIZE(sp)
        sd t3, 27*REG_SIZE(sp)
        sd t4, 28*REG_SIZE(sp)
        sd t5, 29*REG_SIZE(sp)
        sd t6, 30*REG_SIZE(sp)

        csrr a0, mepc
        csrr a1, mcause
        mv a2, sp
        la sp, _stack_end

        // Call the C trap handler in exception_handler.rs
        call exception_handler

        // Restore stackpointer, the stackpointer is returned from the exception_handler function
        mv sp, a0

        // Restore registers.
        ld ra, 0(sp)
        ld sp, 1*REG_SIZE(sp)
        ld gp, 2*REG_SIZE(sp)
        ld tp, 3*REG_SIZE(sp)
        ld t0, 4*REG_SIZE(sp)
        ld t1, 5*REG_SIZE(sp)
        ld t2, 6*REG_SIZE(sp)
        ld s0, 7*REG_SIZE(sp)
        ld s1, 8*REG_SIZE(sp)
        ld a0, 9*REG_SIZE(sp)
        ld a1, 10*REG_SIZE(sp)
        ld a2, 11*REG_SIZE(sp)
        ld a3, 12*REG_SIZE(sp)
        ld a4, 13*REG_SIZE(sp)
        ld a5, 14*REG_SIZE(sp)
        ld a6, 15*REG_SIZE(sp)
        ld a7, 16*REG_SIZE(sp)
        ld s2, 17*REG_SIZE(sp)
        ld s3, 18*REG_SIZE(sp)
        ld s4, 19*REG_SIZE(sp)
        ld s5, 20*REG_SIZE(sp)
        ld s6, 21*REG_SIZE(sp)
        ld s7, 22*REG_SIZE(sp)
        ld s8, 23*REG_SIZE(sp)
        ld s9, 24*REG_SIZE(sp)
        ld s10, 25*REG_SIZE(sp)
        ld s11, 26*REG_SIZE(sp)
        ld t3, 27*REG_SIZE(sp)
        ld t4, 28*REG_SIZE(sp)
        ld t5, 29*REG_SIZE(sp)
        ld t6, 30*REG_SIZE(sp)

        addi sp, sp, 256

        // Return to whatever we were doing in the kernel.
        mret
