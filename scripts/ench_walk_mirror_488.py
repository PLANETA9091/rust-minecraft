#!/usr/bin/env python3
"""ench_walk_mirror_488 — the Python twin the treasure walk's pins
compute (the autopush-488 round's own mirror — the registry tables the
generated file parses, the walk the exact draws the pins answer).
Usage: python3 scripts/ench_walk_mirror_488.py"""
import math
import re
import struct

SRC = open('crates/mc-registry/src/generated/enchantments.rs').read()
ROW = re.compile(r'EnchDef \{ name: "([^"]+)", weight: (\d+), max_level: (\d+), '
                 r'min_cost: \((\d+), (\d+)\), max_cost: \((\d+), (\d+)\), '
                 r'exclusive: (None|Some\("[^"]+"\)), supported: "([^"]+)", '
                 r'primary: (None|Some\("[^"]+"\)), anvil_cost: \d+ \}')
ENCH = []
for m in ROW.finditer(SRC):
    name, w, ml, mnb, mnp, mxb, mxp, exc, sup, pri = m.groups()
    ENCH.append(dict(name=name, weight=int(w), max_level=int(ml),
                     min_cost=(int(mnb), int(mnp)), max_cost=(int(mxb), int(mxp)),
                     exclusive=None if exc == 'None' else exc[6:-2],
                     supported=sup, primary=None if pri == 'None' else pri[6:-2]))
IDX = {d['name']: i for i, d in enumerate(ENCH)}
FAM = {}
fam_block = SRC[SRC.index('ENCHANTABLE_FAMILIES'):SRC.index('ITEM_ENCHANTABILITY')]
for m in re.finditer(r'\("([a-z_]+)", &\[([^\]]*)\]\)', fam_block):
    fam, items = m.groups()
    FAM[fam] = [x.strip().strip('"') for x in items.split(',') if x.strip()]
TABLE = re.findall(r'"([^"]+)"', re.search(
    r'TABLE_MEMBERS: \[&str; 36\] = \[(.*?)\]', SRC, re.S).group(1))
IE = dict((n, int(v)) for n, v in re.findall(
    r'\("([a-z_]+)", (\d+)\)', SRC[SRC.index('ITEM_ENCHANTABILITY'):]))
EXS = []
for m in re.finditer(r'\("([a-z_]+)", &\[([^\]]*)\]\)', SRC[SRC.index('EXCLUSIVE_SETS'):]):
    t, mem = m.groups()
    EXS.append((t, [x.strip().strip('"') for x in mem.split(',') if x.strip()]))
POOL = [IDX[n] for n in TABLE] + [IDX[n] for n in
                                  ["binding_curse", "vanishing_curse", "frost_walker", "mending"]]
MASK = (1 << 64) - 1


def f32(x):
    return struct.unpack('f', struct.pack('f', x))[0]


class Lcg:
    def __init__(self, seed):
        self.state = (seed ^ 0x5EED_C0FF_EE12_3456) & MASK

    def next(self):
        self.state = (self.state * 0x5851F42D4C957F2D + 0x14057B7EF767814F) & MASK
        return self.state >> 16

    def next_f32(self):
        return (self.next() % (1 << 24)) / float(1 << 24)

    def next_int(self, bound):
        if bound == 0:
            return 0
        return self.next() % bound


def accepts(name, item):
    d = ENCH[IDX[name]]
    if item not in FAM.get(d['supported'], []):
        return False
    if d['primary'] is None:
        return True
    return item in FAM.get(d['primary'], [])


def excl(tag, name):
    if tag is None:
        return False
    return any(t == tag and name in mem for t, mem in EXS)


def are_compatible(a, b):
    if a == b:
        return False
    da, db = ENCH[a], ENCH[b]
    return not (excl(da['exclusive'], db['name']) or excl(db['exclusive'], da['name']))


def cost_at(curve, level):
    return curve[0] + curve[1] * (level - 1)


def select_power(r, levels, ench):
    bound = ench // 4 + 1
    lvl = levels + 1 + r.next_int(bound) + r.next_int(bound)
    f = f32(f32(f32(f32(r.next_f32()) + f32(r.next_f32())) - 1.0) * 0.15)
    lf = f32(float(lvl))
    scaled = f32(lf + f32(lf * f))
    return max(1, min(2 ** 31 - 1, math.floor(f32(scaled + 0.5))))


def walk(item, levels, seed):
    r = Lcg(seed)
    ench_v = IE.get(item, 0)
    if ench_v <= 0:
        return [], r, 0
    power = select_power(r, levels, ench_v)
    is_book = item == 'book'
    avail = []
    for idx in POOL:
        d = ENCH[idx]
        if is_book or accepts(d['name'], item):
            lvl = d['max_level']
            while lvl >= 1:
                if power >= cost_at(d['min_cost'], lvl) and power <= cost_at(d['max_cost'], lvl):
                    avail.append((idx, lvl, d['weight']))
                    break
                lvl -= 1
    picked = []
    if not avail:
        return picked, r, power
    weights = [w for _, _, w in avail]
    keep = list(range(len(avail)))

    def pick(ws):
        total = sum(ws)
        if total == 0:
            return None
        i = r.next_int(total)
        for k, w in enumerate(ws):
            i -= w
            if i < 0:
                return k
        return None

    k = pick(weights)
    if k is not None:
        picked.append(k)
    p = power
    while True:
        if r.next_int(50) > p:
            break
        if picked:
            last = picked[-1]
            keep = [c for c in keep if are_compatible(avail[last][0], avail[c][0])]
        if not keep:
            break
        sub = [weights[i] for i in keep]
        k = pick(sub)
        if k is not None:
            picked.append(keep[k])
        p //= 2
    return [(avail[k][0], avail[k][1]) for k in picked], r, power


NAMES = [d['name'] for d in ENCH]


def main():
    for item in ('bow', 'fishing_rod', 'book', 'cod', 'dirt'):
        for seed in (513, 514, 515, 521, 525, 527):
            out, _, power = walk(item, 30, seed)
            pairs = [(NAMES[i], lvl) for i, lvl in out]
            print(f"{item} seed {seed} power={power}: picks={pairs}")
    for s in range(200):
        walk('bow', 30, s)
        walk('book', 30, s)
        walk('fishing_rod', 30, s)
    multi = sum(1 for s in range(200) if len(walk('bow', 30, s)[0]) > 1)
    print(f"bow 200-seed multi-pick rate: {multi}/200")
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
