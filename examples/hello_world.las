data:
    .hello_world "Hello world"

program:
    LDA R0, .hello_world
    OUT R0

    FIN
