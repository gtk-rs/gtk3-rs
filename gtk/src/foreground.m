// Take a look at the license at the top of the repository in the LICENSE file.

#import <AppKit/AppKit.h>

void macos_force_foreground_level() {
    [NSApp activateIgnoringOtherApps: YES];
}
