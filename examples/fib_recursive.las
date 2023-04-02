data:
    .message "Please enter n to computer fibonacci number "

program:
    LDA R0, .message
    OUT R0

    INPN R0
    LDA R2, 0u
    LDA R3, 1u

    CALL @fib_func
    OUTN R0

    FIN

    @fib_func
    LE R0, R3
    JCMP @fib_func_basis

    SUB R0, R3, R0
    PUSH R0
    call @fib_func
    MOV R0, R1
    POP R0
    PUSH R1
    SUB R0, R3, R0
    CALL @fib_func
    POP R1
    ADD R0, R1, R0
    RET

    @fib_func_basis
    LDA R0, 1u
    RET

    @fib_func_rec

