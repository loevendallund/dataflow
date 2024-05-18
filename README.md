# Dataflow analysis implementation
Small implementation for my dataflow analysis masters project made at Aalborg University.

The program uses `box_into_inner`, and by the time the program were developed it requires rust nightly.

## Using the command line
The syntax for the program is as follows `./dataflow {file} {evaluate} {show_debug}`, where the arguments are as follows:
- {file} (required): the path to the file to read
- {evaluate} (optional): boolean, Should the program be evaluated? (instead of only type checked)
- {show_debug} (optional): boolean, should the debug messages be shown?
