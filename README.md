# PAIR - Pinatrace Annotate in Rust

This project was initially started by Frits Hoogland - you can read about it here: 

https://fritshoogland.wordpress.com/2017/12/22/introduction-to-pinatrace-annotate-version-2-a-look-into-latches-again/

PAIR was created to make the whole process faster. Right now it annotate instruction pointers with symbols obtained from a provided binary. 

##USAGE

USAGE:
    pair --binary-file <BINARY_FILE> --pinatrace-file <PINATRACE_FILE> --output-file <OUTPUT_FILE>

OPTIONS:
    -b, --binary-file <BINARY_FILE>          Binary file to get symbols
    -h, --help                               Print help information
    -o, --output-file <OUTPUT_FILE>          Output file to be created with annotations
    -p, --pinatrace-file <PINATRACE_FILE>    Output file from pinatrace to annotate
    -V, --version                            Print version information
