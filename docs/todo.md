## Security

Encryption has been implemented but the user has to scan a code every time they
connect. This could get annoying. Perhaps we can store the encryption key and
only make the user scan after some time has passed (e.g. a week).

## Tap controller expansion

The tap controller can access basically everything, but it's not possible to
save multiple configurations and switch between them.

## Running the server

You could open up the terminal and type in a command but that's not particularly
convenient. Investigate the alternates.
[This](https://apple.stackexchange.com/questions/376778/a-way-to-run-console-commands-in-menu-bar)
is a possibility.

## Android app

Port the iOS app to Android. Using a tool that can generate iOS and Android from
one source might work but since I've already written the iOS app, it would be
the same amount of effort. Also, using a generator might not give me full
control over the app. It just doesn't seem all that appealing.

## Deployment

Release the controller app on the App Store. Also release prebuilt binaries for
the server so that user's don't need to be programmers to use it.

## Maybe...

- Add another platform. I want this to be the best and most fully featured
  library of its kind on crates.io. I could consider targeting another operating
  system.
- Expand `ScreenContext` to handle multiple monitors.
