"""Build-time tool: render the group chants (mantras, prayers, sacred names) the sessions play
as accompaniment, into ../chant/<name>.ogg (stereo Ogg Vorbis loops, embedded in the app).

How a chant is made:
  1. Kokoro-82M (Apache-2.0, https://github.com/hexgrad/kokoro) speaks every syllable of the text
     from IPA phonemes, in several voices (Hindi voices for Sanskrit, Japanese for Japanese...).
  2. The WORLD vocoder (pyworld) analyses each syllable and sings it again: the vowel (or the
     final m/n/ng of a mantra) is held for the length of its note, the speaking pitch is
     replaced by the melody, with each singer's own glide, vibrato, tuning drift, formant shade,
     timing and breath.
  3. The singers are panned across the stereo field and mixed into a loop whose end runs
     straight into its start, with a circular hall reverb, so the clip repeats without a seam.
     Loops are whole numbers of beats, so a chant stays in step with the session's wooden fish,
     cymbals or drum (the app plays every layer from one clock).
All melodies are original, written for this app in the style of each tradition; the texts are
traditional (centuries old) and in the public domain.

Usage:
  python tools/gen_chants.py <kokoro-v1.0.onnx> <voices-v1.0.bin> [--shard i/n] [--out DIR] [name ...]
  python tools/gen_chants.py --mock [--out DIR] [name ...]   # quick test with a buzz "voice", no models

Only chants that are missing, or whose definition changed (chant/index.json keeps a hash of
each), are rendered, unless names are given.

Score syntax (one string per chant, tokens separated by spaces, "|" marks a phrase for reading):
  PHON:NOTE:BEATS[:FLAGS]
  PHON   IPA phonemes of one syllable (Kokoro's symbol set), or "_" for a rest
  NOTE   semitones above the tonic; "0/1,2/1" = a melisma (note/beats, ...);
         "4;0;-12" = notes for harmony parts 0, 1, 2 (see "parts")
  BEATS  length in beats
  FLAGS  h = hold the final nasal (m, n, ng), f = stretch the hiss too (s, sh), w = whispered,
         b = breathy, > = accent, s = soft, L = leader alone (call and response)
"""
import hashlib, json, math, os, subprocess, sys, tempfile, wave
import numpy as np

SR = 24000
FP = 5.0                        # WORLD frame period, ms
HOP = int(SR * FP / 1000)       # samples per frame
OFFSET = 0.4                    # s of lead-in before beat 0 (rolled away at the end)
TAIL = 3.0                      # s rendered past the loop end, wrapped back onto the start
GEN = "chant-gen-3"             # bump to re-render every chant after changing the algorithm
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# ------------------------------------------------------------------ voices
EN_M = ["am_michael", "bm_george", "am_onyx", "am_adam", "bm_lewis", "am_eric", "bm_daniel", "am_liam", "am_echo", "am_puck"]
EN_F = ["af_heart", "bf_emma", "af_bella", "af_nicole", "bf_isabella", "af_sarah", "bf_alice", "af_aoede", "af_kore", "bf_lily"]
POOLS = {  # (men, women): voices whose accent suits the language, then English voices
    "en": (EN_M, EN_F),
    "in": (["hm_omega", "hm_psi", "am_michael", "bm_george", "am_onyx", "bm_lewis", "am_adam", "bm_daniel"],
           ["hf_alpha", "hf_beta", "af_heart", "bf_emma", "af_nicole", "bf_isabella"]),
    "tb": (["am_onyx", "hm_omega", "bm_george", "hm_psi", "am_michael", "bm_daniel", "am_adam", "bm_lewis"],
           ["hf_alpha", "af_heart", "hf_beta", "bf_emma", "af_nicole"]),
    "ja": (["jm_kumo", "am_onyx", "bm_george", "am_michael", "bm_daniel", "am_adam", "bm_lewis"],
           ["jf_alpha", "jf_gongitsune", "jf_nezumi", "jf_tebukuro", "af_heart"]),
    "zh": (["zm_yunjian", "zm_yunxi", "zm_yunxia", "zm_yunyang", "am_onyx", "bm_george"],
           ["zf_xiaobei", "zf_xiaoni", "zf_xiaoxiao", "zf_xiaoyi", "af_heart"]),
    "la": (["im_nicola", "em_alex", "em_santa", "bm_george", "am_michael", "bm_lewis", "am_onyx", "bm_daniel"],
           ["if_sara", "ef_dora", "bf_emma", "af_heart", "af_nicole"]),
    "ar": (["hm_omega", "am_onyx", "hm_psi", "bm_george", "am_michael", "bm_daniel", "am_adam", "bm_lewis"],
           ["hf_alpha", "af_heart", "hf_beta", "bf_emma"]),
}

DEFAULTS = dict(tonic=110.0, beat=0.5, reps=1, pool="en", men=6, women=3, parts=None, vib=8.0, vib_rate=5.2,
                glide=0.07, scoop=20.0, spread=0.03, detune=6.0, rt60=2.6, wet=0.3, formant=1.0, tilt=0.0,
                breath=0.0, speed=0.82, gain=0.0, cluster=None, hum=1.5, lead_f=False)


# ------------------------------------------------------------------ score
def parse(score):
    out = []
    for tok in score.split():
        if tok == "|":
            continue
        f = tok.split(":")
        ph = f[0]
        beats = float(f[2]) if len(f) > 2 and f[2] else 1.0
        flags = f[3] if len(f) > 3 else ""
        if ph == "_":
            out.append(dict(rest=True, beats=beats))
            continue
        parts = []
        for pn in (f[1] if len(f) > 1 and f[1] not in ("", "_") else "0").split(";"):
            sub = []
            for it in pn.split(","):
                if "/" in it:
                    a, b = it.split("/")
                    sub.append([float(a), float(b)])
                else:
                    sub.append([float(it), None])
            known = sum(b for _, b in sub if b is not None)
            unk = [i for i, (_, b) in enumerate(sub) if b is None]
            for i in unk:
                sub[i][1] = max(beats - known, 0.0) / len(unk)
            parts.append([(a, b) for a, b in sub])
        out.append(dict(rest=False, ph=ph, beats=beats, flags=flags, parts=parts))
    return out


def plan(ch):
    """Every item of the loop with its start (beats); sung items also get the next onset."""
    ev = parse(ch["score"])
    items, t = [], 0.0
    for r in range(ch["reps"]):
        for e in ev:
            items.append(dict(e, t=t, rep=r))
            t += e["beats"]
    total = t
    n = len(items)
    for i, it in enumerate(items):
        if it["rest"]:
            continue
        nx = items[(i + 1) % n]
        nt = nx["t"] + (total if i + 1 >= n else 0.0)
        it["next_t"] = nt
        it["next_rest"] = nx["rest"]
        it["next_ph"] = None if nx["rest"] else nx["ph"]
        it["first"] = i == 0 or items[i - 1]["rest"]
    return items, total


def fix_ph(ph):
    ph = ph.replace("g", "ɡ").replace("'", "ˈ").replace("ʱ", "ʰ")  # Kokoro's IPA g (U+0261); no breathy-voice mark
    if "ˈ" not in ph:
        ph = "ˈ" + ph
    return ph


# ------------------------------------------------------------------ speech -> syllable model
class Syl:
    """One spoken syllable, analysed: log spectral envelope, aperiodicity, voicing, where the
    vowel starts, how much each frame may be stretched, and how nasal each frame is."""
    def __init__(self, f0, sp, ap):
        n = min(len(f0), sp.shape[0], ap.shape[0])
        f0, sp, ap = f0[:n], sp[:n], ap[:n]
        e = np.sqrt(sp.mean(axis=1))
        keep = np.nonzero(e > e.max() * 0.02)[0]
        a, b = (keep[0], keep[-1] + 1) if len(keep) else (0, n)
        a, b = max(a - 1, 0), min(b + 2, n)
        f0, sp, ap, e = f0[a:b], sp[a:b], ap[a:b], e[a:b]
        self.n = len(f0)
        self.voiced = f0 > 0
        if self.voiced.sum() < 3:       # nothing voiced found: treat the loud part as the vowel
            self.voiced = e > e.max() * 0.25
        self.on = int(np.argmax(self.voiced))
        self.logsp = np.log(sp + 1e-12)
        self.ap = ap
        self.e = e / (e.max() + 1e-12)
        flux = np.r_[0.0, np.abs(np.diff(self.logsp, axis=0)).mean(axis=1)]
        v = self.voiced
        med = np.median(flux[v]) if v.any() else 1.0
        w = np.exp(-flux / (med + 1e-9)) * v * np.clip(self.e * 3.0, 0, 1)
        self.w = w
        B = sp.shape[1]
        k1, k4 = int(1000 / (SR / 2) * (B - 1)), int(4000 / (SR / 2) * (B - 1))
        r = sp[:, k1:k4].sum(axis=1) / (sp[:, 1:k1].sum(axis=1) + 1e-12)
        self.nas = np.clip((0.06 - r) / 0.05, 0, 1) * v
        self.hiss = np.clip(self.e * 4.0, 0, 1) * (~v)


def stretch_map(s, n_out, fric=False, hum=0.0):
    """Source frame (fractional) for each output frame of the syllable body: steady parts of the
    vowel (and, with `hum`, the nasal) are held, transitions and consonants keep their speed."""
    on = 0 if fric else s.on
    nb = max(s.n - on, 1)
    w = s.w[on:].copy()
    if fric:
        w = np.maximum(w, 0.8 * s.hiss[on:])
    if hum:
        w = w * (1.0 + 3.0 * hum * s.nas[on:])
    if n_out >= nb:
        tot = w.sum()
        m = 1.0 + ((n_out - nb) * w / tot if tot > 1e-9 else (n_out - nb) / nb)
    else:
        m = np.full(nb, n_out / nb)
    edges = np.r_[0.0, np.cumsum(m)]
    j = np.arange(n_out) + 0.5
    src = np.interp(j, edges, np.arange(nb + 1, dtype=float)) + on - 0.5
    return np.clip(src, on, s.n - 1)


def at(arr, src):
    i0 = np.floor(src).astype(int)
    i0 = np.clip(i0, 0, len(arr) - 1)
    i1 = np.minimum(i0 + 1, len(arr) - 1)
    fr = (src - i0)
    if arr.ndim == 2:
        fr = fr[:, None]
    return arr[i0] * (1 - fr) + arr[i1] * fr


class Speech:
    """Kokoro TTS + WORLD analysis, cached per (phonemes, voice)."""
    def __init__(self, model, voices, mock=False):
        self.mock = mock
        self.cache = {}
        self.warned = set()
        if mock:
            self.names = None
            self.vocab = None
            return
        from kokoro_onnx import Kokoro
        import pyworld
        self.pw = pyworld
        self.k = Kokoro(model, voices)
        try:
            self.names = set(self.k.get_voices())
        except Exception:
            self.names = None
        vocab = getattr(getattr(self.k, "tokenizer", None), "vocab", None)
        self.vocab = set(vocab.keys()) if isinstance(vocab, dict) else None

    def voice_ok(self, v):
        return self.names is None or v in self.names

    def tts(self, ph, voice, speed):
        if self.vocab is not None:
            bad = {c for c in ph if c not in self.vocab}
            if bad and ph not in self.warned:
                self.warned.add(ph)
                print(f"  ! symbols not in Kokoro's set, dropped: {''.join(sorted(bad))} in {ph}", flush=True)
        try:
            x, sr = self.k.create(ph, voice=voice, speed=speed, lang="en-us", is_phonemes=True)
        except TypeError:
            x, sr = self.k.create(ph, voice=voice, speed=speed, lang="en-us")
        x = np.asarray(x, dtype=np.float64)
        if sr != SR:
            n = int(round(len(x) * SR / sr))
            x = np.interp(np.linspace(0, len(x) - 1, n), np.arange(len(x)), x)
        return x

    def get(self, ph, voice, speed):
        key = (ph, voice, speed)
        if key in self.cache:
            return self.cache[key]
        if self.mock:
            s = Syl(*mock_analyse(ph, voice))
        else:
            x = self.tts(ph, voice, speed)
            if len(x) < SR * 0.08:      # too short: ask again, slower and with a long vowel
                x = self.tts(ph + "ː", voice, speed * 0.8)
            x = x / (np.abs(x).max() + 1e-9) * 0.5
            x = np.ascontiguousarray(x)
            f0, t = self.pw.harvest(x, SR, f0_floor=55.0, f0_ceil=700.0, frame_period=FP)
            sp = self.pw.cheaptrick(x, f0, t, SR)
            ap = self.pw.d4c(x, f0, t, SR)
            s = Syl(f0, sp, ap)
        self.cache[key] = s
        return s

    def synth(self, f0, sp, ap):
        if self.mock:
            return mock_synth(f0, sp, ap)
        return self.pw.synthesize(np.ascontiguousarray(f0), np.ascontiguousarray(sp), np.ascontiguousarray(ap), SR, FP)


def mock_analyse(ph, voice):
    """A fake syllable (consonant noise, then a buzzy vowel) for testing without the models."""
    h = int(hashlib.md5((ph + voice).encode()).hexdigest()[:8], 16)
    n = 50 + h % 30
    c = ph.lstrip("ˈ")[:1]
    cons = 9 if c in "ptkshfθʃχʦʧbdɡ" else 0
    f0 = np.r_[np.zeros(cons), np.full(n - cons, 110.0 + h % 40)]
    B = 513
    f = np.linspace(0, SR / 2, B)
    f1 = 500 + h % 300
    env = np.exp(-((f - f1) / 160) ** 2) + 0.5 * np.exp(-((f - 1300) / 250) ** 2) + 0.25 * np.exp(-((f - 2600) / 300) ** 2) + 2e-3
    sp = np.tile(env * 1e-3, (n, 1))
    sp[:cons] *= 0.05
    if ph.rstrip("ː").endswith(("m", "n", "ŋ")):
        sp[-12:, 120:] *= 0.002
    ap = np.full((n, B), 0.05)
    ap[:cons] = 1.0
    return f0, sp, ap


def mock_synth(f0, sp, ap):
    n = len(f0) * HOP
    ff = np.repeat(f0, HOP)[:n]
    phs = np.cumsum(2 * np.pi * ff / SR)
    amp = np.repeat(np.sqrt(sp.mean(axis=1)), HOP)[:n] * 20.0
    buzz = np.sign(np.sin(phs)) * 0.3
    noise = np.random.default_rng(1).standard_normal(n) * 0.1
    return np.where(ff > 0, buzz, noise) * amp


# ------------------------------------------------------------------ singing
def lowpass_noise(rng, n, frames_tc, std):
    """Slow random wander: one-pole low-passed noise scaled to `std`."""
    x = rng.standard_normal(n)
    a = math.exp(-1.0 / max(frames_tc, 1.0))
    y = np.empty(n)
    acc = 0.0
    for i in range(n):
        acc = a * acc + (1 - a) * x[i]
        y[i] = acc
    sd = y.std()
    return y / sd * std if sd > 0 else y


def singers(ch, speech, rng):
    """[(voice, part, transpose, is_leader)] for this chant."""
    men_pool, wom_pool = POOLS[ch["pool"]]
    men_pool = [v for v in men_pool if speech.voice_ok(v)] or EN_M
    wom_pool = [v for v in wom_pool if speech.voice_ok(v)] or EN_F
    parts = ch["parts"] or [("m", ch["men"], 0, 0), ("f", ch["women"], 0, 12)]
    out = []
    mi = fi = 0
    for g, cnt, part, tr in parts:
        for _ in range(cnt):
            if g == "m":
                v = men_pool[mi % len(men_pool)]
                mi += 1
            else:
                v = wom_pool[fi % len(wom_pool)]
                fi += 1
            out.append([v, part, tr, False])
    if ch["lead_f"]:
        k = next((i for i, s in enumerate(out) if s[2] >= 12), 0)
    else:
        k = next((i for i, s in enumerate(out) if s[2] < 12), 0)
    out.insert(0, out.pop(k))
    out[0][3] = True
    if ch["cluster"]:
        for s in out:
            s[2] += float(rng.choice(ch["cluster"]))
    return [tuple(s) for s in out]


def sing(ch, items, L, singer, idx, n_singers, speech, rng):
    """One singer's line for the whole loop: mono samples, ~L + TAIL long."""
    voice, part, transpose, leader = singer
    beat = ch["beat"]
    N = int((L + TAIL * SR) / HOP) + 2
    B = None
    acc_sp = acc_ap = None
    acc_w = np.zeros(N)
    acc_v = np.zeros(N)
    acc_wh = np.zeros(N)
    acc_b = np.zeros(N)
    gain = np.zeros(N)
    changes = []        # (frame, semitone)
    onsets = []         # frames where a new syllable starts (vibrato and scoop restart)
    sj = rng.uniform(-ch["spread"] / 2, ch["spread"] / 2) if not leader else 0.0
    lead_boost = 0.6 * math.sqrt(n_singers) if n_singers > 3 else 1.0
    for it in items:
        if it["rest"]:
            continue
        fl = it["flags"]
        if "L" in fl and not leader:
            continue
        if not leader and n_singers >= 5 and not it["first"] and rng.random() < 0.04:
            continue    # this singer takes a breath here
        ph = fix_ph(it["ph"])
        s = speech.get(ph, voice, ch["speed"])
        if B is None:
            B = s.logsp.shape[1]
            acc_sp = np.zeros((N, B))
            acc_ap = np.zeros((N, B))
        jit = sj + rng.normal(0, ch["spread"] / 3)
        t_on = OFFSET + it["t"] * beat + jit
        t_next = OFFSET + it["next_t"] * beat + jit
        on_f = t_on * SR / HOP
        fric = "f" in fl
        if it["next_rest"]:
            end_f = t_next * SR / HOP + 3
        else:
            nxt = speech.get(fix_ph(it["next_ph"]), voice, ch["speed"])
            end_f = t_next * SR / HOP - min(nxt.on, 30) + 2
        n_out = max(int(round(end_f - on_f)), 3)
        cons = 0 if fric else s.on
        cn = min(cons, 30)
        src_c = np.linspace(0, cons, cn, endpoint=False) if cons else np.zeros(0)
        src_b = stretch_map(s, n_out, fric, ch["hum"] if "h" in fl else 0.0)
        src = np.r_[src_c, src_b]
        f0i = int(round(on_f)) - cn
        frames = f0i + np.arange(len(src))
        ok = (frames >= 0) & (frames < N)
        if not ok.any():
            continue
        ls = at(s.logsp, src)
        ap = at(s.ap, src)
        vv = s.voiced[np.clip(np.round(src).astype(int), 0, s.n - 1)].astype(float)
        m = len(src)
        w = np.ones(m)
        fi, fo = min(3, m // 3), min(5, m // 3)
        if fi:
            w[:fi] = np.linspace(0.2, 1, fi)
        if fo:
            w[-fo:] = np.linspace(1, 0.05, fo)
        lvl = 1.0 * (1.3 if ">" in fl else 1.0) * (0.55 if "s" in fl else 1.0)
        if "L" in fl and leader:
            lvl *= lead_boost
        fr = frames[ok]
        acc_sp[fr] += w[ok, None] * ls[ok]
        acc_ap[fr] += w[ok, None] * ap[ok]
        acc_v[fr] += w[ok] * vv[ok]
        acc_wh[fr] += w[ok] * ("w" in fl)
        acc_b[fr] += w[ok] * ("b" in fl)
        acc_w[fr] += w[ok]
        gain[fr] = np.maximum(gain[fr], w[ok] * lvl)
        # the melody: this part's notes, a melisma moving within the held vowel
        notes = it["parts"][part] if part < len(it["parts"]) else it["parts"][0]
        tt = t_on
        for semi, b in notes:
            changes.append((tt * SR / HOP - ch["glide"] * 0.5 * SR / HOP, semi))
            tt += b * beat
        onsets.append(on_f)
    if B is None:
        return np.zeros(int(L + TAIL * SR))
    has = acc_w > 1e-6
    logsp = np.full((N, B), math.log(1e-12))
    logsp[has] = acc_sp[has] / acc_w[has, None]
    ap = np.ones((N, B))
    ap[has] = acc_ap[has] / acc_w[has, None]
    voiced = np.zeros(N, bool)
    voiced[has] = acc_v[has] / acc_w[has] > 0.5
    whisper = np.zeros(N, bool)
    whisper[has] = acc_wh[has] / acc_w[has] > 0.5
    breathy = np.zeros(N, bool)
    breathy[has] = acc_b[has] / acc_w[has] > 0.5

    # ---- pitch curve: steps at each note, glides, scoops, vibrato, drift
    changes.sort()
    semi = np.zeros(N)
    if changes:
        cf = np.array([c[0] for c in changes])
        cv = np.array([c[1] for c in changes])
        k = np.searchsorted(cf, np.arange(N), side="right") - 1
        semi = np.where(k >= 0, cv[np.clip(k, 0, len(cv) - 1)], cv[0])
    g = ch["glide"] * SR / HOP
    if g > 0.5:
        a = math.exp(-1.0 / g)
        y = np.empty(N)
        acc = semi[0]
        for i in range(N):
            acc = a * acc + (1 - a) * semi[i]
            y[i] = acc
        semi = y
    on = np.array(sorted(onsets)) if onsets else np.array([0.0])
    k = np.searchsorted(on, np.arange(N), side="right") - 1
    since = (np.arange(N) - on[np.clip(k, 0, len(on) - 1)]) * HOP / SR
    since[k < 0] = 0
    cents = np.zeros(N)
    if ch["scoop"]:
        cents -= ch["scoop"] * np.exp(-since / 0.06)
    if ch["vib"]:
        rate = ch["vib_rate"] * rng.uniform(0.92, 1.08)
        depth = ch["vib"] * rng.uniform(0.7, 1.3) * np.clip((since - 0.3) / 0.4, 0, 1)
        cents += depth * np.sin(2 * np.pi * rate * np.arange(N) * HOP / SR + rng.uniform(0, 6.28))
    cents += rng.uniform(-ch["detune"], ch["detune"]) * 0.5
    cents += lowpass_noise(rng, N, 0.8 * SR / HOP, ch["detune"])
    cents += lowpass_noise(rng, N, 0.02 * SR / HOP, 2.5)
    f0 = ch["tonic"] * 2.0 ** ((semi + transpose) / 12.0 + cents / 1200.0)
    f0 = np.where(voiced & ~whisper, f0, 0.0)

    # ---- timbre: formant shade (slowly moving), tilt, breath
    alpha = ch["formant"] * rng.uniform(0.96, 1.04) * (1 + lowpass_noise(rng, N, 0.5 * SR / HOP, 0.008))
    kk = np.arange(B)[None, :] / alpha[:, None]
    kk = np.clip(kk, 0, B - 1)
    k0 = np.floor(kk).astype(int)
    k1 = np.minimum(k0 + 1, B - 1)
    frk = kk - k0
    rows = np.arange(N)[:, None]
    logsp = logsp[rows, k0] * (1 - frk) + logsp[rows, k1] * frk
    del kk, k0, k1, frk
    if ch["tilt"]:
        f = np.linspace(0, SR / 2, B)
        logsp += (ch["tilt"] / 8.686) * np.log2(np.maximum(f, 1000.0) / 1000.0)[None, :]
    br = ch["breath"]
    if br:
        ap = 1 - (1 - ap) * (1 - br)
    ap[breathy] = 1 - (1 - ap[breathy]) * 0.45
    ap[whisper] = 1.0
    ap = np.clip(ap, 0.0, 1.0)
    y = speech.synth(f0, np.exp(logsp), ap)
    gs = np.interp(np.arange(len(y)), np.arange(N) * HOP, gain)
    y = y * gs
    act = gs > 0.3
    rms = np.sqrt(np.mean(y[act] ** 2)) if act.any() else 0.0
    if rms > 0:
        y = y / rms * 0.1
    return y


# ------------------------------------------------------------------ room and master
def hall_ir(rt60, n, rng, pre=0.018):
    t = np.arange(n) / SR
    F = np.fft.rfftfreq(n, 1 / SR)
    edges = [0, 200, 500, 1000, 2000, 4000, 8000, SR]
    mult = [1.2, 1.1, 1.0, 0.9, 0.72, 0.52, 0.35]
    ir = np.zeros((2, n))
    for c in range(2):
        Nf = np.fft.rfft(rng.standard_normal(n))
        acc = np.zeros(n)
        for (a, b), m in zip(zip(edges[:-1], edges[1:]), mult):
            acc += np.fft.irfft(Nf * ((F >= a) & (F < b)), n) * np.exp(-6.9078 * t / (rt60 * m))
        acc *= np.clip(t / 0.015, 0, 1)
        p = int(pre * SR)
        acc = np.r_[np.zeros(p), acc[:n - p]]
        sd = acc[p:p + int(0.05 * SR)].std() + 1e-9
        for _ in range(7):
            d = p + int(SR * rng.uniform(0.004, 0.05))
            if d < n:
                acc[d] += rng.uniform(1.5, 4.0) * sd * (1 if rng.random() < 0.5 else -1)
        ir[c] = acc / math.sqrt((acc ** 2).sum())
    return ir


def eq(x, hp=75.0, lp=10500.0):
    n = x.shape[-1]
    X = np.fft.rfft(x, axis=-1)
    f = np.fft.rfftfreq(n, 1 / SR)
    H = 1 / np.sqrt(1 + (hp / np.maximum(f, 1e-3)) ** 4) / np.sqrt(1 + (f / lp) ** 8)
    return np.fft.irfft(X * H, n, axis=-1)


def soft(x, knee=0.5, ceil=0.85):
    a = np.abs(x)
    w = ceil - knee
    return np.sign(x) * np.where(a <= knee, a, knee + w * np.tanh((a - knee) / w))


def render(name, ch, speech):
    rng = np.random.default_rng(int(hashlib.sha1(name.encode()).hexdigest()[:8], 16))
    items, total = plan(ch)
    L = int(round(total * ch["beat"] * SR))
    sg = singers(ch, speech, rng)
    pans = np.linspace(-0.75, 0.75, max(len(sg) - 1, 1)) if len(sg) > 1 else np.zeros(1)
    rng.shuffle(pans)
    loop = np.zeros((2, L))
    for i, s in enumerate(sg):
        y = sing(ch, items, L, s, i, len(sg), speech, rng)
        p = 0.0 if s[3] else float(pans[(i - 1) % len(pans)])
        lv = 1.0 if s[3] else rng.uniform(0.82, 1.12)
        gl, gr = math.cos((p + 1) * math.pi / 4) * lv, math.sin((p + 1) * math.pi / 4) * lv
        for a in range(0, len(y), L):
            seg = y[a:a + L]
            loop[0, :len(seg)] += seg * gl
            loop[1, :len(seg)] += seg * gr
    loop = np.roll(loop, -int(round(OFFSET * SR)), axis=1)
    # circular hall: the reverb tail of the end rings over the start, as it would in a real loop
    n_ir = min(int(ch["rt60"] * 1.6 * SR), L)
    ir = hall_ir(ch["rt60"], n_ir, rng)
    mono = loop.mean(axis=0)
    M = np.fft.rfft(mono)
    wet = np.stack([np.fft.irfft(M * np.fft.rfft(ir[c], L), L) for c in range(2)])
    rl = np.sqrt(np.mean(loop ** 2)) + 1e-12
    wet *= rl / (np.sqrt(np.mean(wet ** 2)) + 1e-12) * ch["wet"] * 1.6
    out = loop * (1 - 0.35 * ch["wet"]) + wet
    out = eq(out)
    out *= 0.125 / (np.sqrt(np.mean(out ** 2)) + 1e-12) * 10 ** (ch["gain"] / 20)
    out = soft(out)
    return out, len(sg), sum(1 for it in items if not it["rest"])


def write_ogg(x, path):
    tmp = tempfile.mkdtemp()
    wav = os.path.join(tmp, "c.wav")
    pcm = np.clip(np.round(x.T * 32767), -32768, 32767).astype("<i2")
    with wave.open(wav, "wb") as w:
        w.setnchannels(2)
        w.setsampwidth(2)
        w.setframerate(SR)
        w.writeframes(pcm.tobytes())
    if path.endswith(".wav"):
        os.replace(wav, path)
        return
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-i", wav, "-c:a", "libvorbis", "-q:a", "2",
                    "-ar", str(SR), "-ac", "2", path], check=True)


def spec_of(name):
    ch = dict(DEFAULTS)
    ch.update(CHANTS[name])
    return ch


def digest(ch):
    return hashlib.sha1((GEN + json.dumps(ch, sort_keys=True, ensure_ascii=False)).encode()).hexdigest()[:16]


def main():
    args = sys.argv[1:]
    mock = "--mock" in args
    out_dir = os.path.join(ROOT, "chant")
    shard = (0, 1)
    rest = []
    i = 0
    while i < len(args):
        a = args[i]
        if a == "--mock":
            pass
        elif a == "--out":
            out_dir = args[i + 1]
            i += 1
        elif a == "--shard":
            x, y = args[i + 1].split("/")
            shard = (int(x), int(y))
            i += 1
        else:
            rest.append(a)
        i += 1
    if mock:
        model = voices = None
        names = rest
    else:
        model, voices, names = rest[0], rest[1], rest[2:]
    os.makedirs(out_dir, exist_ok=True)
    idx_path = os.path.join(ROOT, "chant", "index.json")
    old = json.load(open(idx_path, encoding="utf-8")) if os.path.exists(idx_path) else {}
    todo = []
    for n in sorted(CHANTS):
        if names and n not in names:
            continue
        h = digest(spec_of(n))
        have = os.path.exists(os.path.join(ROOT, "chant", n + ".ogg"))
        if names or not have or old.get(n) != h:
            todo.append((n, h))
    todo = [t for k, t in enumerate(todo) if k % shard[1] == shard[0]]
    print(f"{len(todo)} chant(s) to render: {' '.join(n for n, _ in todo)}", flush=True)
    if not todo:
        return
    speech = Speech(model, voices, mock)
    done = {}
    for n, h in todo:
        ch = spec_of(n)
        x, ns, nsyl = render(n, ch, speech)
        ext = ".wav" if mock else ".ogg"
        write_ogg(x, os.path.join(out_dir, n + ext))
        rms = 20 * math.log10(np.sqrt(np.mean(x ** 2)) + 1e-12)
        pk = 20 * math.log10(np.abs(x).max() + 1e-12)
        print(f"{n:16} {x.shape[1] / SR:6.2f} s  {x.shape[1]:8d} samples  {ns:2d} singers  {nsyl:3d} sung  rms {rms:6.1f} dB  peak {pk:5.1f} dB", flush=True)
        done[n] = h
    json.dump(done, open(os.path.join(out_dir, f"index-{shard[0]}.json"), "w", encoding="utf-8"), indent=1, sort_keys=True)


# ================================================================== the chants
# tonic = the men's pitch for note 0 (women sing an octave higher unless "parts" says otherwise);
# beat = seconds per beat; reps = times the score repeats inside one loop.
CHANTS = {
    # ---------------------------------------------------------- Tibetan Buddhism & Bön
    "mani": dict(  # Om Mani Peme Hung (Chenrezig), Tibetan pronunciation, low and slow
        pool="tb", tonic=98.0, beat=0.5, reps=2, men=6, women=3, vib=10, rt60=3.2, wet=0.34, formant=0.95,
        score="oːm:0:2:h mɑ:0:1 ni:3:1 pɛ:5:1 meː:3:1 huːŋ:0:2:h | oːm:0:2:h mɑ:0:1 ni:3:1 pɛ:5:1 meː:3:1 huːŋ:0:2:h"
              " | oːm:7:2:h mɑ:5:1 ni:3:1 pɛ:5:1 meː:3:1 huːŋ:0:2:h | oːm:0:2:h mɑ:0:1 ni:-2:1 pɛ:0:1 meː:-2:1 huːŋ:-5:2:h"),
    "tara": dict(  # Om Tare Tuttare Ture Soha (Green Tara)
        pool="tb", tonic=110.0, beat=0.45, reps=3, men=3, women=5, lead_f=True, vib=16, rt60=2.8,
        score="oːm:0:2:h tɑ:3:1 ɾeː:5:1 tʊt:7:1 tɑ:5:1 ɾeː:3:1 tʊ:5:1 ɾeː:3:1 soː:0:1 hɑː:0:2"
              " | oːm:7:2:h tɑ:10:1 ɾeː:7:1 tʊt:5:1 tɑ:7:1 ɾeː:5:1 tʊ:3:1 ɾeː:2:1 soː:0:1 hɑː:0:2"),
    "medicine": dict(  # Tayata Om Bekandze Bekandze Maha Bekandze Radza Samudgate Soha
        pool="tb", tonic=98.0, beat=0.32, reps=4, men=6, women=2, vib=0, scoop=0, glide=0.05, rt60=3.0, formant=0.96,
        score="tɑ:0:1 jɑ:0:1 tɑ:0:1 oːm:2:2:h | beː:0:1 kɑn:0:1 ʣeː:0:1 beː:0:1 kɑn:0:1 ʣeː:0:1 | mɑ:2:1 hɑː:0:1"
              " | beː:0:1 kɑn:0:1 ʣeː:0:1 | ɾɑ:0:1 ʣɑ:0:1 | sɑ:0:1 mʊd:0:1 ɡɑ:-2:1 teː:0:1 | soː:0:1 hɑː:-2:2 _:_:3"),
    "guru": dict(  # Om Ah Hung Benza Guru Pema Siddhi Hung (Padmasambhava)
        pool="tb", tonic=98.0, beat=0.4, reps=4, men=6, women=3, vib=8, rt60=3.0, formant=0.96,
        score="oːm:0:2:h ɑː:2:2 hʊŋ:0:2:h | bɛn:3:1 zɑ:2:1 ɡʊ:0:1 ɾʊ:2:1 | pɛ:3:1 mɑ:5:1 sid:3:1 di:2:1 | hʊŋ:0:4:h _:_:2"),
    "vajrasattva": dict(  # Om Benza Satto Hung (the short Vajrasattva mantra)
        pool="tb", tonic=98.0, beat=0.45, reps=7, men=5, women=3, vib=6, rt60=3.0, formant=0.96,
        score="oːm:0:2:h bɛn:0:1 zɑ:2:1 sɑt:3:1 toː:2:1 hʊŋ:0:2:h _:_:2"),
    "manjushri": dict(  # Om A Ra Pa Tsa Na Dhih, then Dhih repeated
        pool="tb", tonic=110.0, beat=0.4, reps=2, men=5, women=3, vib=8, rt60=2.8,
        score="oːm:0:2:h ɑ:3:1 ɾɑ:3:1 pɑ:5:1 ʦɑ:3:1 nɑ:2:1 diː:0:3 _:_:2 | oːm:0:2:h ɑ:3:1 ɾɑ:3:1 pɑ:5:1 ʦɑ:3:1 nɑ:2:1 diː:0:3 _:_:2"
              " | oːm:0:2:h ɑ:3:1 ɾɑ:3:1 pɑ:5:1 ʦɑ:3:1 nɑ:2:1 diː:0:3 _:_:2 | diː:0:1 diː:0:1 diː:0:1 diː:0:1 diː:-2:2 _:_:2"),
    "white_a": dict(  # the long white "A" of Bön and Dzogchen
        pool="tb", tonic=110.0, beat=0.5, reps=5, men=4, women=3, vib=4, scoop=0, rt60=3.5, wet=0.36,
        score="ɑː:0:8 _:_:4"),

    # ---------------------------------------------------------- Japanese and Chinese Buddhism
    "nembutsu": dict(  # Namu Amida Butsu, one syllable per wooden-fish knock (0.75 s)
        pool="ja", tonic=110.0, beat=0.75, reps=3, men=5, women=3, vib=0, scoop=0, glide=0.05, rt60=2.2,
        score="nɑ:0:1 mɯ:0:1 ɑ:0:1 mi:0:1 dɑ:2:1 bɯ:0:2 _:_:1 | nɑ:0:1 mɯ:0:1 ɑ:2:1 mi:2:1 dɑ:0:1 bɯ:-2:2 _:_:1"),
    "daimoku": dict(  # Nam-myoho-renge-kyo, brisk and even with the wooden fish (0.42 s)
        pool="ja", tonic=110.0, beat=0.42, reps=14, men=6, women=4, vib=0, scoop=0, glide=0.03, spread=0.02, rt60=1.8,
        score="nɑm:1:1:> mjoː:0:1 hoː:0:1 ɾeŋ:0:1 ɡe:0:1 kjoː:0:1"),
    "heart": dict(  # the Heart Sutra's mantra and title (Japanese reading), one syllable per knock (0.5 s)
        pool="ja", tonic=98.0, beat=0.5, reps=2, men=6, women=2, vib=0, scoop=0, glide=0.04, rt60=2.4, formant=0.97,
        score="ɡjɑ:0:1 te:0:1 ɡjɑ:0:1 te:0:1 hɑ:0:1 ɾɑ:0:1 ɡjɑ:0:1 te:0:1 | hɑ:0:1 ɾɑ:0:1 soː:0:1 ɡjɑ:0:1 te:0:1"
              " | bo:0:1 ʥi:0:1 so:0:1 wɑ:0:1 kɑ:0:2 | hɑn:0:1 njɑ:0:1 ɕiŋ:0:1 ɡjoː:-2:2 _:_:2"),
    "amituofo": dict(  # Namo Amituofo (Chinese Pure Land), original pentatonic tune
        pool="zh", tonic=110.0, beat=0.5, reps=2, men=4, women=4, vib=10, rt60=2.6,
        score="nɑ:4:1 mo:2:1 ɑ:0:2 mi:2:1 tʰwo:4:1 fo:2:2 | nɑ:7:1 mo:9:1 ɑ:7:2 mi:4:1 tʰwo:2:1 fo:0:2"
              " | nɑ:4:1 mo:2:1 ɑ:4:2 mi:7:1 tʰwo:4:1 fo:2:2 | nɑ:2:1 mo:4:1 ɑ:2:2 mi:0:1 tʰwo:-3:1 fo:0:2"),
    "guanyin": dict(  # Namo Guan Shi Yin Pusa
        pool="zh", tonic=110.0, beat=0.5, reps=1, men=3, women=5, lead_f=True, vib=12, rt60=2.6,
        score="nɑ:0:1 mo:2:1 ɡwɑn:4:2 ʂɨ:7:1 jin:4:1 pʰu:2:1 sɑ:0:1 | nɑ:4:1 mo:7:1 ɡwɑn:9:2 ʂɨ:7:1 jin:4:1 pʰu:2:1 sɑ:4:1"
              " | nɑ:0:1 mo:2:1 ɡwɑn:4:2 ʂɨ:7:1 jin:4:1 pʰu:2:1 sɑ:0:1 | nɑ:2:1 mo:4:1 ɡwɑn:2:2 ʂɨ:0:1 jin:-3:1 pʰu:-3:1 sɑ:0:1"
              " | nɑ:0:1 mo:2:1 ɡwɑn:4:2 ʂɨ:7:1 jin:4:1 pʰu:2:1 sɑ:0:1 | nɑ:4:1 mo:7:1 ɡwɑn:9:2 ʂɨ:7:1 jin:4:1 pʰu:2:1 sɑ:4:1"
              " | nɑ:2:1 mo:4:1 ɡwɑn:2:2 ʂɨ:0:1 jin:-3:1 pʰu:-3:1 sɑ:0:2 _:_:1"),

    # ---------------------------------------------------------- Theravada (Pali), long and short syllables
    "itipiso": dict(  # Iti pi so bhagavā... (recollection of the Buddha), Thai-style reciting tone
        pool="in", tonic=110.0, beat=0.24, reps=1, men=6, women=0, vib=0, scoop=0, glide=0.04, spread=0.02, rt60=1.8,
        score="i:0:1 ti:0:1 pi:0:1 soː:0:2 bʱɑ:0:1 ɡɑ:0:1 vɑː:0:2 ɑ:0:1 ɾɑ:0:1 hɑŋ:0:2"
              " sɑm:0:2 mɑː:0:2 sɑm:0:2 bʊd:0:2 dʱoː:-2:2 _:_:1"
              " vid:0:2 ʤɑː:0:2 ʧɑ:0:1 ɾɑ:0:1 ɳɑ:0:1 sɑm:0:2 pɑn:0:2 noː:0:2 sʊ:0:1 ɡɑ:0:1 toː:0:2 loː:0:2 kɑ:0:1 vi:0:1 duː:-2:2 _:_:1"
              " ɑ:0:1 nʊt:0:2 tɑ:0:1 ɾoː:0:2 pʊ:0:1 ɾi:0:1 sɑ:0:1 dɑm:0:2 mɑ:0:1 sɑː:0:2 ɾɑ:0:1 tʰi:0:1"
              " sɑt:0:2 tʰɑː:0:2 deː:0:2 vɑ:0:1 mɑ:0:1 nʊs:0:2 sɑː:0:2 nɑŋ:-2:2 _:_:1"
              " bʊd:0:2 dʱoː:0:2 bʱɑ:0:1 ɡɑ:0:1 vɑː:-2:2 ti:-4:2 _:_:3"),
    "namo": dict(  # Namo tassa bhagavato arahato sammāsambuddhassa, three times
        pool="in", tonic=110.0, beat=0.24, reps=3, men=6, women=0, vib=0, scoop=0, glide=0.04, spread=0.02, rt60=1.8,
        score="nɑ:0:1 moː:0:2 tɑs:0:2 sɑ:0:1 bʱɑ:0:1 ɡɑ:0:1 vɑ:0:1 toː:0:2 ɑ:0:1 ɾɑ:0:1 hɑ:0:1 toː:0:2"
              " sɑm:0:2 mɑː:0:2 sɑm:0:2 bʊd:0:2 dʱɑs:-2:2 sɑ:-2:1 _:_:2"),
    "metta": dict(  # Sabbe sattā sukhitā hontu... (may all beings be happy)
        pool="in", tonic=110.0, beat=0.26, reps=1, men=5, women=3, vib=0, scoop=0, glide=0.04, rt60=1.9,
        score="sɑb:0:2 beː:0:2 sɑt:0:2 tɑː:0:2 sʊ:0:1 kʰi:0:1 tɑː:0:2 hoːn:-2:2 tʊ:-2:1 _:_:1"
              " sɑb:0:2 beː:0:2 sɑt:0:2 tɑː:0:2 ɑ:0:1 veː:0:2 ɾɑː:0:2 hoːn:-2:2 tʊ:-2:1 _:_:1"
              " sɑb:0:2 beː:0:2 sɑt:0:2 tɑː:0:2 ɑ:0:1 bjɑː:0:2 pɑʤ:0:2 ʤʱɑː:0:2 hoːn:-2:2 tʊ:-2:1 _:_:1"
              " sɑb:0:2 beː:0:2 sɑt:0:2 tɑː:0:2 ɑ:0:1 niː:0:2 ɡʱɑː:0:2 hoːn:-2:2 tʊ:-2:1 _:_:1"
              " sʊ:0:1 kʰiː:0:2 ɑt:0:2 tɑː:0:2 nɑŋ:0:2 pɑ:0:1 ɾi:0:1 hɑ:0:1 ɾɑn:0:2 tʊ:-2:2 _:_:3"),

    # ---------------------------------------------------------- Hindu and yogic
    "shiva": dict(  # Om Namah Shivaya, call and response in raga Bhupali
        pool="in", tonic=138.59, beat=0.4, reps=1, men=4, women=4, vib=14, rt60=2.0,
        score="oːm:0:2:hL nɑ:2:1:L mɑh:4:1:L ʃi:7:2:L vɑː:4:1:L jɑ:2:1:L | oːm:0:2:h nɑ:2:1 mɑh:4:1 ʃi:7:2 vɑː:4:1 jɑ:2:1"
              " | oːm:7:2:hL nɑ:9:1:L mɑh:7:1:L ʃi:4:2:L vɑː:2:1:L jɑ:0:1:L | oːm:7:2:h nɑ:9:1 mɑh:7:1 ʃi:4:2 vɑː:2:1 jɑ:0:1"
              " | oːm:4:2:hL nɑ:2:1:L mɑh:4:1:L ʃi:7:2:L vɑː:9:1:L jɑ:7:1:L | oːm:4:2:h nɑ:2:1 mɑh:4:1 ʃi:7:2 vɑː:9:1 jɑ:7:1"
              " | oːm:4:2:hL nɑ:2:1:L mɑh:0:1:L ʃi:2:2:L vɑː:-3:1:L jɑ:0:1:L | oːm:4:2:h nɑ:2:1 mɑh:0:1 ʃi:2:2 vɑː:-3:1 jɑ:0:1"),
    "gayatri": dict(  # the Gāyatrī (Rig Veda 3.62.10) with its three Vedic pitch accents
        pool="in", tonic=138.59, beat=0.3, reps=2, men=6, women=0, vib=0, scoop=0, glide=0.06, spread=0.02, rt60=1.8,
        score="oːm:0:4:h | bʱuːɾ:0:2 bʱu:-2:1 vɑh:0:2 svɑh:2:3 _:_:2"
              " | tɑt:0:2 sɑ:2:1 vi:-2:1 tuɾ:0:2 vɑ:0:1 ɾeː:2:2 ɳjɑm:-2:2 _:_:1"
              " | bʱɑɾ:0:2 ɡoː:2:2 deː:-2:2 vɑs:0:2 jɑ:2:1 dʱiː:0:2 mɑ:0:1 hi:-2:1 _:_:1"
              " | dʱi:0:1 joː:2:2 joː:0:2 nɑh:2:2 pɾɑ:0:1 ʧoː:-2:2 dɑː:0:1 jɑːt:2:3 _:_:3"),
    "mahamantra": dict(  # Hare Krishna maha-mantra, leader then everyone, one syllable per cymbal stroke (0.42 s)
        pool="in", tonic=138.59, beat=0.42, reps=1, men=4, women=4, vib=10, rt60=1.8, spread=0.025,
        score="hɑ:4:1:L ɾe:4:1:L kɾiʂ:7:1:L ɳɑ:4:1:L hɑ:2:1:L ɾe:2:1:L kɾiʂ:4:1:L ɳɑ:2:1:L"
              " kɾiʂ:0:1:L ɳɑ:2:1:L kɾiʂ:4:1:L ɳɑ:2:1:L hɑ:0:1:L ɾe:-3:1:L hɑ:0:1:L ɾe:0:1:L"
              " hɑ:4:1:L ɾe:4:1:L ɾɑː:7:1:L mɑ:9:1:L hɑ:7:1:L ɾe:4:1:L ɾɑː:7:1:L mɑ:4:1:L"
              " ɾɑː:2:1:L mɑ:4:1:L ɾɑː:2:1:L mɑ:0:1:L hɑ:-3:1:L ɾe:0:1:L hɑ:2:1:L ɾe:0:1:L"
              " | hɑ:4:1 ɾe:4:1 kɾiʂ:7:1 ɳɑ:4:1 hɑ:2:1 ɾe:2:1 kɾiʂ:4:1 ɳɑ:2:1"
              " kɾiʂ:0:1 ɳɑ:2:1 kɾiʂ:4:1 ɳɑ:2:1 hɑ:0:1 ɾe:-3:1 hɑ:0:1 ɾe:0:1"
              " hɑ:4:1 ɾe:4:1 ɾɑː:7:1 mɑ:9:1 hɑ:7:1 ɾe:4:1 ɾɑː:7:1 mɑ:4:1"
              " ɾɑː:2:1 mɑ:4:1 ɾɑː:2:1 mɑ:0:1 hɑ:-3:1 ɾe:0:1 hɑ:2:1 ɾe:0:1"),
    # chakra seed sounds, each held on its own rising note (hy_bija stages 1-6)
    "bija_lam": dict(pool="in", tonic=128.0, beat=0.5, reps=5, men=4, women=3, vib=0, scoop=0, glide=0.1, rt60=2.6, hum=2.5, score="lɑːm:0:6:h _:_:2"),
    "bija_vam": dict(pool="in", tonic=128.0, beat=0.5, reps=5, men=4, women=3, vib=0, scoop=0, glide=0.1, rt60=2.6, hum=2.5, score="vɑːm:2:6:h _:_:2"),
    "bija_ram": dict(pool="in", tonic=128.0, beat=0.5, reps=5, men=4, women=3, vib=0, scoop=0, glide=0.1, rt60=2.6, hum=2.5, score="ɾɑːm:4:6:h _:_:2"),
    "bija_yam": dict(pool="in", tonic=128.0, beat=0.5, reps=5, men=4, women=3, vib=0, scoop=0, glide=0.1, rt60=2.6, hum=2.5, score="jɑːm:5:6:h _:_:2"),
    "bija_ham": dict(pool="in", tonic=128.0, beat=0.5, reps=5, men=4, women=3, vib=0, scoop=0, glide=0.1, rt60=2.6, hum=2.5, score="hɑːm:7:6:h _:_:2"),
    "bija_om": dict(pool="in", tonic=128.0, beat=0.5, reps=5, men=4, women=3, vib=0, scoop=0, glide=0.1, rt60=2.8, hum=2.5, score="oːm:9:6:h _:_:2"),
    # A-U-M, with the emphasis on one sound at a time (Māṇḍūkya stages), and the whole syllable for japa
    "aum": dict(pool="in", tonic=136.1, beat=0.5, reps=4, men=5, women=3, vib=0, scoop=0, glide=0.1, rt60=3.0, hum=2.0, score="ɑː:0:3 uː:0:3 uːm:0:4:h _:_:2"),
    "aum_a": dict(pool="in", tonic=136.1, beat=0.5, reps=4, men=5, women=3, vib=0, scoop=0, glide=0.1, rt60=3.0, hum=2.0, score="ɑː:0:6 uːm:0:4:h _:_:2"),
    "aum_u": dict(pool="in", tonic=136.1, beat=0.5, reps=4, men=5, women=3, vib=0, scoop=0, glide=0.1, rt60=3.0, hum=2.0, score="ɑː:0:2 uː:0:6 uːm:0:2:h _:_:2"),
    "aum_m": dict(pool="in", tonic=136.1, beat=0.5, reps=4, men=5, women=3, vib=0, scoop=0, glide=0.1, rt60=3.0, hum=3.0, score="ɑː:0:2 uː:0:2 uːm:0:6:h _:_:2"),
    "aum10": dict(  # one slow A-U-M every ten seconds, for the Om sessions' ten-second breath
        pool="in", tonic=136.1, beat=0.5, reps=3, men=5, women=3, vib=0, scoop=0, glide=0.1, rt60=3.0, hum=2.5,
        score="ɑː:0:4 uː:0:4 uːm:0:8:h _:_:4"),

    # ---------------------------------------------------------- Kundalini (gentle) and Sikh
    "adi": dict(  # Ong Namo Guru Dev Namo (the Adi mantra that opens a Kundalini class)
        pool="in", tonic=146.83, beat=0.5, reps=4, men=4, women=4, vib=8, rt60=2.2,
        score="oŋ:0:3:h nɑ:0:1 moː:2:2 ɡʊ:4:1 ɾuː:2:1 deːv:4:2 nɑ:2:1 moː:0:3 _:_:2"),
    "satnam": dict(  # Sat Naam: a long Sat, a short Naam
        pool="in", tonic=146.83, beat=0.5, reps=6, men=4, women=4, vib=6, rt60=2.2, hum=2.0,
        score="sɑt:2:6 nɑːm:0:2:h _:_:2"),
    "satanama": dict(  # Sa Ta Na Ma (Kirtan Kriya), one sound a second on the same notes as the session's bells
        pool="in", tonic=196.0, beat=1.0, reps=6, men=4, women=4, vib=4, scoop=0, glide=0.06, rt60=2.0,
        score="sɑː:7:1 tɑː:5:1 nɑː:4:1 mɑː:0:1"),
    "satanama_w": dict(  # the whispered round
        pool="in", tonic=196.0, beat=1.0, reps=6, men=3, women=3, vib=0, scoop=0, rt60=1.6, wet=0.2, gain=-5,
        score="sɑː:7:1:w tɑː:5:1:w nɑː:4:1:w mɑː:0:1:w"),
    "ramadasa": dict(  # Ra Ma Da Sa, Sa Say So Hung (the healing mantra), one round per breath (original tune)
        pool="in", tonic=146.83, beat=0.5, reps=4, men=4, women=4, vib=8, rt60=2.4,
        score="ɾɑː:0:1 mɑː:2:1 dɑː:4:1 sɑː:5:3 | sɑː:5:1 seː:4:1 soː:2:1 hʊŋ:0:3:h _:_:2"),
    "waheguru": dict(  # Waheguru simran, sung over a harmonium
        pool="in", tonic=138.59, beat=0.4, reps=2, men=4, women=4, vib=12, rt60=1.8,
        score="vɑː:4:1 heː:4:1 ɡʊ:2:1 ɾuː:0:1 | vɑː:4:1 heː:4:1 ɡʊ:2:1 ɾuː:0:1 | vɑː:7:1 heː:7:1 ɡʊ:4:1 ɾuː:2:1 | vɑː:7:1 heː:7:1 ɡʊ:4:1 ɾuː:2:1"
              " | vɑː:4:1 heː:2:1 ɡʊ:4:1 ɾuː:7:1 | vɑː:4:1 heː:2:1 ɡʊ:4:1 ɾuː:7:1 | vɑː:4:1 heː:2:1 ɡʊ:0:1 ɾuː:0:1 | vɑː:2:1 heː:0:1 ɡʊ:-1:1 ɾuː:0:1"),
    "mulmantar": dict(  # the Mool Mantar, opening of the Japji Sahib, as a measured recitation
        pool="in", tonic=138.59, beat=0.4, reps=2, men=4, women=4, vib=6, rt60=1.8,
        score="ik:0:1 oːŋ:0:1 kɑːɾ:2:2 | sɑt:0:1 nɑːm:0:2:h _:_:1 | kɑɾ:0:1 tɑː:0:1 pʊ:2:1 ɾɑkʰ:0:2 _:_:1 | niɾ:0:1 bʱɑʊ:2:2 _:_:1"
              " | niɾ:0:1 vɛːɾ:2:2 _:_:1 | ɑ:0:1 kɑːl:0:1 muː:2:1 ɾɑt:0:2 _:_:1 | ɑ:0:1 ʤuː:0:1 niː:2:2 _:_:1 | sɛː:0:1 bʱɑŋ:0:2:h _:_:1"
              " | ɡʊɾ:0:1 pɾɑ:2:1 sɑːd:0:3 _:_:3"),

    # ---------------------------------------------------------- Jain
    "navkar": dict(  # the Navkar (Namokar) Mantra, all nine lines
        pool="in", tonic=138.59, beat=0.36, reps=1, men=4, women=4, vib=10, rt60=2.2,
        score="nɑ:0:1 moː:2:1 ɑ:4:1 ɾi:4:1 hɑn:5:1 tɑː:4:1 ɳɑm:2:2:h _:_:1 | nɑ:2:1 moː:4:1 sid:5:1 dʱɑː:4:1 ɳɑm:2:2:h _:_:1"
              " | nɑ:0:1 moː:2:1 ɑː:4:1 jɑ:4:1 ɾi:5:1 jɑː:4:1 ɳɑm:2:2:h _:_:1 | nɑ:2:1 moː:4:1 u:5:1 vɑʤ:7:1 ʤʱɑː:5:1 jɑː:4:1 ɳɑm:2:2:h _:_:1"
              " | nɑ:4:1 moː:2:1 loː:0:1 eː:2:1 sɑv:4:1 vɑ:2:1 sɑː:0:1 huː:2:1 ɳɑm:0:2:h _:_:2"
              " | eː:0:1 soː:2:1 pɑɲ:4:1 ʧɑ:4:1 nɑ:5:1 mʊk:4:1 kɑː:2:1 ɾoː:0:2 _:_:1 | sɑv:2:1 vɑ:4:1 pɑː:5:1 vɑp:4:1 pɑ:2:1 ɳɑː:4:1 sɑ:2:1 ɳoː:0:2 _:_:1"
              " | mɑŋ:0:1 ɡɑ:2:1 lɑː:4:1 ɳɑm:5:1 ʧɑ:4:1 sɑv:2:1 veː:4:1 sim:2:2:h _:_:1 | pɑ:2:1 ɖʱɑ:4:1 mɑm:2:1 hɑ:0:1 vɑi:2:1 mɑŋ:4:1 ɡɑ:2:1 lɑm:0:3:h _:_:3"),

    # ---------------------------------------------------------- Islamic and Sufi
    "lailaha": dict(  # Lā ilāha illā-llāh, jahri (voiced) dhikr in the daf's 8-step bar (0.3 s steps)
        pool="ar", tonic=110.0, beat=0.3, reps=3, men=7, women=0, vib=0, scoop=10, glide=0.04, spread=0.02, rt60=1.3, wet=0.24, breath=0.12,
        score="lɑː:0:2:> ʔi:0:1 lɑː:1:1 hɑ:0:1 ʔil:0:1 lɑl:1:1 lɑːh:0:1 | lɑː:0:2:> ʔi:0:1 lɑː:1:1 hɑ:0:1 ʔil:0:1 lɑl:1:1 lɑːh:0:1"
              " | lɑː:4:2:> ʔi:4:1 lɑː:5:1 hɑ:4:1 ʔil:1:1 lɑl:1:1 lɑːh:0:1 | lɑː:4:2:> ʔi:4:1 lɑː:5:1 hɑ:4:1 ʔil:1:1 lɑl:1:1 lɑːh:0:1"),
    "lailaha_slow": dict(  # the same dhikr slowed to one phrase every five seconds, for the breath-paced session
        pool="ar", tonic=110.0, beat=0.3125, reps=3, men=6, women=0, vib=0, scoop=10, glide=0.05, spread=0.02, rt60=1.8, wet=0.28, breath=0.1,
        score="lɑː:0:3:> ʔi:0:2 lɑː:1:2 hɑ:0:2 ʔil:0:2 lɑl:1:2 lɑːh:0:3"
              " | lɑː:4:3:> ʔi:4:2 lɑː:5:2 hɑ:4:2 ʔil:1:2 lɑl:1:2 lɑːh:0:3"),
    "hu": dict(  # Allāh Hū, the last breath of the name, deep and breathy
        pool="ar", tonic=98.0, beat=0.4, reps=6, men=6, women=2, vib=0, scoop=10, rt60=2.4, breath=0.1, formant=0.96,
        score="ʔɑl:0:1 lɑːh:2:2 huː:0:5:b _:_:4"),
    "yalatif": dict(pool="ar", tonic=110.0, beat=0.4, reps=4, men=5, women=3, vib=10, rt60=2.2,
        score="jɑː:0:1 lɑ:1:1 tiːf:0:2 | jɑː:0:1 lɑ:1:1 tiːf:0:2 | jɑː:4:1 lɑ:5:1 tiːf:4:2 | jɑː:1:1 lɑ:1:1 tiːf:0:2"),
    "yarahman": dict(pool="ar", tonic=110.0, beat=0.4, reps=4, men=5, women=3, vib=10, rt60=2.2,
        score="jɑː:0:1 ɾɑh:1:1 mɑːn:0:2:h | jɑː:0:1 ɾɑh:1:1 mɑːn:0:2:h | jɑː:4:1 ɾɑh:5:1 mɑːn:4:2:h | jɑː:1:1 ɾɑh:1:1 mɑːn:0:2:h"),
    "yanur": dict(pool="ar", tonic=110.0, beat=0.4, reps=4, men=5, women=3, vib=10, rt60=2.2,
        score="jɑː:0:1 nuːɾ:1:3 | jɑː:0:1 nuːɾ:1:3 | jɑː:4:1 nuːɾ:5:3 | jɑː:1:1 nuːɾ:0:3"),
    "yawadud": dict(pool="ar", tonic=110.0, beat=0.4, reps=4, men=5, women=3, vib=10, rt60=2.2,
        score="jɑː:0:1 wɑ:1:1 duːd:0:2 | jɑː:0:1 wɑ:1:1 duːd:0:2 | jɑː:4:1 wɑ:5:1 duːd:4:2 | jɑː:1:1 wɑ:1:1 duːd:0:2"),
    "tasbih": dict(  # Subḥānallāh, Al-ḥamdu lillāh, Allāhu akbar, murmured
        pool="ar", tonic=110.0, beat=0.32, reps=2, men=4, women=2, vib=0, scoop=0, glide=0.04, rt60=1.4, wet=0.22, gain=-3,
        score="sʊb:0:1:s hɑː:0:1:s nɑl:0:1:s lɑːh:0:2:s sʊb:0:1:s hɑː:0:1:s nɑl:0:1:s lɑːh:0:2:s sʊb:0:1:s hɑː:0:1:s nɑl:0:1:s lɑːh:-1:2:s"
              " ʔɑl:0:1:s hɑm:0:1:s dʊ:0:1:s lil:0:1:s lɑːh:0:2:s ʔɑl:0:1:s hɑm:0:1:s dʊ:0:1:s lil:0:1:s lɑːh:0:2:s ʔɑl:0:1:s hɑm:0:1:s dʊ:0:1:s lil:0:1:s lɑːh:-1:2:s"
              " ʔɑl:0:1:s lɑː:0:1:s hʊ:0:1:s ʔɑk:0:1:s bɑɾ:0:2:s ʔɑl:0:1:s lɑː:0:1:s hʊ:0:1:s ʔɑk:0:1:s bɑɾ:0:2:s ʔɑl:0:1:s lɑː:0:1:s hʊ:0:1:s ʔɑk:0:1:s bɑɾ:-1:2:s _:_:3"),

    # ---------------------------------------------------------- Jewish
    "shema": dict(  # Shema Yisrael, Adonai Eloheinu, Adonai Echad (original tune)
        pool="en", tonic=110.0, beat=0.5, reps=2, men=4, women=4, vib=14, rt60=2.2,
        score="ʃə:0:1 mɑː:3:2 | jis:5:1 ɾɑː:7:1 eːl:5:2 | ʔɑ:3:1 doː:5:1 nɑi:3:2 | ʔe:2:1 loː:3:1 hei:2:1 nuː:0:2"
              " | ʔɑ:3:1 doː:5:1 nɑi:7:2 | ʔe:5:1 χɑːd:3:4 _:_:3"),
    "baruch": dict(  # Barukh shem kevod malkhuto le'olam va'ed, whispered as tradition has it
        pool="en", tonic=110.0, beat=0.4, reps=4, men=2, women=2, rt60=1.6, wet=0.2, gain=-5,
        score="bɑ:0:1:w ɾuːχ:0:1:w ʃeːm:0:1:w kə:0:1:w voːd:0:1:w mɑl:0:1:w χuː:0:1:w toː:0:1:w lə:0:1:w ʔoː:0:1:w lɑːm:0:1:w vɑː:0:1:w ʔɛd:0:2:w _:_:3"),
    "niggun": dict(  # a wordless devotional niggun in the Ahava Rabbah mode (original)
        pool="en", tonic=98.0, beat=0.45, reps=2, men=6, women=2, vib=18, glide=0.12, scoop=30, rt60=2.0,
        score="jɑ:0:1 bɑ:1:1 bɑi:4:2 jɑ:5:1 bɑ:4:1 bɑi:1:2 jɑ:4:1 bɑ:5:1 bɑi:7:2 dɑi:5:1 dɑi:4:1 dɑi:1:2"
              " | jɑ:7:1 bɑ:8:1 bɑi:7:2 jɑ:5:1 bɑ:4:1 bɑi:5:2 jɑ:4:1 dɑi:1:1 dɑi:4:1 dɑi:1:1 dɑi:0:4"),
    "niggun_hum": dict(  # the same niggun hummed, softly
        pool="en", tonic=98.0, beat=0.45, reps=2, men=5, women=2, vib=14, glide=0.14, scoop=20, rt60=2.0, hum=3.0, gain=-3,
        score="hʌm:0:1:h hʌm:1:1:h hʌm:4:2:h hʌm:5:1:h hʌm:4:1:h hʌm:1:2:h hʌm:4:1:h hʌm:5:1:h hʌm:7:2:h hʌm:5:1:h hʌm:4:1:h hʌm:1:2:h"
              " | hʌm:7:1:h hʌm:8:1:h hʌm:7:2:h hʌm:5:1:h hʌm:4:1:h hʌm:5:2:h hʌm:4:1:h hʌm:1:1:h hʌm:4:1:h hʌm:1:1:h hʌm:0:4:h"),
    "aleph": dict(  # Abulafia-style: the letter aleph with the five vowels, one per long breath
        pool="en", tonic=110.0, beat=0.5, reps=2, men=3, women=1, vib=0, scoop=0, rt60=1.8, wet=0.25,
        score="ʔoː:0:4 _:_:2 ʔɑː:0:4 _:_:2 ʔiː:0:4 _:_:2 ʔeː:0:4 _:_:2 ʔuː:0:4 _:_:2"),
    "kadosh": dict(  # Kadosh, kadosh, kadosh Adonai Tzeva'ot, melo khol ha'aretz kevodo (Isaiah 6:3, the Kedushah; original tune)
        pool="en", tonic=110.0, beat=0.45, reps=2, men=5, women=4, vib=12, glide=0.1, scoop=20, rt60=2.6,
        score="kɑ:0:1 doːʃ:3:2 | kɑ:5:1 doːʃ:7:2 | kɑ:8:1 doːʃ:7:3 _:_:1 | ʔɑ:5:1 doː:7:1 nɑi:5:2 ʦə:3:1 vɑː:2:1 ʔoːt:0:2 _:_:1"
              " | mə:3:1 loː:5:1 χɔl:7:1 hɑː:5:1 ʔɑː:3:1 ɾɛts:2:2 _:_:1 | kə:3:1 voː:2:1 doː:0:4 _:_:2"),

    # ---------------------------------------------------------- Christian
    "kyrie": dict(  # Kyrie eleison in plainchant style, Dorian (original melody)
        pool="la", tonic=146.83, beat=0.42, reps=2, men=6, women=0, vib=0, scoop=0, glide=0.07, rt60=4.5, wet=0.42,
        score="ki:0/1,2/1:2 ɾi:3:1 e:2/1,0/1:2 | e:0:1 lei:2/1,3/1,2/1:3 son:0:3 _:_:2"
              " | kɾis:5:1 te:3/1,5/1,7/1:3 | e:5:1 lei:3/1,2/1:2 son:3:3 _:_:2"
              " | ki:3/1,2/1:2 ɾi:0:1 e:2:1 | e:0/1,-2/1:2 lei:0/1,2/1:2 son:0:4 _:_:4"),
    "avemaria": dict(  # the Hail Mary in Latin as a rosary is prayed: leader, then everyone, on one reciting note
        pool="la", tonic=130.81, beat=20.0 / 84.0, reps=1, men=4, women=4, vib=0, scoop=0, glide=0.04, spread=0.02, rt60=3.0,
        score="ɑ:0:1:L ve:0:1:L mɑ:0:1:L ɾi:0:1:L ɑ:0:1:L ɡɾɑ:0:1:L tsi:0:1:L ɑ:0:1:L ple:0:1:L nɑ:-2:1:L do:0:1:L mi:0:1:L nus:0:1:L te:0:1:L kum:-2:2:L _:_:1"
              " be:0:1:L ne:0:1:L dik:0:1:L tɑ:0:1:L tu:0:1:L in:0:1:L mu:0:1:L li:0:1:L e:0:1:L ɾi:0:1:L bus:-2:1:L _:_:1"
              " et:0:1:L be:0:1:L ne:0:1:L dik:0:1:L tus:0:1:L fɾuk:0:1:L tus:0:1:L ven:0:1:L tɾis:0:1:L tu:0:1:L i:-2:1:L je:0:1:L sus:-2:2:L _:_:2"
              " | sɑŋk:0:1 tɑ:0:1 mɑ:0:1 ɾi:0:1 ɑ:-2:1 mɑ:0:1 teɾ:0:1 de:0:1 i:-2:1 _:_:1"
              " o:0:1 ɾɑ:0:1 pɾo:0:1 no:0:1 bis:0:1 pek:0:1 kɑ:0:1 to:0:1 ɾi:0:1 bus:-2:1 _:_:1"
              " nunk:0:1 et:0:1 in:0:1 o:0:1 ɾɑ:0:1 moɾ:0:1 tis:0:1 nos:0:1 tɾe:-2:1 _:_:1 ɑ:0:1 men:-2:3:h _:_:5"),
    "ubicaritas": dict(  # Ubi caritas et amor, Deus ibi est: a short sung prayer in three parts (original)
        pool="la", tonic=146.83, beat=0.55, reps=2, vib=10, rt60=3.2, wet=0.36,
        parts=[("f", 2, 0, 12), ("f", 2, 1, 12), ("m", 2, 0, 0), ("m", 2, 2, 0)],
        score="u:4;0;-12:1 bi:5;2;-7:1 kɑ:7;4;-12:1 ɾi:5;2;-7:1 tɑs:4;0;-12:1 et:2;-1;-5:1 ɑ:4;0;-12:1 moɾ:2;-1;-5:2"
              " | u:4;0;-12:1 bi:5;2;-7:1 kɑ:7;4;-8:1 ɾi:9;5;-3:1 tɑs:7;4;-5:1 de:5;2;-7:1 us:4;0;-8:1 i:2;-1;-5:1 bi:2;-1;-5:1 est:0;-5;-12:4 _:_:2"),
    "jesus": dict(  # the Jesus Prayer in Greek, Byzantine style over the ison: one prayer every ten seconds
        pool="la", tonic=146.83, beat=0.5, reps=3, men=6, women=0, vib=10, glide=0.09, rt60=3.5, wet=0.38,
        score="ki:0:1 ɾi:2:1 e:0:1 i:3:1 i:5:1 su:3:1 χɾi:2:1 ste:0:2 _:_:1"
              " | ʝi:3:0.5 e:5:1 tu:3:0.5 θe:2:0.5 u:0:1 e:0:0.5 le:2:0.5 i:3:0.5 son:2:1 me:0:2.5 _:_:1.5"),
    "jesus_w": dict(  # the same prayer whispered, on the same ten-second breath
        pool="la", tonic=146.83, beat=0.5, reps=3, men=3, women=0, rt60=2.0, wet=0.22, gain=-5,
        score="ki:0:1:w ɾi:2:1:w e:0:1:w i:3:1:w i:5:1:w su:3:1:w χɾi:2:1:w ste:0:2:w _:_:1"
              " | ʝi:3:0.5:w e:5:1:w tu:3:0.5:w θe:2:0.5:w u:0:1:w e:0:0.5:w le:2:0.5:w i:3:0.5:w son:2:1:w me:0:2.5:w _:_:1.5"),
    "vacate": dict(  # Vacate et videte quoniam ego sum Deus (Ps 45:11 Vulgate, "Be still") on a psalm tone
        pool="la", tonic=130.81, beat=0.45, reps=3, men=5, women=0, vib=0, scoop=0, glide=0.06, rt60=4.0, wet=0.4,
        score="vɑ:0:1 kɑ:2:1 te:5:1 et:5:1 vi:5:1 de:7:1 te:5:2 _:_:1 | kwo:5:1 ni:5:1 ɑm:5:1 e:5:1 ɡo:5:1 sum:4:1 de:2:1 us:0:3 _:_:2"),
    "inmanus": dict(  # In manus tuas, Domine, commendo spiritum meum (Compline)
        pool="la", tonic=146.83, beat=0.5, reps=2, men=6, women=0, vib=0, scoop=0, glide=0.07, rt60=4.2, wet=0.42,
        score="in:0:1 mɑ:2:1 nus:3:1 tu:2:1 ɑs:0:2 _:_:1 | do:3:1 mi:5:1 ne:3:2 _:_:1 | kom:2:1 men:3:1 do:5:2 | spi:3:1 ɾi:2:1 tum:0:1 me:2:1 um:0:3 _:_:2"),

    # ---------------------------------------------------------- Western esoteric: vibrated names
    "qcross": dict(  # Ateh, Malkuth, ve-Geburah, ve-Gedulah, le-Olam, Amen (the Qabalistic Cross)
        pool="en", tonic=110.0, beat=0.5, reps=1, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.2, hum=2.0,
        score="ʔɑ:0:2 teː:0:4 _:_:1 mɑl:0:2 kuːθ:0:4 _:_:1 ve:0:1 ɡe:0:1 buː:0:1 ɾɑː:0:4 _:_:1 ve:0:1 ɡe:0:1 duː:0:1 lɑː:0:4 _:_:1"
              " le:0:1 ʔoː:0:1 lɑːm:0:4:h _:_:1 ʔɑː:0:2 meːn:0:5:h _:_:3"),
    "lbrp_names": dict(  # Yod Heh Vav Heh, Adonai, Eheieh, AGLA, vibrated at the four quarters
        pool="en", tonic=110.0, beat=0.5, reps=1, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.2,
        score="joːd:0:3 heː:0:3 vɑːv:0:3 heː:0:4 _:_:3 | ʔɑ:0:1 doː:0:1 nɑi:0:5 _:_:3 | ʔe:0:1 hei:0:1 jeː:0:5 _:_:3 | ʔɑ:0:1 ɡɑ:0:1 lɑː:0:5 _:_:3"),
    "archangels": dict(  # Raphael, Gabriel, Michael, Auriel
        pool="en", tonic=110.0, beat=0.5, reps=1, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.2,
        score="ɾɑː:0:1 fɑ:0:1 eːl:0:4 _:_:2 ɡɑː:0:1 bɾi:0:1 eːl:0:4 _:_:2 mi:0:1 χɑ:0:1 eːl:0:4 _:_:2 ʔɔː:0:1 ɾi:0:1 eːl:0:4 _:_:4"),
    "eheieh": dict(pool="en", tonic=110.0, beat=0.5, reps=3, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.4,
        score="ʔe:0:2 hei:0:2 jeː:0:6 _:_:4"),
    "yhvh_elohim": dict(pool="en", tonic=110.0, beat=0.5, reps=3, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.4, hum=2.0,
        score="je:0:1 hoː:0:2 voː:0:3 ʔe:0:1 loː:0:2 hiːm:0:5:h _:_:4"),
    "yhvh_eloah": dict(pool="en", tonic=110.0, beat=0.5, reps=2, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.4,
        score="je:0:1 hoː:0:2 voː:0:3 ʔe:0:1 loː:0:2 ʔɑː:0:3 ve:0:1 dɑ:0:1 ʔɑːθ:0:4 _:_:4"),
    "shaddai": dict(pool="en", tonic=110.0, beat=0.5, reps=3, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.4,
        score="ʃɑd:0:2 dɑi:0:4 ʔel:0:2 χɑi:0:5 _:_:4"),
    "adonai_haaretz": dict(pool="en", tonic=110.0, beat=0.5, reps=3, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.4,
        score="ʔɑ:0:1 doː:0:2 nɑi:0:4 hɑ:0:1 ʔɑː:0:2 ɾets:0:4 _:_:4"),
    "yeheshuah": dict(  # Yeheshuah, Yehovashah (the Rose Cross ritual)
        pool="en", tonic=110.0, beat=0.5, reps=2, parts=[("m", 2, 0, 0), ("f", 1, 0, 12)], vib=4, scoop=0, rt60=2.4,
        score="je:0:1 he:0:1 ʃuː:0:2 ɑː:0:4 _:_:3 je:0:1 hoː:0:1 vɑː:0:2 ʃɑː:0:4 _:_:3"),
    # galdr: rune names drawn out, then the rune's sound (we_galdr stages 1-4)
    "ansuz": dict(pool="en", tonic=98.0, beat=0.45, reps=3, men=3, women=2, vib=0, scoop=15, rt60=1.6, breath=0.08, formant=0.97, hum=2.0,
        score="ɑːn:0:4:h suːz:0:4:f _:_:2 ɑː:0:3 ɑː:-2:3 _:_:2"),
    "laguz": dict(pool="en", tonic=98.0, beat=0.45, reps=3, men=3, women=2, vib=0, scoop=15, rt60=1.6, breath=0.08, formant=0.97,
        score="lɑː:0:4 ɡuːz:0:4:f _:_:2 luː:0:3 luː:-2:3 _:_:2"),
    "isa": dict(pool="en", tonic=98.0, beat=0.45, reps=3, men=3, women=2, vib=0, scoop=0, rt60=1.6, breath=0.06, formant=0.97,
        score="iː:0:4 sɑː:0:4 _:_:2 iː:0:6 _:_:2"),
    "dagaz": dict(pool="en", tonic=98.0, beat=0.45, reps=3, men=3, women=2, vib=0, scoop=15, rt60=1.6, breath=0.08, formant=0.97,
        score="dɑː:0:4 ɡɑz:0:4:f _:_:2 dɑː:2:3 dɑː:0:3 _:_:2"),
    "awen": dict(  # the Druid Awen, three times, every voice on its own note
        pool="en", tonic=110.0, beat=0.5, reps=1, men=4, women=4, vib=6, scoop=10, glide=0.15, rt60=2.6, wet=0.35, hum=1.5,
        cluster=[-5, 0, 0, 2, 4, 7, 7, 9],
        score="ɑː:0:3 uː:0:2 wen:0:6:h _:_:3 ɑː:0:3 uː:0:2 wen:0:6:h _:_:3 ɑː:0:3 uː:0:2 wen:0:8:h _:_:6"),

    # ---------------------------------------------------------- Zoroastrian, Bahá'í, Shinto, Taoist
    "ashem": dict(  # Ashem Vohu (Avestan), murmured as by a priest at the fire
        pool="in", tonic=110.0, beat=0.3, reps=3, men=4, women=0, vib=0, scoop=0, glide=0.04, spread=0.02, rt60=1.8, wet=0.25, gain=-2,
        score="ɑ:0:1:s ʃəm:0:1:s vo:0:1:s huː:0:1:s vɑ:0:1:s hiʃ:0:1:s təm:0:1:s ɑs:0:1:s tiː:-2:1:s _:_:1"
              " uʃ:0:1:s tɑː:0:1:s ɑs:0:1:s tiː:-2:1:s _:_:1 uʃ:0:1:s tɑː:0:1:s ɑh:0:1:s mɑːi:-2:1:s _:_:1"
              " hjɑt:0:1:s ɑ:0:1:s ʃɑːi:0:1:s vɑ:0:1:s hiʃ:0:1:s tɑːi:0:1:s ɑ:0:1:s ʃəm:-2:2:s _:_:2"),
    "abha": dict(  # Alláh-u-Abhá
        pool="en", tonic=110.0, beat=0.45, reps=2, men=4, women=4, vib=10, rt60=2.0,
        score="ʔɑl:0:1 lɑː:2:1 hu:4:1 ʔɑb:5:1 hɑː:4:3 _:_:1 | ʔɑl:4:1 lɑː:5:1 hu:7:1 ʔɑb:5:1 hɑː:4:3 _:_:1"
              " | ʔɑl:0:1 lɑː:2:1 hu:4:1 ʔɑb:5:1 hɑː:4:3 _:_:1 | ʔɑl:2:1 lɑː:0:1 hu:2:1 ʔɑb:4:1 hɑː:0:3 _:_:1"),
    "harae": dict(  # Harae tamae, kiyome tamae (Shinto purification words)
        pool="ja", tonic=110.0, beat=0.4, reps=4, men=4, women=3, vib=0, scoop=0, glide=0.06, rt60=1.8,
        score="hɑ:0:1 ɾɑ:0:1 e:0:1 tɑ:0:1 mɑ:0:1 e:-2:2 _:_:1 ki:0:1 jo:0:1 me:0:1 tɑ:0:1 mɑ:0:1 e:-2:2 _:_:1"),
    "qingjing": dict(  # opening of the Qingjing Jing (Classic of Purity and Stillness), one syllable a knock (0.5 s)
        pool="zh", tonic=110.0, beat=0.5, reps=1, men=4, women=2, vib=0, scoop=0, glide=0.05, rt60=2.0,
        score="lɑu:0:1 ʨyn:0:1 ɥe:-2:2 | tɑ:0:1 tɑu:0:1 u:0:1 ɕiŋ:-2:1 | ʂəŋ:0:1 y:0:1 tʰjɛn:0:1 ti:-2:1 _:_:1"
              " | tɑ:0:1 tɑu:0:1 u:0:1 ʨʰiŋ:-2:1 | yn:0:1 ɕiŋ:0:1 ɻɨ:0:1 ɥe:-2:1 _:_:1"
              " | tɑ:0:1 tɑu:0:1 u:0:1 miŋ:-2:1 | ʈʂɑŋ:0:1 jɑŋ:0:1 wɑn:0:1 u:-2:2 _:_:2"),
    # the six healing sounds, whispered on a long out-breath (ta_sixsounds stages 1-6)
    "six_xu": dict(pool="zh", beat=0.5, reps=3, men=2, women=1, rt60=1.2, wet=0.18, gain=-4, score="ɕyː:0:12:wf _:_:6"),
    "six_he": dict(pool="zh", beat=0.5, reps=3, men=2, women=1, rt60=1.2, wet=0.18, gain=-4, score="hɤː:0:12:wf _:_:6"),
    "six_hu": dict(pool="zh", beat=0.5, reps=3, men=2, women=1, rt60=1.2, wet=0.18, gain=-4, score="huː:0:12:wf _:_:6"),
    "six_si": dict(pool="zh", beat=0.5, reps=3, men=2, women=1, rt60=1.2, wet=0.18, gain=-4, score="sɨː:0:12:wf _:_:6"),
    "six_chui": dict(pool="zh", beat=0.5, reps=3, men=2, women=1, rt60=1.2, wet=0.18, gain=-4, score="ʈʂʰweiː:0:12:wf _:_:6"),
    "six_xi": dict(pool="zh", beat=0.5, reps=3, men=2, women=1, rt60=1.2, wet=0.18, gain=-4, score="ɕiː:0:12:wf _:_:6"),
}

if __name__ == "__main__":
    main()
