program:
    LDA R1, 10u

    @loop
    INP R0
    OUTR R0
    EQ R0, R1
    JNCMP @loop

    FIN
