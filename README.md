# metronorust
A simple CLI metronome written in rust

# How to use it?
Choose a bpm, a time signature (bar division) and a subdivision

## Examples

### 4 clicks in each bar, each click subdivided in 3 sub-clicks:

    metronorust --bpm 120 --time-signature 4 --subdiv 3

### 7 clicks in each bar, each click subdivided in 2 sub-clicks:

    metronorust --bpm 90 --time-signature 7 --subdiv 2