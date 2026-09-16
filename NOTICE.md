# NOTICE — what this repository does and does not distribute

## The short law

The project's OWN code (the generator, the engine, the client, the
tests) is MIT-licensed — see `LICENSE`. **Minecraft's assets, data
files, and binary code are NOT licensed under that MIT line, and this
repository must not be read as distributing them.**

- *Minecraft* is a trademark of Mojang Synergies AB. This project is
  NOT an official Minecraft product, is NOT approved by or associated
  with Mojang or Microsoft, and is a from-scratch reimplementation for
  interop study.
- The official client jar (and every texture, sound, font glyph,
  translation string, splash, colormap, and model JSON inside it) is
  and stays Mojang's copyrighted property, distributed only through
  Mojang's own endpoints under Mojang's own terms. The generator
  downloads the official jar AT RUNTIME from Mojang's published
  metadata URL (`piston-meta`) and extracts what it needs locally.
  The CI's "Cache Mojang downloads" step caches that fetch — it never
  sources the jar from this repository.
- The rendered PNGs under `goldens/` and `shots/` are screenshots of
  the reimplementation at work, shared under the same usage rights
  Mojang grants for in-game screenshots and footage.

## The exposure audit (the migration in progress)

The repository is mid-migration AWAY from committing jar-derived
material. The state this NOTICE speaks:

| Material | Where | Status |
| --- | --- | --- |
| javap disassembly of `net.minecraft` classes (`scripts/*.txt`) | untracked as of v0.3.200 | **REMOVED from git** — regenerate with `python3 scripts/classfile_dump.py <client.jar> <class-path> > scripts/<name>.txt`; the legal walks live baked into the Rust doc comments |
| generated registry tables compiled into the binary (`crates/mc-registry/src/generated/`) — textures (the atlas/entity skins as raw RGBA8), font glyph bitmaps, colormap, blockstate/model JSON | still committed (v0.3.200; **splashes REMOVED as of v0.3.202; lang REMOVED as of v0.3.204; sounds REMOVED as of v0.3.205**) | **MIGRATION IN PROGRESS** — the first three lanes landed (v0.3.202, v0.3.204, v0.3.205): the splashes table lives OFF the compiled source — the jar's `texts/splashes.txt` the generator copies verbatim into the runtime cache (`assets-cache/jar/assets/minecraft/texts/splashes.txt`), the client's loader parses it at boot; the lang table the same walk speaks (v0.3.204) — the jar's `lang/en_us.json` (519KB) the runtime cache keeps verbatim (`assets-cache/jar/assets/minecraft/lang/en_us.json`), the client's loader the only parser (8123 entries the flat + nested dotted-key walk collects), the 552KB `generated/lang.rs` off the published tree; the sound-event table the third lane (v0.3.205) — the official `sounds.json` (612KB, asset-index fetch sha1-verified) the generator copies verbatim into the runtime cache (`assets-cache/jar/assets/minecraft/sounds.json`), the client's loader the only parser (1968 events / 8024 file refs the old 1MB `generated/sounds.rs` held), the parse law the loader's own walk speaks; the remaining lanes land the same walk: the tables move out of the compiled source into locally-generated cache files the loader reads at boot, so a fresh clone builds and runs from the jar the user/CI fetches itself |
| rendered screenshots (`goldens/`, `shots/`) | committed | kept — screenshots are the permitted-use lane; they are derivative *renderings*, not asset data files |
| hand-written Rust (models, algorithms, packet codecs, UI) | committed | the project's own MIT work — behavior reimplementation, not copied code |

## The rule every future round keeps

**Nothing extracted verbatim from an official jar may be committed to
this repository.** The generator's output stays in local caches
(`.mc2rs/`, `assets-cache/` — both gitignored). If a lane needs a new
table, the pipeline generates it at build/runtime from the jar, and
the review checks the diff for jar-derived bytes before they reach
git.

## Takedown / contact posture

If you believe any file in this repository infringes rights you hold,
the intended resolution is removal on request — the project's purpose
is interoperability study, and every jar-derived byte it once carried
is regenerable from Mojang's own distribution.
