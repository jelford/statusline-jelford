# statusline-jelford

A very simple status line for my own PCs
Intended to be used with Sway's statusbar as:

    bar {
        status_command statusline --continuous
	...
    }

Note: there are no options for customization; the line prints
just what I want in my statusbar. Changes are done in the code,
and require a re-compile.

# License

MIT (see LICENSE for details)

# Arguments

    --continuous[=INTERVAL]    run forever, printing out a newline every INTERVAL seconds (default: 1)

