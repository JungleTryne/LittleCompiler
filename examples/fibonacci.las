data:
    .message "Please enter n to computer fibonacci number "

program:
    LDA R0, .message
    OUT R0

    INPN R0
    LDA R1, 0u
    LDA R2, 1u
    LDA R3, 1u

    @fibonacci_loop
    EQ R0, R3
    JCMP @fibonacci_loop_end
    ADD R2, R1, R2
    SUB R2, R1, R1
    SUB R0, R3, R0
    JMP @fibonacci_loop
    @fibonacci_loop_end

    OUTN R2
    FIN
