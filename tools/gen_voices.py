"""Build-time tool: render every spoken cue with the Kokoro-82M neural TTS model
(Apache-2.0, https://github.com/hexgrad/kokoro) and save as Ogg Vorbis into ../voice/<voice>/<cue_id>.ogg.
These files are embedded into the Rust exe at compile time; the app itself never runs Python
or touches the network.

Usage:
  focus_binaural.exe --dump-cues tools/cues.json
  python tools/gen_voices.py <kokoro-v1.0.onnx> <voices-v1.0.bin>
"""
import json, os, subprocess, sys, tempfile
import numpy as np
import soundfile as sf
from kokoro_onnx import Kokoro

VOICES = {  # voice id -> language
    "af_heart": "en-us",   # warm US female (top-rated Kokoro voice)
    "bf_emma": "en-gb",    # calm UK female
    "bm_george": "en-gb",  # calm UK male
    "am_michael": "en-us", # US male
}
SPEED = 0.86  # slower, meditative delivery

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
cues = json.load(open(os.path.join(root, "tools", "cues.json"), encoding="utf-8"))
k = Kokoro(sys.argv[1], sys.argv[2])
tmp = tempfile.mkdtemp()

for voice, lang in VOICES.items():
    outdir = os.path.join(root, "voice", voice)
    os.makedirs(outdir, exist_ok=True)
    for n, c in enumerate(cues, 1):
        dst = os.path.join(outdir, c["id"] + ".ogg")
        if os.path.exists(dst):
            continue
        samples, sr = k.create(c["text"], voice=voice, speed=SPEED, lang=lang)
        samples = np.asarray(samples, dtype=np.float32)
        peak = float(np.max(np.abs(samples))) or 1.0
        samples = samples / peak * 0.7                       # consistent level
        pad = np.zeros(int(sr * 0.4), dtype=np.float32)      # breathing room
        samples = np.concatenate([pad, samples, pad])
        wav = os.path.join(tmp, "c.wav")
        sf.write(wav, samples, sr)
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-i", wav, "-c:a", "libvorbis", "-q:a", "4", "-ac", "1", dst], check=True)
        print(f"[{voice}] {n}/{len(cues)}", flush=True)
print("done")
