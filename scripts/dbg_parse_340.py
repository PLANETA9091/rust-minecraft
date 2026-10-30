import re

s = open('crates/mc-client/src/brewing_sim.rs').read()
colors = re.search(
    r'pub const MOB_EFFECT_COLORS: &\[\(&str, u32\)\] = &\[(.*?)\n\];', s, re.S
)
print('colors block found:', bool(colors))
color_names = set(re.findall(r'\("(\w+)", 0x', colors.group(1)))
eff = re.search(
    r'pub const POTION_KIND_EFFECTS: \[&\[\(&str, i32\)\]; 46\] = \[(.*?)\n\];',
    s,
    re.S,
)
print('effects block found:', bool(eff))
used = set(re.findall(r'\("(\w+)", -?\d+\)', eff.group(1)))
missing = used - color_names
print('color rows:', len(color_names), '| effect keys used:', len(used))
print('MISSING:', missing if missing else 'none')
base = re.search(
    r'pub const POTION_KIND_BASE_NAMES: \[&str; 46\] = \[(.*?)\];', s, re.S
).group(1)
base_names = re.findall(r'"(\w+)"', base)
print('base names:', len(base_names), base_names[:6], '...', base_names[-3:])
