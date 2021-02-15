## TCP

It might make sense to use TCP directly rather than using web sockets. This
would involve dropping a lot of working code. It may improve performance
slightly.

[Using `NWFramework`](https://rderik.com/blog/building-a-server-client-aplication-using-apple-s-network-framework/)

## Security

Currently, the only form of security is only allowing one client to connect to
the server. We could have the client and server maintain random number
generators and send a random number with each message. The message would have to
match the number from the server for the server to accept the message.

This would involve establishing a shared secret. The server would need to
generate the seed and let the user transfer this to their phone. Perhaps using
a QR code.

Something like this could also be used for encryption. The sequence of random
numbers could be used as the key for an XOR stream cipher.

## Tap controller expansion

Allowing the tap controller to access all commands could be interesting. Moving
the mouse around and clicking things at the tap of a button would be pretty
cool. I'm not sure exactly what it could be used for though. Doing this would
require reworking to configuration screen.

If this was to be implemented, then it would make sense to be able to save and
load multiple configurations. This would be a big overhaul of the configuration
page.

[This](https://stackoverflow.com/questions/2855857/how-to-display-multiple-columns-in-a-uitableview)
might be helpful.

## Running the server

You could open up the terminal and type in a command but that's not particularly
convenient. Investigate the alternates.
[This](https://apple.stackexchange.com/questions/376778/a-way-to-run-console-commands-in-menu-bar)
is a possibility.

## Better tab bar

Currently, the tab bar has six tabs. Apple's guidelines say that you can only
have five before you need to use a "more" button that shows a list of the rest.
I really don't like the "more" button. It's very unpleasant to use. It would be
better if the tab bar was scrollable. This is especially necessary if we add
another tab. Six is cramped. Seven and beyond will be unusable.
[Here](https://stackoverflow.com/questions/8482661/how-to-make-a-horizontal-scrollable-uitabbar-in-ios)
are some options.

## Android app

Port the iOS app to Android. Using a tool that can generate iOS and Android from
one source might work but since I've already written the iOS app, it would be
the same amount of effort. Also, using a generator might not give me full
control over the app. It just doesn't seem all that appealing.

## Support Linux

I think there are different Windowing systems or something... That might mean
implementing this for Linux requires multiple modules. I'll obviously need to
look into this.

## Unicode

The ability to type any character that can be typed without worrying about the
keyboard layout would be really useful. Being able to type any unicode character
(even ones that can't be typed) is even better but these two things have
different implementations.

On Windows, you can send unicode characters directly.
[SO](https://stackoverflow.com/a/38625599/4093378).
There's also
[`VkKeyScanW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-vkkeyscanw)
for mapping characters to key codes.

On Linux, you can use `CTRL+SHIFT+U+<HEX>` to enter a unicode character. You
would still need to worry about keyboard layout for the `0123456789ABCDEF` keys.
Also, it seems to be application specific and isn't built into the operating
system. Windows has a similar thing. macOS has it too but it needs to be enabled
in settings. Translating characters to keycodes is possible with
[X11](https://stackoverflow.com/a/42691752/4093378).
That has the disadvantage of not working everywhere.
[`dumpkeys`](https://man7.org/linux/man-pages/man1/dumpkeys.1.html)
might help. Maybe
[`EVIOCGKEYCODE`](https://github.com/torvalds/linux/blob/358feceebbf68f33c44c6650d14455389e65282d/include/uapi/linux/input.h#L99-L121).

On macOS, it's possible to convert characters to a keycode and set of modifiers.
[SO](https://stackoverflow.com/questions/1918841/how-to-convert-ascii-character-to-cgkeycode).
For sending arbitrary unicode characters,
[`CGEventKeyboardSetUnicodeString`](https://developer.apple.com/documentation/coregraphics/1456028-cgeventkeyboardsetunicodestring)
can be used but this has the problem of not responding to modifiers. Also,
`CGEventKeyboardSetUnicodeString` only accepts up to 20 characters at once so if
we want to send strings, we'd need to split them up. Even splitting strings is
non-trivial because we'd need to split them by graphemes. Probably need a
library for that.

There are really two problems to solve here. There's generating key codes
independent of the keyboard layout and generating arbitrary unicode characters.
Both of these require different solutions. It seems like it might be relatively
easy to solve this for Windows. macOS is a little more difficult but still
doable. Linux is what I'm not sure about.
