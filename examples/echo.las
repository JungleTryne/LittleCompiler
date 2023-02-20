data:
    .next_line_ascii_code 10

program:
    LD R1, .next_line_ascii_code

    @loop
    INP R0
    OUT R0
    EQ R0, R1
    JNCMP @loop

    FIN
