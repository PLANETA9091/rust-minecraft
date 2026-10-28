#!/usr/bin/env python3
"""autopush-316 — revert the probe + stage the camera seat (the animate
block's own body_cell the camera proxy keeps — the furnace lanes' own
idiom the test never staged)."""

path = 'crates/mc-client/src/screens/tests/part_11.rs'
src = open(path).read()

PROBE = '''    // PROBE (temporary — the diagnostic the 0-peak answer needs)
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
}'''
assert PROBE in src, 'probe missing'
src = src.replace(PROBE, '''    assert!(
        gateway_puff_peak > 0,
        "the gateway the sampler hears: the peak {} portal puffs over 600 ticks",
        gateway_puff_peak
    );
}''', 1)

CAMERA_MARK = '''    // THE CUBE'S OWN BAKE (javap the facesToShow walk the mesher's
    // Y-face arm mirrors'''
CAMERA_STAGED = '''    // THE CAMERA'S OWN SEAT (the animate block's body_cell the camera
    // proxy keeps — the furnace lanes' own staged look, the gateway
    // the down-look's own cell the sampler reaches)
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(gx as f32 + 0.5, (py + 7) as f32, gz as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
''' + CAMERA_MARK
assert CAMERA_MARK in src, 'camera marker missing'
src = src.replace(CAMERA_MARK, CAMERA_STAGED, 1)

open(path, 'w').write(src)
print('reverted + camera staged')
