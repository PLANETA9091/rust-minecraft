#!/usr/bin/env python3
"""autopush-316 — the puff-arm probe patch: a temporary test that counts
every Portal particle + the gateway arm's own reach (the diagnostic walk
the failure's 0-peak answer needs). NOT part of the round's bank — the
probe runs, prints, and the patch reverts."""
import subprocess

PATCH = '''
    // PROBE (temporary — the diagnostic the 0-peak answer needs)
    let mut any_portal = 0usize;
    let mut portal_anchors: Vec<(i32, i32, i32)> = Vec::new();
    for _ in 0..600 {
        app.update();
        for p in app.particles.iter() {
            if let crate::particles::ParticleKind::Portal { sx, sy, sz, .. } = p.kind {
                any_portal += 1;
                if portal_anchors.len() < 8 {
                    portal_anchors.push((sx.floor() as i32, sy.floor() as i32, sz.floor() as i32));
                }
            }
        }
    }
    panic!(
        "PROBE: any_portal={any_portal} anchors={portal_anchors:?} gate_cell={gate_cell:?} body={:?}",
        app.body.as_ref().map(|b| (b.x, b.y, b.z))
    );
'''

path = 'crates/mc-client/src/screens/tests/part_11.rs'
src = open(path).read()
marker = '''    assert!(
        gateway_puff_peak > 0,
        "the gateway the sampler hears: the peak {} portal puffs over 600 ticks",
        gateway_puff_peak
    );
}'''
assert marker in src, 'marker missing'
src = src.replace(marker, PATCH + '}', 1)
open(path, 'w').write(src)
print('patched')
