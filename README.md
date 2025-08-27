i more or less finished this in 2 days! what a triumph!! _(get it? get it?)_

its what it says on the tin. portal's still alive, but in your terminal. i put good effort (maybe too much good effort) into making this as close to the original as possible while under the limitations of a terminal, which means a lot of time spent on premiere pro watching a recording of it frame by frame.

speaking of which, i made the mistake of doing half of the lyrics while interpreting the premiere timecode of mins:secs:frames as mins:secs:millis (to the nearest 10), making me waste a more than few hours pulling my hair out over why it just wouldnt sync.

and of course, working with the terminal was hard. i saw many many ansi control codes written to the screen instead of being executed before i discovered rust has the amazing feature of locking the stdout stream between threads! (how does rust always have some awesome feature for the problem?)

efficiency was not prioritised here, mainly because just getting the timings and threads and whatever else to cooperate was hard enough.
