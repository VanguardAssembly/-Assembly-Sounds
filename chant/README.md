Group chants of mantras and prayers, rendered at build time by `tools/gen_chants.py`
(Kokoro-82M voices, re-pitched and re-timed into chant with the WORLD vocoder) and committed by
the `chants.yml` workflow. Each `<name>.ogg` is a stereo loop that the app embeds and plays as a
session's accompaniment (`chant` kind in `src/music.rs`).
