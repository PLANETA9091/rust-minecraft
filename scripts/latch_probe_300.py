#!/usr/bin/env python3
"""THE LATCH'S OWN DEDICATED REGION probe (autopush-300 round).

Q1: the vanilla chest/normal.png sheet — the lock region's painted rows:
    the north band (1..3, 1..5) vs the south band (4..6, 1..5) vs the
    west (0..1,1..5) / east (3..4,1..5) — which px the latch art wears.
Q2: the 26.2 jar ChestModel.createSingleBodyLayer — the lock child's
    addBox dims + texOffs + pivot (the live disasm re-pin).
"""
import struct, zlib, sys, subprocess, os

JAR = os.path.expanduser("~/.gradle")  # unused; jar lives in repo
JAR = "/home/z/my-project/rust-minecraft/.mc2rs/classpath/client.jar"
SHEET = "assets/minecraft/textures/entity/chest/normal.png"

def read_jar_entry(jar, name):
    import zipfile
    with zipfile.ZipFile(jar) as z:
        return z.read(name)

def decode_png_rgba(data):
    from PIL import Image
    import io
    im = Image.open(io.BytesIO(data)).convert("RGBA")
    return im.width, im.height, im.tobytes()

def px_name(p):
    r, g, b, a = p
    if a == 0:
        return "....."
    if (r, g, b) == (0, 0, 0):
        return "BLACK"
    if r > 200 and g > 200 and b > 200:
        return "WHITE"
    return f"{r:02x}{g:02x}{b:02x}"

def band_ascii(w, h, rgba, x0, x1, y0, y1, label):
    print(f"== {label}  x{x0}..{x1}  y{y0}..{y1} ==")
    for y in range(y0, y1):
        row = []
        for x in range(x0, x1):
            i = ((y * w) + x) * 4
            row.append(px_name(rgba[i:i+4]))
        print(f"  y={y:2d}: " + " ".join(row))

def main():
    raw = read_jar_entry(JAR, SHEET)
    w, h, rgba = decode_png_rgba(raw)
    print(f"sheet {SHEET}: {w}x{h}")
    # the lock box (2,4,1) @ texOffs(0,0) the box-UV walk answers:
    #   up (1,0)-(3,1)  down (3,0)-(4,1)  west (0,1)-(1,5)
    #   north (1,1)-(3,5)  east (3,1)-(4,5)  south (4,1)-(6,5)
    band_ascii(w, h, rgba, 0, 8, 0, 6, "the lock's own box-UV seat (cols 0..8)")
    # the lid's front band (14..28, 14..19) the top row the seam speaks
    band_ascii(w, h, rgba, 14, 20, 14, 19, "the lid front's left edge (cols 14..20)")

    # Q2: the live disasm re-pin
    cj = os.path.join(os.path.dirname(JAR), "client.jar")
    print("\n== javap ChestModel ==")
    for tool in ("javap", "/usr/lib/jvm/*/bin/javap"):
        pass
    rc = subprocess.run(
        ["javap", "-p", "-c", "-cp", cj,
         "net.minecraft.client.model.geom.builders.ChestModel"],
        capture_output=True, text=True)
    if rc.returncode != 0:
        # class name guess — hunt the jar for ChestModel*
        zc = subprocess.run(["python3", "-c", """
import zipfile
with zipfile.ZipFile({jar!r}) as z:
    for n in z.namelist():
        if 'ChestModel' in n:
            print(n)
""".format(jar=JAR)], capture_output=True, text=True)
        print(zc.stdout or zc.stderr)
    else:
        out = rc.stdout
        keep = False
        for line in out.splitlines():
            if "createSingleBodyLayer" in line or "setupAnim" in line:
                keep = True
            if keep:
                print(line)
        # the lock's addBox + offset the bytecode speaks
        for marker in ("addBox", "PartPose", "offset", "texOffs"):
            pass
    sys.exit(0)

if __name__ == "__main__":
    main()
