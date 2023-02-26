data:
    .message "Please enter n to computer fibonacci number "

program:
    OUT .message

    LDA R3, 10u
    INP R0
    SUB R0, R3
    LD R1, .multiplier

    @input_loop
    LDA R3, 48u
    INP R2
    SUB R2, R3

    LDA R3, 10u
    EQ R2, R3
    JCMP @input_loop_end

    MUL R0, R1
    ADD R0, R2
    JMP @input_loop

    @input_loop_end
    LDA R1, 0u
    LDA R2, 1u
    LDA R3, 1u

    @fibonacci_loop
    EQ R0, R3
    JCMP @fibonacci_loop_end
    ADD R2, R1
    SUB R2, R1, R1
    SUB R0, R3
    JMP @fibonacci_loop
    @fibonacci_loop_end

    OUTN R2
