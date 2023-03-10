data:
    .hello_world "Hello world"

program:
    LDA R1, 5u
    LDA R0, 0u
    LDA R2, 1u

    @subprogram
    LDA R3, .hello_world
    OUT R3
    OUTN R1

    SUB R1, R2, R1
    OUTN R1

    EQ R1, R0
    
    JCMP @if_end
    call @subprogram
    @if_end

    ret

    FIN
