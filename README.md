# Assembly Sounds — Binaural Beats Generator

![Assembly Sounds](branding/logo_wide.png)

*(formerly "Focus")* · by Vanguard Assembly

A native Windows app written in **Rust** that generates binaural, monaural, isochronic, AM, hybrid, bilateral and spatial (8D) beats in real time. It includes guided sessions inspired by the Monroe Institute's *Gateway Experience* Focus levels, and a library of **221 sessions** drawn from published research, contemplative traditions and popular frequency traditions. Every session opens with its own spoken "settle in" cue written for that practice.

## Features

- **Sound Systems Library**: one session for every known way of delivering a rhythm to the brain: binaural, monaural, isochronic, smooth AM tones, hybrid binaural + isochronic, bilateral left↔right (EMDR-style), spatial/8D rotation, layered (Hemi-Sync-inspired) beats, descending carriers (Holosync-inspired), coloured noise and drumming. Any session can be played through any beat type.
- **Astral Projection & OBE** (23 sessions): Monroe's vibrational state and roll-out, Robert Bruce's rope technique, Muldoon's target technique, Raduga's phase technique and technique cycling, wake-back-to-bed, theta–gamma stack, spatial floating, bilateral calming, the researched induction methods (hypnotic suggestion and imagined viewpoint after Facco 2019, kinesthetic floating after Smith & Messier 2014, a 1970s laboratory-style Ganzfeld induction, SSILD), the swing and Christos techniques, Golden Dawn rising on the planes and tattva vision (with on-screen tattva cards), Tibetan phowa and dream yoga, shamanic upper- and middle-world journeys, and after-practice grounding — each labelled by evidence with the real neuroscience explained.
- **Spiritual & Mystical Experiences** (17 sessions): centering prayer, Tibetan image visualisation, free vocalisation, jhāna absorption, nondual awareness, self-inquiry, effortless mantra, a dry floatation-REST, a sober mystical-experience arc, Sufi dhikr, the Jesus Prayer, nāda yoga, vipassana body sweeping, the psychomanteum, the Taoist microcosmic orbit, hitbodedut and an inner-guide journey, each with its history and what research has (and hasn't) found.
- **Opt-in experimental features**: ⚡ **Light journeys** (flicker-light closed-eye visuals at ~10 Hz, screen or Android flashlight, synced to the beat; includes a Sweet-Spot Sweep, Ganzflicker and Dreamachine-inspired sessions), 🌫 **Infrasound** layer (18.98 Hz "ghost frequency", 17 Hz, 7 Hz…), and 🌬 **Breathwork** (Wim Hof-style power breathing rounds and conscious connected breathing with an on-screen guide). Each requires accepting a full warning first.
- **Research and practitioner presets for light and breath**: every light session has a matching protocol in the Light journey panel, picked automatically: Bartossek 2021 (10 Hz, 3 Hz and steady-light conditions), Amaya 2023 (30% duty, rhythmic and arrhythmic blocks), the original eyes-open Ganzflicker (asks again every time, brightness capped), a 3–12 Hz frequency scan, Lucia-style flicker with steady glow, a Dreamachine, audio-visual entrainment and a phone-flashlight journey. Every breathing session has its own guide: the Kox 2014 study protocol, Stanford's cyclic hyperventilation and cyclic sighing (Balban 2023), box breathing with a CO₂ tolerance test, 4-7-8, coherent breathing, bhramari, nadi shodhana, gentle and forceful tummo, kapalabhati (Telles 2011), bhastrika (fast and Pramanik's slow pace), breath of fire, a Sudarshan Kriya-inspired sequence, rebirthing and classical 1:4:2 ratio breathing. Gentle techniques need no warning; fast breathing and long holds still do.
- **Windows XP / 2006 Y2K look**: Luna title bars, task-pane session list, glossy buttons, chrome visualiser bezel (the visualiser itself is unchanged).
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
| Inspired by the Gateway Experience | 5 | Focus 3, 10, 12, 15, 21 |
| Inspired by Lifeline (Focus 22–27) | 4 | Interpretations, labelled speculative |
| Research-based | 17 | Each cites its study (Lane 1998, Jirakittayakorn 2017/2018, Lutz 2004, Voss 2014, Lacaux 2021, Martorell 2019 …) |
| **Altered States** | 10 | Ranked by research volume: self-hypnosis, Ganzfeld (red light), drumming trance, lucid-dream TLR, slow-wave sleep pulses, Kirtan Kriya, singing bowls, mantra rhythm, rain soundscape, Om |
| Purpose & Tradition | 15 | OBE, remote viewing, shamanic, Om, sleep, study, energy … |
| Sound Systems Library | 11 | Every beat delivery method, side by side |
| Astral Projection & OBE | 23 | Researched inductions plus traditional and modern techniques |
| Spiritual & Mystical Experiences | 17 | Contemplative practices from Christian, Buddhist, Hindu, Sufi, Jewish and Taoist traditions |
| Light Journeys (Flicker, opt-in) | 15 | Study protocols and practitioner styles |
| Infrasound (opt-in) | 4 | 18.98, 17, 7 and 7.83 Hz |
| Breathwork (opt-in) | 10 | Hyperventilation and breath-hold protocols |
| Breathing Techniques | 9 | Slow, gentle methods; no warning needed |
| Brainwave Library | 42 | Every common frequency from 0.5 Hz epsilon to 200 Hz "lambda" |
| Schumann Resonances | 6 | Modes 1–5 and a layered stack |
| Solfeggio | 10 | All 9 tones and a full-scale journey |
| Chakra Series | 8 | 7 chakras and a root-to-crown journey |
| Cosmic Octave | 12 | Hans Cousto planetary tones |
| Tunings | 3 | 432 / 440 / 256 Hz |

Every preset is tagged **Research**, **Gateway-inspired**, **Traditional** or **Speculative**, so you always know how much evidence is behind it.

### Also
- **Android app** (Android 8.0+): the same engine and sessions with a phone layout (Sessions / Player / Info tabs) and a lock-screen media player. Built by `.github/workflows/android.yml`.
- **Remembers your settings:** narrator voice, volumes, noise, beat type, breath pacer, custom generator values and the last session you played are saved in `settings.json` in the app's data folder.
- **VHS-style intro** on launch (tap or press Esc to skip; no flashing).

## Safety & privacy
- **No network access.** The UI runs under a strict Content-Security-Policy (`default-src 'none'`), and the app makes no web requests and collects no telemetry.
- **Higher-risk methods are opt-in only:** flicker light (⚡ Light journey), infrasound (🌫) and hyperventilation breathwork (🌬) never run by default. Each has its own warning screen; every box must be ticked (health questions, safe setting, 18+, at-your-own-risk acknowledgement) before it can be used. Flicker is capped at 3–12 Hz (below the 15–25 Hz seizure-risk peak), including irregular "arrhythmic" timing, and two flashes are never closer than 1/12 s on any screen; it ramps in, goes dark between study blocks and at the end, auto-stops, and stops instantly on any tap or key or when the app is left. Infrasound is quiet (capped well below the tones) with slow fades. Breath holds are never forced.
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
Or render them in the cloud: commit `tools/cues.json`, then run the **Narration** workflow (`gh workflow run voices.yml`), which renders each voice in its own job and commits the new clips.

## Evidence summary
A 2019 meta-analysis (Garcia-Argibay et al., *Psychological Research*) found small-to-moderate effects of binaural beats on anxiety, memory, attention and pain perception. EEG studies (e.g. Orozco Perez et al., 2020, *eNeuro*) found little evidence that brainwaves actually lock onto the beat. The Gateway presets are interpretations: Monroe's Hemi-Sync frequency mixes are proprietary.
