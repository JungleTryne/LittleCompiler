data:
    .message "Please enter n to computer fibonacci number "
    .next_line_ascii_code 10
    .multiplier 10
    .zero_char_ascii_code 48

program:
    OUT .message

    LD R3, .zero_char_ascii_code
    INP R0
    SUB R0, R3
    LD R1, .multiplier

    @input_loop
    LD R3, .zero_char_ascii_code
    INP R2
    SUB R2, R3

    LD R3, .next_line_ascii_code
    EQ R2, R3
    JCMP @input_loop_end

    MUL R0, R1
    ADD R0, R2
    JMP @input_loop

    @input_loop_end
    LDA R1, 0
    LDA R2, 1
    LDA R3, 1

    @fibonacci_loop
    EQ R0, R3
    JCMP @fibonacci_loop_end
    SUM R2, R1
    SUB R2, R1, R1
    SUB R0, R3
    JMP @fibonacci_loop
    @fibonacci_loop_end

    OUTRN R2
