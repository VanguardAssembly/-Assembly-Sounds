# Assembly Sounds — Binaural Beats Generator

![Assembly Sounds](branding/logo_wide.png)

*(formerly "Focus")* · by Vanguard Assembly

A native Windows app written in **Rust** that generates binaural, monaural and isochronic beats in real time. It includes guided sessions modelled on the Monroe Institute's *Gateway Experience* Focus levels, and a library of **132 sessions** drawn from published research and popular frequency traditions.

## Features
- **Real-time Rust audio engine** (cpal/WASAPI). Tones are phase-continuous and glide smoothly between stages, with up to 8 layered tone pairs per session.
- **Three beat types:** binaural (headphones), monaural, and isochronic (works on speakers and above 40 Hz).
- **Natural neural narration:** spoken cues recorded with the open-source [Kokoro-82M](https://github.com/hexgrad/kokoro) model, in four voices. They're embedded in the exe, so no internet is needed.
- **Noise beds:** pink, brown, white, and ocean surf.
- **Breath pacer** (6 breaths/min), stage timeline with seek, and a stereo Lissajous visualiser.
- **Sound textures:** frame drum, singing bowls with theta-rate beating, Kirtan/lucid-dream/breath chimes, 0.8 Hz sleep pulses and rain, plus a steady (never-flickering) Ganzfeld red-light screen.
- **Custom generator:** carrier 40–1000 Hz, beat 0.1–250 Hz, optional delta anchor layer, timer.

### Session library
| Group | Count | Notes |
|---|---|---|
| Gateway Experience | 5 | Focus 3, 10, 12, 15, 21 |
| Lifeline (Focus 22–27) | 4 | Interpretations, labelled speculative |
| Research-based | 17 | Each cites its study (Lane 1998, Jirakittayakorn 2017/2018, Lutz 2004, Voss 2014, Lacaux 2021, Martorell 2019 …) |
| **Altered States** | 10 | Ranked by research volume: self-hypnosis, Ganzfeld (red light), drumming trance, lucid-dream TLR, slow-wave sleep pulses, Kirtan Kriya, singing bowls, mantra rhythm, rain soundscape, Om |
| Purpose & Tradition | 15 | OBE, remote viewing, shamanic, Om, sleep, study, energy … |
| Brainwave Library | 42 | Every common frequency from 0.5 Hz epsilon to 200 Hz "lambda" |
| Schumann Resonances | 6 | Modes 1–5 and a layered stack |
| Solfeggio | 10 | All 9 tones and a full-scale journey |
| Chakra Series | 8 | 7 chakras and a root-to-crown journey |
| Cosmic Octave | 12 | Hans Cousto planetary tones |
| Tunings | 3 | 432 / 440 / 256 Hz |

Every preset is tagged **Research**, **Gateway**, **Traditional** or **Speculative**, so you always know how much evidence is behind it.

### Also
- **Android app** (Android 8.0+): the same engine and sessions with a phone layout (Sessions / Player / Info tabs) and a lock-screen media player. Built by `.github/workflows/android.yml`.
- **Remembers your settings:** narrator voice, volumes, noise, beat type, breath pacer, custom generator values and the last session you played are saved in `settings.json` in the app's data folder.
- **VHS-style intro** on launch (tap or press Esc to skip; no flashing).

## Safety & privacy
- **No network access.** The UI runs under a strict Content-Security-Policy (`default-src 'none'`), and the app makes no web requests and collects no telemetry.
- **Harmful methods excluded:** flicker light (seizure risk), infrasound and hyperventilation techniques are deliberately not included.
- **No admin rights** needed. Settings live in `%LOCALAPPDATA%\FocusBinaural`.
- **Hard output limiter:** output can never exceed about −1.4 dBFS, whatever the slider settings.
- **Custom input is clamped:** carriers 20–1500 Hz, beats ≤ 250 Hz, layers ≤ 8.
- **Health warning:** not for use while driving. People with epilepsy, seizure disorders or heart conditions, or who are pregnant, should talk to a doctor first. This is not medical treatment.

## Building
```powershell
# MSVC toolchain: produces one self-contained exe
cargo build --release --target x86_64-pc-windows-msvc
```
With the MinGW (`windows-gnu`) toolchain, the exe also needs `WebView2Loader.dll` (Microsoft-signed) next to it.

### Regenerating voice cues (optional, build-time only)
```powershell
FocusBinaural.exe --dump-cues tools/cues.json
python tools/gen_voices.py kokoro-v1.0.onnx voices-v1.0.bin   # needs kokoro-onnx, soundfile, ffmpeg
```

## Evidence summary
A 2019 meta-analysis (Garcia-Argibay et al., *Psychological Research*) found small-to-moderate effects of binaural beats on anxiety, memory, attention and pain perception. EEG studies (e.g. Orozco Perez et al., 2020, *eNeuro*) found little evidence that brainwaves actually lock onto the beat. The Gateway presets are interpretations: Monroe's Hemi-Sync frequency mixes are proprietary.
