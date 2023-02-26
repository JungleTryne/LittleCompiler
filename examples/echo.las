program:
    LDA R1, 10u

    @loop
    INP R0
    OUT R0
    EQ R0, R1
    JNCMP @loop

    FIN
