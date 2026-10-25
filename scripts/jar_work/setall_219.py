#!/usr/bin/env python3
"""autopush-219 recon: the setAll in-game-focus walk — the world-entry
lane the KeyMapping.setAll the isKeyDown re-read serves: WHERE the call
site lives (setScreen? runTick? grabMouse?), the whole setAll body the
release+re-read law keeps, the shouldSetOnIngameFocus the Screen class
the override map the lane answers."""
import struct
import sys

sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')

# 1. KeyMapping.setAll body
ms = methods_of(z, 'net.minecraft.client.KeyMapping')
print("=== KeyMapping methods (set*/isDown/release):")
for sig in sorted(ms):
    if sig.startswith('set') or 'elease' in sig or 'isDown' in sig:
        print("  ", sig)
for sig in sorted(ms):
    if sig.startswith('setAll') or 'eleaseAll' in sig:
        cp, body = ms[sig]
        if body is None:
            print(f"--- KeyMapping.{sig}: abstract")
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- KeyMapping.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print("  ", l)
        print()

# 2. callers of setAll across the client's own classes
print("=== setAll / releaseAll callers:")
TARGETS = ('net.minecraft.client.Minecraft', 'net.minecraft.client.MouseHandler',
           'net.minecraft.client.KeyboardHandler', 'net.minecraft.client.KeyMapping')
for cls in TARGETS:
    ms2 = methods_of(z, cls)
    if not ms2:
        continue
    for sig, (cp2, body) in sorted(ms2.items()):
        if body is None:
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        try:
            lines = walk_code(cp2, code)
        except Exception:
            continue
        hits = [l for l in lines if '.setAll' in l or '.releaseAll' in l]
        if hits:
            print(f"  {cls}.{sig}:")
            for h in hits:
                print("   ", h)

# 3. shouldSetOnIngameFocus — which classes declare/override it
print("\n=== shouldSetOnIngameFocus declarations:")
SCAN = ('net.minecraft.client.Minecraft', 'net.minecraft.client.gui.screens.Screen',
        'net.minecraft.client.gui.screens.ChatScreen',
        'net.minecraft.client.gui.screens.options.OptionsScreen',
        'net.minecraft.client.gui.screens.options.controls.ControlsScreen',
        'net.minecraft.client.gui.screens.inventory.AbstractContainerScreen',
        'net.minecraft.client.gui.screens.TitleScreen')
for cls in SCAN:
    ms3 = methods_of(z, cls)
    if not ms3:
        continue
    for sig in sorted(ms3):
        if 'ngameFocus' in sig or 'nFocus' in sig:
            print("  ", cls, "->", sig)

# 4. Minecraft.setScreen whole walk (the world-entry lane)
ms4 = methods_of(z, 'net.minecraft.client.Minecraft')
cp, body = ms4['setScreen(Lnet/minecraft/client/gui/screens/Screen;)V']
clen = struct.unpack_from('>I', body, 4)[0]
code = body[8:8 + clen]
print(f"\n--- Minecraft.setScreen ({len(code)} bytes)")
for l in walk_code(cp, code):
    print("  ", l)
