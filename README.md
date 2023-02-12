# `wav-maker`

`wav-maker` is, at the moment, a very barebones script for creating a WAV file
of a simple piece of music, given an input file as an argument. This input file
is expected to be organized in a bespoke format described below---`wav-maker`
will return with an error if the file I/O fails, or with the first syntax error
in the input file, if such errors exist.

The input file format is fairly simple, but rather specific:

- The first line should just be a floating-point number by itself; this will
correspond to the tempo of the output in beats per minute (BPM), where 1 quarter
note is assumed to fill 1 beat.
- All subsequent lines correspond to a note being played for a duration, and are
divided into five columns separated by whitespace:
    - The time at which this note is played, expressed as the number of
    16ths-of-a-beat (64th-note beats; called "ticks" in the code) since the
    beginning of the audio;
    - The duration of the signal, expressed by an abbreviation for the type of
    note (e.g. `Q` for "quarter"---complete list below), or simply as the number
    of ticks;
    - The note name in scientific pitch notation (e.g. `A4` or `C#5`), with
    limitations: both `D#4` and `Eb4` are valid and enharmonic, for example,
    but at the moment, `Cb3` or `F##2` would be an error;
    - A scaling factor for the amplitude of the note, expressed as a
    floating-point integer---1.0 represents the base amplitude, which is a bit
    quiet in the current version of the code;
    - The type of waveform to use for the note, expressed by an abbreviation for
    the name of that waveform. Currently four types are supported: sine (`S`),
    square (`Q`), sawtooth (`A`), or triangle (`T`).

The note duration abbreviations are as follows:

| Abbreviation | Name           | Ticks (16ths-of-a-beat) |
|--------------|----------------|-------------------------|
| `TS`         | 32nd           | 2                       |
| `DTS`        | Dotted 32nd    | 3                       |
| `S`          | 16th           | 4                       |
| `DS`         | Dotted 16th    | 6                       |
| `E`          | 8th            | 8                       |
| `DE`         | Dotted 8th     | 12                      |
| `Q`          | Quarter        | 16                      |
| `DQ`         | Dotted quarter | 24                      |
| `H`          | Half           | 32                      |
| `DH`         | Dotted half    | 48                      |
| `W`          | Whole          | 64                      |

My current intention is to give the user more control over things like the base
amplitude of note waveforms. Ideally, I'd like to move to having an *optional*
header line of parameters like BPM, with sensible defaults and with the ability
to override these parameters using command-line flags.

A medium-term improvement I'd like to make is to add some kind of looping
construct for sequences of notes to the language. That will require significant
reworking of the parsing, as the present implementation is very naive, but will
have a massive benefit in terms of expressiveness.

## Example

Suppose the file `lick` contains the following contents:

```
BPM 120
AMPL 2048
0  S D4  1.02 T
4  S E4  1.0  T
8  S F4  1.02 T
12 S G4  0.99 T
16 E E4  1.05 T
24 S C4  0.95 T
28 Q D4  1.03 T
```

Then running `wav-maker lick` will create the file `lick.wav`, which will be a
somewhat stiff rendition of [The Lick](https://www.youtube.com/watch?v=krDxhnaKD7Q)
in D minor at 120 BPM, using triangle waves.

## Why?

I just wanted something with which I could somewhat quickly template out a
musical idea, like a melody or a chord progression, and preserve it for later,
in a way that doesn't require recording with an instrument, fiddling with MIDI
or musical notation software, or trying to write tablature in a way that also
marks rhythm consistently. This isn't at present meant to be a softsynth with
any meaningful capability to make interesting textures and music, just a sort of
"sketch renderer" for smaller ideas.
