//! The session library. Every preset carries an honest evidence tag:
//!   Gateway     – interpretation of the Monroe Institute Focus-level model
//!   Research    – based on a published study (cited in `about`)
//!   Traditional – popular / historical association, not scientifically established
//!   Speculative – fringe claim; included for completeness, labelled as such
use crate::plan::{cue_id, Preset, Stage, Texture};

#[path = "library.rs"]
mod library;

fn st(name: &str, min: f64, beats: &[f64], cue: &str) -> Stage {
    Stage {
        name: name.into(),
        min,
        beats: beats.to_vec(),
        carriers: None,
        ramp: None,
        cue: cue.into(),
        cue_id: String::new(),
        breath: false,
        tex: None,
        mus: None,
    }
}
trait StageExt {
    fn breath(self) -> Self;
    fn car(self, c: &[f64]) -> Self;
    fn ramp(self, s: f64) -> Self;
    fn tex(self, g: f64) -> Self;
    fn mus(self, g: f64) -> Self;
}
impl StageExt for Stage {
    fn breath(mut self) -> Self { self.breath = true; self }
    fn car(mut self, c: &[f64]) -> Self { self.carriers = Some(c.to_vec()); self }
    fn ramp(mut self, s: f64) -> Self { self.ramp = Some(s); self }
    fn tex(mut self, g: f64) -> Self { self.tex = Some(g); self }
    fn mus(mut self, g: f64) -> Self { self.mus = Some(g); self }
}
fn with_min(mut s: Stage, min: f64) -> Stage { s.min = min; s }
fn tx(kind: &str, level: f64, root: f64, interval: f64) -> Option<Texture> {
    Some(Texture { kind: kind.into(), level, root, interval, name: String::new() })
}
/// Accompaniment (music.rs): kind, variant/clips, level, root (Hz, 0 = default), interval (s, 0 = default).
fn mx(kind: &str, name: &str, level: f64, root: f64, interval: f64) -> Option<Texture> {
    Some(Texture { kind: kind.into(), level, root, interval, name: name.into() })
}
fn with_tex(mut p: Preset, t: Option<Texture>) -> Preset { p.texture = t; p }

#[allow(clippy::too_many_arguments)]
fn p(group: &str, id: &str, name: &str, short: &str, carriers: &[f64], gains: &[f64], noise: &str,
     mode: &str, evidence: &str, about: &str, stages: Vec<Stage>) -> Preset {
    Preset {
        id: id.into(), group: group.into(), name: name.into(), short: short.into(),
        carriers: carriers.to_vec(), gains: gains.to_vec(), noise: noise.into(), mode: mode.into(),
        evidence: evidence.into(), about: about.into(), stages, live: false,
        texture: None, music: None, visual: String::new(), tags: Vec::new(), trad: Vec::new(), info: None, guide: None,
    }
}

// ---------- shared cue text ----------
const C_SETTLE: &str = "Settle in. Close your eyes, let your breathing slow, and let the tone carry you.";
const C_REST: &str = "Rest here. There is nothing you need to do. Just listen.";
const C_RETURN_SHORT: &str = "Gently begin to return. Move your fingers and toes, and open your eyes when you are ready.";
const C_FOCUS: &str = "Bring your attention to the task in front of you. Let the tone hold your focus steady.";

fn gateway_prep() -> Vec<Stage> {
    vec![
        st("Arrival", 2.0, &[10.0, 10.0, 14.0], "Lie back or sit comfortably. Loosen anything tight. Let your eyes close and simply listen to the tones."),
        st("Energy Conversion Box", 2.0, &[8.0, 10.0, 14.0], "Picture a strong container with a heavy lid. Place every worry, task and distraction inside it, close the lid, and set it aside. You can return for them later."),
        st("Resonant Tuning", 3.0, &[6.0, 8.0, 12.0], "Breathe in slowly and deeply. As you breathe out, hum gently, and feel the vibration in your chest and head. Let your hum blend with the tones.").breath(),
        st("Energy Balloon", 2.0, &[4.0, 7.0, 12.0], "Imagine energy rising from the top of your head like a fountain, flowing down around you, and forming a protective balloon that surrounds your whole body."),
        st("Intention", 1.0, &[4.0, 6.0, 12.0], "Silently state, in your own words, what you wish to explore or learn in this session. Then let the words go."),
    ]
}
fn f10() -> Stage { st("Focus 10", 5.0, &[1.5, 4.0, 12.0], "Mind awake, body asleep. Let your body drift toward sleep while your awareness stays clear and alert, simply observing.") }
fn f12() -> Stage { st("Focus 12", 5.0, &[2.0, 6.0, 10.0], "Expanded awareness. Let your awareness spread beyond the edges of your body in every direction. Notice whatever arises, without effort.") }
fn f15() -> Stage { st("Focus 15", 5.0, &[1.0, 3.0, 7.83], "The state of no time. Release the past and the future. There is only this open, still moment.") }
fn f21() -> Stage { st("Focus 21", 12.0, &[0.5, 3.5, 40.0], "The bridge. Rest at the edge where deep stillness meets bright, high awareness. Observe without holding on.") }
fn ret(min: f64) -> Stage { st("Return", min, &[10.0, 12.0, 18.0], "Begin to return. Slowly count from one to five. Feel your body, the surface beneath you, and the room around you. When you are ready, open your eyes, awake and refreshed.") }

const GW_C: [f64; 3] = [100.0, 200.0, 300.0];
const GW_G: [f64; 3] = [1.0, 0.6, 0.3];

fn band_carrier(hz: f64) -> f64 {
    if hz < 4.0 { 150.0 } else if hz < 8.0 { 200.0 } else if hz < 13.0 { 220.0 } else if hz < 30.0 { 250.0 } else if hz < 60.0 { 300.0 } else if hz < 150.0 { 500.0 } else { 800.0 }
}

/// Simple 30-minute single-frequency session: tune in at alpha, glide to target, hold.
fn single(group: &str, id: &str, name: &str, short: &str, hz: f64, carrier: f64, noise: &str, mode: &str, evidence: &str, about: &str) -> Preset {
    let start = if hz >= 13.0 { hz.min(14.0) } else { 10.0 };
    p(group, id, name, short, &[carrier], &[1.0], noise, mode, evidence, about,
      vec![st("Tune in", 1.0, &[start], C_SETTLE), st(name, 29.0, &[hz], C_REST).ramp(120.0)])
}

pub fn all() -> Vec<Preset> {
    let mut v: Vec<Preset> = Vec::new();

    // ================= Gateway Experience =================
    let g = "Inspired by the Gateway Experience";
    v.push(p(g, "f3", "Focus 3 — Introduction", "Wave I primer · 12 min", &GW_C, &GW_G, "surf", "binaural", "Gateway",
      "<p>A short introduction to layered tones: you feel the tones \"sync up\" while staying fully awake. In the Gateway model, Focus 3 is the first step: the two halves of the brain working in step with each other.</p>",
      vec![st("Arrival", 2.0, &[10.0, 10.0, 14.0], "Lie back or sit comfortably. Let your eyes close and simply listen to the tones."),
           st("Synchronise", 8.0, &[8.0, 10.0, 14.0], "Notice the gentle pulse within the sound. Let your attention rest on it, relaxed and awake.").breath(),
           st("Return", 2.0, &[10.0, 12.0, 18.0], C_RETURN_SHORT)]));
    let mut s = gateway_prep(); s.push(with_min(f10(), 8.0)); s.push(ret(2.0));
    v.push(p(g, "f10", "Focus 10 — Mind Awake, Body Asleep", "Orientation · 20 min", &GW_C, &GW_G, "surf", "binaural", "Gateway",
      "<p>The first Gateway milestone: the body relaxes fully toward sleep while the mind stays awake. It uses three tone layers at once, in the Hemi-Sync style: a <b>delta</b> layer for the body, a <b>theta</b> layer for depth, and a quiet <b>beta/alpha</b> layer to keep you awake.</p><p>The prep stages (Energy Conversion Box, Resonant Tuning, Energy Balloon, Intention) follow the routine described in the Gateway program, written here in original wording.</p>", s));
    let mut s = gateway_prep(); s.push(with_min(f10(), 4.0)); s.push(with_min(f12(), 8.0)); s.push(ret(2.0));
    v.push(p(g, "f12", "Focus 12 — Expanded Awareness", "23 min", &GW_C, &GW_G, "surf", "binaural", "Gateway",
      "<p>Builds from Focus 10 into a state where awareness feels like it reaches past the body. The 1983 U.S. Army assessment by Lt. Col. Wayne McDonnell (declassified by the CIA in 2003) described it as a state suited to problem-solving and intuition.</p>", s));
    let mut s = gateway_prep(); s.push(with_min(f10(), 3.0)); s.push(with_min(f12(), 4.0)); s.push(with_min(f15(), 8.0)); s.push(ret(3.0));
    v.push(p(g, "f15", "Focus 15 — No Time", "27 min", &GW_C, &GW_G, "pink", "binaural", "Gateway",
      "<p>A deep, still state where the sense of time drops away. Every layer slows down; the top layer is tuned to 7.83 Hz (the Schumann resonance). That choice is symbolic, not proven.</p>", s));
    let mut s = gateway_prep(); s.push(with_min(f10(), 3.0)); s.push(with_min(f12(), 3.0)); s.push(with_min(f15(), 4.0)); s.push(f21()); s.push(ret(3.0));
    v.push(p(g, "f21", "Focus 21 — The Bridge", "35 min · advanced", &GW_C, &GW_G, "pink", "binaural", "Gateway",
      "<p>The furthest point in the Gateway sequence, described as a bridge to \"other energy systems\". Very deep delta (0.5 Hz) is paired with a <b>40 Hz gamma</b> layer. The gamma layer is based on findings of high-amplitude gamma in long-term meditators (Lutz et al., 2004, <i>PNAS</i>).</p>", s));

    // ================= Lifeline (Focus 22–27) =================
    let g = "Inspired by Lifeline (Focus 22–27)";
    let lifeline = |id: &str, name: &str, short: &str, beats: [f64; 3], cue: &str, about: &str| {
        let mut s = gateway_prep();
        s.push(with_min(f10(), 3.0)); s.push(with_min(f12(), 3.0)); s.push(with_min(f15(), 3.0));
        s.push(with_min(f21(), 4.0));
        s.push(st(name.split(" — ").next().unwrap_or(name), 10.0, &beats, cue));
        s.push(ret(3.0));
        p(g, id, name, short, &GW_C, &GW_G, "pink", "binaural", "Speculative", about, s)
    };
    v.push(lifeline("f22", "Focus 22 — Dreamlike Edge", "36 min", [0.5, 3.0, 5.0],
        "Rest at the soft edge of dreaming. Images may come and go. Let them pass like clouds.",
        "<p>In Monroe's later <i>Lifeline</i> program, Focus 22 describes partial, dreamlike awareness. This is a deep theta/delta interpretation. These levels are spiritual ideas from the Monroe tradition; there is no scientific evidence for them.</p>"));
    v.push(lifeline("f23", "Focus 23 — Quiet Reflection", "36 min", [0.5, 2.5, 4.5],
        "Let memories and feelings surface gently, and let them go with kindness.",
        "<p>Lifeline's Focus 23 is described in spiritual terms. Here it's an interpretation used as a gentle reflective rest. Speculative.</p>"));
    v.push(lifeline("f25", "Focus 24–26 — Belief Systems", "36 min", [0.5, 2.0, 40.0],
        "Notice your beliefs as if from a distance, with curiosity rather than judgement.",
        "<p>Lifeline's Focus 24–26 are described as \"belief system territories\". Here they're an interpretation: very deep delta paired with a gamma layer, used as a reflective meditation. Speculative.</p>"));
    v.push(lifeline("f27", "Focus 27 — The Park", "36 min", [0.5, 1.5, 40.0],
        "Imagine a peaceful place of your own creation: a garden, a shore, a meadow. Rest there in complete safety.",
        "<p>Monroe called Focus 27 \"the Park\", a peaceful imagined place. Here it's a guided visualisation of a safe place over deep delta and gamma. Speculative.</p>"));

    // ================= Research-based =================
    let g = "Research-based";
    v.push(p(g, "alpha", "Alpha Calm — 10 Hz", "Relaxation · 15 min", &[200.0], &[1.0], "pink", "binaural", "Research",
      "<p>Alpha (8–13 Hz) is the classic relaxed, eyes-closed state. A 2019 meta-analysis (Garcia-Argibay et al., <i>Psychological Research</i>) found binaural beats had small-to-moderate effects on anxiety and attention. This is a gentle first session.</p>",
      vec![st("Settle", 3.0, &[14.0], "Settle in, and let your breathing slow."), st("Alpha", 10.0, &[10.0], "Relaxed, calm awareness."), st("Return", 2.0, &[12.0], C_RETURN_SHORT)]));
    v.push(p(g, "theta", "Theta Meditation — 6 Hz", "Meditation · 20 min", &[250.0], &[1.0], "pink", "binaural", "Research",
      "<p>Based on Jirakittayakorn &amp; Wongsawat (2017, <i>Frontiers in Neuroscience</i>). They reported that a 6 Hz beat on a 250 Hz carrier increased frontal theta activity, which is linked to meditative states.</p>",
      vec![st("Settle", 3.0, &[10.0], "Let your attention rest on your breathing."), st("Descend", 3.0, &[7.0], "Go deeper with each breath out."),
           st("Theta", 12.0, &[6.0], "Rest in quiet, open awareness.").breath(), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "delta", "Deep Sleep — 3 Hz", "Sleep · 30 min", &[250.0], &[1.0], "brown", "binaural", "Research",
      "<p>Based on Jirakittayakorn &amp; Wongsawat (2018, <i>Frontiers in Human Neuroscience</i>). They reported that a 3 Hz beat lengthened deep (N3) sleep in a small sample. Keep the volume very low.</p>",
      vec![st("Unwind", 5.0, &[10.0], "Let the day go."), st("Drowsy", 7.0, &[6.0], "Let yourself drift."), st("Deep", 18.0, &[3.0], "Sleep.")]));
    v.push(p(g, "beta", "Beta Focus — 16 Hz", "Vigilance · 25 min", &[200.0], &[1.0], "white", "binaural", "Research",
      "<p>Based on Lane et al. (1998, <i>Physiology &amp; Behavior</i>). In that study, beta-range beats (16–24 Hz) improved vigilance and lowered negative mood compared with theta/delta beats.</p>",
      vec![st("Warm up", 3.0, &[12.0], "Open the task you are about to work on."), st("Focus", 22.0, &[16.0], C_FOCUS)]));
    v.push(p(g, "gamma", "Gamma — 40 Hz", "Creativity · 20 min", &[300.0], &[1.0], "pink", "binaural", "Research",
      "<p>Gamma-range binaural beats affected divergent-thinking scores in Reedijk et al. (2015, <i>Frontiers in Human Neuroscience</i>). The effect depended on each person's baseline dopamine level.</p>",
      vec![st("Settle", 3.0, &[14.0], C_SETTLE), st("Gamma", 17.0, &[40.0], "Bright, integrated awareness.")]));
    v.push(p(g, "genus", "40 Hz Isochronic (GENUS-style)", "Auditory 40 Hz · 30 min", &[440.0], &[1.0], "none", "isochronic", "Research",
      "<p>MIT studies (Martorell et al., 2019, <i>Cell</i>) found that 40 Hz auditory tone pulses improved memory measures in mouse models of Alzheimer's disease. Human trials are ongoing. This plays <b>isochronic</b> pulses, the type of stimulus those studies used, not binaural beats. It is not a treatment.</p>",
      vec![st("Tune in", 1.0, &[40.0], C_SETTLE), st("40 Hz", 29.0, &[40.0], C_REST)]));
    v.push(p(g, "anx", "Anxiety Ease", "Calming · 20 min", &[200.0], &[1.0], "surf", "binaural", "Research",
      "<p>Anxiety reduction is among the strongest effects in the 2019 meta-analysis, and Padmanabhan et al. (2005, <i>Anaesthesia</i>) reported less anxiety before surgery. This session adds slow breathing at about six breaths a minute, which has good evidence of its own (via heart-rate variability).</p>",
      vec![st("Arrive", 3.0, &[10.0], "Breathe in for five seconds, and out for five.").breath(), st("Soften", 7.0, &[7.0], "Let your shoulders and jaw soften.").breath(), st("Rest", 10.0, &[5.0], "Nothing to do. Just rest.")]));
    v.push(p(g, "zen", "Zen Alpha → Theta", "Kasamatsu & Hirai · 25 min", &[200.0], &[1.0], "pink", "binaural", "Research",
      "<p>Kasamatsu &amp; Hirai (1966) recorded the EEG of Zen monks. Alpha appeared first, slowed over time, and gave way to rhythmic theta in experienced practitioners. This session follows that same path.</p>",
      vec![st("Zazen", 5.0, &[11.0], "Sit upright and relaxed. Count your breaths from one to ten, then begin again."),
           st("Slowing alpha", 8.0, &[9.0], "If you lose count, simply begin again at one."),
           st("Theta trains", 10.0, &[6.5], "Let counting fall away. Just sit."), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "edison", "Hypnagogic Insight (Edison)", "Lacaux 2021 · 20 min", &[200.0], &[1.0], "pink", "binaural", "Research",
      "<p>Lacaux et al. (2021, <i>Science Advances</i>) found that spending a short time in the first stage of sleep (N1, the hypnagogic \"twilight\") tripled the chance of finding a hidden rule in a problem. Hold a light object in one hand; if it drops, that's your cue to come back.</p>",
      vec![st("Pose the problem", 3.0, &[10.0], "Hold a light object loosely in one hand. Bring to mind a problem you want to solve."),
           st("Twilight", 14.0, &[5.0], "Drift toward sleep while keeping the problem loosely in mind. If the object drops, gently return."),
           st("Capture", 3.0, &[14.0], "Return now, and write down whatever came to you.")]));
    v.push(p(g, "alphatheta", "Alpha-Theta Training", "Peniston protocol · 30 min", &[220.0], &[1.0], "surf", "binaural", "Research",
      "<p>Alpha-theta neurofeedback (Peniston &amp; Kulkosky, 1989) guides people to the boundary between alpha and theta, a deeply relaxed, imaginative state. This is a binaural version of that idea. It is not a substitute for clinical neurofeedback.</p>",
      vec![st("Alpha", 8.0, &[10.0], C_SETTLE), st("Crossover", 14.0, &[7.5], "Hover at the edge of sleep, where images begin to form."), st("Return", 8.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "hrv", "Resonance Breathing 0.1 Hz", "HRV coherence · 15 min", &[200.0], &[1.0], "surf", "binaural", "Research",
      "<p>Breathing at about 6 breaths a minute (0.1 Hz) maximises heart-rate variability (Lehrer &amp; Gevirtz, 2014). It is one of the best-supported self-calming techniques there is. A gentle 10 Hz alpha beat plays alongside the breath pacer.</p>",
      vec![st("Breathe", 15.0, &[10.0], "Follow the circle. Breathe in as it grows, and out as it shrinks.").breath()]));
    v.push(p(g, "lucid", "Lucid Dreaming Prompt", "Voss 2014 · 30 min", &[100.0, 300.0], &[1.0, 0.25], "brown", "binaural", "Research",
      "<p>Voss et al. (2014, <i>Nature Neuroscience</i>) found that 25–40 Hz electrical stimulation (tACS) during REM sleep increased lucid dreaming. This session pairs theta with a quiet 40 Hz layer. Binaural beats are not tACS, so treat this as an experiment.</p>",
      vec![st("Intention", 4.0, &[10.0, 14.0], "Repeat silently: next time I am dreaming, I will remember that I am dreaming."),
           st("Drift", 10.0, &[6.0, 25.0], "Let yourself drift toward sleep, holding the intention lightly."),
           st("Dream window", 16.0, &[4.0, 40.0], "Rest.")]));
    v.push(p(g, "nap", "Power Nap", "Brooks & Lack 2006 · 20 min", &[200.0], &[1.0], "brown", "binaural", "Research",
      "<p>Brooks &amp; Lack (2006, <i>Sleep</i>) found that a 10-minute nap gave the best balance of alertness and grogginess. This session settles you down, holds theta for about 10 minutes, then brings you back up through alpha into beta.</p>",
      vec![st("Wind down", 4.0, &[10.0], "Close your eyes. Let your body grow heavy."), st("Nap", 10.0, &[4.0], "Rest."),
           st("Rise", 6.0, &[16.0], "Time to wake. Stretch, breathe deeply, and open your eyes.").ramp(180.0)]));
    v.push(p(g, "nsdr", "Yoga Nidra / NSDR", "Kjaer 2002 · 25 min", &[200.0], &[1.0], "surf", "binaural", "Research",
      "<p>A PET study of yoga nidra (Kjaer et al., 2002, <i>Cognitive Brain Research</i>) found increased dopamine release during the practice. This session follows the classic structure: settle, body scan, deep rest, return.</p>",
      vec![st("Sankalpa", 3.0, &[10.0], "Set a short, positive intention, and repeat it silently three times."),
           st("Body scan", 8.0, &[7.0], "Move your attention slowly through your body: right hand, arm, shoulder, left hand, arm, shoulder, down through your torso, legs and feet."),
           st("Deep rest", 11.0, &[3.5], "Awareness without effort. Rest."), st("Return", 3.0, &[10.0], "Repeat your intention once more, then gently return.")]));
    v.push(p(g, "smr", "SMR Calm Focus — 13.5 Hz", "Study / ADHD-style · 30 min", &[220.0], &[1.0], "white", "binaural", "Research",
      "<p>Sensorimotor rhythm (SMR, 12–15 Hz) is the target of well-studied neurofeedback protocols for attention (Arns et al., 2009 meta-analysis). Binaural beats are <i>not</i> neurofeedback, and trials of binaural beats for ADHD have been mixed (Kennel et al., 2010).</p>",
      vec![st("Warm up", 2.0, &[11.0], C_FOCUS), st("SMR", 28.0, &[13.5], C_FOCUS)]));
    v.push(p(g, "pain", "Pain Relief Support", "Meta-analysis · 20 min", &[200.0], &[1.0], "surf", "binaural", "Research",
      "<p>The 2019 meta-analysis found binaural beats moderately reduced how much pain people perceived. This session combines a theta beat with slow breathing. It is not medical treatment; see a doctor about any persistent pain.</p>",
      vec![st("Settle", 4.0, &[10.0], "Breathe slowly. Let the area of discomfort soften with each breath out.").breath(), st("Theta", 16.0, &[5.0], C_REST)]));
    v.push(p(g, "monks", "Compassion Gamma", "Lutz 2004 · 25 min", &[100.0, 300.0], &[0.8, 0.5], "pink", "binaural", "Research",
      "<p>Lutz et al. (2004, <i>PNAS</i>) recorded very high gamma activity in Tibetan monks during \"non-referential compassion\" meditation. This session layers a 40 Hz beat over an alpha base.</p>",
      vec![st("Settle", 3.0, &[10.0, 14.0], C_SETTLE), st("Loving-kindness", 7.0, &[10.0, 30.0], "Bring to mind someone you love. Wish them well: may you be happy, may you be safe."),
           st("Open compassion", 15.0, &[8.0, 40.0], "Let that wish expand to include everyone, without exception.")]));

    // ================= Purpose & Tradition =================
    let g = "Purpose & Tradition";
    v.push(p(g, "obe", "Astral Projection / OBE", "6.3 Hz · 30 min", &[100.0, 200.0], &[1.0, 0.6], "pink", "binaural", "Speculative",
      "<p>Online communities often link 6.3 Hz theta to out-of-body experiences. There is no scientific evidence for this. The session uses the Gateway-style \"mind awake, body asleep\" approach and a rope-climbing visualisation from the OBE literature.</p>",
      vec![st("Relax", 5.0, &[10.0, 10.0], C_SETTLE), st("Body asleep", 10.0, &[1.5, 6.3], "Let your body fall asleep while your mind stays awake."),
           st("Rope", 13.0, &[1.5, 6.3], "Imagine a rope hanging above you. Reach up with imagined hands and climb, hand over hand."), st("Return", 2.0, &[10.0, 12.0], C_RETURN_SHORT)]));
    v.push(p(g, "rv", "Remote Viewing Practice", "Stargate-style · 25 min", &GW_C, &GW_G, "pink", "binaural", "Speculative",
      "<p>The U.S. government's Stargate program (1978–1995) studied remote viewing. A 1995 evaluation concluded it had never produced useful intelligence. Use this session as a game of imagination, not a skill test.</p>",
      vec![st("Cool down", 5.0, &[8.0, 10.0, 14.0], C_SETTLE), with_min(f10(), 5.0),
           st("Viewing", 12.0, &[2.0, 6.0, 10.0], "Hold your target in mind. Notice first impressions: shapes, colours, textures. Do not name them; just notice."),
           st("Record", 3.0, &[10.0, 12.0, 16.0], "Return now, and sketch whatever you noticed.")]));
    v.push(p(g, "shaman", "Shamanic Journey — 4.5 Hz", "Drum-rate · 20 min", &[180.0], &[1.0], "brown", "isochronic", "Traditional",
      "<p>Shamanic drumming typically beats at about 4–7 beats per second. Michael Harner's core-shamanism method uses about 4.5. Maxfield (1990) reported theta-like changes in listeners. This session plays isochronic pulses at drum rate.</p>",
      vec![st("Intention", 2.0, &[4.5], "Set your intention for the journey."), st("Journey", 15.0, &[4.5], "Imagine an opening in the earth, and travel through it to a peaceful landscape."),
           st("Callback", 3.0, &[7.0], "The drum calls you back. Return the way you came.")]));
    v.push(p(g, "pineal", "Third Eye / Pineal", "6.3 Hz on 963 Hz · 25 min", &[963.0], &[0.7], "pink", "binaural", "Speculative",
      "<p>Spiritual traditions link the \"third eye\" to the pineal gland, and online communities link it to 6.3 Hz theta and 963 Hz. There is no scientific evidence; this is purely traditional/speculative. It uses a gentle carrier volume.</p>",
      vec![st("Settle", 4.0, &[10.0], C_SETTLE), st("Brow focus", 21.0, &[6.3], "Rest your attention lightly at the space between your eyebrows.")]));
    v.push(p(g, "om", "Om Meditation — 136.1 Hz", "Earth-year tone · 20 min", &[136.1], &[1.0], "none", "binaural", "Traditional",
      "<p>136.1 Hz is Hans Cousto's \"Earth year\" tone (The Cosmic Octave, 1978) and is widely used as the pitch for chanting Om. A 7.83 Hz beat runs underneath.</p>",
      vec![st("Settle", 3.0, &[10.0], C_SETTLE), st("Om", 17.0, &[7.83], "On each breath out, chant or silently hear the sound Om, matched to the tone.").breath()]));
    v.push(p(g, "heal", "Deep Restoration — Delta", "0.5–2 Hz · 40 min", &[150.0], &[1.0], "brown", "binaural", "Traditional",
      "<p>Deep sleep (delta) is when the body does much of its physical repair, such as growth-hormone release. Whether binaural beats can deepen it is unproven. Use it while falling asleep.</p>",
      vec![st("Unwind", 5.0, &[8.0], C_SETTLE), st("Descend", 10.0, &[3.0], C_REST), st("Delta", 25.0, &[1.0], "Sleep deeply.")]));
    v.push(p(g, "vis", "Visualisation / Goals", "Alpha → theta · 20 min", &[210.0], &[1.0], "surf", "binaural", "Traditional",
      "<p>Mental rehearsal is used widely in sport psychology. Alpha and theta states are traditionally linked to vivid mental imagery.</p>",
      vec![st("Relax", 4.0, &[10.0], C_SETTLE), st("Imagine", 12.0, &[6.0], "Picture yourself having already reached your goal. See, hear and feel it in rich detail."), st("Return", 4.0, &[12.0], C_RETURN_SHORT)]));
    v.push(p(g, "wake", "Morning Energiser", "Alpha → beta → gamma · 15 min", &[220.0], &[1.0], "white", "binaural", "Traditional",
      "<p>Brings you up from relaxed alpha into energetic beta and a short burst of gamma. A gentle alternative to an alarm or a second coffee.</p>",
      vec![st("Wake", 3.0, &[10.0], "Good morning. Take a deep breath and stretch."), st("Rise", 6.0, &[18.0], "Feel energy rising through your body.").ramp(120.0),
           st("Bright", 6.0, &[30.0], "Alert, clear and ready.")]));
    v.push(p(g, "study", "Study & Memory", "Alpha/SMR · 45 min", &[220.0], &[1.0], "white", "binaural", "Traditional",
      "<p>Relaxed focus for reading and learning: a short alpha settle, then SMR/low-beta. The evidence for memory effects is mixed (see the 2019 meta-analysis).</p>",
      vec![st("Settle", 3.0, &[10.0], C_FOCUS), st("Study", 42.0, &[14.0], C_FOCUS)]));
    v.push(p(g, "create", "Creative Flow", "7.5–10 Hz · 30 min", &[210.0], &[1.0], "surf", "binaural", "Traditional",
      "<p>The alpha-theta border (around 7.5–8 Hz) is popularly linked to creative \"flow\". This session swings gently between 10 and 7.5 Hz.</p>",
      vec![st("Open", 5.0, &[10.0], "Let ideas come without judging them."), st("Flow", 10.0, &[7.5], "Follow whatever images or ideas arise."),
           st("Shape", 7.0, &[10.0], "Begin to shape what you found."), st("Flow 2", 8.0, &[7.5], "Let go again, and see what else appears.")]));
    v.push(p(g, "stress", "Stress Release", "10 → 6 Hz · 15 min", &[200.0], &[1.0], "surf", "binaural", "Traditional",
      "<p>A short wind-down for after work. Slow breathing does most of the work here; the beat is a gentle companion.</p>",
      vec![st("Exhale", 5.0, &[10.0], "Breathe out longer than you breathe in. Let tension leave with each breath out.").breath(), st("Release", 10.0, &[6.0], C_REST)]));
    v.push(p(g, "insomnia", "Insomnia Wind-down", "Glide to 1.5 Hz · 45 min", &[150.0], &[1.0], "brown", "binaural", "Traditional",
      "<p>A long, slow glide from alpha into deep delta, meant to play while you fall asleep. Keep it very quiet; brown noise masks household sounds.</p>",
      vec![st("Lie down", 5.0, &[10.0], "Get comfortable in bed. Nothing more to do today."), st("Glide", 15.0, &[4.0], C_REST).ramp(600.0), st("Sleep", 25.0, &[1.5], "Sleep.").ramp(600.0)]));
    v.push(p(g, "headache", "Tension Headache Ease", "10 Hz · 20 min", &[180.0], &[0.8], "pink", "binaural", "Speculative",
      "<p>Some people report relief from tension headaches with alpha relaxation, but there is no reliable evidence. Keep the volume low. See a doctor for severe, sudden or unusual headaches.</p>",
      vec![st("Soften", 20.0, &[10.0], "Let your forehead, jaw and neck soften with each breath.").breath()]));
    v.push(p(g, "euphoria", "Euphoria / Bliss", "Gamma + alpha · 20 min", &[200.0, 300.0], &[0.8, 0.4], "pink", "binaural", "Speculative",
      "<p>Online claims link certain beats to \"endorphin release\" and euphoria. There is no evidence for this. Use it as an uplifting gratitude meditation.</p>",
      vec![st("Gratitude", 8.0, &[10.0, 20.0], "Bring to mind three things you are grateful for."), st("Bliss", 12.0, &[10.0, 40.0], "Let the warmth of gratitude spread through your whole body.")]));
    v.push(p(g, "grounding", "Grounding — Schumann", "7.83 Hz · 20 min", &[194.18], &[1.0], "surf", "binaural", "Speculative",
      "<p>7.83 Hz on Cousto's \"Earth day\" tone of 194.18 Hz. The Schumann resonance is a real atmospheric phenomenon, but there is no evidence that matching it affects the body.</p>",
      vec![st("Ground", 20.0, &[7.83], "Feel the weight of your body supported by the earth beneath you.")]));


    // ================= Altered States (research-ranked) =================
    let g = "Altered States";
    v.push(p(g, "hypno", "Self-Hypnosis — Deep Trance", "Most-researched ASC · 19 min", &[200.0], &[0.6], "surf", "binaural", "Research",
      "<p><b>Hypnosis has more research behind it than any other altered state here.</b> An umbrella review covered 49 meta-analyses and 261 studies (Rosendahl et al., 2024). A meta-analysis of 85 trials found it relieves pain, and it can be delivered as recorded audio (Thompson et al., 2019). The APA defines it as \"a state of consciousness involving focused attention and reduced peripheral awareness\".</p><p>This is a classic induction: eye fixation, progressive relaxation, a staircase deepener, a safe place, a calm-anchor suggestion and a count-up emergence. Self-hypnosis works best with practice: three or more sessions. You stay in control and can open your eyes at any time. <b>Skip this if you have a psychotic or dissociative disorder.</b></p>",
      vec![st("Welcome", 1.0, &[10.0], "This is a session of self hypnosis for deep relaxation. You remain in control throughout, and you can open your eyes and return at any time, simply by choosing to."),
           st("Eye fixation", 2.0, &[9.0], "Choose a spot slightly above eye level and rest your gaze on it. As you watch it, your eyelids begin to feel heavier. When they want to close, let them close."),
           st("Relaxation", 3.0, &[7.0], "Let a wave of relaxation begin at the top of your head. Let it flow down through your face, your jaw, your neck and shoulders, down your arms to your fingertips, through your chest and back, and down through your legs to the soles of your feet."),
           st("Staircase", 3.0, &[6.0], "Imagine a staircase with ten steps. With each step down, you go twice as deep into calm. Ten. Nine. Eight. Seven. Six. Five. Four. Three. Two. One. Deeply relaxed."),
           st("Safe place", 3.0, &[5.0], "At the bottom of the stairs is a place of complete peace, real or imagined. Notice what you see, the sounds around you, the temperature of the air. You are safe here."),
           st("Suggestion", 3.0, &[5.0], "In this calm state, your mind is open to helpful ideas. Each day you feel calmer, clearer, and more at ease. Stress passes through you, and leaves you steady."),
           st("Anchor", 2.0, &[6.0], "Gently press your thumb and forefinger together. From now on, this simple touch can bring back this feeling of calm whenever you need it."),
           st("Emergence", 2.0, &[14.0], "In a moment, count with me from one to five. One, energy returning. Two, aware of your body. Three, breathing deeper. Four, ready to open your eyes. Five, eyes open, wide awake and refreshed.").ramp(60.0)]));
    v.push(with_tex(p(g, "ganzfeld", "Ganzfeld — Red Light Imagery", "Sensory homogenisation · 30 min", &[200.0], &[0.0], "pink", "binaural", "Research",
      "<p>The <b>Ganzfeld</b> (\"whole field\") gives the senses a perfectly even input: steady noise in the ears and a uniform soft light on closed eyes. After a few minutes the brain starts generating its own imagery, from colours and shapes to vivid dream-like scenes. It is among the most-studied altered states, with decades of EEG work (Pütz et al., 2006; Wackermann et al., 2008) and fMRI showing the thalamus progressively decoupling from the cortex.</p><p>A recent multimodal-Ganzfeld study found that <b>red light</b> produced the most complex imagery, statistically equivalent on that measure to a moderate dose of psilocybin (Fort et al.).</p><p><b>How to use it:</b> dim the room, press <b>🔴 Ganzfeld light</b> to fill the screen with steady red (it never flickers), sit facing it at arm's length with your eyes closed, and just observe. Keep the noise at a comfortable, steady level.</p>",
      vec![st("Settle", 3.0, &[0.0], "Dim the room and turn on the Ganzfeld light. Sit facing the screen, close your eyes, and let your gaze rest in the soft red glow."),
           st("Ganzfeld", 25.0, &[0.0], "Simply observe. Let colours, shapes and images arise on their own, without trying to hold on to them."),
           st("Return", 2.0, &[0.0], "Slowly open your eyes. Take a moment to recall what you saw.")]), None));
    v.last_mut().unwrap().visual = "ganzfeld".into();
    v.push(with_tex(p(g, "drumtrance", "Rhythmic Drumming Trance", "Hove 2016 · 20 min", &[100.0], &[0.25], "none", "binaural", "Research",
      "<p>Steady drumming at about four beats per second is the backbone of trance rituals worldwide. In an fMRI study of experienced practitioners (Hove et al., 2016, <i>Cerebral Cortex</i>), drum-induced trance coupled the default-mode hub (posterior cingulate) with control regions and <b>decoupled the auditory pathway</b>: the brain tuned out the drum and turned inward. This session plays a synthesised frame drum with a slight human swing over a quiet theta layer, then the traditional faster \"call-back\" rhythm at the end.</p>",
      vec![st("Settle", 2.0, &[4.0], "Close your eyes. Let the steady drum carry your attention."),
           st("Trance", 15.0, &[4.2], "Let the drum fade into the background. Follow whatever images, memories or sensations arise, as if on a journey."),
           st("Call-back", 3.0, &[7.0], "The drum quickens. Gently return the way you came, and bring back what you found.").ramp(20.0)]), tx("drum", 1.0, 0.0, 0.0)));
    v.push(with_tex(p(g, "tlr", "Lucid Dreaming — Targeted Lucidity Reactivation", "Carr 2023 · 95 min", &[150.0], &[0.3], "brown", "binaural", "Research",
      "<p><b>Targeted Lucidity Reactivation (TLR)</b> pairs a sound cue with a lucid mindset before sleep, then replays the cue quietly while you sleep. In the lab, 50% of participants had a lucid dream in a single nap (Carr et al., 2023). An at-home version still beat controls on the second night (Northwestern, 2024).</p><p><b>For best results:</b> sleep for 5–6 hours, get up for about 20 minutes, then start this session as you go back to bed. That puts the cue window in the REM-rich early morning. Keep the volume very low so the cues don't wake you.</p>",
      vec![st("Training 1", 3.0, &[10.0], "Each time you hear this chime, pause and ask yourself: am I dreaming? Look at your hands, notice your breathing, and look around with curiosity.").tex(1.0),
           st("Training 2", 3.0, &[9.0], "Again, when you hear the chime, become aware. Notice that you are aware. Imagine noticing the chime inside a dream, and realising: this is a dream.").tex(1.0),
           st("Training 3", 3.0, &[8.0], "One more time. Chime, awareness, and the thought: I am dreaming. Now lie back and let yourself drift off to sleep, keeping that intention lightly in mind.").tex(1.0),
           st("Fall asleep", 26.0, &[4.0], "Sleep.").tex(0.0).ramp(300.0),
           st("Cue window", 60.0, &[4.0], "").tex(0.35)]), tx("tlr", 0.8, 0.0, 30.0)));
    v.push(with_tex(p(g, "swspulses", "Slow-Wave Sleep Pulses — 0.8 Hz", "Ngo 2013 · 75 min", &[150.0], &[0.0], "brown", "binaural", "Research",
      "<p>Ngo et al. (2013, <i>Journal of Sleep Research</i>) found that soft sound pulses at 0.8 Hz <b>increased and entrained the brain's slow oscillations</b> once stable deep sleep was established. The same group's closed-loop version, which times each pulse to the brain wave, also improved memory (<i>Neuron</i>, 2013). The study found that pulses <i>before</i> sleep delayed sleep onset, so this session waits 25 minutes before starting them.</p><p>This is an <b>open-loop approximation</b>: without an EEG headset, the pulses can't be timed to your brain waves. Keep the volume just barely audible.</p>",
      vec![st("Fall asleep", 25.0, &[0.8], "Get comfortable and let yourself fall asleep. The soft pulses will begin later, once you are asleep.").tex(0.0),
           st("Slow-wave pulses", 50.0, &[0.8], "").tex(0.5)]), tx("pulses", 0.6, 0.0, 0.0)));
    v.push(with_tex(p(g, "kirtan", "Kirtan Kriya (Sa-Ta-Na-Ma)", "Innes 2017 RCT · 13 min", &[200.0], &[0.2], "none", "binaural", "Research",
      "<p>A 12-minute chanting meditation from Kundalini yoga. In a randomised trial of adults with memory complaints, 12 minutes a day for 3 months significantly improved memory and cognitive performance, and the gains held at 6 months (Innes et al., 2017, <i>Journal of Alzheimer's Disease</i>). A bell plays one note per second to pace each syllable.</p><p><b>Technique:</b> on each syllable, touch your thumb to a fingertip: <b>Sa</b> index, <b>Ta</b> middle, <b>Na</b> ring, <b>Ma</b> little finger. Chant aloud, then whisper, then silently, then whisper, then aloud.</p>",
      vec![st("Aloud", 2.0, &[10.0], "Chant aloud with the bell, one syllable per note. Saa. Taa. Naa. Maa. Touch your thumb to each fingertip in turn: index, middle, ring, little finger."),
           st("Whisper", 2.0, &[8.0], "Now whisper the sounds, keeping the finger movements going."),
           st("Silent", 4.0, &[6.0], "Now chant silently, in your mind only, still moving your fingers with the bell."),
           st("Whisper", 2.0, &[8.0], "Return to a whisper."),
           st("Aloud", 2.0, &[10.0], "Chant aloud once more."),
           st("Rest", 1.0, &[10.0], "Stop. Breathe in deeply, stretch your arms up, and breathe out slowly. Rest in the silence.").tex(0.0)]), tx("kirtan", 0.8, 392.0, 0.0)));
    v.push(with_tex(p(g, "bowls", "Singing Bowl Sound Bath", "Theta-beating bowls · 25 min", &[200.0], &[0.0], "none", "binaural", "Research",
      "<p>Real singing bowls produce slow acoustic beats between nearly matched vibration modes. An EEG study found brain activity synchronising at these theta-range beat frequencies (Ahn et al., 2023). An observational study of 62 adults found a sharp drop in tension, anger and fatigue after a bowl meditation (Goldsby et al., 2017), and a 2025 systematic review covered 9 randomised trials. These bowls are synthesised so their modes beat at 6 Hz, gliding down to 4.5 Hz.</p>",
      vec![st("Arrive", 3.0, &[7.0], "Lie down and let the sound of the bowls wash over you."),
           st("Bath", 19.0, &[6.0], "Let the shimmer of each bowl ripple through your body.").ramp(120.0),
           st("Deep", 3.0, &[4.5], "Rest in the stillness between the sounds.")]), tx("bowls", 0.9, 220.0, 9.0)));
    v.push(with_tex(p(g, "mantra6", "Mantra / Rosary Rhythm — 6 Breaths per Minute", "Bernardi 2001 BMJ · 15 min", &[200.0], &[0.25], "none", "binaural", "Research",
      "<p>Bernardi et al. (2001, <i>BMJ</i>) found that reciting either the rosary (Ave Maria) or a yoga mantra slowed breathing to about <b>six breaths a minute</b>. That synchronised heart-rate, blood-pressure and breathing rhythms and raised baroreflex sensitivity. A soft bell marks each 10-second breath cycle.</p>",
      vec![st("Rhythm", 15.0, &[10.0], "Choose a short phrase: a prayer, a mantra, or a single word like peace. Breathe in as the bell sounds, then silently recite your phrase slowly through the whole breath out.").breath()]), tx("breathbell", 0.7, 528.0, 10.0)));
    v.push(with_tex(p(g, "rain", "Natural Soundscape — Rain", "Gould van Praag 2017 · 30 min", &[200.0], &[0.15], "none", "binaural", "Research",
      "<p>In an fMRI study, natural soundscapes shifted default-mode connectivity toward an outward, less ruminative focus and increased parasympathetic heart activity compared with artificial sounds (Gould van Praag et al., 2017, <i>Scientific Reports</i>). This session plays synthesised rain over a barely-audible alpha beat.</p>",
      vec![st("Rain", 30.0, &[10.0], "Listen to the rain. Let your thoughts drift like the sound of falling water.")]), tx("rain", 1.0, 0.0, 0.0)));
    v.push(with_tex(p(g, "omdrone", "Om Chanting — 136.1 Hz", "Kalyani 2011 fMRI · 15 min", &[136.1], &[0.9], "none", "monaural", "Research",
      "<p>In an fMRI study, audible Om chanting (compared with a \"ssss\" control) <b>deactivated limbic regions</b> including the amygdala, hippocampus and anterior cingulate. The authors compare this to vagus-nerve stimulation (Kalyani et al., 2011, <i>International Journal of Yoga</i>). A slow bell paces each long chant, and the tone sits at 136.1 Hz, the traditional Om pitch.</p>",
      vec![st("Om", 15.0, &[7.83], "Breathe in as the bell sounds. On the long breath out, chant Om aloud, feeling the vibration in your chest and head.").breath()]), tx("breathbell", 0.5, 272.2, 10.0)));

    // ================= Sound Systems Library =================
    // One clear example of every known way of delivering a beat or rhythm to the brain.
    let g = "Sound Systems Library";
    const C_BODY: &str = "Let a wave of relaxation begin at the top of your head. Let it flow down through your face, your jaw, your neck and shoulders, down your arms to your fingertips, through your chest and back, and down through your legs to the soles of your feet.";
    const C_PULSE: &str = "Notice the gentle pulse within the sound. Let your attention rest on it, relaxed and awake.";
    const C_EXHALE: &str = "Breathe out longer than you breathe in. Let tension leave with each breath out.";
    const C_HEAVY: &str = "Close your eyes. Let your body grow heavy.";
    let demo = |id: &str, name: &str, short: &str, car: &[f64], gains: &[f64], noise: &str, mode: &str, ev: &str, hz: &[f64], hold: &[f64], about: &str| {
        p(g, id, name, short, car, gains, noise, mode, ev, about,
          vec![st("Tune in", 2.0, hz, C_SETTLE), st("Listen", 3.0, hz, C_PULSE),
               st("Settle", 8.0, hold, C_REST).ramp(90.0), st("Return", 2.0, &vec![10.0; hz.len()], C_RETURN_SHORT)])
    };
    v.push(demo("sys_binaural", "Binaural Beats — 6 Hz", "Two tones, one per ear · 15 min", &[200.0], &[1.0], "pink", "binaural", "Research", &[10.0], &[6.0],
      "<p><b>Binaural beats</b>: a slightly different pure tone in each ear (here 197 Hz left, 203 Hz right). The brainstem combines them and you hear a slow 6 Hz \"wobble\" that isn't in the air at all. <b>Needs stereo headphones.</b> Discovered by Heinrich Wilhelm Dove in 1839 and popularised by Gerald Oster (<i>Scientific American</i>, 1973).</p>"));
    v.push(demo("sys_monaural", "Monaural Beats — 6 Hz", "Tones mixed before the ear · 15 min", &[200.0], &[1.0], "pink", "monaural", "Research", &[10.0], &[6.0],
      "<p><b>Monaural beats</b>: the same two tones as binaural beats, but mixed together <i>before</i> they reach your ears, so the beat is a real, physical rise and fall in loudness. They work on speakers, and EEG studies find they drive a stronger brain response than binaural beats (Schwarz &amp; Taylor, 2005).</p>"));
    v.push(demo("sys_isochronic", "Isochronic Tones — 6 Hz", "Single tone switched on/off · 15 min", &[200.0], &[1.0], "none", "isochronic", "Research", &[10.0], &[6.0],
      "<p><b>Isochronic tones</b>: one tone switched cleanly on and off at the target rate, with soft edges so there are no clicks. They work on speakers and produce the clearest \"auditory steady-state response\" in EEG. The term was popularised by Arturo Manns in the 1980s.</p>"));
    v.push(demo("sys_am", "Amplitude-Modulated Tones — 6 Hz", "Smooth swell and fade · 15 min", &[200.0], &[1.0], "pink", "am", "Research", &[10.0], &[6.0],
      "<p><b>Amplitude modulation (AM)</b>: a tone that swells and fades smoothly, like a slow tremolo, instead of switching hard on and off. This is the technique behind \"functional music\" research; Woods et al. (2024, <i>Communications Biology</i>) found that beta-rate amplitude-modulated music improved sustained attention, especially in people with ADHD symptoms. Gentler than isochronic pulses and works on speakers.</p>"));
    v.push(demo("sys_hybrid", "Hybrid Binaural + Isochronic — 6 Hz", "Both at once · 15 min", &[200.0], &[1.0], "pink", "hybrid", "Traditional", &[10.0], &[6.0],
      "<p><b>Hybrid beats</b>: a binaural pair (a different tone in each ear) that is <i>also</i> pulsed isochronically at the same rate. Popular in commercial apps on the theory that it combines both paths into the brain. There is no study showing it beats either method alone. Best on headphones.</p>"));
    v.push(demo("sys_bilateral", "Bilateral Alternating Tones — 1 Hz", "Left ↔ right, EMDR-style · 15 min", &[220.0], &[0.8], "pink", "bilateral", "Research", &[1.0], &[0.8],
      "<p><b>Bilateral stimulation</b>: a tone that alternates between the left and right ear, about once a second. It's the audio form of the left–right stimulation used in <b>EMDR therapy</b> (Francine Shapiro, 1987), which the WHO recommends for PTSD. Studies suggest alternating stimulation can lower vividness and emotional charge of upsetting memories. This session is for relaxation only; it is <b>not EMDR therapy</b>, which needs a trained therapist. Needs headphones.</p>"));
    v.push(demo("sys_spatial", "Spatial / 8D Rotation — 0.1 Hz", "Sound circles your head · 15 min", &[180.0], &[0.8], "surf", "spatial", "Traditional", &[0.15], &[0.1],
      "<p><b>Spatial (\"8D\") audio</b>: the tone glides slowly from one side of your head to the other and back, about once every ten seconds. \"8D audio\" became an internet trend in 2018; the gentle motion holds attention and many people find it absorbing and calming. There's no clinical research on it as entrainment. Needs headphones.</p>"));
    v.push(demo("sys_layered", "Layered Beats (Hemi-Sync-inspired)", "Three beat layers at once · 15 min", &GW_C, &GW_G, "surf", "binaural", "Gateway", &[10.0, 10.0, 14.0], &[4.0, 7.0, 12.0],
      "<p><b>Layered (multi-frequency) beats</b>: several binaural pairs at once, each on its own carrier, so the brain gets a \"chord\" of rates rather than one. This is the idea behind the Monroe Institute's Hemi-Sync (patented 1975) and the Gateway tapes. Here: 4, 7 and 12 Hz on 100, 200 and 300 Hz carriers.</p>"));
    v.push(p(g, "sys_descend", "Descending Carriers (Holosync-inspired)", "Deepening layers · 30 min", &[100.0, 150.0], &[1.0, 0.5], "surf", "binaural", "Traditional",
      "<p><b>Descending layered tones</b>: commercial programs such as Centerpointe's Holosync (1989) use layered binaural beats that sink in stages from alpha toward delta, often under rain or bell sounds, and sell \"levels\" that get deeper over months. This is an original interpretation of that idea, not their audio. Claims of rewiring the brain are marketing; there is no independent research on these programs.</p>",
      vec![st("Alpha", 5.0, &[10.0, 12.0], C_SETTLE), st("Alpha–theta", 7.0, &[7.0, 10.0], C_EXHALE).ramp(120.0),
           st("Theta", 8.0, &[5.0, 7.0], C_REST).ramp(120.0), st("Delta", 8.0, &[2.5, 4.0], "Nothing to do. Just rest.").ramp(120.0),
           st("Return", 2.0, &[10.0, 12.0], C_RETURN_SHORT)]));
    v.push(p(g, "sys_noise", "Coloured Noise Only (Pink)", "No beats · 20 min", &[200.0], &[0.0], "pink", "binaural", "Research",
      "<p><b>Coloured noise</b>: no tones or beats at all, just a steady pink-noise bed. White noise has equal energy at every frequency; pink falls 3 dB per octave (softer, like rain); brown falls further (deep, like surf). Steady noise masks distractions, and a 2021 review found it can modestly help some people fall asleep, though the evidence is mixed (Riedy et al., <i>Sleep Medicine Reviews</i>). Useful as a control: compare how you feel with and without beats.</p>",
      vec![st("Noise", 20.0, &[0.0], "Rest in the stillness between the sounds.")]));
    v.push(with_tex(p(g, "sys_drum", "Rhythmic Auditory Driving — Frame Drum", "Drum at 4–7 beats/s · 15 min", &[100.0], &[0.0], "none", "binaural", "Research",
      "<p><b>Rhythmic auditory driving</b>: the oldest sound system of all. A steady drum, rattle or chant at roughly 4–7 beats per second has been used for trance across cultures for thousands of years. Andrew Neher (1962) first proposed that drumming drives brain rhythms; Hove et al. (2016, <i>Cerebral Cortex</i>) showed in fMRI how drum trance turns attention inward. No beat tones here: only the synthesised drum.</p>",
      vec![st("Settle", 2.0, &[4.2], "Close your eyes. Let the steady drum carry your attention."),
           st("Drive", 11.0, &[4.5], "Follow whatever images or ideas arise."),
           st("Call-back", 2.0, &[7.0], "The drum calls you back. Return the way you came.").ramp(20.0)]), tx("drum", 1.0, 0.0, 0.0)));

    // ================= Astral Projection & OBE =================
    // Classic out-of-body techniques from the literature. All are spiritual / experiential
    // practices: labelled Speculative, with the real neuroscience in each explanation.
    let g = "Astral Projection & OBE";
    const C_BODY_ASLEEP: &str = "Let your body fall asleep while your mind stays awake.";
    const C_ROPE: &str = "Imagine a rope hanging above you. Reach up with imagined hands and climb, hand over hand.";
    const C_EDGE: &str = "Hover at the edge of sleep, where images begin to form.";
    const C_SCAN: &str = "Move your attention slowly through your body: right hand, arm, shoulder, left hand, arm, shoulder, down through your torso, legs and feet.";
    const C_COUNT: &str = "In a moment, count with me from one to five. One, energy returning. Two, aware of your body. Three, breathing deeper. Four, ready to open your eyes. Five, eyes open, wide awake and refreshed.";
    const C_GROUND: &str = "Feel your connection to the ground beneath you, stable and safe.";
    let mut s = gateway_prep(); s.push(with_min(f10(), 8.0));
    s.push(st("Vibrations", 8.0, &[1.5, 4.0, 10.0], "Notice any tingling, buzzing or waves of sensation. Let them grow without fear. They are a normal part of deep relaxation."));
    s.push(st("Roll out", 8.0, &[1.5, 4.0, 10.0], "Without moving your physical body, imagine gently rolling sideways, as if turning over in bed, and floating up and away."));
    s.push(ret(3.0));
    v.push(p(g, "ap_monroe", "Monroe Vibrational State & Roll-out", "Journeys Out of the Body · 37 min", &GW_C, &GW_G, "pink", "binaural", "Speculative",
      "<p>Robert Monroe's method from <i>Journeys Out of the Body</i> (1971): deep relaxation into \"mind awake, body asleep\", then the <b>vibrational state</b>, then imagining <b>rolling out</b> of the body like turning over in bed. The tingling and buzzing are well-known sensations of the edge of sleep and sleep paralysis. Uses the Gateway-style layered tones.</p>", s));
    v.push(p(g, "ap_rope", "Rope Technique (Robert Bruce)", "Astral Dynamics · 30 min", &[100.0, 200.0], &[1.0, 0.6], "pink", "hybrid", "Speculative",
      "<p>Robert Bruce's <b>rope technique</b> (<i>Astral Dynamics</i>, 1999): relax deeply, then imagine an invisible rope above you and climb it with imagined hands, pulling yourself up until you feel dizzy or a strong tug. Here on hybrid beats (binaural + isochronic) at 1.5 and 6.3 Hz.</p>",
      vec![st("Relax", 4.0, &[10.0, 10.0], C_SETTLE), st("Body scan", 5.0, &[4.0, 8.0], C_SCAN).ramp(90.0),
           st("Body asleep", 7.0, &[1.5, 6.3], C_BODY_ASLEEP).ramp(90.0),
           st("Rope", 12.0, &[1.5, 6.3], C_ROPE), st("Return", 2.0, &[10.0, 12.0], C_RETURN_SHORT)]));
    v.push(p(g, "ap_muldoon", "Target Technique (Muldoon & Carrington)", "1929 classic · 28 min", &[150.0], &[1.0], "brown", "binaural", "Speculative",
      "<p>From <i>The Projection of the Astral Body</i> (Muldoon &amp; Carrington, 1929): memorise a familiar room or object in detail, relax fully, then rehearse being there in the imagination with all your senses. Muldoon also suggested going to bed slightly thirsty so the dream body \"goes for water\". Uses a slow glide from alpha to deep theta.</p>",
      vec![st("Settle", 4.0, &[10.0], C_SETTLE), st("Relax", 6.0, &[7.0], C_BODY).ramp(120.0),
           st("Target", 15.0, &[5.0], "Picture yourself standing in a familiar room. Notice the light, the textures, the sounds and the smell of the place, in rich detail.").ramp(120.0),
           st("Return", 3.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "ap_phase", "Phase Technique (after waking)", "Raduga · WBTB · 20 min", &[150.0], &[0.8], "brown", "am", "Speculative",
      "<p>Michael Raduga's <b>phase</b> method: wake after about 6 hours of sleep, lie still with your eyes closed, and in the first minutes try to separate: rolling out, floating up, or imagining movement. Raduga's group report that many beginners succeed within a few attempts, and lab work shows these experiences are vivid lucid dreams or sleep-paralysis states (Raduga et al., 2020–2021). Uses gentle AM theta at very low volume.</p>",
      vec![st("Stay still", 3.0, &[6.0], "Get comfortable in bed. Nothing more to do today."),
           st("Edge", 12.0, &[5.0], C_EDGE).ramp(60.0),
           st("Separate", 5.0, &[4.5], "Without moving your physical body, imagine gently rolling sideways, as if turning over in bed, and floating up and away.")]));
    v.push(with_tex(p(g, "ap_wbtb", "Wake-Back-to-Bed OBE Prompt", "Lucid-dream cues · 60 min", &[150.0], &[0.3], "brown", "binaural", "Speculative",
      "<p>Combines the best-supported lucid-dream technique, <b>wake-back-to-bed (WBTB)</b> with a memory intention (Stumbrys et al., 2012 review; Aspy et al., 2017), with out-of-body imagery. Most induced \"astral projections\" happen in REM sleep near waking, so the same timing helps. Sleep 5–6 hours, get up for 20 minutes, then start this as you go back to bed. A soft chime plays while you set the intention, and again very quietly from 15 minutes in, when you are likely to be dreaming. Very low volume.</p>",
      vec![st("Intention", 3.0, &[6.0], "Repeat silently: next time I am dreaming, I will remember that I am dreaming.").tex(1.0),
           st("Drift", 12.0, &[4.0], "Let yourself drift toward sleep, holding the intention lightly.").tex(0.0),
           st("Cue window", 45.0, &[3.0], "").tex(0.3).ramp(300.0)]), tx("tlr", 0.8, 0.0, 30.0)));
    v.push(p(g, "ap_theta_gamma", "Theta–Gamma Astral Stack", "6.3 + 40 Hz · 30 min", &[150.0, 300.0], &[1.0, 0.35], "pink", "binaural", "Speculative",
      "<p>A popular online \"astral\" recipe: the 6.3 Hz \"OBE\" theta beat layered with a quiet 40 Hz gamma beat for lucidity. The 6.3 Hz claim has no scientific basis; gamma activity (around 40 Hz) does rise during lucid dreaming (Voss et al., 2009). Treat it as an experiment.</p>",
      vec![st("Relax", 5.0, &[10.0, 40.0], C_SETTLE), st("Descend", 5.0, &[7.0, 40.0], C_EXHALE).ramp(120.0),
           st("Stack", 17.0, &[6.3, 40.0], C_BODY_ASLEEP).ramp(120.0), st("Return", 3.0, &[10.0, 14.0], C_COUNT)]));
    v.push(p(g, "ap_float", "Floating — Slow Spatial Drift", "8D rotation · 25 min", &[150.0, 200.0], &[0.8, 0.4], "surf", "spatial", "Speculative",
      "<p>The tone slowly circles from ear to ear (about once every 12 seconds) while you imagine floating. Slow, smooth spatial motion is commonly felt as drifting or weightlessness, which many OBE teachers describe as the moment before separation. Needs headphones; keep the volume low.</p>",
      vec![st("Settle", 4.0, &[0.1, 0.1], C_HEAVY), st("Relax", 6.0, &[0.09, 0.09], C_BODY),
           st("Float", 13.0, &[0.08, 0.08], "Feel open space above the top of your head."), st("Return", 2.0, &[0.1, 0.1], C_RETURN_SHORT)]));
    v.push(p(g, "ap_bilateral", "Bilateral Release → Body Asleep", "Calm first · 25 min", &[220.0], &[0.7], "pink", "bilateral", "Speculative",
      "<p>Fear is the most common thing that stops an out-of-body attempt (often at the first buzzing or sleep-paralysis sensation). This session starts with slow left–right alternating tones to settle the nervous system, then hands over to body-asleep imagery. Not therapy.</p>",
      vec![st("Settle", 4.0, &[1.0], C_SETTLE), st("Release", 8.0, &[0.8], C_EXHALE),
           st("Body asleep", 11.0, &[0.6], C_BODY_ASLEEP), st("Return", 2.0, &[1.0], C_RETURN_SHORT)]));

    // ---- more researched and traditional methods (v1.6)
    v.push(p(g, "ap_hypnotic", "Hypnotic Out-of-Body Suggestion (Facco 2019)", "Hypnosis-induced OBE · 24 min", &[200.0], &[0.6], "surf", "binaural", "Research",
      "<p>In a lab study of a highly hypnotisable person, hypnotic suggestions to leave the body produced vivid out-of-body experiences with measurable EEG changes, which differed from simply imagining the same scene (<b>Facco et al., 2019, <i>International Journal of Clinical and Experimental Hypnosis</i></b>). This session uses the same building blocks: a hypnotic induction, a deepener, suggestions to rise out of the body and look back from above, then a careful return. How vivid it gets depends a lot on how hypnotisable you are. You stay in control and can open your eyes at any time. <b>Skip this if you have a psychotic or dissociative disorder.</b></p>",
      vec![st("Welcome", 1.0, &[10.0], "Welcome to the hypnotic out of body session. You remain in control throughout, and you can return at any time simply by opening your eyes. Lie back somewhere you will not be disturbed, and rest your gaze on a spot on the ceiling."),
           st("Eyelids", 2.0, &[9.0], "Keep watching that spot. Your eyelids grow heavier with each breath, heavier and heavier. When they want to close, let them close, and let a wave of comfort spread through you."),
           st("Relaxation", 3.0, &[7.0], C_BODY),
           st("Deepener", 3.0, &[5.0], "Now count down slowly with me, and with each number drift twice as deep. Ten, nine, eight, seven, six, five, four, three, two, one. Deeply, comfortably relaxed."),
           st("Lightness", 3.0, &[4.5], "Notice a pleasant lightness spreading through you. Your body rests heavy and safe, while your awareness feels lighter and lighter, as light as a feather."),
           st("Rising", 4.0, &[4.5], "Now, with the next breath out, let your awareness gently float up, out of your body, rising slowly toward the ceiling. Easily and safely, you are floating above your body."),
           st("Looking down", 4.0, &[5.0], "From up here, look down. See your body resting peacefully below you. Notice the room from this new viewpoint: the light, the shapes, the colours. Everything is calm."),
           st("Return to the body", 2.0, &[6.0], "Now drift slowly back down, and settle gently into your body, fitting it perfectly, like a hand into a glove. Feel its weight, and your breathing.").ramp(30.0),
           st("Emergence", 2.0, &[14.0], C_COUNT).ramp(60.0)]));
    v.push(p(g, "ap_imagined", "Imagined Out-of-Body Viewpoint (Facco 2019)", "Imagination, no hypnosis · 15 min", &[200.0], &[0.7], "pink", "binaural", "Research",
      "<p>The comparison condition in <b>Facco et al. (2019)</b>: simply <i>imagining</i> seeing your body from a point above it, without hypnosis. In that study the imagined version felt less real and showed different brain activity from the hypnotic one, but it is the easiest way to practise shifting your viewpoint, and many OBE teachers start here.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the imagined viewpoint exercise. Lie on your back with your eyes closed. This is pure imagination, with no hypnosis: you will picture your body from the outside, as if from a camera above you."),
           st("Your body", 3.0, &[8.0], "Picture your own body lying here, as if seen from the foot of the bed. Notice your clothes, your hands, the shape of your face, the way you are lying."),
           st("Rise", 3.0, &[7.0], "Let the imagined viewpoint rise slowly upward, until it is near the ceiling, looking straight down at your body below."),
           st("Look around", 4.0, &[6.0], "From the ceiling, look around the room. Notice the furniture, the door, the window, the light. Then look down at your body again, resting peacefully."),
           st("Back", 3.0, &[10.0], "Let the viewpoint float down and slip back behind your own eyes. Feel your body from the inside again. Breathe a little deeper, move your fingers, and open your eyes when you are ready.").ramp(60.0)]));
    v.push(p(g, "ap_kinesthetic", "Kinesthetic Floating (Smith & Messier 2014)", "Feel, don't picture · 22 min", &[200.0], &[0.7], "pink", "binaural", "Research",
      "<p><b>Smith &amp; Messier (2014, <i>Frontiers in Human Neuroscience</i>)</b> scanned a woman who could leave her body at will. She described feeling herself float above her body, spinning, rolling and moving horizontally, while staying aware of her motionless physical body. Her brain showed strong activity in areas for <b>kinesthetic imagery</b>, the felt sense of movement, rather than visual imagery. She had discovered it as a child, at naptime. This session trains that felt sense of movement over a theta beat.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to kinesthetic floating. Lie flat on your back, arms by your sides, eyes closed. In this session you will not picture anything. Instead, you will feel movement, as if from the inside."),
           st("Stillness", 3.0, &[8.0], "Feel your body lying completely still: the weight of your head, your back against the surface, your heels. Know that your body will stay right here, safe and motionless."),
           st("Lift", 4.0, &[6.5], "Now feel a sense of lifting, as if a second, weightless body is rising a hand's width above the first. Don't picture it; feel it, the way you feel your hand without looking."),
           st("Rotate", 4.0, &[6.0], "Feel that weightless body slowly rotating, turning along its length like a log in water, around and around, while your physical body stays still."),
           st("Roll and drift", 5.0, &[5.5], "Now let it roll, swing, or drift sideways through the air above you. Notice that you can feel both at once: the floating body moving, and the physical body still."),
           st("Settle back", 2.0, &[8.0], "Let the floating body sink gently back down and merge with your physical body. Feel your whole weight again.").ramp(60.0),
           st("Return", 2.0, &[12.0], C_RETURN_SHORT).ramp(30.0)]));
    {
        const WIGGLE: &str = "Phantom wiggling. Imagine moving your hand back and forth, without any real movement.";
        const IMAGES: &str = "Observing images. Look into the darkness in front of your closed eyes, and watch for any picture.";
        const ROTATE: &str = "Rotation. Imagine your body turning around its long axis, like a log in water.";
        const LISTEN: &str = "Listening in. Listen for any noise inside your head, and try to make it louder.";
        let mut s = vec![st("Stay still", 1.0, &[6.0], "This is phase technique cycling, for the first moments after waking. Don't move, and keep your eyes closed. When you hear each technique, try it for a few seconds, then switch when the next one comes."),
                         st("Separate", 0.5, &[5.5], "Try to separate straight away: roll out sideways, or float up, without moving your physical body.")];
        for _ in 0..4 {
            s.push(st("Phantom wiggling", 0.25, &[5.0], WIGGLE));
            s.push(st("Observe images", 0.25, &[5.0], IMAGES));
            s.push(st("Rotation", 0.25, &[5.0], ROTATE));
            s.push(st("Listening in", 0.25, &[5.0], LISTEN));
        }
        s.push(st("Separate again", 0.5, &[5.0], "Whichever technique felt strongest, keep it going, then try to separate again: roll out, or float up."));
        s.push(st("Drift and retry", 12.0, &[4.5], "If nothing worked, that's normal. Let yourself drift back to sleep, holding the intention to try again the moment you next wake.").ramp(60.0));
        v.push(p(g, "ap_cycling", "Phase Technique Cycling (Raduga)", "Indirect method · after waking · 18 min", &[150.0], &[0.6], "brown", "am", "Research",
          "<p>Michael Raduga's <b>indirect method</b>: in the first moments after waking, before moving or opening your eyes, try to separate straight away (roll out or float up). If that does not work within a few seconds, <b>cycle</b> through short techniques: phantom wiggling, watching images, rotation and listening in, a few seconds each, until one of them starts to work; then use it and try to separate again. In studies run by Raduga's centre, volunteers following such instructions at home reported phase experiences within a few attempts (Raduga et al., <i>International Journal of Dream Research</i>); independent replication is limited. Best used after about 6 hours of sleep, at very low volume.</p>", s));
    }
    v.push(p(g, "ap_lab", "Lab OBE Induction — Relaxation, Ganzfeld & Imagery", "1970s laboratory recipe · 28 min", &[200.0], &[0.0], "pink", "binaural", "Research",
      "<p>In the 1970s, parapsychologist <b>John Palmer</b> and colleagues tried to induce out-of-body experiences in the laboratory by combining <b>progressive muscle relaxation</b>, a <b>Ganzfeld</b> (even light and steady noise) and suggestions or imagery of leaving the body. A minority of volunteers reported OBE-like experiences (reviewed by Blanke &amp; Thut, 2007). This session recreates that recipe: press <b>🔴 Ganzfeld light</b> for the even red field, keep the noise steady, and follow the relaxation and imagery.</p>",
      vec![st("Settle", 3.0, &[0.0], "Welcome to the laboratory out of body induction. Dim the room, turn on the Ganzfeld light, and lie or sit back facing the soft red glow. Close your eyes. The steady noise is part of the method."),
           st("Progressive relaxation", 7.0, &[0.0], "Tense the muscles of your feet for a moment, then let them go. Now your calves, then your thighs, your belly, your hands, your arms and shoulders, and finally your face. With each release, sink deeper."),
           st("Ganzfeld", 8.0, &[0.0], "Now simply rest in the even glow and the steady sound. Let your senses go quiet. Notice any images or sensations, without holding on to them."),
           st("Leaving the body", 8.0, &[0.0], "Imagine that you are slowly rising out of your body, light and free, and drifting up toward the ceiling. Look around the room from above, and notice everything you can."),
           st("Return", 2.0, &[0.0], "Drift back down into your body. Feel its weight and warmth. Slowly open your eyes, turn off the light, and take a moment to recall what you experienced.")]));
    v.last_mut().unwrap().visual = "ganzfeld".into();
    v.push(p(g, "ap_swing", "Swing Technique — Rocking Sound", "Balance imagery · spatial audio · 22 min", &[180.0], &[0.7], "surf", "spatial", "Speculative",
      "<p>A popular projection technique: imagine swinging on a swing, higher and higher, until at the top of an arc you keep going, out of your body. The sense of balance (the <b>vestibular system</b>) is central to out-of-body experiences in neuroscience (Blanke et al., 2004, <i>Brain</i>), and swinging imagery targets exactly that. Here the tone sways from ear to ear like a swing, once every four seconds. Needs headphones; keep the volume low.</p>",
      vec![st("Settle", 3.0, &[0.25], "Welcome to the swing technique. Wear headphones, lie back, and close your eyes. The tone will sway gently from ear to ear, like a swing moving back and forth. Let your body grow heavy."),
           st("Body asleep", 4.0, &[0.25], C_BODY_ASLEEP),
           st("Swinging", 7.0, &[0.25], "Imagine you are sitting on a swing. Feel it carry you forward and back, in time with the sound. With each arc, swing a little higher, and feel the drop in your stomach at each end."),
           st("Release", 6.0, &[0.25], "At the top of the next arc, let go, and keep rising, floating up and out, as if the swing has launched you gently into the air."),
           st("Return", 2.0, &[0.2], C_RETURN_SHORT)]));
    v.push(p(g, "ap_christos", "Christos Technique (Glaskin 1974)", "Stretch, expand, fly · 30 min", &[200.0], &[0.7], "pink", "binaural", "Traditional",
      "<p>Australian writer <b>G. M. Glaskin</b> described the <b>Christos technique</b> in <i>Windows of the Mind</i> (1974). Traditionally two helpers rub the feet and the centre of the forehead while the traveller imagines stretching beyond the body, first through the feet, then through the head, then expanding like a balloon, before visiting their own front door, rising above the house and flying to a landscape. Glaskin and later writers presented it as a route to out-of-body and past-life experiences; those claims are untested, but it works as a strong guided-imagery journey. If you have a partner, they can gently rub your feet and forehead during the first stage.</p>",
      vec![st("Settle", 3.0, &[10.0], "Welcome to the Christos technique. Lie on your back with your eyes closed. If you have a partner, ask them to rub the soles of your feet and the centre of your forehead, firmly and gently. If not, simply rest, and imagine it."),
           st("Through the feet", 3.0, &[8.0], "Imagine that your legs are stretching out through the soles of your feet, a few centimetres longer than they really are. Feel it. Then let them return to normal."),
           st("Through the head", 3.0, &[7.5], "Now imagine stretching out through the top of your head, a few centimetres taller. Feel it, then return. Now alternate: longer through the feet, then taller through the head."),
           st("Expand", 4.0, &[7.0], "Now let your whole body expand in every direction, like a balloon slowly filling with air, until you fill the whole room."),
           st("Front door", 4.0, &[6.3], "Imagine that you are standing outside your own front door. Notice the door, the ground under your feet, the light, the weather. Look down at your feet."),
           st("Above the house", 4.0, &[6.0], "Now rise up, above the roof of your house. Look down at the roof, the street and the gardens around it. Rise higher, until you see the whole neighbourhood."),
           st("Fly and land", 7.0, &[5.5], "Fly wherever you wish. When you are ready, come down and land somewhere new. Look at your feet. What are you wearing? Look around, and explore."),
           st("Return", 2.0, &[10.0], "Now fly back the way you came, above your house, through the front door, and into your body. Feel your weight again. Move your fingers and toes, and open your eyes when you are ready.").ramp(60.0)]));
    v.push(p(g, "ap_rising", "Rising on the Planes (Golden Dawn)", "Western esoteric ascent · 25 min", &[200.0], &[0.7], "pink", "binaural", "Traditional",
      "<p><b>Rising on the planes</b> is an exercise of the Hermetic Order of the <b>Golden Dawn</b> (founded in London in 1888) and later Western esoteric schools. After relaxation, you imagine rising straight upward, through layers of light and colour, toward a brilliant light above; it was taught alongside <i>astral travel</i>, which explores imagined scenes. This is an original, non-ceremonial version.</p>",
      vec![st("Settle", 3.0, &[10.0], "Welcome to rising on the planes. Sit upright or lie flat, and close your eyes. Breathe slowly and deeply. Imagine a column of soft light running straight up through your body, from below your feet to far above your head."),
           st("Grounding", 3.0, &[8.0], "Feel the earth beneath you, solid and dark, like rich soil. This is your starting point, and you can always return here."),
           st("First veil", 4.0, &[7.0], "Now begin to rise, slowly, up the column of light. Pass through a veil of deep violet, cool and silvery, like moonlight on water."),
           st("Golden light", 5.0, &[6.0], "Keep rising, through layers of colour, until you come into a warm golden light, like a sun at the centre of everything. Rest here, and feel it in your chest."),
           st("Brilliance", 5.0, &[5.0], "Rise further still, toward a brilliant white light above. Go only as far as feels right. Notice what you sense here, without trying to hold on to it."),
           st("Descend", 3.0, &[8.0], "Now descend, slowly, the way you came: through the gold, through the violet, down into the dark, solid earth. Feel your body again.").ramp(60.0),
           st("Return", 2.0, &[12.0], C_RETURN_SHORT).ramp(30.0)]));
    v.push(p(g, "ap_tattva", "Tattva Vision — Symbol Doorway (Golden Dawn)", "Afterimage gateway · 20 min", &[200.0], &[0.6], "pink", "binaural", "Traditional",
      "<p><b>Tattva vision</b> is a Golden Dawn practice that borrows the five Hindu <i>tattvas</i>, simple coloured symbols of the elements: a red triangle (fire), a yellow square (earth), a blue circle (air), a silver crescent (water) and an indigo egg (spirit). You gaze at the symbol for about 20 seconds, look at a plain surface to see its glowing afterimage in the opposite colour, then close your eyes and imagine it growing into a doorway you pass through. Press <b>🔺 Tattva card</b> to show a symbol full-screen; it is completely still and never flashes.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to tattva vision. Sit upright in a softly lit room. Press Tattva card, choose an element, and follow the gaze timer: look at the dot in the centre of the symbol, steadily, for about twenty seconds."),
           st("Afterimage", 3.0, &[9.0], "When the symbol disappears, keep looking at the plain screen, and watch the afterimage glow in the opposite colour. Then close your eyes, and hold that image in your mind."),
           st("Doorway", 4.0, &[7.0], "With your eyes closed, see the symbol grow larger and larger, until it is a doorway in front of you, the size of a real door. When you are ready, step through it."),
           st("The other side", 8.0, &[6.0], "Notice the landscape on the other side: the light, the colours, the ground, the air. Explore gently, and let the scene unfold by itself. If you meet anyone, you may ask what this element has to teach."),
           st("Return", 3.0, &[10.0], "Turn back to the doorway, and step through it again. See it shrink back to a small symbol, and fade. Feel your body, take a deep breath, and open your eyes.").ramp(60.0)]));
    v.last_mut().unwrap().visual = "tattva".into();
    v.push(p(g, "ap_phowa", "Phowa — Transference of Consciousness (Tibetan)", "Crown-ascent visualisation · 20 min", &[136.1], &[0.7], "pink", "binaural", "Traditional",
      "<p><b>Phowa</b> (\"transference\") is one of the Six Yogas of Naropa in Tibetan Buddhism, practised to prepare for death and as a regular meditation. The practitioner visualises the body's central channel, their awareness as a small sphere of light at the heart, and an awakened presence, often the Buddha Amitabha, above the crown; with each out-breath, often with the syllable \"hik\", the sphere rises toward it. Traditionally it is received from a qualified teacher; this is a gentle introductory version for meditation only.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to phowa. Sit upright, with your spine straight and relaxed, and close your eyes. Breathe naturally. Bring to mind a feeling of compassion for yourself and for all beings."),
           st("Central channel", 4.0, &[7.0], "Imagine a clear channel of light running through the centre of your body, from below your navel up to the crown of your head, open at the top like a skylight."),
           st("Light above", 3.0, &[6.0], "Above your head, imagine a presence of pure light and kindness: a buddha, a saint, or simply radiant light. Feel welcomed by it."),
           st("Rising sphere", 6.0, &[5.5], "At your heart, see a small, bright sphere of light, your own awareness. With each breath out, let it rise gently up the channel toward the light above, and settle back to the heart as you breathe in."),
           st("Rest in light", 3.0, &[6.0], "Now let it rest in the light above, and dissolve, open and spacious. Rest in that openness for a while."),
           st("Return", 2.0, &[10.0], "Let the light come down and dissolve into you, filling your body with warmth. Seal the channel at the crown, feel your body, and open your eyes slowly.").ramp(60.0)]));
    v.push(p(g, "ap_dreamyoga", "Dream Yoga — Recognising the Dream (Tibetan)", "Bedtime practice · 30 min", &[150.0], &[0.5], "brown", "binaural", "Traditional",
      "<p><b>Dream yoga</b>, another of the Six Yogas of Naropa, trains you to recognise dreams as dreams and to stay aware into sleep. In the form taught in the West by Tenzin Wangyal Rinpoche (<i>The Tibetan Yogas of Dream and Sleep</i>, 1998), you lie on your side in the \"lion posture\", review the day as if it had been a dream, and rest your attention on a soft red light at the throat as you fall asleep. Play it in bed at very low volume.</p>",
      vec![st("Settle", 3.0, &[8.0], "Welcome to dream yoga. Lie down in bed, on your side, with one hand under your cheek, in the posture of a sleeping lion. Keep the volume very low, and let your eyes close."),
           st("Day as a dream", 4.0, &[7.0], "Look back over your day, and see it as if it had been a dream: the places, the people, the feelings, all appearing and passing, vivid but insubstantial."),
           st("Throat light", 5.0, &[6.0], "Bring your attention to your throat. Imagine a soft, glowing red light there, warm and clear, like a small flame. Rest your awareness in it, gently."),
           st("Intention", 3.0, &[5.0], "Make a heartfelt intention: tonight, I will recognise my dreams as dreams. Repeat it silently a few times, then let the words go."),
           st("Into sleep", 15.0, &[4.0], "").ramp(300.0)]));
    v.push(with_tex(p(g, "ap_upper", "Shamanic Upper-World Journey", "Drum journey upward · 20 min", &[100.0], &[0.25], "none", "binaural", "Traditional",
      "<p>In <b>core shamanism</b> (Michael Harner, <i>The Way of the Shaman</i>, 1980) journeys go to three worlds. The <b>upper world</b> is reached by climbing: a tree, a ladder, smoke or a mountain top, up through a thin membrane into the sky, often to meet a teacher. The drum beats at about four to four and a half times a second, with the faster call-back at the end; the fMRI study of drum trance by Hove et al. (2016) used this kind of rhythm.</p>",
      vec![st("Intention", 2.0, &[4.2], "Welcome to the upper world journey. Lie down and cover your eyes. Choose a way up that you know, perhaps a tall tree, a ladder, or rising smoke. Set your intention: to meet a teacher who can help you."),
           st("Ascent", 5.0, &[4.5], "Begin to climb. Go higher and higher, past the treetops and the clouds, until you feel a thin, soft membrane above you. Push gently through it."),
           st("Upper world", 10.0, &[4.5], "You are in the upper world. Notice the light, the colours, the ground beneath you. Ask to meet your teacher, and see who comes. Ask your question, and listen."),
           st("Call-back", 3.0, &[7.0], "The drum quickens. Thank your teacher, and come back the way you came, down through the membrane, down the tree, back into your body.").ramp(20.0)]), tx("drum", 1.0, 0.0, 0.0)));
    v.push(with_tex(p(g, "ap_middle", "Shamanic Middle-World Journey", "Drum journey across the land · 20 min", &[100.0], &[0.25], "none", "binaural", "Traditional",
      "<p>The <b>middle world</b> of core shamanism is this world, the land, sea and sky around us, visited in the imagination rather than in the body, which is why many practitioners compare it to out-of-body travel. Journeyers move across familiar or distant landscapes, often to find a place of power or to see how things are elsewhere. The drum plays at about four beats a second.</p>",
      vec![st("Intention", 2.0, &[4.0], "Welcome to the middle world journey. Lie down and cover your eyes. Picture a place in nature that you know well, real and alive. Set your intention to travel out from there, across the land."),
           st("Setting out", 5.0, &[4.3], "Stand in that place, and feel the ground, the air and the sounds around you. Then begin to travel, walking, flying or gliding, across the landscape."),
           st("Journey", 10.0, &[4.5], "Go wherever you are drawn: over hills, rivers or cities. Notice what you see from above and up close. If you find a place that feels powerful, rest there for a while."),
           st("Call-back", 3.0, &[7.0], "The drum calls you home. Travel back the way you came, to your starting place, and return into your body, bringing back what you found.").ramp(20.0)]), tx("drum", 1.0, 0.0, 0.0)));
    {
        const SIGHT: &str = "Now slow down. Sight: gently watch the darkness behind your eyelids, with no effort.";
        const SOUND: &str = "Sound: listen to the quiet sounds around you, and any sounds inside your head.";
        const BODY: &str = "Body: feel your weight, the bed, and any tingling or heaviness.";
        let mut s = vec![st("Settle", 1.0, &[6.0], "Welcome to the senses technique. You have just woken after about five hours of sleep and been up for a few minutes. Lie down comfortably, close your eyes, and keep the volume very low."),
                         st("Quick cycles", 2.0, &[6.0], "Cycle quickly through your senses, two or three seconds each. Sight: the darkness behind your eyelids. Sound: whatever you can hear. Body: the feel of the bed. Then begin again, sight, sound, body, around and around.")];
        for _ in 0..3 {
            s.push(st("Slow: sight", 0.5, &[5.5], SIGHT));
            s.push(st("Slow: sound", 0.5, &[5.5], SOUND));
            s.push(st("Slow: body", 0.5, &[5.5], BODY));
        }
        s.push(st("Fall asleep", 22.5, &[4.0], "Now let go of the technique completely, and drift off to sleep as quickly as you can. If you notice you are dreaming, stay calm, and remember: this is a dream.").ramp(120.0));
        v.push(p(g, "ap_ssild", "Senses-Initiated Lucid Dream (SSILD) — Aspy 2020", "Lucid-dream route · 30 min", &[150.0], &[0.4], "brown", "binaural", "Research",
          "<p>In the <b>International Lucid Dream Induction Study</b> (Aspy, Madden &amp; Delfabbro, 2020, <i>Frontiers in Psychology</i>), the <b>senses-initiated</b> technique (SSILD) worked about as well as MILD, the classic intention technique, and success was more likely when people fell asleep soon afterwards. After about 5 hours of sleep you get up briefly, then lie down and cycle your attention through sight, hearing and body sensations: several quick cycles, then a few slow ones, then let yourself fall asleep. Lucid dreams are the most common doorway to induced out-of-body experiences.</p>", s));
    }

    v.push(p(g, "ap_ground", "After-Practice Grounding", "Integration · 10 min", &[194.18], &[0.8], "surf", "binaural", "Traditional",
      "<p>A short alpha-to-beta return for after any deep or out-of-body practice: feel your weight, the ground, and come back fully alert. Many traditions stress \"grounding\" after inner journeys; physiologically it simply brings you out of drowsy, dreamy states before you stand up or drive.</p>",
      vec![st("Ground", 4.0, &[10.0], C_GROUND), st("Weight", 3.0, &[12.0], "Feel the weight of your body supported by the earth beneath you."),
           st("Awake", 3.0, &[16.0], C_COUNT)]));

    // ================= Spiritual & Mystical Experiences =================
    // Contemplative practices from the world's traditions, and the ones brain-imaging studies
    // have looked at. Spiritual claims are reported as claims; research is cited where it exists.
    let g = "Spiritual & Mystical Experiences";
    v.push(p(g, "my_centering", "Centering Prayer (Keating)", "Newberg 2003 · 22 min", &[200.0], &[0.5], "pink", "binaural", "Research",
      "<p><b>Centering prayer</b> was developed in the 1970s by Trappist monks <b>Thomas Keating</b>, William Meninger and Basil Pennington, drawing on the medieval <i>Cloud of Unknowing</i>. You choose a sacred word as a symbol of your consent to God's presence and, whenever you notice you are engaged with a thought, return to it ever so gently. In a brain-imaging study of Franciscan nuns in this kind of prayer, <b>Newberg et al. (2003, <i>Perceptual and Motor Skills</i>)</b> found changes in frontal and parietal blood flow resembling those seen in Buddhist meditators. Keating recommends 20 minutes, twice a day.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to centering prayer. Sit comfortably, with your back upright and your eyes closed. Choose a sacred word, such as God, Jesus, Abba, peace or love, as a symbol of your intention to rest in God's presence."),
           st("Sacred word", 2.0, &[8.0], "Silently introduce your sacred word, gently, like a feather on cotton. You do not need to repeat it; let it simply be there."),
           st("Resting", 16.0, &[6.5], "When you notice you are engaged with any thought, feeling or image, return ever so gently to the sacred word. Then rest again in silence."),
           st("Closing", 2.0, &[8.0], "Let the sacred word go. Remain in silence with your eyes closed for a couple of minutes, perhaps praying a familiar prayer slowly, before returning to your day.")]));
    v.push(p(g, "my_tibetan", "Tibetan Visualisation Meditation (Newberg 2001)", "Image, absorption, oneness · 25 min", &[200.0], &[0.6], "pink", "binaural", "Research",
      "<p>In a SPECT brain-imaging study of experienced Tibetan Buddhist meditators, <b>Newberg et al. (2001, <i>Psychiatry Research: Neuroimaging</i>)</b> found more activity in the prefrontal cortex and less in the superior parietal lobe, an area that helps map the body in space. The meditators described absorption and a loss of the usual sense of self. Their practice centred on visualising an image in detail and becoming absorbed in it; this session follows that structure.</p>",
      vec![st("Settle", 3.0, &[10.0], "Welcome to Tibetan visualisation meditation. Sit upright and still, and close your eyes. Choose one image to work with: a buddha, a deity, a mandala, or simply a sphere of clear light."),
           st("Build the image", 6.0, &[8.0], "Build your image slowly and in detail: its shape, its colours, the light coming from it, every fine feature. If it fades, rebuild it patiently."),
           st("Hold", 7.0, &[6.5], "Now hold the whole image steady and vivid, in front of you or above your head. Rest your full attention on it."),
           st("Absorption", 7.0, &[6.0], "Let the boundary between you and the image soften. Let it come closer and dissolve into light, and let that light dissolve into you, until there is no observer and nothing observed."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT).ramp(60.0)]));
    v.push(p(g, "my_vocal", "Free Vocalisation — Singing in Tongues (Newberg 2006)", "Glossolalia-style vocal flow · 20 min", &[110.0, 220.0], &[0.6, 0.4], "none", "binaural", "Research",
      "<p>In <b>Newberg et al. (2006, <i>Psychiatry Research: Neuroimaging</i>)</b>, Pentecostal Christians who regularly speak in tongues (<i>glossolalia</i>) were scanned while singing and while speaking in tongues. During glossolalia, activity in the frontal lobes fell, matching their sense of not being in deliberate control. If speaking in tongues is part of your faith, this gives you a space to practise; if not, treat it as free vocal improvisation: sounds with no meaning, letting go of words. A soft drone gives you a note to sing around.</p>",
      vec![st("Settle", 2.0, &[10.0, 10.0], "Welcome to free vocalisation. Sit or stand somewhere you can make sound freely, without being overheard if that helps. Close your eyes, relax your jaw, and take a few deep breaths."),
           st("Hum", 3.0, &[8.0, 8.0], "Begin to hum along with the drone, softly. Let the hum move up and down as it likes."),
           st("Sing", 4.0, &[7.0, 7.0], "Open the hum into vowels: ah, oh, ee. Sing freely around the drone, with no tune you have to follow."),
           st("Let go of words", 8.0, &[6.0, 6.0], "Now let the sounds become syllables, flowing on their own, without meaning or plan. Don't judge them; let your voice lead, and simply follow it."),
           st("Silence", 3.0, &[8.0, 8.0], "Let your voice fall quiet. Sit in the silence, and notice how you feel.")]));
    v.push(p(g, "my_jhana", "Jhana Absorption (Hagerty 2013)", "Ecstatic concentration · 30 min", &[200.0], &[0.6], "pink", "binaural", "Research",
      "<p>The <b>jhānas</b> are states of deep absorption described in early Buddhist texts: rapture (<i>pīti</i>), happiness (<i>sukha</i>), contentment, and finally equanimity. <b>Hagerty et al. (2013, <i>Neural Plasticity</i>)</b> scanned a meditator with over 17,000 hours of practice entering the jhānas at will: activity rose in the brain's reward system, including the nucleus accumbens, and fell in areas for outside sensory processing. This session follows the approach of Western jhāna teachers such as Leigh Brasington: concentration on the breath, then a pleasant feeling, then the four jhānas in turn.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to jhana practice. Sit upright, relaxed and still, and close your eyes. This practice builds concentration, then lets joy arise from it. Begin by feeling the breath at the tip of your nose."),
           st("Access concentration", 10.0, &[8.0], "Keep your attention on the touch of the breath at your nostrils, breath after breath. When the mind wanders, come back. Let concentration deepen until the breath is easy to follow."),
           st("Pleasant feeling", 4.0, &[7.0], "Now shift your attention to a pleasant sensation somewhere in the body: perhaps in your hands, your chest, or a gentle smile. Rest on it, and let it grow by itself."),
           st("First jhana", 4.0, &[6.5], "If an energetic joy, rapture, begins to rise and spread, let it fill you. Don't grab it; just stay with it."),
           st("Second jhana", 3.0, &[6.0], "Let the excitement of rapture fade into the background, and rest in a quieter, warm happiness."),
           st("Third jhana", 3.0, &[5.5], "Let the happiness settle further, into deep, calm contentment, peaceful and still."),
           st("Fourth jhana", 3.0, &[5.0], "Rest now in equanimity: neither pleasant nor unpleasant, completely still, clear and balanced."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT).ramp(30.0)]));
    v.push(p(g, "my_nondual", "Nondual Awareness (Josipovic 2012)", "Open presence · 22 min", &[200.0], &[0.5], "pink", "binaural", "Research",
      "<p>In <b>nondual awareness</b>, taught in Tibetan Dzogchen and Mahamudra and in Advaita, attention rests in awareness itself, without dividing experience into a self inside and a world outside. <b>Josipovic et al. (2012, <i>Frontiers in Human Neuroscience</i>)</b> found that in experienced practitioners this state reduced the usual competition (anti-correlation) between the brain's inward-focused and outward-focused networks. The session gradually opens attention from objects to awareness itself.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to nondual awareness. Sit comfortably, eyes softly open or closed. There is nothing to achieve here. Simply notice that you are already aware: of sounds, of sensations, of this voice."),
           st("Open attention", 5.0, &[9.0], "Let your attention widen to include everything at once: all the sounds, the whole body, the space of the room. Hold nothing in particular."),
           st("Awareness itself", 7.0, &[8.0], "Now notice the awareness in which all of this appears. Rest as that, the knowing itself, rather than any thing that is known."),
           st("Not two", 6.0, &[7.83], "Notice that there is no line where awareness ends and its contents begin. Sounds, thoughts and sensations are like waves in the same water. Rest, without effort."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "my_inquiry", "Self-Inquiry — Who Am I? (Ramana Maharshi)", "Advaita practice · 24 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Self-inquiry</b> (<i>ātma vichāra</i>) is the method taught by the Indian sage <b>Ramana Maharshi</b> (1879–1950), set out in <i>Nān Yār?</i> (\"Who am I?\"). Whenever a thought arises, you ask: to whom does this arise? The answer, \"to me\", turns attention back to the sense of \"I\" itself, until the thinking mind falls quiet in its source. It is one of the core practices of Advaita Vedanta.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to self inquiry. Sit quietly, with your eyes closed. Let the body settle, and the breath find its own rhythm. Notice the simple feeling of being here: I am."),
           st("The I-feeling", 5.0, &[8.0], "Turn your attention toward that feeling of I. Not your name or your story, just the bare sense of being yourself. Where is it? What is it like?"),
           st("To whom?", 8.0, &[7.0], "Whenever a thought arises, ask: to whom does this thought arise? To me. Then ask: who am I? Don't answer with words. Turn back to the feeling of I itself."),
           st("Abiding", 7.0, &[6.0], "Now let even the question rest. Stay as the quiet sense of being, before any thought. If thoughts return, ask again, and come back."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "my_mantra", "Effortless Mantra Meditation (TM-inspired)", "20 min, twice a day", &[200.0], &[0.5], "pink", "binaural", "Research",
      "<p><b>Transcendental Meditation</b> (TM), brought to the West by Maharishi Mahesh Yogi from the late 1950s, uses a mantra thought effortlessly, without concentration, for 20 minutes twice a day. It has a large research literature; an American Heart Association scientific statement (Brook et al., 2013, <i>Hypertension</i>) concluded it may be considered to help lower blood pressure. TM itself is taught one-to-one by certified teachers with a personal mantra; this is <b>not TM</b>, but a general effortless-mantra session using a soft sound of your choice.</p>",
      vec![st("Settle", 1.0, &[10.0], "Welcome to effortless mantra meditation. Sit comfortably, close your eyes, and rest for a moment. Choose a soft, pleasant sound with no meaning for you, such as ah hum."),
           st("Begin", 2.0, &[8.0], "Begin to think the sound quietly, without moving your lips. Don't force it or concentrate on it. Let it come and go, as faint as it likes."),
           st("Effortless", 15.0, &[6.0], "When you notice you have drifted into thoughts, gently come back to the sound. There is no right or wrong; even the drifting is part of it."),
           st("Come out slowly", 2.0, &[8.0], "Let the sound go. Sit with your eyes closed for two more minutes, without the mantra, before slowly opening your eyes.").ramp(60.0)]));
    v.push(p(g, "my_float", "Floatation-REST at Home — Dark, Still, Weightless", "Feinstein 2018-inspired · 45 min", &[100.0], &[0.3], "brown", "binaural", "Research",
      "<p><b>Floatation-REST</b> (Reduced Environmental Stimulation Therapy) means lying in a dark, silent tank of skin-temperature salt water. In <b>Feinstein et al. (2018, <i>PLOS ONE</i>)</b>, a single one-hour float sharply reduced anxiety and muscle tension and improved mood in 50 people with anxiety and stress-related disorders. John C. Lilly, who built the first isolation tank in 1954, described out-of-body and mystical states in it. This is a <b>dry, at-home</b> imitation: lie in a dark, quiet room. <b>Never use it in a bath or any water.</b></p>",
      vec![st("Settle", 3.0, &[10.0], "Welcome to the dry float. Make the room as dark and quiet as you can, and lie on your back on a soft surface, never in water. Let your arms rest loosely, palms up. Keep the volume low."),
           st("Weightless", 10.0, &[6.0], "Imagine you are floating on warm, still water at exactly the temperature of your skin. Feel the water hold you completely. You don't need to hold yourself up at all.").ramp(120.0),
           st("Dark and still", 27.0, &[4.5], "Let even the water disappear. There is only darkness, stillness and breath. Rest here. Nothing to do, nowhere to be.").ramp(300.0),
           st("Surfacing", 5.0, &[10.0], "Slowly, the sense of your body returns: its weight, the surface under you, the air on your skin. Take your time. Breathe deeper, stretch, and open your eyes when you are ready.").ramp(120.0)]));
    v.push(p(g, "my_arc", "Mystical Experience Arc — Sober (Griffiths 2006-inspired)", "Preparation, peak, integration · 40 min", &[136.1, 272.2], &[0.8, 0.3], "surf", "binaural", "Speculative",
      "<p>Johns Hopkins researchers (<b>Griffiths et al., 2006, <i>Psychopharmacology</i></b>) found that a carefully prepared session with psilocybin produced mystical-type experiences that many volunteers ranked among the most meaningful of their lives. These experiences are measured on dimensions first described by the philosopher W. T. Stace: <b>unity</b>, a sense of the <b>sacred</b> and of <b>truth</b>, deep <b>peace and joy</b>, going <b>beyond time and space</b>, and being <b>hard to put into words</b>. This session uses only the non-drug parts of that setting: preparation, intention, eyeshades, music-like tones, the guidance to trust, let go and be open, and time to integrate. It involves <b>no drugs</b>, and whether it can bring about such states has not been tested.</p>",
      vec![st("Preparation", 3.0, &[10.0, 10.0], "Welcome to the mystical experience arc. This session uses no substances, only stillness and sound. Lie down comfortably, and cover your eyes with a soft mask or cloth. Make sure you will not be disturbed for forty minutes."),
           st("Intention", 3.0, &[8.0, 10.0], "Set an intention: a question, or simply an openness to whatever comes. Then let it go. If anything difficult arises in this session, remember three words: trust, let go, be open."),
           st("Letting go", 8.0, &[6.0, 8.0], "Let yourself sink into the sound. Release any effort to control what happens. Like a leaf on a river, let yourself be carried."),
           st("Unity", 6.0, &[4.5, 7.83], "Notice the edges of your body softening, and the sense of being separate fading. Imagine everything as one whole, including you."),
           st("Sacred", 5.0, &[4.5, 7.83], "Let a sense of reverence arise: for life, for this moment, for existence itself. Whatever is sacred to you, let it be near."),
           st("Timeless", 5.0, &[4.0, 7.83], "Let go of time. There is no before and after here, only this vast, open now."),
           st("Return", 5.0, &[8.0, 10.0], "Slowly, let the familiar world return: your body, your breath, the room. Bring back whatever was meaningful.").ramp(120.0),
           st("Integration", 5.0, &[10.0, 12.0], "Lie still for a few moments with your eyes closed. What would you like to remember from this? When you are ready, sit up slowly, and write it down.")]));
    v.push(with_tex(p(g, "my_dhikr", "Sufi Dhikr — Remembrance of God", "Breath and phrase · 20 min", &[200.0], &[0.3], "none", "binaural", "Traditional",
      "<p><b>Dhikr</b> (\"remembrance\") is the central Sufi practice: rhythmic repetition of a sacred phrase, most often <i>Lā ilāha illā-llāh</i> (\"there is no god but God\") or the name <i>Allāh</i>, aloud or silently, in time with the breath and often with a gentle movement of the head toward the heart. Practised in groups it can build to ecstatic states. A soft bell marks a slow rhythm, one phrase every five seconds. Use a sacred phrase from your own tradition if you prefer.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to dhikr, the remembrance of God. Sit cross legged or on a chair, with your back upright and your eyes closed. Place your attention at your heart. When the bell begins, let each ring mark one repetition of your phrase.").tex(0.0),
           st("Aloud", 5.0, &[8.0], "Recite the phrase aloud with each bell: la ilaha illallah, there is no god but God. On the first half, turn your head gently to the right; on the second, bring it down toward your heart."),
           st("Softly", 5.0, &[7.0], "Let the recitation become softer, a murmur, still one phrase with each bell, the movement smaller."),
           st("In the heart", 6.0, &[6.0], "Now let the phrase continue silently, in the heart alone, with each bell and each breath."),
           st("Silence", 2.0, &[7.0], "Let the phrase fall away. Sit in the silence it leaves, with your attention resting at your heart.").tex(0.0)]), tx("breathbell", 0.6, 432.0, 5.0)));
    v.push(with_tex(p(g, "my_jesus", "The Jesus Prayer (Hesychasm)", "Prayer of the heart · 20 min", &[200.0], &[0.3], "none", "binaural", "Traditional",
      "<p>The <b>Jesus Prayer</b>, \"Lord Jesus Christ, Son of God, have mercy on me\", is the heart of <b>hesychasm</b> (\"stillness\"), the contemplative tradition of Eastern Orthodox Christianity, gathered in the <i>Philokalia</i> (1782) and made famous by <i>The Way of a Pilgrim</i>. It is joined to the breath and repeated until it becomes the \"prayer of the heart\". Slow rhythmic prayer like this tends to bring breathing down to about six breaths a minute (Bernardi et al., 2001, <i>BMJ</i>, studied the rosary and yoga mantras). A soft bell marks each ten-second breath.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Jesus Prayer. Sit still, with your head slightly bowed and your eyes closed. Bring your attention down from your head into your heart. The bell will mark each slow breath.").tex(0.0),
           st("Aloud", 3.0, &[8.0], "Pray aloud, slowly, with each breath. Breathing in: Lord Jesus Christ, Son of God. Breathing out: have mercy on me.").breath(),
           st("Whisper", 3.0, &[7.0], "Now whisper the prayer, keeping the same slow rhythm with the breath and the bell.").breath(),
           st("Silent", 8.0, &[6.0], "Now pray silently, in the heart, with each breath in and each breath out. When your mind wanders, gently return to the words.").breath(),
           st("Heart", 4.0, &[6.0], "Let the words become fewer, until only the sense of prayer remains, resting in the heart, in stillness.").tex(0.0)]), tx("breathbell", 0.6, 528.0, 10.0)));
    v.push(with_tex(p(g, "my_nada", "Nada Yoga — Following the Inner Sound", "Outer sound to inner sound · 25 min", &[200.0], &[0.0], "none", "binaural", "Traditional",
      "<p><b>Nāda yoga</b>, the yoga of sound, is described in the <i>Hatha Yoga Pradipika</i> (15th century) and the <i>Nāda Bindu Upanishad</i>. The practitioner listens to outer sounds, then ever subtler ones, until attention turns to the inner sound (<i>anāhata nāda</i>, the \"unstruck sound\"), said to progress from ocean-like roars to bells, flutes and humming bees. Here the singing bowls slowly fade into silence, leaving you listening inward. <b>If you have tinnitus and focusing on it upsets you, skip this session.</b></p>",
      vec![st("Settle", 3.0, &[7.0], "Welcome to nada yoga. Sit comfortably, close your eyes, and let the sound of the bowls fill the space around you. Listen to each tone from the moment it is struck until it fades.").tex(1.0),
           st("Subtle sound", 6.0, &[6.0], "Now listen for the subtlest part of each sound: the shimmer at its edge, the long, quiet tail as it dies away.").tex(0.6),
           st("Fading", 6.0, &[6.0], "As the bowls grow quieter, follow the sound into silence, and keep listening.").tex(0.25),
           st("Inner sound", 8.0, &[5.0], "Now turn your listening inward. In the silence you may notice a faint inner sound: a hum, a hiss, a ringing, or a soft roar. Rest your attention on it gently, and let it draw you inward.").tex(0.0),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT).tex(0.4)]), tx("bowls", 0.8, 220.0, 9.0)));
    v.push(p(g, "my_vipassana", "Vipassana Body Sweep (Goenka-style)", "Insight through sensation · 30 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Vipassana</b> (\"insight\") as taught by <b>S. N. Goenka</b> (1924–2013) in the tradition of Sayagyi U Ba Khin: first <i>anapana</i>, awareness of the breath at the nostrils, then sweeping attention through the body part by part, observing every sensation with equanimity and seeing that each one arises and passes away. Goenka's ten-day silent courses are held worldwide. Studies of Vipassana and related mindfulness practice report lower stress and better emotional regulation, though most are small.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to vipassana. Sit with your back straight and your eyes closed, and resolve to stay still for this sitting. Bring your attention to the area below your nostrils, above your upper lip."),
           st("Anapana", 6.0, &[8.0], "Observe the natural breath as it enters and leaves, just as it is. Feel its touch at the nostrils. Don't control it; simply observe."),
           st("Top to toe", 9.0, &[7.0], "Now move your attention part by part, from the top of your head down to your toes, observing whatever sensation you find: warmth, tingling, pressure, or nothing at all. Observe without craving or aversion."),
           st("Toe to top", 7.0, &[6.5], "Now move back up, from the toes to the top of the head, part by part, with the same calm, balanced attention. Every sensation arises, and passes away."),
           st("Free flow", 4.0, &[6.5], "If the sensations feel subtle and flowing, let your attention sweep through the whole body in one smooth movement, up and down."),
           st("Metta", 2.0, &[10.0], "Finish with loving kindness. Let goodwill flow from you to all beings: may all beings be happy, may all beings be peaceful.")]));
    v.push(p(g, "my_psychomanteum", "Psychomanteum — Mirror Reunion (Moody)", "Grief and presence · 40 min", &[136.1], &[0.4], "pink", "binaural", "Speculative",
      "<p>Psychiatrist <b>Raymond Moody</b> revived the ancient Greek \"oracles of the dead\" as the <b>psychomanteum</b> (<i>Reunions</i>, 1993): a dim room where you gaze into a mirror angled so that it shows no reflection, only depth, after spending time remembering someone who has died. Many people report sensing or seeing the person. In a study with bereaved volunteers, <b>Hastings et al. (2002, <i>Omega</i>)</b> reported reduced grief afterwards. Gazing into a mirror in low light also produces strange visual effects on its own (Caputo, 2010, <i>Perception</i>). This can bring up strong feelings: have someone you trust nearby, and stop whenever you need to.</p>",
      vec![st("Prepare", 3.0, &[10.0], "Welcome to the psychomanteum. Dim the room to a single soft light behind you, and place a mirror so that you see only darkness in it, not your own face. Have photographs or keepsakes of the person you are remembering with you."),
           st("Remembering", 10.0, &[8.0], "Look at the photographs and keepsakes, one by one. Remember the times you shared: their voice, their laugh, the things they said. Let any feelings come, and let them be."),
           st("Gazing", 20.0, &[7.0], "Now set the keepsakes aside, and gaze softly into the depth of the mirror. Don't strain or expect anything. Just keep gazing, relaxed and open, with the person in your heart."),
           st("Farewell", 5.0, &[8.0], "Whatever you experienced, or did not, take a moment to say what you would like to say to them, aloud or silently. Thank them."),
           st("Return", 2.0, &[10.0], "Look away from the mirror, and turn up the lights. Feel your feet on the floor. Take a few deep breaths, and be gentle with yourself for the rest of the day.")]));
    v.push(p(g, "my_orbit", "Microcosmic Orbit (Taoist Inner Alchemy)", "Circulating energy · 22 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>microcosmic orbit</b> (<i>xiao zhoutian</i>, \"small heavenly circuit\") comes from Taoist inner alchemy (<i>neidan</i>). Attention, joined with the breath, circulates the body's energy (<i>qi</i>) up the back along the Governor channel and down the front along the Conception channel, with the tongue touching the roof of the mouth to connect them. It was popularised in the West by Mantak Chia (<i>Awaken Healing Energy Through the Tao</i>, 1983). Practitioners describe warmth and tingling and, at advanced stages, out-of-body and light experiences; these are traditional claims.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the microcosmic orbit. Sit upright on the front of a chair, with your feet flat and your hands resting in your lap. Close your eyes, and touch the tip of your tongue lightly to the roof of your mouth."),
           st("Lower dantian", 3.0, &[8.0], "Bring your attention to a point about three finger widths below your navel, deep inside the belly. Feel warmth gathering there, like a small sun."),
           st("Up the back", 4.0, &[7.0], "Now let the warmth flow down to the base of your body, then slowly up along your spine, between your shoulder blades, up the back of your neck to the crown of your head."),
           st("Down the front", 4.0, &[7.0], "From the crown, let it flow down over your forehead, between your eyebrows, through the tongue, down the throat and the centre of your chest, back to the point below your navel."),
           st("Orbit with breath", 7.0, &[6.0], "Now join it to the breath. Breathing in, the energy rises up the spine. Breathing out, it flows down the front. Circle after circle, smooth and unhurried."),
           st("Gather", 2.0, &[8.0], "Let the circling slow and stop. Gather all the warmth back into the point below your navel, and rest your hands there for a moment.")]));
    v.push(p(g, "my_hitbodedut", "Hitbodedut — Secluded Personal Prayer (Breslov)", "Speak from the heart · 20 min", &[200.0], &[0.4], "surf", "binaural", "Traditional",
      "<p><b>Hitbodedut</b> (\"self-seclusion\") was taught by <b>Rebbe Nachman of Breslov</b> (1772–1810): every day, go somewhere alone, ideally outdoors in nature or at night, and speak to God in your own language, as to a close friend, pouring out everything in your heart. If no words come, he advised simply repeating a single phrase such as \"Master of the World\". It is a central practice of Breslov Hasidism. This session gives the practice a quiet background and a gentle structure.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to hitbodedut. Find a place where you can be alone, outdoors if possible, or simply a quiet room. Sit, or walk slowly. You will speak in your own words, as to a close and loving friend."),
           st("Begin", 3.0, &[8.0], "Begin to speak, aloud or in a whisper, in your own language. Say what is on your mind today, however small. If no words come, repeat: Master of the World."),
           st("Pour out your heart", 12.0, &[7.0], "Keep speaking from the heart: your thanks, your worries, your hopes, your regrets. There is no wrong thing to say. Let it all come out."),
           st("Listening silence", 3.0, &[7.83], "Now fall silent. Listen inwardly. Rest in the closeness that the words have opened.")]));
    v.push(p(g, "my_guide", "Meeting an Inner Guide", "Guided imagery · 20 min", &[200.0], &[0.6], "pink", "binaural", "Traditional",
      "<p>Meeting a wise inner figure is one of the most widespread spiritual visualisations, from shamanic \"power animals\" and spirit guides to Carl Jung's active imagination. In psychology it is used as the <b>inner advisor</b> technique of guided imagery, taught by Martin Rossman and David Bresler's Academy for Guided Imagery (founded 1989), as a way of listening to your own intuition. Whether a guide is part of your own mind or something more is a question each tradition answers differently.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to meeting an inner guide. Sit or lie comfortably, and close your eyes. Bring to mind a question you would like help with, something that matters to you. Hold it gently."),
           st("Path", 4.0, &[8.0], "Imagine yourself on a path in a beautiful, peaceful place. Notice the ground under your feet, the sounds, the smells. Follow the path, slowly."),
           st("Meeting", 6.0, &[6.5], "Ahead of you, someone is waiting: a wise and kind presence. It may be a person, an animal, or a light. Let it appear in its own way, and greet it."),
           st("Asking", 5.0, &[6.0], "Ask your question, and listen. The answer may come as words, an image, a feeling, or a gift. Accept whatever comes, with thanks."),
           st("Return", 3.0, &[10.0], "Thank your guide, and know you can return. Walk back along the path, feel your body again, and open your eyes when you are ready.").ramp(60.0)]));

    // ================= The world's contemplative traditions (v1.7) =================
    traditions(&mut v);
    library::more(&mut v);

    // ================= Light Journeys (flicker) =================
    // Opt-in stroboscopic closed-eye imagery. The light only runs after the user accepts the
    // full warning screen and presses Start; rate is locked to the beat and clamped to 3–12 Hz.
    let g = "Light Journeys (Flicker)";
    const C_OBSERVE: &str = "Simply observe. Let colours, shapes and images arise on their own, without trying to hold on to them.";
    const C_AGAIN: &str = "Let go again, and see what else appears.";
    const C_FOLLOW: &str = "Follow whatever images or ideas arise.";
    let lj = |id: &str, name: &str, short: &str, ev: &str, about: &str, stages: Vec<Stage>| {
        let mut x = p(g, id, name, short, &[200.0], &[0.7], "pink", "binaural", ev, about, stages);
        x.visual = "flicker".into();
        x
    };
    v.push(lj("lj_alpha", "Light Journey — Alpha 10 Hz", "Strongest visuals · 15 min", "Research",
      "<p>The core light journey: light flashes at <b>10 Hz</b> on your closed eyes, locked to a 10 Hz binaural beat. In flicker studies, about 10 Hz (the brain's own alpha rhythm) gives the most vivid geometric patterns and colours (Allefeld et al., 2011; Bartossek et al., 2021). Press <b>⚡ Light journey</b>, read and accept the warnings, close your eyes and face the light.</p>",
      vec![st("Settle", 1.0, &[10.0], C_SETTLE), st("Journey", 12.0, &[10.0], C_OBSERVE), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(lj("lj_sweep", "Sweet-Spot Sweep — 8 → 12 Hz", "Find your peak · 14 min", "Research",
      "<p>Everyone's alpha peak is slightly different, usually 8–12 Hz, and flicker tends to work best near your own peak. This session climbs slowly from 8 to 12 Hz, two minutes per step, then settles on 10 Hz. Notice where the visuals are strongest; then set that rate as your fixed rate in the Light journey panel.</p>",
      vec![st("8 Hz", 2.0, &[8.0], C_SETTLE), st("9 Hz", 2.0, &[9.0], C_OBSERVE).ramp(20.0), st("10 Hz", 2.0, &[10.0], C_AGAIN).ramp(20.0),
           st("11 Hz", 2.0, &[11.0], C_AGAIN).ramp(20.0), st("12 Hz", 2.0, &[12.0], C_AGAIN).ramp(20.0),
           st("Settle at 10 Hz", 2.0, &[10.0], C_FOLLOW).ramp(30.0), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(lj("lj_geometry", "Kaleidoscope — 12 Hz Geometry", "Sharp patterns · 10 min", "Research",
      "<p>Upper-alpha flicker (about 11–12 Hz) tends to produce crisp geometric forms: checkerboards, spirals, honeycombs and radiating lines. These are the \"form constants\" that Heinrich Klüver catalogued in 1926; Ermentrout &amp; Cowan (1979) showed they match simple waves of activity across the visual cortex.</p>",
      vec![st("Settle", 1.0, &[10.0], C_SETTLE), st("Geometry", 7.0, &[12.0], C_OBSERVE).ramp(30.0), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(lj("lj_descent", "Alpha → Theta Imagery Descent", "Patterns into dreamlike scenes · 20 min", "Traditional",
      "<p>Starts at 10 Hz for strong patterns, then slows toward 6 Hz theta, where people more often report soft, dreamlike or scene-like imagery instead of sharp geometry. Light and sound stay locked together. Slower flicker gives less intense patterns but a deeper, drowsier state.</p>",
      vec![st("Settle", 1.0, &[10.0], C_SETTLE), st("Alpha", 5.0, &[10.0], C_OBSERVE), st("Slowing", 5.0, &[8.0], C_AGAIN).ramp(120.0),
           st("Theta", 7.0, &[6.0], C_FOLLOW).ramp(120.0), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(lj("lj_dream", "Dreamachine-inspired Wander — 8–12 Hz", "Drifting rhythm · 20 min", "Traditional",
      "<p>Inspired by Brion Gysin's 1959 <b>Dreamachine</b>, a spinning slotted cylinder around a bulb that flickered at about 8–13 Hz. Here the rate wanders slowly between 8 and 12 Hz, so the patterns keep changing. The Dreamachine was revived as a large public experience and study in the UK in 2022.</p>",
      vec![st("Settle", 1.0, &[10.0], C_SETTLE), st("Wander 1", 3.5, &[9.0], C_OBSERVE).ramp(60.0), st("Wander 2", 3.5, &[11.0], C_AGAIN).ramp(60.0),
           st("Wander 3", 3.5, &[8.5], C_FOLLOW).ramp(60.0), st("Wander 4", 3.5, &[12.0], C_AGAIN).ramp(60.0), st("Wander 5", 3.0, &[10.0], C_OBSERVE).ramp(60.0),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.last_mut().unwrap().visual = "flicker:dreamachine".into();
    let mut gf = lj("lj_ganzflicker", "Ganzflicker — Red & Black", "10 Hz red flicker · 10 min", "Research",
      "<p><b>Ganzflicker</b> alternates full red and black at about 10 Hz. A 2024 study (Shenyan et al., <i>Scientific Reports</i>) found it produced visual hallucinations more often and faster than the steady Ganzfeld, mostly simple shapes and colours. It has also been used to study people with aphantasia (no mind's eye), many of whom still saw patterns (Königsmark et al., 2021).</p>",
      vec![st("Settle", 1.0, &[10.0], C_SETTLE), st("Ganzflicker", 7.0, &[10.0], C_OBSERVE), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]);
    gf.visual = "ganzflicker".into();
    v.push(gf);

    // ---- research and practitioner protocols (v1.6). Each has a matching preset in the light
    // panel, picked automatically from `visual` = "flicker:<protocol>". Stage names steer the light:
    // "(dark)" = light off, "Steady" = constant light, "arrhythmic" = jittered flash timing.
    let ljp = |id: &str, name: &str, short: &str, ev: &str, proto: &str, about: &str, stages: Vec<Stage>| {
        let mut x = p(g, id, name, short, &[200.0], &[0.7], "pink", "binaural", ev, about, stages);
        x.visual = format!("flicker:{}", proto);
        x
    };
    const C_DARK_PAUSE: &str = "Pause. The light is off. Keep your eyes closed, and rate how strong the visuals were, from zero to ten.";
    v.push(ljp("lj_bart10", "Bartossek 2021 — 10 Hz, 20 Minutes", "Study protocol · 23 min", "Research", "bartossek",
      "<p>The 10 Hz condition of <b>Bartossek et al. (2021, <i>PLOS ONE</i>)</b>: twenty minutes of white light flashing ten times a second on closed eyes, lit for half of every cycle, from a Lucia N°03 lamp in a dark room. Of the three conditions (10 Hz, 3 Hz and steady light), 10 Hz tended to give the strongest altered-state and visual experiences, though the differences were modest. Volunteers were screened for photosensitivity with an EEG test first, and one stopped after migraine-like symptoms. The light panel loads the matching preset: white, 50% on-time, eyes closed; the light stays dark for the first minute. The study used earplugs; turn the tone and noise down for a silent version.</p>",
      vec![st("Settle (dark)", 1.0, &[10.0], "Welcome to the Bartossek ten hertz protocol. Lie back or sit facing the light in a dark room, and close your eyes. Press Light journey and start it now: the light stays off during this first minute, then flickers for twenty minutes."),
           st("10 Hz flicker", 20.0, &[10.0], "The flicker begins. Keep your eyes closed, relax, and simply observe whatever colours, patterns or images appear.").ramp(0.0),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(ljp("lj_bart3", "Bartossek 2021 — 3 Hz, 20 Minutes", "Slow-flicker condition · 23 min", "Research", "bartossek",
      "<p>The 3 Hz condition of <b>Bartossek et al. (2021, <i>PLOS ONE</i>)</b>: the same twenty minutes of white light at 50% on-time, but only three flashes a second. Slow flicker gives fewer and simpler geometric patterns than 10 Hz, and in that study it lowered positive mood slightly; it is included so you can compare. The beat plays at 3 Hz (delta).</p>",
      vec![st("Settle (dark)", 1.0, &[3.0], "Welcome to the Bartossek three hertz protocol, the slow flicker condition. Close your eyes in a dark room, facing the light. Start the light journey now: it stays dark for one minute, then flashes three times a second for twenty minutes."),
           st("3 Hz flicker", 20.0, &[3.0], "Slow flicker begins. Keep your eyes closed and observe. Notice how slow flashes look different from fast ones.").ramp(0.0),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    {
        let mut x = ljp("lj_steady", "Bartossek 2021 — Constant-Light Control", "No flicker · 23 min", "Research", "bartossek",
          "<p>The control condition of <b>Bartossek et al. (2021)</b>: the same light on closed eyes for twenty minutes, but <b>steady</b>, never flashing, so you can tell how much of what you see comes from the flicker itself. There are no beats here, only soft pink noise, which makes it close to a Ganzfeld. Steady light still produced some imagery in that study, but less than flicker.</p>",
          vec![st("Settle (dark)", 1.0, &[0.0], "Welcome to the constant light control. Sit or lie facing the light with your eyes closed, in a dark room. Start the light journey now. After one dark minute the light comes on and stays steady for twenty minutes, with no flicker at all."),
               st("Steady light", 20.0, &[0.0], "The steady light is on. Keep your eyes closed, and simply observe what you see, with no flicker at all."),
               st("Return", 2.0, &[0.0], C_RETURN_SHORT)]);
        x.gains = vec![0.0];
        v.push(x);
    }
    v.push(ljp("lj_amaya", "Amaya 2023 — Frequency & Rhythm Series", "2-minute blocks · 19 min", "Research", "amaya",
      "<p>Based on <b>Amaya et al. (2023, <i>PLOS ONE</i>)</b>, who flashed white light on closed eyes for two-minute periods at 3, 8, 10 and 18 Hz, each either <b>rhythmic</b> or <b>arrhythmic</b>, with the light on for 30% of each cycle. Rhythmic 10 Hz gave the strongest simple hallucinations and visual movement; rhythmic beat arrhythmic at every rate. Here: one practice minute, then 10, 8 and 3 Hz, each rhythmic then arrhythmic (flash gaps drawn at random around the average, as in the study), with 30-second dark pauses to rate what you saw. The 18 Hz blocks and the study's paired-flash pattern are left out because they go above this app's 12 Hz safety limit.</p>",
      vec![st("Settle (dark)", 1.0, &[10.0], "Welcome to the Amaya frequency and rhythm series. Close your eyes and face the light, and start the light journey now. You will get two minute blocks at ten, eight and three hertz, each rhythmic then irregular, with short dark pauses between them."),
           st("Practice 10 Hz", 1.0, &[10.0], "A one minute practice block at ten hertz, so you know what to expect. Just observe.").ramp(0.0),
           st("10 Hz rhythmic", 2.0, &[10.0], "Ten hertz, rhythmic. Watch the patterns.").ramp(0.0),
           st("Pause (dark)", 0.5, &[10.0], C_DARK_PAUSE).ramp(0.0),
           st("10 Hz arrhythmic", 2.0, &[10.0], "Ten hertz, irregular. The same average speed, but uneven. Notice the difference.").ramp(0.0),
           st("Pause (dark)", 0.5, &[8.0], C_DARK_PAUSE).ramp(0.0),
           st("8 Hz rhythmic", 2.0, &[8.0], "Eight hertz, rhythmic.").ramp(0.0),
           st("Pause (dark)", 0.5, &[8.0], C_DARK_PAUSE).ramp(0.0),
           st("8 Hz arrhythmic", 2.0, &[8.0], "Eight hertz, irregular.").ramp(0.0),
           st("Pause (dark)", 0.5, &[3.0], C_DARK_PAUSE).ramp(0.0),
           st("3 Hz rhythmic", 2.0, &[3.0], "Three hertz, rhythmic. Slow, distinct flashes.").ramp(0.0),
           st("Pause (dark)", 0.5, &[3.0], C_DARK_PAUSE).ramp(0.0),
           st("3 Hz arrhythmic", 2.0, &[3.0], "Three hertz, irregular.").ramp(0.0),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(ljp("lj_ganzopen", "Ganzflicker — Original Eyes-Open Protocol", "Red/black, eyes open · 13 min", "Research", "ganzflicker",
      "<p>The original <b>Ganzflicker</b>, created by Reshanne Reeder, is watched with <b>eyes open</b>: the whole screen alternates red and black about ten times a second for ten minutes, in a dim room. In a large online study, most people with normal mental imagery saw illusory patterns, and many saw complex scenes, objects or faces, while people with aphantasia mostly saw simple patterns (Königsmark, Bergmann &amp; Reeder, 2021, <i>Cortex</i>); it produced hallucinations more often and faster than a steady Ganzfeld (Shenyan et al., 2024, <i>Scientific Reports</i>). Eyes-open flicker is stronger than flicker through closed eyelids, so the light panel asks you to confirm once more before it starts, and it always uses the screen, never the phone flashlight.</p>",
      vec![st("Settle (dark)", 1.0, &[10.0], "Welcome to the original Ganzflicker. Unlike the other light journeys, this one is done with your eyes open. Dim the room, sit at arm's length from the screen, and start the light journey. It stays dark for one minute."),
           st("Ganzflicker (eyes open)", 10.0, &[10.0], "The flicker begins. With your eyes open, rest your gaze softly on the centre of the screen. Let your eyes relax, and watch what appears.").ramp(0.0),
           st("Return", 2.0, &[10.0], "The flicker has stopped. Close your eyes, rest, and take a moment to recall what you saw.")]));
    {
        let names = ["Three hertz: slow, distinct flashes.", "Four hertz.", "Five hertz.", "Six hertz, theta.", "Seven hertz.", "Eight hertz, the start of alpha.",
                     "Nine hertz.", "Ten hertz, the alpha peak. For most people, the strongest patterns.", "Eleven hertz.", "Twelve hertz: the fastest rate here."];
        let mut s = vec![st("Settle (dark)", 1.0, &[3.0], "Welcome to the frequency scan. Close your eyes, face the light, and start the light journey now. The flicker begins at three hertz and steps up by one hertz each minute, to twelve. Notice how the patterns change with the speed.")];
        for (i, cue) in names.iter().enumerate() {
            let hz = 3.0 + i as f64;
            s.push(st(&format!("{} Hz", hz), 1.0, &[hz], cue).ramp(0.0));
        }
        s.push(st("Return", 2.0, &[10.0], C_RETURN_SHORT));
        v.push(ljp("lj_scan", "Frequency Scan 3 → 12 Hz (Allefeld 2011-style)", "1 minute per rate · 13 min", "Research", "scan",
          "<p>Flicker imagery changes with the rate. <b>Allefeld et al. (2011, <i>Consciousness and Cognition</i>)</b> showed volunteers flicker at many different frequencies through a Ganzfeld and found that colours and geometric forms depended on frequency, peaking around 10 Hz; earlier work by Grey Walter (1949) and John Smythies (1959) catalogued the patterns at each rate. This scan steps from 3 to 12 Hz, one minute per rate, so you can map your own response.</p>", s));
    }
    v.push(ljp("lj_lucia", "Hypnagogic Light — Flicker with Steady Glow (Lucia-style)", "Flicker and glow · 21 min", "Traditional", "lucia",
      "<p>The <b>Lucia N°03</b> \"hypnagogic light\" lamp, developed in Austria by Engelbert Winkler and Dirk Proeckl, mixes flickering light with phases of steady light to guide people toward the dreamy hypnagogic state; it was the lamp used by Bartossek et al. (2021). This original programme alternates warm-white flicker at 9, 8 and 7 Hz with two-minute periods of steady glow. It is not Lucia's own programme.</p>",
      vec![st("Settle (dark)", 1.0, &[9.0], "Welcome to the hypnagogic light journey. Lie back with your eyes closed and your face toward the light, in a dark room. Start the light journey now. Flicker alternates with a soft steady glow, guiding you toward the dreamy edge of sleep."),
           st("Flicker 9 Hz", 4.0, &[9.0], "The flicker begins at nine hertz. Let the colours wash over you.").ramp(0.0),
           st("Steady glow 1", 2.0, &[9.0], "A soft, steady glow now. Let the images linger and drift."),
           st("Flicker 8 Hz", 4.0, &[8.0], "Flicker again, a little slower, at eight hertz.").ramp(10.0),
           st("Steady glow 2", 2.0, &[8.0], "Steady glow again. Notice what remains when the flashing stops."),
           st("Flicker 7 Hz", 4.0, &[7.0], "Slower still, seven hertz. Let yourself drift toward dreaming.").ramp(10.0),
           st("Steady glow 3", 2.0, &[7.0], "Rest in the glow, half awake, half dreaming."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    {
        let mut x = ljp("lj_ave", "Audio-Visual Entrainment — Alpha Relaxation (AVE)", "Light and sound pulse together · 19 min", "Research", "ave",
          "<p><b>Audio-visual entrainment</b> (AVE) devices, sold since the 1980s, pair light flashes in goggles with sound pulses at the same rate. Reviews report benefits for relaxation, mood and some clinical problems such as headaches (Huang &amp; Charyton, 2008, <i>Alternative Therapies in Health and Medicine</i>), though most studies are small and many were run by device makers. Here the session plays <b>isochronic</b> sound pulses at the same rate as the light, gliding from 10 Hz down to 7.5 Hz.</p>",
          vec![st("Settle (dark)", 1.0, &[10.0], "Welcome to audio visual entrainment. Close your eyes and face the light, and start the light journey. The sound pulses and the light flashes run at the same rate, starting at ten hertz and slowing gently toward deep relaxation."),
               st("Alpha 10 Hz", 5.0, &[10.0], "Sound and light now pulse together at ten hertz. Let them carry you.").ramp(0.0),
               st("Glide down", 5.0, &[8.0], "The rhythm slows gently toward eight hertz. Let your body grow heavier as it slows.").ramp(240.0),
               st("Deep alpha", 6.0, &[7.5], "Seven and a half hertz, at the edge of theta. Deeply relaxed, calm and still.").ramp(120.0),
               st("Return", 2.0, &[10.0], C_RETURN_SHORT)]);
        x.mode = "isochronic".into();
        v.push(x);
    }
    v.push(ljp("lj_phone", "Phone-Light Journey (Lumenate-style)", "Flashlight or screen · 19 min", "Traditional", "phone",
      "<p>Inspired by phone apps such as <b>Lumenate</b>, which flash the phone's flashlight on closed eyes in time with music; Lumenate's own lamp was used in the Amaya et al. (2023) study. On Android this preset uses the <b>phone flashlight</b>: hold the phone 10 to 20 cm from your closed eyes, light facing you. On a computer it uses the screen. This is an original programme, not Lumenate's: a gentle arc from 11 Hz down to 6 Hz and back up to 10 Hz.</p>",
      vec![st("Settle (dark)", 1.0, &[11.0], "Welcome to the phone light journey. On a phone, choose the flashlight in the light panel, and hold it ten to twenty centimetres from your closed eyes. On a computer, face the screen. Start the light journey when you are ready."),
           st("Open", 3.0, &[11.0], "The light begins. Breathe slowly, and let the colours open up.").ramp(0.0),
           st("Deepen", 4.0, &[9.0], "The rhythm slows. Let the patterns grow and change.").ramp(60.0),
           st("Drift", 4.0, &[7.0], "Slower still. Let shapes turn into scenes, and follow them.").ramp(90.0),
           st("Theta float", 3.0, &[6.0], "Six hertz. Float in whatever appears.").ramp(60.0),
           st("Rise", 2.0, &[10.0], "The rhythm lifts again, bright and clear.").ramp(60.0),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));

    // ================= Infrasound (opt-in) =================
    // The infrasound layer itself only plays after the user accepts its warning and turns it on.
    let g = "Infrasound (opt-in)";
    let inf = |id: &str, name: &str, short: &str, hz: f64, beat: f64, car: f64, ev: &str, about: &str| {
        let mut x = p(g, id, name, short, &[car], &[0.7], "brown", "binaural", ev, about,
          vec![st("Settle", 2.0, &[10.0], C_SETTLE), st("Infrasound", 11.0, &[beat], C_REST).ramp(60.0), st("Return", 2.0, &[10.0], C_RETURN_SHORT)]);
        x.visual = format!("infra:{}", hz);
        x
    };
    v.push(inf("inf_ghost", "18.98 Hz — Tandy's \"Ghost Frequency\"", "Infrasound + alpha · 15 min", 18.98, 10.0, 200.0, "Speculative",
      "<p>In 1998 engineer <b>Vic Tandy</b> traced a \"haunted\" feeling in his lab (unease, cold sweat, a grey shape at the edge of vision) to a fan making an <b>18.98 Hz</b> standing wave. He proposed it vibrated the eyeball (Tandy &amp; Lawrence, 1998, <i>Journal of the Society for Psychical Research</i>). Here a quiet 18.98 Hz layer plays under a 10 Hz alpha beat. Press <b>🌫 Infrasound</b> and accept the warning to switch the layer on.</p>"));
    v.push(inf("inf_concert", "17 Hz — Soundless Music", "2003 concert experiment · 15 min", 17.0, 8.0, 200.0, "Speculative",
      "<p>At a 2003 London concert, researchers (Richard Wiseman, Sarah Angliss and others) secretly played <b>17 Hz</b> infrasound under some pieces. About 22% more listeners reported unusual feelings (unease, sorrow, chills, nervousness) during those pieces. It was a public experiment, not a peer-reviewed study.</p>"));
    v.push(inf("inf_7", "7 Hz — Deep Body Rumble", "Felt more than heard · 15 min", 7.0, 7.0, 150.0, "Speculative",
      "<p>7 Hz is often called the \"body resonance\" frequency, linked to stories about engineer Vladimir Gavreau's 1960s infrasound experiments. The dramatic claims (organ damage, \"brown note\") are myths at any level a speaker can produce. Here it's a gentle rumble you may feel more than hear, under a matching 7 Hz theta beat.</p>"));
    v.push(inf("inf_schumann", "7.83 Hz — Schumann as Infrasound", "Earth resonance · 15 min", 7.83, 7.83, 194.18, "Speculative",
      "<p>The Schumann resonance (7.83 Hz) is an electromagnetic resonance of the Earth's atmosphere, not a sound. Here it is played literally as a sound wave, an infrasound layer under a matching binaural beat. Purely experiential.</p>"));

    // ================= Breathwork (opt-in) =================
    // The breathing guide only starts after the user accepts the breathwork warning.
    let g = "Breathwork (opt-in)";
    let mut br = p(g, "br_power", "Power Breathing Rounds (Wim Hof-style)", "3 rounds · about 15 min", &[200.0], &[0.5], "surf", "binaural", "Research",
      "<p>Rounds of 30 deep, fast breaths, then holding with empty lungs until you need to breathe, then one recovery breath held for 15 seconds. This is the method popularised by <b>Wim Hof</b>. In a well-known study (Kox et al., 2014, <i>PNAS</i>), trained people using it raised adrenaline and dampened their immune reaction to an injected bacterial toxin. Tingling, light-headedness and vivid sensations are common. Press <b>🌬 Breathwork</b> and accept the warning to start the guide. <b>Never in or near water, never driving or standing.</b></p>",
      vec![st("Settle", 1.0, &[10.0], C_SETTLE), st("Rounds", 13.0, &[7.0], "").ramp(60.0), st("Rest", 2.0, &[8.0], C_REST)]);
    br.visual = "breath:power".into();
    v.push(br);
    let mut br = p(g, "br_connected", "Conscious Connected Breathing (Holotropic-inspired)", "Continuous breathing · 20 min", &[200.0], &[0.5], "surf", "binaural", "Speculative",
      "<p>Continuous, slightly fast and deep breathing with no pauses between in and out, the core of <b>Holotropic Breathwork</b> (Stanislav &amp; Christina Grof, 1970s) and rebirthing. Practitioners report intense emotions, images and body sensations. This is an original, shorter and gentler guide, not a Grof session; those are done in groups with trained facilitators.</p>",
      vec![st("Settle", 2.0, &[10.0], C_SETTLE), st("Breathing", 15.0, &[6.0], "").ramp(120.0), st("Integration", 3.0, &[8.0], C_REST)]);
    br.visual = "breath:connected".into();
    v.push(br);

    // ---- research and practitioner protocols (v1.6). `visual` = "breath:<protocol>" selects the
    // matching preset in the breathing guide; the guide itself drives the breathing, so the middle
    // stage is silent. Higher-risk protocols stay behind the breathwork warning.
    const C_BR_REST: &str = "Rest now. Let your breathing return to its own natural rhythm, and notice how you feel. Stay sitting or lying down for a minute before you get up.";
    let brp = |g: &str, id: &str, name: &str, short: &str, ev: &str, proto: &str, about: &str, first: &str, mid: f64, rest: f64| {
        let mut x = p(g, id, name, short, &[200.0], &[0.45], "surf", "binaural", ev, about,
          vec![st("Settle", 1.0, &[10.0], first), st("Breathing", mid, &[7.0], "").ramp(60.0), st("Rest", rest, &[8.0], C_BR_REST)]);
        x.visual = format!("breath:{}", proto);
        x
    };
    v.push(brp(g, "br_kox", "Kox 2014 Study Protocol (Wim Hof Method)", "Hold-until-urge rounds · 20 min", "Research", "kox",
      "<p>The breathing exercise from <b>Kox et al. (2014, <i>PNAS</i>)</b>: about 30 deep breaths, then breathe out and hold with empty lungs for as long as feels right (trained volunteers typically held about 2–3 minutes), then breathe in deeply and hold for 10 seconds; repeat for several rounds. After ten days of training, volunteers had much higher adrenaline, an inflammatory response about half as large, and fewer flu-like symptoms after an injected bacterial toxin. Their blood oxygen also dipped to about 50% for a few seconds during holds, which is why this stays opt-in. The guide ends any hold at 2½ minutes. <b>Never in or near water, never standing or driving.</b></p>",
      "Welcome to the Kox study protocol. Sit or lie down somewhere safe, never in or near water, and never while standing or driving. Press Breathwork and start the guide when you are ready. Rest your hands, and let your breathing settle.", 17.0, 2.0));
    v.push(brp(g, "br_cyclic", "Cyclic Hyperventilation (Balban 2023)", "Stanford 5-minute protocol · 8 min", "Research", "cyclic",
      "<p>One of three breathing practices in a month-long randomised study at Stanford (<b>Balban et al., 2023, <i>Cell Reports Medicine</i></b>): each round is 30 breaths (a deep breath in through the nose, a passive breath out through the mouth), then breathe out fully and hold with empty lungs for 15 seconds; three rounds, five minutes a day. Breathwork as a whole lifted mood more than mindfulness meditation did; cyclic sighing (in Breathing Techniques) stood out most, and also slowed breathing at rest.</p>",
      "Welcome to cyclic hyperventilation, the Stanford five minute protocol. Sit or lie somewhere safe, away from water. Press Breathwork to start the guide: thirty deep breaths, then a fifteen second hold with empty lungs, three times.", 5.5, 1.5));
    v.push(brp(g, "br_tummo", "Tummo Inner-Fire Vase Breathing (forceful)", "Kozhevnikov 2013 · 13 min", "Research", "tummo",
      "<p><b>G-tummo</b> (\"inner fire\") is a Tibetan Buddhist practice. In <b>Kozhevnikov et al. (2013, <i>PLOS ONE</i>)</b>, experienced practitioners using <b>forceful vase breathing</b>, a breath held after breathing in while contracting the belly and pelvic floor, together with visualising a flame rising from below the navel, raised their body temperature as high as 38.3 °C. Western volunteers using the breathing alone warmed slightly; their holds lasted about 20 to 40 seconds. The guide holds for as long as is comfortable, up to 45 seconds. Strong breath holds with straining raise blood pressure: <b>not for people with heart conditions, high blood pressure, glaucoma or hernia, or in pregnancy.</b></p>",
      "Welcome to inner fire breathing. Sit upright, cross legged or on a chair, somewhere safe and away from water. Press Breathwork to start the guide. Imagine a small flame, glowing just below your navel.", 10.0, 2.0));
    v.push(brp(g, "br_kapal", "Kapalabhati — Skull-Shining Breath (Telles 2011)", "1 breath a second · 3 × 5 min", "Research", "kapal",
      "<p><b>Kapalabhati</b> is a yogic cleansing breath: short, forceful breaths out through the nose, pulling the belly in, with each breath in happening by itself. In <b>Telles et al. (2011, <i>BioPsychoSocial Medicine</i>)</b>, 38 men practised it at about one breath a second for three 5-minute rounds with a minute's rest between; heart-rate variability showed less vagal (calming) activity during and after it, the opposite of slow breathing. It is energising rather than calming. <b>Beginners should start with the 1-minute-rounds preset.</b> Not in pregnancy, or with hernia, high blood pressure, heart disease or epilepsy.</p>",
      "Welcome to kapalabhati, the skull shining breath. Sit upright with your hands on your knees. Press Breathwork to start the guide. Each breath out is a short, sharp push from the belly; each breath in happens by itself.", 17.0, 2.0));
    v.push(brp(g, "br_bhastrika", "Bhastrika — Bellows Breath", "3 rounds of 20 · 8 min", "Traditional", "bhastrika",
      "<p><b>Bhastrika</b> (\"bellows\") is a classical pranayama from the <i>Hatha Yoga Pradipika</i>: forceful breaths both in and out through the nose, about one a second, driven by the belly, in rounds followed by calm, natural breathing. It is strongly energising, and it is a form of hyperventilation. For a gentle, research-tested version, see Slow-Pace Bhastrika in Breathing Techniques.</p>",
      "Welcome to bhastrika, the bellows breath. Sit upright, with your spine tall and your shoulders relaxed. Press Breathwork to start the guide. Breathe in and out forcefully through the nose, like a bellows, driven by the belly.", 5.0, 2.0));
    v.push(brp(g, "br_fire", "Breath of Fire (Kundalini Yoga)", "Rapid, even breathing · 8 min", "Traditional", "fire",
      "<p><b>Breath of fire</b> is a signature breath of Kundalini yoga as taught by Yogi Bhajan from 1969: rapid, rhythmic, equal breaths in and out through the nose, two to three a second, pumped by the navel, usually for one to three minutes. It is energising and can cause light-headedness. Keep it light and even, never straining, and stop if you feel dizzy.</p>",
      "Welcome to breath of fire. Sit upright, with your spine straight. Press Breathwork to start the guide. The breath is quick, light and even, in and out through the nose, pumped from the navel.", 5.0, 2.0));
    v.push(brp(g, "br_sky", "Sudarshan Kriya-inspired Cyclic Breathing", "Slow, medium and fast cycles · 20 min", "Research", "sky",
      "<p><b>Sudarshan Kriya Yoga (SKY)</b>, taught by the Art of Living Foundation since the 1980s, combines <i>ujjayi</i> (slow ocean breath), <i>bhastrika</i> (bellows breath), chanting Om, and the kriya itself: cycles of slow, medium and fast rhythmic breathing. Reviews of trials report benefits for stress, anxiety and depression (Brown &amp; Gerbarg, 2005; Zope &amp; Zope, 2013, <i>International Journal of Yoga</i>). The real SKY is taught in person by trained teachers; this is an original approximation of its structure, with the kriya cycles at about 20, 45 and 70 breaths a minute, followed by rest.</p>",
      "Welcome to the cyclic breathing journey, inspired by Sudarshan Kriya. Sit upright, cross legged or on a chair, somewhere safe. Press Breathwork to start the guide; it leads you through each part, then asks you to lie down and rest.", 15.0, 4.0));
    v.push(brp(g, "br_rebirth", "Rebirthing — Relaxed Connected Breathing (Orr)", "Gentle circular breath · 20 min", "Speculative", "rebirth",
      "<p><b>Rebirthing</b> was developed by <b>Leonard Orr</b> in the 1970s: continuous, connected breathing with no pause between in and out, the breath in drawn actively and the breath out simply released, usually for an hour with a trained \"rebirther\". Practitioners report releasing old emotions and memories, which they connect with birth; those ideas are untested. This guide is gentler than Holotropic-style breathing: fuller than normal, but relaxed.</p>",
      "Welcome to rebirthing breath. Lie down on a soft surface, somewhere safe, and loosen tight clothing. Press Breathwork to start the guide. Draw each breath in, and simply let it go, with no pause in between.", 16.0, 3.0));

    // ================= Breathing Techniques (no warning needed) =================
    // Slow and gentle methods; no hyperventilation and no long holds.
    let g = "Breathing Techniques";
    v.push(brp(g, "br_sigh", "Cyclic Sighing (Balban 2023)", "5 minutes a day · 7 min", "Research", "sigh",
      "<p>The standout of the Stanford breathing study (<b>Balban et al., 2023, <i>Cell Reports Medicine</i></b>): a slow breath in through the nose, a second short sip of air to fill the lungs completely, then a long, slow breath out through the mouth, repeated for five minutes. Done daily for a month, it lifted mood more than mindfulness meditation and slowed breathing even at rest. The double breath in is the same pattern as the body's natural \"physiological sigh\".</p>",
      "Welcome to cyclic sighing. Sit comfortably. Press Breathwork to start the guide. Breathe in through your nose, take one more short sip of air at the top, then let it all out slowly through your mouth.", 5.0, 1.0));
    v.push(brp(g, "br_box", "Box Breathing + CO₂ Tolerance Test (Balban 2023)", "Test, then 5 min · 8 min", "Research", "box",
      "<p><b>Box breathing</b>, equal counts in, hold, out, hold, was one of the three Stanford breathing practices (<b>Balban et al., 2023</b>). The count was set by a <b>CO₂ tolerance test</b>: four normal breaths, then the biggest breath in you can take, then breathe out as slowly as possible. The seconds it takes to empty your lungs set the box: up to 20 seconds, 3–4 seconds per side; 25–45 seconds, 5–6; 50 seconds or more, 8–10. The guide runs the test and sets your count, or you can choose one. It was popularised by former Navy SEAL Mark Divine.</p>",
      "Welcome to box breathing. Sit upright and relaxed. Press Breathwork to start the guide. It begins with a short test of how slowly you can breathe out, then sets the size of your box.", 6.0, 1.0));
    v.push(brp(g, "br_478", "4-7-8 Relaxing Breath (Weil)", "In 4, hold 7, out 8 · 6 min", "Traditional", "478",
      "<p>Dr <b>Andrew Weil</b>'s <b>4-7-8 breath</b>, adapted from pranayama: breathe in quietly through the nose for 4 counts, hold for 7, then breathe out through the mouth with a soft whoosh for 8. Weil recommends four cycles at a time, twice a day, building up to eight after a month. Slow breathing with long breaths out lowers heart rate in many studies; 4-7-8 itself has little direct research.</p>",
      "Welcome to the four seven eight breath. Sit with your back straight, and rest the tip of your tongue just behind your upper front teeth. Press Breathwork to start the guide: in for four, hold for seven, out for eight.", 3.0, 2.0));
    v.push(brp(g, "br_coherent", "Coherent Breathing — 5 Breaths a Minute", "6 s in, 6 s out · 12 min", "Research", "coherent",
      "<p><b>Coherent breathing</b> (Stephen Elliott, 2005) is resonance breathing at about five breaths a minute: 6 seconds in, 6 seconds out, smooth and silent. Breathing near your resonance frequency (about 4.5 to 7 breaths a minute) maximises heart-rate variability and is among the best-supported self-calming methods (Lehrer &amp; Gevirtz, 2014; Zaccaro et al., 2018). Psychiatrists Richard Brown and Patricia Gerbarg teach it in their Breath-Body-Mind programme.</p>",
      "Welcome to coherent breathing. Sit or lie comfortably, and let your belly soften. Press Breathwork to start the guide: six seconds in, six seconds out, smooth and silent, five breaths a minute.", 10.0, 1.0));
    v.push(brp(g, "br_bhramari", "Bhramari — Humming Bee Breath", "Hummed breaths out · 8 min", "Research", "bhramari",
      "<p><b>Bhramari</b> pranayama: breathe in through the nose, then breathe out slowly while humming like a bee, sometimes with the ears gently closed by the thumbs. Humming raises nitric oxide in the nose about fifteen-fold compared with a quiet breath out (<b>Weitzberg &amp; Lundberg, 2002, <i>American Journal of Respiratory and Critical Care Medicine</i></b>), and the long, vibrating breath out is deeply calming.</p>",
      "Welcome to the humming bee breath. Sit comfortably with your eyes closed. Press Breathwork to start the guide. On each long breath out, hum softly, and feel the vibration in your face and head.", 6.0, 1.0));
    v.push(brp(g, "br_nadi", "Nadi Shodhana — Alternate-Nostril Breathing", "Left, right, balance · 12 min", "Traditional", "nadi",
      "<p><b>Nadi shodhana</b> (\"channel cleaning\") is a classical pranayama: close the right nostril with the thumb and breathe in through the left; close the left with the ring finger and breathe out through the right; then in through the right and out through the left. Yoga texts say it balances the body's energy channels. Small studies report slower heart rate and lower blood pressure, and that breathing through one nostril can shift the balance of the nervous system (Telles et al., 1994, <i>Indian Journal of Physiology and Pharmacology</i>).</p>",
      "Welcome to alternate nostril breathing. Sit upright. Rest your left hand in your lap, and bring your right hand to your face: the thumb will close the right nostril, the ring finger the left. Press Breathwork to start the guide.", 10.0, 1.0));
    v.push(brp(g, "br_vase", "Gentle Vase Breathing (Tummo, gentle form)", "No strain · 12 min", "Traditional", "vase",
      "<p>The gentle form of tummo breathing (<b>Kozhevnikov et al., 2013</b>): soft, unstrained breaths with a short, light hold, while imagining your whole body filled with warmth and bliss. In that study the gentle form did not raise body temperature by itself; practitioners use it to maintain the heat after the forceful form. Here it is a calm, safe introduction to the practice.</p>",
      "Welcome to gentle vase breathing. Sit upright and relaxed. Press Breathwork to start the guide. Breathe softly into the lower belly, and imagine warmth spreading through your whole body.", 10.0, 1.0));
    v.push(brp(g, "br_ratio", "Classical Ratio Breathing 1:4:2 (Kumbhaka)", "In 4, hold 16, out 8 · 10 min", "Traditional", "ratio",
      "<p>Hatha yoga texts such as the <i>Hatha Yoga Pradipika</i> teach breath retention (<i>kumbhaka</i>) in the classical ratio 1:4:2: breathe in for one count, hold for four, breathe out for two. Here that is 4 seconds in, 16 held, 8 out. B. K. S. Iyengar's <i>Light on Pranayama</i> (1981) stresses building up gradually and never straining; if 16 seconds is a struggle, choose the shorter preset.</p>",
      "Welcome to classical ratio breathing. Sit upright, with your hands resting on your knees. Press Breathwork to start the guide. The holds should feel comfortable; if they don't, choose the shorter preset.", 8.0, 1.0));
    v.push(brp(g, "br_slowbh", "Slow-Pace Bhastrika — 6 a Minute (Pramanik 2009)", "Deep, slow bellows · 8 min", "Research", "slowbh",
      "<p>In <b>Pramanik et al. (2009, <i>Journal of Alternative and Complementary Medicine</i>)</b>, five minutes of slow-pace bhastrika, deep breaths in for 4 seconds and out for 6 seconds through both nostrils, lowered blood pressure and heart rate in healthy volunteers, probably through the calming (parasympathetic) nervous system. It keeps the full, deep breaths of bhastrika at a slow, safe pace.</p>",
      "Welcome to slow pace bhastrika. Sit upright with your shoulders relaxed. Press Breathwork to start the guide: a deep, full breath in for four seconds, and a deep breath out for six, through both nostrils.", 5.0, 2.0));

    // ================= Brainwave frequency library =================
    let g = "Brainwave Library";
    let lib: &[(f64, &str, &str)] = &[
        (0.5, "Epsilon 0.5 Hz", "Below delta. Associated with very deep meditation in some reports. Speculative."),
        (1.0, "Delta 1 Hz", "Deep, dreamless sleep."),
        (1.5, "Delta 1.5 Hz", "Restorative sleep; the Monroe \"body\" layer."),
        (2.0, "Delta 2 Hz", "Deep sleep, detachment."),
        (2.5, "Delta 2.5 Hz", "Popularly claimed for pain relief (unproven)."),
        (3.0, "Delta 3 Hz", "N3 deep sleep (Jirakittayakorn &amp; Wongsawat, 2018)."),
        (3.5, "Delta 3.5 Hz", "Popularly linked to receptivity, \"unity\"."),
        (4.0, "Theta 4 Hz", "The hypnagogic threshold between waking and sleep."),
        (4.5, "Theta 4.5 Hz", "Shamanic drumming rate."),
        (5.0, "Theta 5 Hz", "Popularly linked to intuition and problem solving."),
        (5.5, "Theta 5.5 Hz", "Popularly linked to \"inner guidance\"."),
        (6.0, "Theta 6 Hz", "Meditation (Jirakittayakorn &amp; Wongsawat, 2017)."),
        (6.3, "Theta 6.3 Hz", "Popularly claimed for astral projection (speculative)."),
        (7.0, "Theta 7 Hz", "Mental imagery, visualisation."),
        (7.5, "Theta 7.5 Hz", "Alpha-theta border; creativity."),
        (7.83, "Schumann 7.83 Hz", "Earth's electromagnetic resonance (association only)."),
        (8.0, "Alpha 8 Hz", "Relaxed, receptive learning."),
        (8.5, "Alpha 8.5 Hz", "Relaxed awareness."),
        (9.0, "Alpha 9 Hz", "Body awareness, calm."),
        (10.0, "Alpha 10 Hz", "Relaxation and mood: the classic alpha peak."),
        (10.5, "Alpha 10.5 Hz", "Popularly linked to mind–body healing."),
        (11.0, "Alpha 11 Hz", "Relaxed alertness."),
        (12.0, "Alpha 12 Hz", "Centering, mental stability."),
        (13.0, "SMR 13 Hz", "Calm focus: the sensorimotor rhythm band."),
        (14.0, "SMR 14 Hz", "Focused attention."),
        (15.0, "Low beta 15 Hz", "Active thinking."),
        (16.0, "Beta 16 Hz", "Vigilance (Lane et al., 1998)."),
        (18.0, "Beta 18 Hz", "Alertness, engagement."),
        (20.0, "Beta 20 Hz", "Energy, arousal."),
        (24.0, "Beta 24 Hz", "Used in Lane et al. (1998)."),
        (25.0, "Beta 25 Hz", "High alertness."),
        (30.0, "High beta 30 Hz", "Strong stimulation. Use sparingly; can feel agitating."),
        (33.0, "Gamma 33 Hz", "Low gamma."),
        (35.0, "Gamma 35 Hz", "Low gamma."),
        (38.0, "Gamma 38 Hz", "Gamma."),
        (40.0, "Gamma 40 Hz", "Perceptual binding; the most-studied gamma frequency."),
        (45.0, "Gamma 45 Hz", "Gamma."),
        (50.0, "Gamma 50 Hz (isochronic)", "Above about 40 Hz binaural beats become hard to perceive, so this plays isochronic pulses."),
        (63.0, "Gamma 63 Hz (isochronic)", "High gamma."),
        (80.0, "High gamma 80 Hz (isochronic)", "High gamma."),
        (100.0, "Hypergamma 100 Hz (isochronic)", "Speculative \"hypergamma\" band from popular sources."),
        (200.0, "Lambda 200 Hz (isochronic)", "Speculative \"lambda\" band from popular sources."),
    ];
    for (hz, name, note) in lib {
        let mode = if *hz >= 50.0 { "isochronic" } else { "binaural" };
        let ev = if note.contains("Speculative") || note.contains("speculative") || note.contains("unproven") { "Speculative" }
                 else if note.contains("et al") || note.contains("&amp;") { "Research" } else { "Traditional" };
        let id = format!("lib{}", (hz * 100.0) as u32);
        v.push(single(g, &id, name, &format!("{} · 30 min", crate_band(*hz)), *hz, band_carrier(*hz), "pink", mode, ev,
            &format!("<p>{}</p><p>Starts at a gentle tuning tone, glides to {} Hz over two minutes, then holds. For the most direct effect, set the noise to None.</p>", note, hz)));
    }

    // ================= Schumann resonances =================
    let g = "Schumann Resonances";
    for (i, hz) in [7.83, 14.3, 20.8, 27.3, 33.8].iter().enumerate() {
        v.push(single(g, &format!("sch{}", i + 1), &format!("Schumann mode {} — {} Hz", i + 1, hz), "30 min", *hz, 194.18, "surf", "binaural", "Speculative",
            &format!("<p>The Schumann resonances are real standing electromagnetic waves between Earth's surface and the ionosphere. Mode {} is about {} Hz. Any effect from listening to them is speculative.</p>", i + 1, hz)));
    }
    v.push(p(g, "schstack", "Schumann Stack (all 5 modes)", "Layered · 30 min", &[136.1, 194.18, 256.0, 320.0, 384.0], &[1.0, 0.7, 0.5, 0.35, 0.25], "surf", "binaural", "Speculative",
      "<p>All five Schumann modes play at once, each on its own carrier.</p>",
      vec![st("Stack", 30.0, &[7.83, 14.3, 20.8, 27.3, 33.8], C_REST)]));

    // ================= Solfeggio =================
    let g = "Solfeggio Frequencies";
    let sol: &[(f64, &str)] = &[(174.0, "Foundation, grounding"), (285.0, "Restoration"), (396.0, "Releasing fear and guilt"), (417.0, "Facilitating change"),
        (528.0, "\"Love frequency\", transformation"), (639.0, "Connection, relationships"), (741.0, "Expression, problem solving"), (852.0, "Intuition"), (963.0, "Crown, oneness")];
    for (hz, meaning) in sol {
        v.push(p(g, &format!("sol{}", *hz as u32), &format!("Solfeggio {} Hz", hz), &format!("{} · 20 min", meaning.replace('"', "")), &[*hz], &[0.6], "pink", "binaural", "Traditional",
            &format!("<p>Traditional association: <b>{}</b>. The modern \"Solfeggio\" set was popularised in the 1990s (Puleo; Horowitz). Its claims of ancient origin and healing power are not supported by evidence. A 6 Hz theta beat rides on top. The tone volume is reduced because higher carriers sound louder.</p>", meaning, ),
            vec![st("Settle", 2.0, &[10.0], C_SETTLE), st(&format!("{} Hz", hz), 18.0, &[6.0], C_REST)]));
    }
    let journey: Vec<Stage> = sol.iter().map(|(hz, m)| st(&format!("{} Hz", hz), 3.0, &[6.0], &format!("{}.", m.replace('"', ""))).car(&[*hz]).ramp(20.0)).collect();
    v.push(p(g, "soljourney", "Full Solfeggio Scale Journey", "9 tones · 27 min", &[174.0], &[0.6], "pink", "binaural", "Traditional",
      "<p>Climbs through all nine Solfeggio tones, three minutes each, with a steady 6 Hz theta beat.</p>", journey));

    // ================= Chakras =================
    let g = "Chakra Series";
    let ch: &[(&str, f64, f64, &str)] = &[
        ("Root (Muladhara)", 396.0, 4.0, "Feel your connection to the ground beneath you, stable and safe."),
        ("Sacral (Svadhisthana)", 417.0, 5.0, "Bring warmth and gentle flow to your lower belly."),
        ("Solar Plexus (Manipura)", 528.0, 6.0, "Feel a steady golden warmth just above your navel."),
        ("Heart (Anahata)", 639.0, 7.83, "Let your chest soften and open with kindness."),
        ("Throat (Vishuddha)", 741.0, 9.0, "Relax your throat and jaw. Let your voice feel free."),
        ("Third Eye (Ajna)", 852.0, 10.0, "Rest your attention between your eyebrows."),
        ("Crown (Sahasrara)", 963.0, 12.0, "Feel open space above the top of your head."),
    ];
    for (name, car, beat, cue) in ch {
        v.push(p(g, &format!("ch{}", *car as u32), name, &format!("{} Hz · 15 min", car), &[*car], &[0.6], "pink", "binaural", "Traditional",
            &format!("<p>Traditional chakra meditation, using the commonly cited Solfeggio–chakra pairing ({} Hz) with a {} Hz beat. These are traditional associations, not scientific findings.</p>", car, beat),
            vec![st("Settle", 2.0, &[10.0], C_SETTLE), st(name, 13.0, &[*beat], cue)]));
    }
    let cj: Vec<Stage> = ch.iter().map(|(n, c, b, cue)| st(n, 4.0, &[*b], cue).car(&[*c]).ramp(30.0)).collect();
    v.push(p(g, "chjourney", "Chakra Journey (root → crown)", "7 stages · 28 min", &[396.0], &[0.6], "pink", "binaural", "Traditional",
      "<p>Rises from root to crown, four minutes on each chakra, with the carrier tone and beat gliding upward between stages.</p>", cj));

    // ================= Cosmic Octave =================
    let g = "Cosmic Octave (Planets)";
    let pl: &[(&str, f64)] = &[("Earth Year / Om", 136.10), ("Earth Day", 194.18), ("Sun", 126.22), ("Moon (synodic)", 210.42), ("Mercury", 141.27),
        ("Venus", 221.23), ("Mars", 144.72), ("Jupiter", 183.58), ("Saturn", 147.85), ("Uranus", 207.36), ("Neptune", 211.44), ("Pluto", 140.25)];
    for (name, hz) in pl {
        v.push(p(g, &format!("pl{}", (*hz * 100.0) as u32), &format!("{} — {} Hz", name, hz), "Theta 6 Hz · 20 min", &[*hz], &[1.0], "pink", "binaural", "Traditional",
            &format!("<p>Hans Cousto's <i>Cosmic Octave</i> (1978) takes an orbital period and doubles its frequency until it becomes audible. For {} that gives {} Hz. The maths is real; the meanings given to these tones are traditional.</p>", name, hz),
            vec![st("Settle", 2.0, &[10.0], C_SETTLE), st(name, 18.0, &[6.0], C_REST)]));
    }

    // ================= Tunings =================
    let g = "Tunings";
    v.push(p(g, "t432", "A = 432 Hz", "Alpha 8 Hz · 20 min", &[432.0], &[0.6], "pink", "binaural", "Speculative",
      "<p>Claims that 432 Hz tuning is \"natural\" or healing aren't supported by evidence. A small 2019 pilot study (Calamassi &amp; Pomponi) reported slightly lower heart rate compared with 440 Hz.</p>",
      vec![st("432", 20.0, &[8.0], C_REST)]));
    v.push(p(g, "t440", "A = 440 Hz (concert standard)", "Alpha 8 Hz · 20 min", &[440.0], &[0.6], "pink", "binaural", "Traditional",
      "<p>The ISO 16 standard concert pitch, for comparison with 432 Hz.</p>", vec![st("440", 20.0, &[8.0], C_REST)]));
    v.push(p(g, "t256", "Verdi C = 256 Hz (scientific pitch)", "Alpha 10 Hz · 20 min", &[256.0], &[1.0], "pink", "binaural", "Traditional",
      "<p>\"Scientific pitch\" sets middle C to 256 Hz (a power of two). It's often associated with Verdi.</p>", vec![st("256", 20.0, &[10.0], C_REST)]));

    // Keep each session's original opening line for tagging, so the tailored openings
    // only ever add search tags and never take one away.
    library::music(&mut v);
    let first: Vec<String> = v.iter().map(|p| p.stages.first().map(|s| s.cue.clone()).unwrap_or_default()).collect();
    crate::settle::apply(&mut v);
    for p in v.iter_mut() {
        if p.trad.is_empty() { p.trad = trad_of(&p.id, &p.group); }
        // third-level schools (1.8): only ever added after the family and branch keys
        for s in schools_of(&p.id) {
            if !p.trad.iter().any(|x| x == s) { p.trad.push(s.to_string()); }
        }
    }
    crate::info::fill(&mut v);
    auto_tags(&mut v, &first);

    // assign cue ids
    for pr in v.iter_mut() {
        for s in pr.stages.iter_mut() {
            s.cue_id = if s.cue.is_empty() { String::new() } else { cue_id(&s.cue) };
        }
    }
    v
}

/// The world's contemplative and ritual traditions, one group per tradition.
/// Spiritual claims are reported as claims; studies are cited where they exist. Nothing here
/// needs a warning: no fast breathing, no breath holds, no flicker, and the gazing aids are still images.
fn traditions(v: &mut Vec<Preset>) {
    const C_DEDICATE: &str = "Dedicate whatever good came from this practice to the happiness of all beings.";

    // ================= Christian mysticism =================
    let g = "Christian Mysticism";
    v.push(p(g, "cm_lectio", "Lectio Divina — The Ladder of Monks (Guigo II)", "Read, reflect, pray, rest · 26 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Lectio divina</b> (\"divine reading\") is the slow, prayerful reading of scripture kept in Benedictine monasteries since the 6th-century Rule of St Benedict. Around 1150 the Carthusian prior <b>Guigo II</b> described it as a ladder with four rungs in <i>The Ladder of Monks</i>: <b>lectio</b> (reading), <b>meditatio</b> (reflecting), <b>oratio</b> (responding in prayer) and <b>contemplatio</b> (resting wordlessly in God). Choose a short passage before you start; a psalm or a Gospel scene works well.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to lectio divina, the ladder of monks. Sit quietly with a short passage of scripture in front of you, just a few verses. Take a slow breath, and ask to be open to what you read."),
           st("Lectio", 5.0, &[10.0], "Lectio, reading. Read the passage slowly, perhaps aloud, two or three times. Notice the word or phrase that catches your heart, and stop there."),
           st("Meditatio", 6.0, &[8.0], "Meditatio. Put the text down and close your eyes. Repeat your word or phrase gently, again and again, and let it speak to your life."),
           st("Oratio", 5.0, &[7.0], "Oratio, prayer. Let the word become a prayer. Speak to God honestly, in your own words, about whatever it stirred in you."),
           st("Contemplatio", 7.0, &[6.0], "Contemplatio. Now let even the words fall away. Rest in silence, in the presence beyond words, simply receiving."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "cm_maranatha", "Christian Meditation — Maranatha (John Main)", "Four-syllable prayer word · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The Benedictine monk <b>John Main</b> (1926–1982) learned mantra meditation from an Indian teacher in Malaya, then found the same practice in the <i>Conferences</i> of <b>John Cassian</b>, who around 420 recorded how the Desert Fathers repeated one short prayer to keep the mind on God. From 1975 Main taught it to lay groups in London and Montreal, and the World Community for Christian Meditation continues it today. You say one word, <b>maranatha</b> (Aramaic for \"Come, Lord\"), silently in four equal syllables, for 20 to 30 minutes each morning and evening.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to Christian meditation as John Main taught it. Sit still and upright, with your eyes lightly closed. Relax, but stay alert. Your word is maranatha, an ancient Aramaic prayer meaning come, Lord."),
           st("The word", 8.0, &[8.0], "Silently, interiorly, begin to say the word in four equal syllables: ma, ra, na, tha. Listen to it as you say it, gently and faithfully."),
           st("Faithful return", 12.0, &[6.5], "When thoughts or images come, don't fight them, and don't follow them. Simply return to the word, ma ra na tha, from the beginning of the meditation to the end."),
           st("Silence", 3.0, &[8.0], "Let the word go now. Sit for a moment in the silence it has led you into, then slowly open your eyes.")]));
    v.push(p(g, "cm_ignatian", "Ignatian Contemplation — Praying with the Imagination", "Spiritual Exercises · 25 min", &[200.0], &[0.55], "pink", "binaural", "Traditional",
      "<p>In the <b>Spiritual Exercises</b> (approved in 1548), <b>St Ignatius of Loyola</b>, founder of the Jesuits, teaches a way of praying a Gospel story by entering it with the imagination. You begin with a <i>preparatory prayer</i>, ask for the grace you want, build the scene (<i>composition of place</i>), take part in it with all five senses (<i>application of the senses</i>), and end with a <i>colloquy</i>, a heart-to-heart conversation \"as one friend speaks to another\". Jesuits still give the Exercises as 30-day retreats.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to Ignatian contemplation, prayer with the imagination. Choose a Gospel scene you know well, such as the calming of the storm, or a healing by the lake. Sit comfortably and close your eyes."),
           st("Grace", 2.0, &[9.0], "Become aware that God is looking at you with love. Ask for the grace you desire from this prayer: perhaps to know Christ more closely, and to love him more."),
           st("Composition of place", 4.0, &[8.0], "Compose the place. See the scene in your imagination: the landscape, the light, the time of day, the people. Take your time to build it."),
           st("The senses", 9.0, &[6.5], "Now step into the scene. What do you see? What do you hear: voices, wind, water? What can you smell and touch? Let the story unfold, and take part in it."),
           st("Colloquy", 5.0, &[7.0], "The scene comes to rest. Speak with Jesus, as one friend speaks to another. Say what is in your heart, and listen."),
           st("Close", 3.0, &[9.0], "Finish with the Our Father, slowly. Notice what moved you, and take it with you as you open your eyes.")]));
    v.push(p(g, "cm_castle", "The Interior Castle — Prayer of Quiet (Teresa of Ávila)", "Walking inward · 28 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>In <b>The Interior Castle</b> (1577) the Carmelite reformer <b>St Teresa of Ávila</b> pictures the soul as a castle made of a single diamond, with many rooms in seven \"dwelling places\" and God shining at its centre; prayer is the gate. In <i>The Way of Perfection</i> she teaches the <b>prayer of recollection</b>, gathering the senses inward, which can open into the <b>prayer of quiet</b>, where the will rests in God even while the mind wanders. This session walks inward through the castle. The deeper states she describes she regarded as gifts, not techniques.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the interior castle of Saint Teresa of Avila. Sit quietly and close your eyes. Teresa pictured the soul as a castle made of a single diamond, with God shining at its centre. Today you will walk inward."),
           st("Recollection", 4.0, &[9.0], "Gather yourself in. Draw your senses inward, the way a tortoise draws into its shell. The one you seek is not far away, but within."),
           st("Outer rooms", 5.0, &[8.0], "You are at the gate. Walk slowly through the outer rooms, where noise and distraction still reach you. Don't be troubled by them. Keep walking toward the light."),
           st("Inner rooms", 6.0, &[6.5], "The rooms grow quieter and brighter. Prayer becomes simpler here, more a resting than an effort."),
           st("Prayer of quiet", 8.0, &[5.5], "This is the prayer of quiet. Let your will rest in God, like a child resting in its mother's arms. If the mind wanders, let it; the heart stays still."),
           st("The centre", 2.0, &[6.0], "At the centre the light is brightest. Rest in it, and in the one who dwells there."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "cm_cloud", "The Cloud of Unknowing — Dart of Longing Love", "Prayer beyond thought · 22 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>The Cloud of Unknowing</b> is a guide to contemplative prayer written by an anonymous English priest in the late 14th century. God, it says, cannot be reached by thinking, only by love. You put a <b>cloud of forgetting</b> beneath you, covering every thought and creature, and beat on the <b>cloud of unknowing</b> above you \"with a sharp dart of longing love\", using a single short word, of one syllable if possible, such as <i>God</i> or <i>love</i>. Centering prayer grew from this book.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the prayer of the cloud of unknowing. Sit still and close your eyes. Choose one short word, of one syllable if you can, such as God, or love."),
           st("Cloud of forgetting", 4.0, &[8.0], "Put a cloud of forgetting beneath you. Every thought, plan and memory, even holy ones, let them sink below it, out of sight."),
           st("Cloud of unknowing", 4.0, &[7.0], "Above you is a cloud of unknowing. God cannot be grasped by thought, only by love. Rest in that darkness with a simple longing."),
           st("Dart of love", 10.0, &[6.0], "Now and then, send your word up into the cloud like a sharp dart of longing love. Then rest. When thoughts come, press them back under the cloud of forgetting with your word."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "cm_taize", "Chant and Silence (Taizé-style)", "Repeated sung prayer · 20 min", &[220.0, 330.0], &[0.55, 0.3], "none", "binaural", "Traditional",
      "<p>The ecumenical community of <b>Taizé</b> in France, begun by Brother Roger in 1940, is known for prayer made of short chants sung over and over, so that the words sink from the mind into the heart, followed by a long silence; tens of thousands of young people visit every year. The Taizé melodies belong to the community, so this session plays only a soft drone on A and E. Sing any short sacred phrase on it, for example the ancient Latin <i>Ubi caritas et amor, Deus ibi est</i> (\"where charity and love are, God is there\").</p>",
      vec![st("Settle", 2.0, &[10.0, 10.0], "Welcome to chant and silence, in the style of the Taize community in France. Sit or kneel, with a candle lit if you like. Choose a short phrase to sing, perhaps a line from a psalm."),
           st("Chant", 8.0, &[8.0, 8.0], "Begin to sing your phrase softly on the note of the drone, and repeat it, again and again. Let the repetition carry you, until the song becomes a prayer of the heart."),
           st("Softer", 4.0, &[7.0, 7.0], "Sing more quietly now, until the chant is almost a whisper, and then only a hum."),
           st("Silence", 6.0, &[6.0, 6.0], "Let the singing end. At Taize the chant gives way to a long silence. Rest in it, and let the words keep echoing inside you.")]));
    v.push(p(g, "cm_quaker", "Quaker Meeting — Expectant Waiting", "Silent worship · 30 min", &[200.0], &[0.4], "brown", "binaural", "Traditional",
      "<p>In an unprogrammed <b>Quaker meeting for worship</b>, Friends sit together in silence, with no minister or set words, waiting for the <b>inward light</b>, which George Fox, who founded the Religious Society of Friends in 1650s England, called \"that of God in everyone\". Anyone may stand and speak if they feel truly moved. Settling into stillness is called <i>centering down</i>, and a meeting deeply united in its silence is called a <i>gathered meeting</i>.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to Quaker waiting worship. Friends gather in silence, without a leader or set words, waiting on the light within. Close your eyes, or rest them on the floor, and let the busy mind slow down."),
           st("Centering down", 6.0, &[8.0], "This is centering down. Let your thoughts settle, like mud settling in still water. You are not emptying your mind, only becoming quiet enough to listen."),
           st("Expectant waiting", 18.0, &[6.5], "Wait in expectant silence, open to that of God within you. If a message rises that feels as if it is not only your own, notice it, and hold it gently."),
           st("Close", 4.0, &[8.0], "Meetings end when Friends shake hands. Take a moment to feel connected to the people you care about, and carry the stillness back with you.")]));
    v.push(p(g, "cm_vigil", "Night Vigil — The Watch Between Two Sleeps", "Ekirch 2001 · Wehr 1992 · 30 min", &[150.0], &[0.35], "brown", "binaural", "Research",
      "<p>Historian <b>A. Roger Ekirch</b> (2001, <i>American Historical Review</i>) found hundreds of pre-industrial references to a \"first sleep\" and a \"second sleep\", with an hour or so awake in between that people used to pray, reflect, talk or make love. When psychiatrist <b>Thomas Wehr</b> (1992, <i>Journal of Sleep Research</i>) gave volunteers 14-hour nights, their sleep split the same way, with a calm, quiet waking spell between the two sleeps. Monks have long risen in the night for the office of <b>Vigils</b> (Rule of St Benedict, chapter 8). Use this session when you wake in the night, with the volume low. If you have insomnia, sleep specialists usually advise getting up after about 20 minutes awake, so follow your clinician's advice.</p>",
      vec![st("Settle", 2.0, &[6.0], "Welcome to the night vigil. Use this when you wake in the night. Keep the room dark, the volume low, and stay lying down. For centuries people woke between a first and second sleep, and used this quiet time to pray and reflect."),
           st("The quiet hour", 10.0, &[5.0], "There is nothing wrong with being awake. Let your body stay heavy and warm. If you pray, pray slowly, a psalm or a few familiar words. If not, reflect gently on the day, or on the people you love."),
           st("Keeping watch", 8.0, &[4.0], "Let the prayer grow simpler: a few words, then one. Rest in the stillness of the night, calm and alert, as monks have kept watch before dawn."),
           st("Second sleep", 10.0, &[2.0], "Now let go of watching. Let the words dissolve, and let the second sleep come and take you.").ramp(120.0)]));
    v.push(p(g, "cm_bestill", "Be Still — Psalm 46 Breath Prayer", "Shortening the verse · 15 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>A much-loved contemplative exercise prays <b>Psalm 46:10</b>, \"Be still, and know that I am God\", and then shortens it a phrase at a time: \"Be still, and know that I am\", \"Be still, and know\", \"Be still\", \"Be\", until only stillness is left. It is used in retreat houses, churches and recovery groups. Each line is prayed slowly with the breath for a couple of minutes; the breath pacer can guide you.</p>",
      vec![st("Settle", 1.5, &[10.0], "Welcome to the be still prayer, from Psalm forty six. Sit comfortably and close your eyes. You will pray one line, then let it shorten, word by word, until only stillness is left."),
           st("Be still, and know that I am God", 2.5, &[9.0], "Breathe slowly. With each breath, pray silently: be still, and know that I am God.").breath(),
           st("Be still, and know that I am", 2.5, &[8.0], "Be still, and know that I am.").breath(),
           st("Be still, and know", 2.5, &[7.0], "Be still, and know.").breath(),
           st("Be still", 2.5, &[6.5], "Be still.").breath(),
           st("Be", 3.5, &[6.0], "Be. And then rest, in the silence beyond the words.")]));
    let mut x = p(g, "cm_visio", "Visio Divina — Praying with an Icon", "Contemplative gazing · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>In Eastern Orthodox Christianity icons are called \"windows into heaven\": they are prayed with, not just looked at, and the Second Council of Nicaea (787) affirmed their veneration. <b>Visio divina</b> (\"divine seeing\") applies the pattern of lectio divina to an image: gaze, notice what draws you, respond in prayer, and rest. Use an icon or sacred image of your own. The <b>🕯 gazing aid</b> can show a steady candle flame like the lamp kept before icons; it never flickers.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to visio divina, praying with an icon. Place an icon or sacred image where you can see it comfortably, with a candle beside it if you like. Sit, and let your breathing slow."),
           st("Gaze", 6.0, &[8.0], "Let your eyes rest softly on the image. Don't analyse it. Let your gaze wander until something holds it, a face, a hand, a colour, and stay with that."),
           st("Being seen", 6.0, &[7.0], "In the icon tradition, the image also gazes at you. Let yourself be seen. Notice what rises in you: a feeling, a memory, a longing."),
           st("Respond", 3.0, &[7.0], "Respond in prayer, with words or without them, to what you have received."),
           st("Rest", 3.0, &[6.0], "Close your eyes and rest, holding the image gently within.")]);
    x.visual = "trataka:flame".into();
    v.push(x);
    v.push(with_tex(p(g, "cm_rosary", "The Rosary — Five Decades with the Mysteries", "Bead prayer and contemplation · 20 min", &[200.0], &[0.3], "none", "binaural", "Traditional",
      "<p>The <b>rosary</b> took its familiar form in the 15th century, spread by the Dominican Alan de la Roche. Each of its five <b>decades</b> is an Our Father, ten Hail Marys and a Glory Be, prayed while contemplating a <b>mystery</b> from the lives of Jesus and Mary: the Joyful, Sorrowful and Glorious mysteries, and the Luminous mysteries added by Pope John Paul II in 2002. Bernardi et al. (2001, <i>BMJ</i>) found that reciting the rosary slows breathing to about six breaths a minute; a soft bell here marks that slow, ten-second rhythm.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the rosary, with time to contemplate each mystery. Take your beads if you have them, and choose today's mysteries. Begin with the sign of the cross, the creed and the opening prayers, at your own pace.").tex(0.0),
           st("First decade", 3.4, &[8.0], "The first decade. Picture the first mystery. Pray the Our Father, then ten Hail Marys, and the Glory Be, keeping the scene before you."),
           st("Second decade", 3.4, &[7.5], "The second decade. Picture the second mystery, and enter it as you pray."),
           st("Third decade", 3.4, &[7.0], "The third decade. Let the words become a quiet rhythm beneath the third mystery."),
           st("Fourth decade", 3.4, &[6.5], "The fourth decade. Rest in the fourth mystery, letting the prayers flow."),
           st("Fifth decade", 3.4, &[6.5], "The fifth decade. Stay with whatever moves you most in the fifth mystery."),
           st("Close", 1.0, &[8.0], "Close with the Hail Holy Queen, or a prayer of your own, and the sign of the cross.").tex(0.0)]), tx("breathbell", 0.45, 392.0, 10.0)));

    // ================= Tibetan Buddhism & Bön =================
    let g = "Tibetan Buddhism & Bön";
    v.push(p(g, "tb_shamatha", "Shamatha — Calm Abiding (Shamatha Project)", "MacLean 2010 · 30 min", &[200.0], &[0.5], "pink", "binaural", "Research",
      "<p><b>Shamatha</b> (\"calm abiding\") is the Tibetan training of stable, clear attention, mapped in nine stages by Asanga and Kamalashila. In the <b>Shamatha Project</b>, people spent three months on retreat with the teacher <b>B. Alan Wallace</b>; compared with a waiting-list group they improved in fine visual discrimination and in sustained attention (<b>MacLean et al., 2010, <i>Psychological Science</i></b>), and later analyses reported gains in emotional well-being. This session follows Wallace's sequence: mindfulness of breathing, settling the mind in its natural state, and awareness of awareness.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to shamatha, calm abiding, as taught on the Shamatha Project retreat. Sit or lie comfortably, with your eyes half open, resting in the space in front of you. Breathe naturally. Three qualities matter here: relaxation, stillness and clarity."),
           st("Breathing", 10.0, &[9.0], "Rest your attention on the sensations of the breath throughout the body. Don't control it at all. Let it flow as if you were deeply asleep, while you stay awake and clear."),
           st("Settling the mind", 10.0, &[7.0], "Now turn your attention to the space of the mind, and whatever arises there. Watch thoughts and images come and go, without being carried away and without pushing them away."),
           st("Awareness of awareness", 6.0, &[6.0], "Now let even the thoughts be. Rest your attention on awareness itself, the simple knowing that you are aware, without any object."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "tb_tonglen", "Tonglen — Sending and Taking (Lojong)", "Breathing compassion · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Tonglen</b> (\"giving and taking\") comes from the Tibetan <b>lojong</b> or mind-training teachings, which the Bengali master <b>Atisha</b> (982–1054) brought to Tibet and Geshe Chekawa set down as the <i>Seven Points of Mind Training</i> in the 12th century. Riding the breath, you take in suffering and send out relief, reversing the habit of self-protection. This session follows the four steps taught by Pema Chödrön: a flash of openness, working with texture, a specific person, then all beings. It can stir strong feelings; go gently, and begin with your own pain if that is easier.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to tonglen, the Tibetan practice of sending and taking. Sit upright and close your eyes. You will breathe in suffering and breathe out relief. It opens the heart, and can stir strong feelings, so go gently."),
           st("Openness", 2.0, &[8.0], "First, rest your mind for a moment in openness and stillness, like a clear sky."),
           st("Texture", 4.0, &[7.0], "Now breathe in a feeling of heat, darkness and heaviness, through every pore of your body. Breathe out coolness, brightness and lightness, in every direction."),
           st("One person", 6.0, &[6.0], "Bring to mind someone who is suffering. Breathe in their pain as dark smoke, and let it dissolve in your heart. Breathe out to them whatever would help, as warm, bright light."),
           st("All beings", 5.0, &[6.0], "Now widen it to everyone who feels as they do, and to all beings everywhere. Breathe in their suffering; breathe out relief and happiness. If it feels too much, return to yourself, then widen again."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "tb_mani", "Chenrezig — Om Mani Padme Hum", "Compassion mantra · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Chenrezig</b> (Sanskrit <i>Avalokiteśvara</i>) is the bodhisattva of compassion and Tibet's protector; the Dalai Lamas are regarded as his emanations. His six-syllable mantra <b>Om mani padme hum</b>, first recorded in the <i>Kāraṇḍavyūha Sūtra</i> (around the 5th century), is carved on stones, spun in prayer wheels and counted on malas across the Himalaya. The practice follows the usual shape of a deity meditation: visualise, recite while light streams out to all beings, dissolve, and dedicate.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the practice of Chenrezig, the bodhisattva of compassion, and his six syllable mantra, om mani padme hum. Sit upright and close your eyes. If you have a mala, hold it in your left hand."),
           st("Visualise", 4.0, &[8.0], "In the space before you, picture Chenrezig, white as moonlight, seated on a lotus and a moon disc, with four arms and a gentle smile. His eyes look on every being with love."),
           st("Recite", 10.0, &[6.5], "Recite the mantra softly: om mani padme hum. With each repetition, light flows from his heart into you, and out to all beings in every direction."),
           st("Dissolve", 3.0, &[6.0], "Now Chenrezig melts into light and dissolves into you. Your mind and his compassion are not two. Rest in that."),
           st("Dedicate", 1.0, &[8.0], C_DEDICATE)]));
    v.push(p(g, "tb_tara", "Green Tara — The Swift Protector", "Mantra and visualisation · 18 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Green Tara</b> is the female buddha of enlightened activity, loved in Tibet as a mother who comes swiftly to anyone in fear; the master Atisha was devoted to her. She sits with her right leg extended, ready to rise and help, her right hand in the gesture of giving. Her mantra is <b>Om tare tuttare ture soha</b>, and she is called on for protection from the \"eight fears\" and for removing obstacles.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the practice of Green Tara, the swift protector. Sit comfortably and close your eyes. Bring to mind anything that frightens or burdens you, and offer it to her."),
           st("Visualise", 4.0, &[8.0], "Picture Tara before you, emerald green and radiant, on a lotus, her right leg extended, ready to rise and help. Her right hand is open in the gesture of giving."),
           st("Mantra", 8.0, &[6.5], "Recite her mantra: om tare tuttare ture soha. With every repetition, green light streams from her into you, dissolving fear and obstacles."),
           st("Dissolve", 3.0, &[6.0], "Tara dissolves into green light and melts into your heart. Rest, protected and at ease."),
           st("Dedicate", 1.0, &[8.0], C_DEDICATE)]));
    v.push(p(g, "tb_medicine", "Medicine Buddha — Lapis Light", "Healing visualisation · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Medicine Buddha</b> (<i>Bhaiṣajyaguru</i>, Tibetan <i>Sangye Menla</i>) is deep blue like lapis lazuli and holds a bowl of healing nectar and a medicinal plant. Tibetan doctors traditionally recite his mantra while preparing medicines. In Tibetan pronunciation it runs <b>Tayata om bekandze bekandze maha bekandze radza samudgate soha</b>. This is a spiritual practice, not a substitute for medical care.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Medicine Buddha practice. Sit or lie comfortably and close your eyes. Hold in mind anyone who needs healing, yourself included."),
           st("Visualise", 4.0, &[8.0], "Above your head, picture the Medicine Buddha, deep blue like lapis lazuli, radiant and translucent, holding a bowl of healing nectar."),
           st("Mantra", 9.0, &[6.0], "Recite his mantra slowly: tayata, om bekandze bekandze, maha bekandze, radza samudgate soha. With each repetition, blue light pours down through your body, washing away illness and pain."),
           st("Healing light", 4.0, &[5.0], "The Medicine Buddha melts into blue light and dissolves into you. Rest in that clear light, and send it to anyone who needs it."),
           st("Dedicate", 1.0, &[8.0], C_DEDICATE)]));
    v.push(p(g, "tb_vajra", "Vajra Recitation — Om Ah Hum with the Breath", "Natural breath, three syllables · 15 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>In the <b>vajra recitation</b> (Tibetan <i>dorje depa</i>) of Vajrayana practice, the three seed syllables <b>Om Ah Hum</b>, standing for enlightened body, speech and mind, are joined to the natural breath: Om as it comes in, Ah as it rests, Hum as it goes out. They are usually pictured as white light at the crown, red at the throat and blue at the heart. Breathe naturally; this practice never forces or holds the breath.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the vajra recitation. Sit upright and close your eyes. In this practice the seed syllables om, ah and hum are joined to the breath. Breathe naturally through the nose; never strain or hold the breath."),
           st("Three syllables", 3.0, &[8.0], "First, meet the syllables. Om, white light at the crown of the head. Ah, red light at the throat. Hum, blue light at the heart."),
           st("With the breath", 8.0, &[6.5], "Now join them to the breath. As it comes in, om. As it rests, ah. As it goes out, hum. Let the breath carry the sounds, without effort."),
           st("Rest", 2.0, &[6.0], "Let the syllables fade, and rest in the breath and the open, aware mind.")]));
    v.push(p(g, "tb_phat", "Three Words That Strike the Vital Point — PHAT!", "Dzogchen (Garab Dorje) · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Three Words That Strike the Vital Point</b> are said to be the last testament of <b>Garab Dorje</b>, first human teacher of <b>Dzogchen</b> (\"Great Perfection\"), explained in a famous 19th-century commentary by <b>Patrul Rinpoche</b>: recognise your own nature directly, decide on this one thing, and gain confidence in liberation. For the first, you relax completely and then shout a short, forceful <b>PHAT!</b> (said roughly \"pay\"), which cuts through thought and leaves bare, wide-open awareness. Traditionally a qualified teacher points this out, so treat this session as an introduction.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the three words that strike the vital point, a Dzogchen teaching of Garab Dorje. Sit upright, with your eyes open, gazing into the space before you. Traditionally a teacher points this out, so treat this as an introduction."),
           st("Relax", 3.0, &[8.0], "Relax completely. Let your mind be, without thinking of past or future, without effort, without even trying to meditate."),
           st("PHAT!", 1.0, &[7.83], "Now, suddenly, shout a short, sharp syllable: phat, said like pay. Let it cut straight through your thoughts."),
           st("The gap", 6.0, &[7.0], "Rest in the gap that opens: bare, fresh, wide awake, with nothing to grasp. This is the first word. Recognise your own nature directly."),
           st("One thing", 5.0, &[6.5], "The second word: decide on this one thing. Whether calm or busy, joy or sorrow arises, recognise the same open awareness. If you drift, shout phat again, and rest."),
           st("Confidence", 3.0, &[6.0], "The third word: confidence in liberation. Thoughts arise and dissolve by themselves, like drawings on water. Nothing needs fixing. Rest.")]));
    v.push(p(g, "tb_tilopa", "Tilopa's Six Words of Advice (Mahamudra)", "Six instructions, then silence · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Tilopa</b> (988–1069), the Indian mahasiddha at the root of the Kagyu lineage, gave his student Naropa six brief instructions for resting the mind in <b>Mahamudra</b> (\"the great seal\"), famous as Tilopa's six words: <b>don't recall, don't imagine, don't think, don't examine, don't control, rest</b>. Each is given here with a few minutes of silence to try it.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to Tilopa's six words of advice, a Mahamudra teaching from a thousand years ago. Sit comfortably, with your eyes open or closed. Six short instructions will follow, each with time to rest in it."),
           st("Don't recall", 3.0, &[9.0], "Don't recall. Leave the past alone; let memories go."),
           st("Don't imagine", 3.0, &[8.0], "Don't imagine. Leave the future alone; no plans, no fantasies."),
           st("Don't think", 3.0, &[7.0], "Don't think. Leave the present alone; no commentary on what is happening."),
           st("Don't examine", 3.0, &[6.5], "Don't examine. No analysing, no working anything out."),
           st("Don't control", 3.0, &[6.0], "Don't control. No steering, no trying to make anything happen."),
           st("Rest", 3.0, &[6.0], "Rest. Just relax, here and now, and rest.")]));
    v.push(p(g, "tb_chod", "Feeding Your Demons (after Machig Labdrön's Chöd)", "Five steps · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Chöd</b> (\"cutting through\") was founded by the Tibetan yogini <b>Machig Labdrön</b> (1055–1149): instead of fighting \"demons\", the practitioner offers them their own body, visualised as nectar, and so cuts through clinging to the self. <b>Lama Tsultrim Allione</b> turned its essence into a five-step method anyone can use, in <i>Feeding Your Demons</i> (2008): find the demon in your body, personify it and ask what it needs, become it, feed it until it is satisfied, and meet the ally it becomes. A \"demon\" here is anything that troubles you, such as a fear, a craving or an illness. This is not therapy; work through trauma with a professional.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to feeding your demons, a five step practice drawn from the Tibetan practice of chod. Sit comfortably and close your eyes. A demon here simply means something that troubles you: a fear, a craving, a painful pattern. Choose one that feels manageable today."),
           st("Find it", 3.0, &[8.0], "Step one. Find the demon in your body. Where do you feel it most? Notice its colour, its texture, its temperature."),
           st("Personify it", 5.0, &[7.0], "Step two. Let it take shape in front of you as a creature or figure. Look at its size, its colour, its eyes. Ask it: what do you want from me? What do you really need? How will you feel when you get it?"),
           st("Become it", 4.0, &[6.5], "Step three. Change places. Become the demon, and answer those questions from inside its body. Then come back to yourself, and look at it again."),
           st("Feed it", 5.0, &[6.0], "Step four. Imagine your body dissolving into a warm nectar of exactly what it needs, love, safety or rest. Feed it generously until it is satisfied. See how it changes. It may become an ally. Ask how it will help you."),
           st("Rest", 4.0, &[6.0], "Step five. Let the ally, the demon and yourself dissolve into open space. Rest in awareness, with nothing to do."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(with_tex(p(g, "tb_bardo", "Dissolution of the Elements — Rehearsing the Bardo", "Contemplation of the clear light · 30 min", &[150.0], &[0.5], "brown", "binaural", "Traditional",
      "<p>Tibetan teachings on the <b>bardo</b>, the passage between death and rebirth, include the <i>Bardo Thödol</i> (the \"Tibetan Book of the Dead\", revealed by Karma Lingpa in the 14th century). They describe dying as a <b>dissolution of the elements</b>, each with an inner sign: earth into water (a mirage), water into fire (smoke), fire into wind (sparks like fireflies), wind into consciousness (a butter-lamp flame), then white, red and black appearances, and finally the <b>clear light</b>. Practitioners rehearse this so they can meet death without fear and recognise the clear light. A soft gong marks the stages. If you are grieving or feeling low, choose another session.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the dissolution of the elements, a contemplation from the Tibetan teachings on the bardo. Lie down comfortably and close your eyes. Practitioners rehearse this to meet death without fear. You are safe; this is only a contemplation.").tex(0.0),
           st("Earth into water", 4.0, &[8.0], "Earth dissolves into water. Imagine your body becoming very heavy, sinking, too heavy to move. The outer world shimmers like a mirage on a hot road."),
           st("Water into fire", 4.0, &[7.0], "Water dissolves into fire. The body's fluids seem to dry; mouth and eyes are dry. Inwardly, a vision of swirling smoke."),
           st("Fire into wind", 4.0, &[6.0], "Fire dissolves into wind. Warmth withdraws from the hands and feet toward the heart. Inwardly, sparks, like fireflies in the night."),
           st("Wind into consciousness", 4.0, &[5.0], "Wind dissolves into consciousness. The breath feels faint and long. Inwardly, the steady glow of a butter lamp."),
           st("White, red, black", 5.0, &[4.5], "Now three appearances: a white light like moonlight, then a red glow like a sunset sky, then deep darkness, like a moonless night."),
           st("Clear light", 5.0, &[4.0], "Then the darkness clears, and there is only the clear light: open, empty and luminous, like a dawn sky without a cloud. Rest as it."),
           st("Return to life", 2.0, &[10.0], "Now let the elements return. Feel the breath, the warmth in your body, the weight of your limbs and the ground beneath you. You are alive, here and now. Slowly open your eyes.").tex(0.0)]),
      tx("gong", 0.5, 98.0, 120.0)));
    v.push(p(g, "tb_gyuto", "Deep Chant Drone — Gyuto-style One-Voice Chord", "68 Hz with its 5th harmonic · 20 min", &[68.0, 136.0, 340.0], &[1.0, 0.3, 0.45], "none", "monaural", "Traditional",
      "<p>Monks of the <b>Gyuto</b> and <b>Gyume</b> Tantric Colleges chant in an extraordinarily deep voice that carries a clear second pitch above it, a \"one-voice chord\". An acoustic analysis (<b>Smith, Stevens &amp; Tomlinson, 1967, <i>Journal of the Acoustical Society of America</i></b>) showed the lamas strongly boosting the <b>fifth harmonic</b>, two octaves and a major third above the fundamental; the scholar Huston Smith had recorded the chant in 1964. This drone imitates that chord with a 68 Hz fundamental and its 2nd and 5th harmonics, pulsing slowly in theta. It uses monaural beats, so speakers work, but the low note needs headphones or good speakers. Chant along gently, and never strain your voice.</p>",
      vec![st("Settle", 2.0, &[6.0, 6.0, 6.0], "Welcome to the deep chant drone, modelled on the one voice chords of the Gyuto monks, who sing a very low note with a clear overtone high above it. Sit upright, relax your throat and jaw, and let the low sound fill you."),
           st("Listen", 4.0, &[5.0, 5.0, 5.0], "Listen first. Notice the deep fundamental, and the bright overtone shimmering above it like a second voice."),
           st("Chant", 10.0, &[4.5, 4.5, 4.5], "Now chant along, as low as is comfortable, on a long om or hum from deep in your belly. Never push your voice; it should feel easy. Feel it vibrate in your chest and skull."),
           st("Silence", 4.0, &[4.0, 4.0, 4.0], "Let your voice rest. Listen to the drone, and to the silence inside it.")]));
    v.push(p(g, "tb_sleepyoga", "Sleep Yoga — Falling Asleep in the Clear Light", "Bön and Nyingma · 30 min", &[150.0], &[0.45], "brown", "binaural", "Traditional",
      "<p>Alongside dream yoga, the Tibetan Bön and Nyingma traditions teach <b>sleep yoga</b>: falling asleep without losing awareness, so that even dreamless sleep becomes recognition of the mind's <b>clear light</b>. Tenzin Wangyal Rinpoche's <i>The Tibetan Yogas of Dream and Sleep</i> (1998) describes resting the mind in a luminous light at the heart as sleep comes. Lie down in bed with the volume low; the beat slows into deep delta.</p>",
      vec![st("Settle", 2.0, &[6.0], "Welcome to sleep yoga, the Tibetan practice of falling asleep with awareness. Lie down in bed, ready for sleep, on whichever side is comfortable, with the light off and the volume low."),
           st("Light at the heart", 6.0, &[5.0], "Bring your attention to the centre of your chest. Picture a small sphere of clear, luminous light there, soft and steady, like a pearl lit from within. Rest your awareness in it."),
           st("Merging", 8.0, &[3.5], "Let the boundary between you and the light soften. You are not looking at the light; you are resting as it, clear and aware."),
           st("Into sleep", 14.0, &[1.5], "Now let sleep come, without letting go of the light. As the senses fade, stay with it, the way you would stay with a friend while falling asleep.").ramp(180.0)]));
    v.push(p(g, "tb_sky", "Sky Gazing — Mixing Mind and Space (Dzogchen)", "Eyes open, outdoors · 20 min", &[200.0], &[0.45], "none", "binaural", "Traditional",
      "<p>In <b>sky gazing</b>, a classic Dzogchen practice, you gaze into a clear sky and let awareness mix with space until there is no sense of inside and outside, which is said to reveal the mind's own open, luminous nature. Teachers advise practising with your back to the sun, in the morning or evening, with relaxed eyes. <b>Never look at the sun.</b> Some advanced Tibetan practices involve the sun, but staring at it can permanently damage your eyes, so they are not included here.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to sky gazing, a Dzogchen practice of mixing mind and space. Sit outside, or at a window with a wide view of open sky. Keep the sun behind you, and never look at the sun. Keep your eyes open and relaxed."),
           st("Gaze", 6.0, &[9.0], "Let your gaze rest in the open sky without focusing on anything: not clouds, not birds, not the horizon. Just space. Blink whenever you need to."),
           st("Mixing", 8.0, &[8.0], "With each breath out, let your awareness flow out and mix with the sky. Inside and outside become one open space. Thoughts drift through like clouds, and leave the sky untouched."),
           st("Rest", 4.0, &[7.83], "Rest as the sky itself: vast, clear and unchanging. Nothing to hold, nothing to push away.")]));

    // ================= Buddhist meditation (Theravada, Zen, Pure Land, Nichiren, Shingon) =================
    let g = "Buddhist Meditation";
    v.push(p(g, "bu_metta", "Loving-Kindness (Mettā) Phrases", "Fredrickson 2008 · 20 min", &[200.0], &[0.5], "pink", "binaural", "Research",
      "<p><b>Mettā</b> (loving-kindness) meditation offers goodwill in widening circles: to yourself, a benefactor, a friend, a neutral person, a difficult person and all beings, an order set out by Buddhaghosa in the 5th-century <i>Path of Purification</i> and taught in the West by Sharon Salzberg. In a seven-week trial, <b>Fredrickson et al. (2008, <i>Journal of Personality and Social Psychology</i>)</b> found it increased daily positive emotions, which in turn built personal resources and life satisfaction, and <b>Hutcherson, Seppala &amp; Gross (2008, <i>Emotion</i>)</b> found that even a few minutes increased feelings of connection to strangers.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to loving kindness, metta. Sit comfortably, close your eyes, and place a hand on your heart if you like. You will offer simple wishes of goodwill, first to yourself, then outward, step by step."),
           st("Yourself", 3.0, &[8.0], "Begin with yourself. Silently repeat: may I be safe. May I be happy. May I be healthy. May I live with ease. Let the meaning soak in, even if the feeling is faint."),
           st("A benefactor", 3.0, &[7.5], "Picture someone who has been kind to you. Wish them the same: may you be safe, may you be happy, may you be healthy, may you live with ease."),
           st("Friend and stranger", 4.0, &[7.0], "Now a dear friend, and then someone neutral, perhaps a person you passed in the street. Offer each the same wishes."),
           st("A difficult person", 3.0, &[6.5], "If it feels manageable, bring to mind someone you find difficult, and offer the wishes as far as you honestly can. If it's too hard, return to a friend."),
           st("All beings", 5.0, &[6.0], "Finally, let goodwill spread in every direction, to all beings everywhere, without exception. May all beings be safe, happy, healthy and at ease.")]));
    v.push(p(g, "bu_brahma", "The Four Immeasurables — Boundless Radiation", "Early sutta method · 24 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>four immeasurables</b> or <i>brahmavihāras</i> (\"divine abodes\") are loving-kindness, compassion, appreciative joy and equanimity. In the early Pali suttas they are not repeated as phrases but <b>radiated</b>: one dwells pervading one direction with a mind full of loving-kindness, then the second, the third and the fourth, above, below, around and everywhere, with a mind abundant, exalted and immeasurable. Many Theravada teachers regard this boundless form as a path into deep absorption.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the four immeasurables: loving kindness, compassion, appreciative joy and equanimity. Sit comfortably and close your eyes. In the early texts they are radiated in every direction, without limit."),
           st("Loving-kindness", 5.0, &[8.0], "Let a warm feeling of goodwill arise in your heart. Radiate it to the front, as far as you can imagine, then to the right, behind you, and to the left. Then above, below, and everywhere, without limit."),
           st("Compassion", 5.0, &[7.0], "Now compassion, the heart that is moved by suffering and wishes it to end. Radiate it in the same way: front, right, back, left, above, below, everywhere."),
           st("Appreciative joy", 5.0, &[6.5], "Now appreciative joy, gladness at the happiness and good fortune of others. Let it shine out in every direction, boundless."),
           st("Equanimity", 5.0, &[6.0], "Now equanimity, a calm, balanced heart that cares for all equally, without clinging or pushing away. Fill the whole world with it."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "bu_anapana", "Ānāpānasati — The Sixteen Steps of Breathing", "Majjhima Nikāya 118 · 30 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>In the <b>Ānāpānasati Sutta</b> (Majjhima Nikāya 118) the Buddha teaches mindfulness of breathing in <b>sixteen steps</b>, four sets of four: the body (long and short breaths, the whole body, calming the body), feelings (rapture, happiness, mental activity, calming it), the mind (knowing, gladdening, steadying and releasing it) and insight (impermanence, fading, cessation, letting go). The breath is never controlled, only known. Buddhadāsa Bhikkhu and Thich Nhat Hanh made this sequence widely known.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to anapanasati, mindfulness of breathing in sixteen steps. Sit upright and close your eyes. Never control the breath; only know it. The guidance moves through four sets of four steps."),
           st("Body", 7.0, &[9.0], "The first four steps: the body. Breathing in long, know it is long; breathing short, know it is short. Then breathe sensitive to the whole body. Then breathe, calming the body."),
           st("Feelings", 7.0, &[7.0], "The second four: feelings. Breathe sensitive to rapture, if it is there, then to happiness. Then sensitive to how feelings move the mind, and breathe, calming them."),
           st("Mind", 7.0, &[6.5], "The third four: the mind. Breathe sensitive to the mind itself. Then gladdening it. Then steadying it. Then releasing it from whatever binds it."),
           st("Insight", 6.0, &[6.0], "The last four: insight. Breathe focusing on impermanence, how everything changes. Then on fading away, then on cessation, and finally on letting go."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "bu_noting", "Mahasi Noting — Rising, Falling", "Burmese insight method · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Mahasi Sayadaw</b> (1904–1982), a Burmese monk, developed the <b>noting</b> method of insight (<i>vipassanā</i>) meditation, which spread worldwide through teachers such as Joseph Goldstein and Jack Kornfield. You follow the rising and falling of the abdomen with a light mental note, and note anything else that takes your attention (\"thinking\", \"hearing\", \"itching\") before returning. On intensive retreats the noting becomes fast and precise, revealing how every experience arises and passes away.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to noting practice in the style of Mahasi Sayadaw. Sit upright and close your eyes. Rest a hand on your belly if it helps you feel the breath."),
           st("Rising, falling", 6.0, &[9.0], "Feel the belly move with the breath. As it rises, make a soft mental note: rising. As it falls: falling. Keep the note light, with most of your attention on the sensation itself."),
           st("Noting", 10.0, &[7.0], "When something else takes your attention, note it, then return to rising and falling. Thinking, thinking. Hearing, hearing. Itching. Restless. Note it once or twice, and let it go."),
           st("Moment to moment", 5.0, &[6.0], "Let the noting become quick and precise, moment to moment, catching each sensation as it arises and passes away."),
           st("Rest", 2.0, &[8.0], "Stop noting now. Sit quietly, simply aware.")]));
    v.push(p(g, "bu_mu", "Zen Koan — Mu", "The Gateless Gate, case 1 · 30 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The first case of the <b>Gateless Gate</b> (<i>Wumenguan</i>, 1228): a monk asked Master <b>Zhaozhou</b>, \"Does a dog have Buddha-nature?\" Zhaozhou answered, \"<b>Mu</b>\" (\"no\", or \"not\"). In Rinzai Zen this is often the first koan. You don't solve it with thought; you become one with it. Wumen's commentary urges you to rouse \"great doubt\" with your whole body, as if you had swallowed a red-hot iron ball you can neither spit out nor swallow. Koans are traditionally worked on with a teacher in private interview.</p>",
      vec![st("Settle", 2.0, &[11.0], "Welcome to koan practice with mu. A monk asked Master Zhaozhou: does a dog have Buddha nature? Zhaozhou answered: mu. Sit upright, eyes lowered and half open, and breathe from your belly."),
           st("Breathe mu", 8.0, &[10.0], "On each long breath out, silently sound mu, from the belly, to the very end of the breath. Don't think about what it means. Just mu."),
           st("Great doubt", 12.0, &[7.0], "Put your whole self into mu, as if you had swallowed a red hot iron ball you can neither spit out nor swallow. Mu. Let it become a burning question with no words."),
           st("Only mu", 6.0, &[6.0], "Let there be no one asking. Only mu, breathing, sitting, filling the whole universe."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "bu_shikantaza", "Shikantaza — Just Sitting (Dōgen)", "Sōtō Zen, one period · 40 min", &[200.0], &[0.45], "pink", "binaural", "Traditional",
      "<p><b>Shikantaza</b> (\"just sitting\") is the heart of Sōtō Zen, taught by <b>Eihei Dōgen</b> (1200–1253), who brought the lineage from China to Japan. His <i>Fukanzazengi</i> (1227) gives the instructions: sit upright, ears in line with the shoulders and nose with the navel, eyes open and lowered, hands in the cosmic mudra, and \"think not-thinking\", which is \"non-thinking\". There is no object and no goal; sitting itself is awakening. Sōtō practitioners face a wall, and a period traditionally lasts one stick of incense, about 40 minutes.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to shikantaza, just sitting, as taught by Dogen. Sit facing a plain wall, eyes open and lowered, back upright, hands resting in the cosmic mudra, thumbs lightly touching. There is nothing to attain."),
           st("Posture", 4.0, &[10.0], "Settle the posture: ears in line with the shoulders, nose in line with the navel. Let the breath be natural. Sway gently left and right, then come to stillness."),
           st("Just sitting", 16.0, &[8.0], "Now just sit. Don't hold on to anything, and don't push anything away. Let sounds, thoughts and sensations come and go, while you simply sit."),
           st("Non-thinking", 16.0, &[6.5], "Think not thinking. How? Non thinking. Not trying to stop thought, not following it. Just this sitting, wholeheartedly."),
           st("End of the period", 2.0, &[10.0], "The period ends. Bow, stretch gently, and rise slowly.")]));
    v.push(with_tex(p(g, "bu_nembutsu", "Pure Land Nembutsu — Namu Amida Butsu", "Chanting with the wooden fish · 20 min", &[200.0], &[0.4], "none", "binaural", "Traditional",
      "<p><b>Pure Land Buddhism</b>, the largest Buddhist tradition in East Asia, centres on <b>Amida</b> (Amitābha), the Buddha of Infinite Light, who vowed to bring everyone who calls his name to his Pure Land. In Japan <b>Hōnen</b> (1133–1212) and <b>Shinran</b> (1173–1263) taught the <b>nembutsu</b>, <i>Namu Amida Butsu</i> (in Chinese, <i>Namo Amituofo</i>), as the whole of practice. The <i>Contemplation Sutra</i> begins its sixteen visualisations with the setting sun in the west. A wooden fish (<i>mokugyo</i>) keeps the chanting rhythm here.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the nembutsu, the heart practice of Pure Land Buddhism, calling on Amida, the Buddha of infinite light. Sit comfortably, with your palms together or resting in your lap.").tex(0.0),
           st("Setting sun", 4.0, &[8.0], "Picture the sun setting in the west, round and red gold, like a drum hanging in the sky, the first visualisation of the Contemplation Sutra. Hold the image until it is clear.").tex(0.0),
           st("Chanting", 10.0, &[6.5], "Now chant with the wooden fish, aloud or softly: namu amida butsu. Or in Chinese: namo amituofo. Let voice, ear and heart move together."),
           st("Entrusting", 4.0, &[6.0], "Let the chanting slow to a whisper, then silence. In Shinran's teaching, the nembutsu is not your own effort but Amida's gift. Rest, entrusting yourself to the infinite light.").tex(0.0)]),
      tx("mokugyo", 0.6, 460.0, 0.75)));
    v.push(with_tex(p(g, "bu_daimoku", "Daimoku — Nam-myoho-renge-kyo (Nichiren)", "Rhythmic chanting · 20 min", &[200.0], &[0.4], "none", "binaural", "Traditional",
      "<p>The Japanese reformer <b>Nichiren</b> (1222–1282) taught that chanting the title of the <i>Lotus Sutra</i>, <b>Nam-myoho-renge-kyo</b> (\"devotion to the mystic law of the Lotus Sutra\"), is the direct way to awaken one's Buddha-nature. Called <i>daimoku</i>, it is the main practice of Nichiren Buddhism, including Nichiren Shū and Soka Gakkai International, and is usually chanted with the eyes open before a scroll called the Gohonzon. A wooden block keeps a brisk rhythm here.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to daimoku, the chant at the heart of Nichiren Buddhism: nam myoho renge kyo. Sit upright, palms together in front of your chest, with your eyes open and resting on a point before you.").tex(0.0),
           st("Chanting", 14.0, &[8.0], "Begin chanting with the wooden block, in a clear, steady voice: nam myoho renge kyo. Let the rhythm carry you, and let any wish or worry become part of the chant."),
           st("Deeper", 3.0, &[7.0], "Keep chanting, a little more inwardly now, feeling the vibration in your chest.").tex(0.7),
           st("Close", 1.0, &[8.0], "To close, chant nam myoho renge kyo three times, slowly, and bow.").tex(0.0)]),
      tx("mokugyo", 0.55, 520.0, 0.42)));
    let mut x = p(g, "bu_kasina", "Kasina — Coloured-Disc Concentration (Visuddhimagga)", "Gazing to the inner sign · 30 min", &[200.0], &[0.45], "pink", "binaural", "Traditional",
      "<p>The <b>kasiṇas</b> are ten meditation objects described in Buddhaghosa's <i>Path of Purification</i> (5th century): earth, water, fire, air, blue, yellow, red, white, light and space. For the earth kasina you make a disc of dawn-coloured clay about a span and four fingers across, set it about two and a half cubits away, and gaze at it, alternating with eyes closed, until its image appears clearly in the mind (the <b>learning sign</b>), and then a purer, brighter version, the <b>counterpart sign</b>, \"like the moon's disc coming out from behind a cloud\", which leads into absorption. The <b>🟠 gazing aid</b> shows a still coloured disc; it never flickers. Keep the screen brightness comfortable and blink normally.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to kasina meditation, from Buddhaghosa's Path of Purification. Open the kasina disc on this screen, or use a disc of colour of your own, about one and a half arm's lengths away at eye level. Sit upright and comfortable."),
           st("Gaze", 8.0, &[10.0], "Look at the disc with relaxed, steady eyes, blinking when you need to. Don't stare hard. Silently name its colour or element, red, red, or earth, earth, and let everything else fade."),
           st("Learning sign", 8.0, &[8.0], "Now close your eyes and look for the disc in your mind. When it fades, open your eyes and look at the disc again. Repeat until the image stays clear with your eyes closed."),
           st("Counterpart sign", 10.0, &[6.5], "With your eyes closed, rest with the inner image. Let it become brighter, smoother and purer than the real disc, like a polished mirror, or the moon coming out from behind a cloud. Don't grab at it."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]);
    x.visual = "kasina".into();
    v.push(x);
    let mut x = p(g, "bu_ajikan", "Ajikan — Moon Disc and the Letter A (Shingon)", "Kūkai's esoteric meditation · 20 min", &[200.0], &[0.45], "pink", "binaural", "Traditional",
      "<p><b>Ajikan</b> (\"contemplation of the letter A\") is the core meditation of <b>Shingon</b>, the esoteric Buddhism brought to Japan by <b>Kūkai</b> (Kōbō Daishi, 774–835). Before a scroll showing the Sanskrit letter A on a lotus inside a white full-moon disc, the practitioner breathes the sound \"ah\", visualises the moon disc in the heart, expands it to fill the universe and draws it back. In Shingon, A is the unborn source of all sounds and all things. The <b>🌕 gazing aid</b> shows a still moon disc for the gazing stage.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to ajikan, the meditation on the letter A, from Kukai's Shingon Buddhism. Traditionally you sit before a scroll of a white moon disc holding a lotus and the letter A. Open the moon disc on screen if you like, and sit with your eyes half open."),
           st("Breathing A", 4.0, &[9.0], "Breathe in, and as you breathe out, let a long, soft ahh flow out. In Shingon, A is the unborn source of all sounds and all things. Repeat a few times, then let the sound fall silent."),
           st("Moon disc", 6.0, &[7.5], "Gaze at the moon disc, cool, round, clear and radiant. Then close your eyes and see it in your chest, at the heart. Bring it back as clearly as you can."),
           st("Expand and return", 5.0, &[6.5], "Let the moon in your heart grow slowly, filling your body, then the room, then the whole universe, with the letter A shining within it. Then let it shrink back to the size of your heart."),
           st("Rest", 3.0, &[6.0], "Rest, with the moon and the letter A at your heart, and your mind as clear as the moon.")]);
    x.visual = "kasina:moon".into();
    v.push(x);

    // ================= Hindu and yogic meditation =================
    let g = "Hindu & Yogic Meditation";
    let mut x = p(g, "hy_trataka", "Trāṭaka — Steady Gazing (Hatha Yoga Pradīpikā)", "Talwadkar 2014 · 15 min", &[200.0], &[0.45], "pink", "binaural", "Research",
      "<p><b>Trāṭaka</b> is one of the six cleansing practices of hatha yoga. The <i>Hatha Yoga Pradīpikā</i> (15th century) describes it as gazing at a small object with an unwavering gaze until the eyes water, and says it should be kept as carefully as a golden casket. Usually the object is a candle flame, and its afterimage is then held at the brow. In a small randomised trial in older adults, <b>Talwadkar, Jagannathan &amp; Raghuram (2014, <i>International Journal of Yoga</i>)</b> reported better scores on several attention and memory tests after about a month of daily trāṭaka. The <b>🕯 gazing aid</b> shows a still flame that never flickers. Never strain your eyes, and if you use a real candle, never leave it unattended.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to trataka, the yogic practice of steady gazing. Open the flame on this screen, or place a real candle at arm's length and at eye level, in a dim room without draughts. Sit upright and still."),
           st("Gazing", 4.0, &[10.0], "Gaze at the flame, at its brightest point, steadily and without moving your eyes. Blink as little as is comfortable, but never strain. If your eyes water, that's traditional; close them whenever you need."),
           st("Inner flame", 3.0, &[7.83], "Now close your eyes, and see the afterimage of the flame at the space between your eyebrows. Hold it steady. When it fades, open your eyes."),
           st("Second gaze", 3.0, &[8.0], "Gaze at the flame again, steady and still, letting your mind become as still as your eyes."),
           st("Inner light", 3.0, &[6.5], "Close your eyes once more. Hold the inner light as long as it stays, then rest in the darkness behind it.")]);
    x.visual = "trataka".into();
    v.push(x);
    v.push(p(g, "hy_soham", "So'ham — The Breath's Own Mantra (Ajapa Japa)", "Natural breath mantra · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>Yoga and tantra texts call <b>So'ham</b> (\"I am That\") the <i>ajapa</i>, the \"unrecited\" mantra, because the breath is said to repeat it by itself about 21,600 times a day: <i>so</i> with the in-breath and <i>ham</i> with the out-breath, as most modern teachers give it (some lineages reverse it as <i>hamsa</i>, \"swan\"). You don't say the mantra; you notice it in the breath, then rest in the pause where the breath turns.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to so hum, the breath's own mantra. Yoga texts say the breath repeats it by itself, about twenty one thousand six hundred times a day. Sit comfortably, close your eyes, and let your breathing be natural."),
           st("Listening", 4.0, &[8.0], "Listen to the breath. As it comes in, hear so, silently. As it flows out, hear hum. You are not saying it; you are noticing that the breath already does."),
           st("So'ham", 10.0, &[6.5], "So, the in breath. Hum, the out breath. So hum means I am that. Let mantra and breath become one effortless movement."),
           st("The gaps", 4.0, &[6.0], "Now notice the tiny pause where the in breath turns into the out breath, and the out breath into the in breath. Rest in those gaps, where the mantra falls silent.")]));
    v.push(with_tex(p(g, "hy_vbt", "Vijñāna Bhairava — Five Doorways (Kashmir Shaivism)", "Five dhāraṇās · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Vijñāna Bhairava Tantra</b>, a Kashmiri text from around the 8th century, is a dialogue in which Bhairava (Shiva) gives the goddess <b>112 dhāraṇās</b>, short ways into pure awareness through the breath, sound, space, the body, emotion and the edges of sleep. Paul Reps brought them to the West in <i>Zen Flesh, Zen Bones</i> (1957). This session opens five: the turning of the breath, a sound fading into silence (a bell rings for this one), the empty body, the open sky, and the threshold of sleep.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to five doorways from the Vijnana Bhairava Tantra, an ancient text from Kashmir with a hundred and twelve ways into pure awareness. Sit comfortably and close your eyes. We will open five of them, one after another.").tex(0.0),
           st("The turning breath", 5.0, &[8.0], "First doorway: the turning of the breath. Notice where the in breath ends before the out breath begins, and where the out breath ends before the next one comes. Rest in that turning point.").tex(0.0),
           st("Fading sound", 5.0, &[7.0], "Second doorway: sound fading into silence. Listen to the bell. Follow its sound as it fades, all the way, until there is only silence. Enter the silence."),
           st("The empty body", 4.0, &[6.5], "Third doorway: the empty body. Imagine your body completely hollow, a vessel of clear space with nothing solid inside, enclosed by a thin skin, and then not even that.").tex(0.0),
           st("Open sky", 4.0, &[6.0], "Fourth doorway: the open sky. Imagine a vast, cloudless sky, and let your inner gaze rest in it, without centre and without edge.").tex(0.0),
           st("Edge of sleep", 3.0, &[5.5], "Fifth doorway, for tonight: the edge of sleep. When the outer world has faded but sleep has not yet come, that moment is a door. For now, rest in the stillness these doorways have opened.").tex(0.0),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT).tex(0.0)]),
      tx("breathbell", 0.6, 432.0, 20.0)));
    v.push(p(g, "hy_bija", "Chakra Seed Sounds — LAM VAM RAM YAM HAM OM", "Bīja mantras, base to crown · 22 min", &[256.0], &[0.5], "none", "binaural", "Traditional",
      "<p>In tantric yoga each <b>chakra</b> has a <b>bīja</b> (\"seed\") syllable: <b>LAM</b> for the root (mūlādhāra), <b>VAM</b> for the sacral centre (svādhiṣṭhāna), <b>RAM</b> for the navel (maṇipūra), <b>YAM</b> for the heart (anāhata), <b>HAM</b> for the throat (viśuddha) and <b>OM</b> for the brow (ājñā), with silence at the crown (sahasrāra). These correspondences come from texts such as Pūrṇānanda's <i>Ṣaṭ-cakra-nirūpaṇa</i> (1577), which Arthur Avalon translated in <i>The Serpent Power</i> (1919). The carrier tone climbs a just-intonation scale from C to B as you rise, a modern musical touch. Chant gently; there is no need to force or \"awaken\" anything.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the chakra seed sounds of tantric yoga. Sit upright with your spine tall and your eyes closed. You will chant one seed sound at each centre, from the base of the spine to the crown, feeling each one vibrate where it belongs."),
           st("Root · LAM", 3.0, &[8.0], "The root centre, at the base of the spine: lam. Chant it slowly, lammm, and feel the vibration low in the body, steady like the earth."),
           st("Sacral · VAM", 3.0, &[7.5], "The sacral centre, below the navel: vam. Let the sound flow, like water.").car(&[288.0]),
           st("Navel · RAM", 3.0, &[7.0], "The navel centre: ram. Feel warmth and strength gather there, like fire.").car(&[320.0]),
           st("Heart · YAM", 3.0, &[6.5], "The heart centre: yam. Let the sound open the chest, light as air.").car(&[341.33]),
           st("Throat · HAM", 3.0, &[6.0], "The throat centre: ham. Feel the throat vibrate, clear and spacious as the sky.").car(&[384.0]),
           st("Brow · OM", 3.0, &[6.0], "The brow centre, between the eyebrows: om. Let it resonate inside your head.").car(&[426.67]),
           st("Crown · silence", 2.0, &[5.5], "The crown, at the top of the head, has no sound. Sit in silence, and feel the whole column of your body vibrating, from base to crown.").car(&[480.0])]));
    v.push(p(g, "hy_shiva", "Om Namah Shivaya — Japa with 108 Beads", "Five-syllable mantra · 18 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Om Namaḥ Śivāya</b> (\"I bow to Shiva\") is the <i>Pañcākṣara</i>, the five-syllable mantra of Shaivism, found in the <i>Śrī Rudram</i> of the Yajur Veda. In <b>japa</b> it is repeated on a <b>mala</b> of 108 beads, held in the right hand and moved with the thumb over the middle finger; at the large guru bead you turn the mala around rather than crossing it. Repetition moves from spoken, to whispered, to purely mental japa, which the tradition considers the most powerful.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to japa with the five syllable mantra, om namah shivaya. Sit upright and close your eyes. If you have a mala, hold it in your right hand, starting at the bead next to the large guru bead."),
           st("First round", 7.0, &[8.0], "Begin your japa: om namah shivaya, one repetition for each bead, moving the beads over your middle finger with your thumb. Say it aloud, whisper it, or say it silently."),
           st("Second round", 7.0, &[6.5], "At the guru bead, don't cross it. Turn the mala around and continue the other way, more inwardly now, letting the mantra sink from the lips into the mind."),
           st("Silence", 2.0, &[6.0], "Let the mantra go, and sit in the silence it leaves behind, which the tradition calls Shiva, pure awareness.")]));
    v.push(p(g, "hy_gayatri", "Gāyatrī Mantra — Sandhyā at Dawn or Dusk", "Rig Veda 3.62.10 · 15 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Gāyatrī mantra</b> (Rig Veda 3.62.10), attributed to the sage Viśvāmitra, asks <b>Savitṛ</b>, the divine light of the sun, to illumine the mind: <i>Om bhūr bhuvaḥ svaḥ, tat savitur vareṇyaṃ, bhargo devasya dhīmahi, dhiyo yo naḥ pracodayāt</i>. It is recited at the <b>sandhyās</b>, the junctions of the day at dawn, noon and dusk, traditionally 108 times. In some orthodox families it is given only at initiation (<i>upanayana</i>); many modern teachers share it with everyone.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Gayatri mantra, a verse from the Rig Veda praying to the divine light of the sun. It is chanted at dawn, noon and dusk. Sit facing east in the morning, or west in the evening, and close your eyes."),
           st("The verse", 3.0, &[9.0], "The mantra is: om bhur bhuva swaha, tat savitur varenyam, bhargo devasya dhimahi, dhiyo yo nah prachodayat. It means, roughly: we meditate on the glorious light of the divine sun; may it inspire our minds."),
           st("Chanting", 8.0, &[7.0], "Chant it slowly, aloud or silently, again and again, with full attention. Picture the light of the rising or setting sun filling your mind."),
           st("Light", 2.0, &[6.0], "Let the chanting stop, and rest in the light the mantra asks for.")]));
    v.push(with_tex(p(g, "hy_mahamantra", "Hare Krishna Mahā-mantra — Japa and Kīrtan", "With hand cymbals · 20 min", &[200.0], &[0.35], "none", "binaural", "Traditional",
      "<p>The <b>mahā-mantra</b>, <i>Hare Kṛṣṇa Hare Kṛṣṇa, Kṛṣṇa Kṛṣṇa Hare Hare, Hare Rāma Hare Rāma, Rāma Rāma Hare Hare</i>, appears in the <i>Kali-santaraṇa Upaniṣad</i>. The Bengali saint <b>Chaitanya Mahāprabhu</b> (1486–1534) spread it through ecstatic group singing (<b>kīrtan</b>), and A. C. Bhaktivedanta Swami Prabhupada brought it to the West in 1965–66, founding ISKCON. Devotees chant it quietly on beads (<i>japa</i>) and sing it together with drums and small hand cymbals (<i>kartals</i>), whose ching-ching-chik pattern plays here.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the maha mantra, the great chant of the Hare Krishna tradition, spread by Chaitanya Mahaprabhu five centuries ago. Sit comfortably. You can chant aloud, and clap or play along with the cymbals.").tex(0.0),
           st("Japa", 5.0, &[8.0], "Begin softly, as japa: hare krishna, hare krishna, krishna krishna, hare hare, hare rama, hare rama, rama rama, hare hare. Hear each word clearly as you say it.").tex(0.35),
           st("Kirtan", 9.0, &[7.0], "Now let it become kirtan. Sing it out with the rhythm of the cymbals, and let your voice rise and fall. Let yourself be carried by the sound and the joy of it. Sway if you like."),
           st("Quiet", 4.0, &[6.0], "Let the chant slow down and grow softer, to a whisper, then silence. Sit in the feeling it leaves.").tex(0.0)]),
      tx("kartal", 0.5, 2350.0, 0.42)));
    v.push(p(g, "hy_antarmouna", "Antar Mouna — Inner Silence (Satyananda)", "Bihar School of Yoga · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Antar mouna</b> (\"inner silence\") is a stage-by-stage meditation taught by <b>Swami Satyananda Saraswati</b> of the Bihar School of Yoga, drawing on tantric <i>pratyāhāra</i>, withdrawal of the senses. You become a witness, first of outer sounds, then of spontaneous thoughts; then you create and dismiss thoughts at will, watch and let go of them, and finally rest in the gaps where no thought arises. Satyananda described six stages in <i>Meditations from the Tantras</i> (1974); this session covers the first five.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to antar mouna, inner silence, from the Bihar School of Yoga. Sit comfortably and close your eyes. You will become a witness, first of sounds, then of thoughts, until silence opens by itself."),
           st("Sounds", 4.0, &[9.0], "Stage one: the senses. Become aware of the sounds around you, far and near. Move from sound to sound, without naming or judging. Just witness."),
           st("Spontaneous thoughts", 5.0, &[8.0], "Stage two: spontaneous thoughts. Turn your attention inward and watch whatever thoughts arise, without stopping them or following them, like watching a film."),
           st("Chosen thought", 4.0, &[7.0], "Stage three: create a thought on purpose, any theme you like, and think about it for a while. Then throw it out deliberately, and rest."),
           st("Watch and let go", 4.0, &[6.5], "Stage four: watch spontaneous thoughts again. As each one arises, look at it, then let it go, one by one, until fewer and fewer come."),
           st("Thoughtlessness", 4.0, &[6.0], "Stage five: thoughtlessness. Notice the gaps where no thought is present, and rest in that silence. If a thought comes, let it pass, and return to the silence."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "hy_mandukya", "AUM — The Four States (Māṇḍūkya Upaniṣad)", "Waking, dream, sleep, turīya · 24 min", &[150.0, 400.0], &[0.7, 0.25], "pink", "binaural", "Traditional",
      "<p>The <b>Māṇḍūkya Upaniṣad</b>, only twelve verses long, teaches that the syllable <b>AUM</b> is everything: <b>A</b> is the waking state, <b>U</b> the dreaming state, <b>M</b> deep dreamless sleep, and the <b>silence</b> after it is <b>turīya</b>, \"the fourth\", pure consciousness present in all three. Gauḍapāda's commentary made it a founding text of Advaita Vedanta. The beats follow the states: alpha for waking, theta for dreaming, delta for deep sleep, and delta with a faint 40 Hz gamma layer for turīya, a modern touch suggesting deep stillness with bright awareness.</p>",
      vec![st("Settle", 2.0, &[10.0, 14.0], "Welcome to the meditation of the Mandukya Upanishad, which teaches that the syllable aum holds the four states of consciousness. Sit upright and close your eyes. You will chant each sound, then rest in the silence after it."),
           st("A · waking", 5.0, &[10.0, 14.0], "The sound ah, the first part of aum, is the waking state, aware of the outer world. Chant a long ahh, and feel the world around you."),
           st("U · dreaming", 5.0, &[6.0, 10.0], "The sound oo is the dreaming state, aware of an inner world of images. Chant a long ooo, and let images come and go."),
           st("M · deep sleep", 5.0, &[2.0, 6.0], "The sound mm is deep, dreamless sleep, where everything rests as one. Hum a long mmm, and feel the stillness gather."),
           st("Silence · turīya", 6.0, &[2.0, 40.0], "The silence after aum is the fourth, turiya: the pure awareness present in waking, dreaming and sleep alike. Chant aum once, whole, and rest in the silence that follows."),
           st("Return", 1.0, &[10.0, 14.0], C_RETURN_SHORT)]));

    // ================= Sufi mysticism =================
    let g = "Sufi Mysticism";
    v.push(p(g, "su_sema", "Seated Sema — Turning Within (Mevlevi-inspired)", "The sound turns, you stay still · 20 min", &[220.0], &[0.55], "none", "spatial", "Traditional",
      "<p>The <b>sema</b> of the <b>Mevlevi</b> order, founded in Konya after the death of the poet <b>Jalāl al-Dīn Rūmī</b> (1207–1273), is a ceremony in which dervishes turn to the reed flute (<i>ney</i>) and drums, right palm raised to receive from God and left palm lowered to pass it to the earth, silently remembering God with each turn. UNESCO lists it as intangible cultural heritage. Whirling carries a real risk of falls, so here you stay <b>seated</b> while the tone sweeps slowly from ear to ear on headphones (spatial mode), quickening and then coming to rest.</p>",
      vec![st("Settle", 2.0, &[0.05], "Welcome to the seated sema, inspired by the turning ceremony of the Mevlevi order of Rumi. The dervishes whirl, but here you turn only in attention, safely seated. Sit upright, cross your arms over your chest, hands on your shoulders, and bow your head slightly."),
           st("Unfolding", 3.0, &[0.08], "Slowly unfold your arms. Raise your right hand, palm up, to receive from heaven, and lower your left hand, palm down, to give to the earth. Rest your hands wherever is comfortable."),
           st("Turning", 8.0, &[0.15], "Listen as the sound sweeps around you, and let your attention turn with it, around the still point of your heart. With each turn, silently remember: Allah, or simply, love."),
           st("Deeper turning", 4.0, &[0.22], "The turning quickens. Let go of the one who turns. There is only the turning, and the stillness at its centre."),
           st("Stillness", 3.0, &[0.03], "The turning slows to stillness. Fold your arms across your chest again, and sit in silence, as the dervishes do at the end of the sema.")]));
    v.push(with_tex(p(g, "su_muraqaba", "Murāqaba — Watching over the Heart (Naqshbandi)", "Heart attention · 25 min", &[200.0], &[0.4], "brown", "binaural", "Traditional",
      "<p><b>Murāqaba</b> (\"watching over\") is the Sufi practice of attentive waiting: sitting with head bowed and eyes closed, the heart turned toward God, watching it closely and waiting for divine grace (<i>fayḍ</i>) to enter. It is central to the <b>Naqshbandi</b> order, named after Bahāʾ al-Dīn Naqshband (1318–1389) and known for silent remembrance of the heart. A soft heartbeat plays at about sixty beats a minute to help you rest your attention in the chest.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to muraqaba, the Sufi practice of watchful waiting, as kept in the Naqshbandi order. Sit with your head slightly bowed and your eyes closed. Muraqaba means to watch over, and here you watch over your heart.").tex(0.0),
           st("The heart", 5.0, &[8.0], "Bring your attention to your heart, a little to the left in your chest. Feel it beating. Let the breath be quiet. Picture the heart turned toward God, like a flower turned toward the sun."),
           st("Waiting", 12.0, &[6.0], "Now simply wait, watching the heart with total, patient attention, like a cat watching a mouse hole. Hold the intention that grace will come into it. Don't force anything; keep watching and waiting."),
           st("Presence", 4.0, &[5.5], "If a sense of presence, warmth or light comes, rest in it quietly. If nothing comes, the watching itself is the practice.").tex(0.5),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT).tex(0.0)]),
      tx("heartbeat", 0.45, 52.0, 1.0)));
    v.push(p(g, "su_lataif", "The Laṭāʾif — Five Subtle Centres (Mujaddidi)", "Remembrance in the chest · 25 min", &[200.0], &[0.45], "pink", "binaural", "Traditional",
      "<p>The <b>laṭāʾif</b> (\"subtle centres\") are organs of spiritual perception taught in the <b>Naqshbandi-Mujaddidi</b> line of Shaykh <b>Aḥmad Sirhindī</b> (1564–1624). The disciple awakens each one by silently remembering the divine name <i>Allāh</i> in it: <b>qalb</b> (heart), <b>rūḥ</b> (spirit), <b>sirr</b> (secret), <b>khafī</b> (hidden) and <b>akhfā</b> (most hidden). The positions most often taught are below and above the left and right breast and the centre of the chest, with lights of yellow, red, white, black and green, but teachers differ, so treat these as a guide.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the lataif, the subtle centres of the Naqshbandi Mujaddidi tradition. Sit with your eyes closed. You will rest your attention on five points in the chest, silently remembering the divine name, Allah, at each one. Teachers differ on the places and colours, so treat these as a guide."),
           st("Qalb · heart", 4.0, &[8.0], "The heart, qalb, two finger widths below the left breast. Rest your attention there, and let Allah, Allah be remembered in it. Its light is said to be yellow."),
           st("Rūḥ · spirit", 4.0, &[7.5], "The spirit, ruh, two finger widths below the right breast. Remember Allah there. Its light is said to be red."),
           st("Sirr · secret", 4.0, &[7.0], "The secret, sirr, two finger widths above the left breast. Remember Allah there. Its light is white."),
           st("Khafī · hidden", 4.0, &[6.5], "The hidden, khafi, two finger widths above the right breast. Remember Allah there. Its light is black, the luminous darkness of night."),
           st("Akhfā · most hidden", 4.0, &[6.0], "The most hidden, akhfa, in the centre of the chest. Remember Allah there. Its light is green. Then feel all five centres remembering together."),
           st("Return", 3.0, &[8.0], "Rest in the whole chest, filled with remembrance. Then slowly return, and open your eyes.")]));
    v.push(p(g, "su_breaths", "Elemental Purification Breaths (Hazrat Inayat Khan)", "Earth, water, fire, air · 15 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Hazrat Inayat Khan</b> (1882–1927), an Indian musician and Sufi of the Chishti order, brought Sufism to the West in 1910. His <b>purification breaths</b> work with the four elements, five gentle breaths each, changing between nose and mouth: earth (in and out through the nose), water (in through the nose, out through the mouth), fire (in through the mouth, out through the nose) and air (in and out through the mouth). They are still taught in the Inayatiyya and the Sufi Ruhaniat. Breathe slowly; this is not fast breathing.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the elemental purification breaths of Hazrat Inayat Khan. Stand or sit upright, shoulders relaxed. You will take five slow breaths for each of the four elements. Breathe gently, never straining."),
           st("Earth", 3.0, &[9.0], "Earth. Breathe in through the nose and out through the nose, five times. Breathing in, draw up the strength of the earth. Breathing out, let what is heavy sink into the ground."),
           st("Water", 3.0, &[8.0], "Water. Breathe in through the nose and out through the mouth, five times. Feel the breath flow down over you like a cool waterfall, washing you clean."),
           st("Fire", 3.0, &[7.0], "Fire. Breathe in through the mouth and out through the nose, five times. Feel warmth and light gather in the heart, and radiate from it."),
           st("Air", 3.0, &[6.5], "Air. Breathe in through the mouth and out through the mouth, five times. Feel yourself becoming light and open, as if a breeze blew right through you."),
           st("Ether", 1.0, &[7.83], "Now breathe naturally. Let the breath become so fine it feels like ether, the space in which everything lives.")]));
    v.push(p(g, "su_names", "Wazīfa — The Beautiful Names (Asmāʾ al-Ḥusnā)", "Repeating divine names · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>A <b>wazīfa</b> is the regular repetition of one or more of the <b>ninety-nine Beautiful Names</b> of God (<i>al-asmāʾ al-ḥusnā</i>), each a quality the reciter hopes to reflect: <i>al-Laṭīf</i> the Subtle, <i>ar-Raḥmān</i> the Compassionate, <i>an-Nūr</i> the Light, <i>al-Wadūd</i> the Loving. Sufi teachers often give particular names to particular students. The session ends with <i>Hū</i>, the divine pronoun (\"He\"), breathed out softly.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to wazifa, the Sufi practice of repeating the beautiful names of God. Sit comfortably, close your eyes, and place your right hand over your heart if you like. You will repeat four names, one at a time."),
           st("Yā Laṭīf", 4.0, &[8.0], "Ya Latif, the Subtle, the Gentle. Repeat it softly, again and again: ya latif. Feel a gentle subtlety enter your breath."),
           st("Yā Raḥmān", 4.0, &[7.0], "Ya Rahman, the Compassionate. Ya rahman. Let mercy flow into you, and from you, like warm rain."),
           st("Yā Nūr", 4.0, &[6.5], "Ya Nur, the Light. Ya nur. Let light fill your heart and spread through your whole body."),
           st("Yā Wadūd", 4.0, &[6.0], "Ya Wadud, the Loving. Ya wadud. Rest in love, both given and received."),
           st("Hū", 2.0, &[6.0], "Let the names dissolve into hu, the divine breath, breathed out softly: huuu. Then rest in silence.")]));

    // ================= Jewish mysticism and Kabbalah =================
    let g = "Jewish Mysticism & Kabbalah";
    v.push(p(g, "jk_shema", "Shema — Echad, the Oneness Meditation", "Deuteronomy 6:4 · 15 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Shema</b>, <i>Shema Yisrael, Adonai Eloheinu, Adonai Echad</i> (\"Hear, O Israel: the Lord is our God, the Lord is One\", Deuteronomy 6:4), is the central declaration of Jewish faith, said morning and evening and again at bedtime. The Talmud (<b>Berakhot 13b</b>) tells how Rabbi Judah the Prince covered his eyes for the first verse, and teaches that one should draw out the last word, <b>echad</b> (\"one\"), long enough to proclaim God sovereign above, below and in the four directions. Kabbalists and Hasidic teachers made this a meditation on divine unity (<i>yiḥud</i>). The second line, <i>Barukh shem kevod malkhuto le'olam va'ed</i>, is traditionally whispered.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Shema, the central declaration of Jewish faith, said morning, evening and at bedtime. Sit comfortably. For the first line you will cover your eyes with your right hand, as the Talmud describes."),
           st("Hear", 3.0, &[8.0], "Cover your eyes with your right hand, and say slowly: Shema Yisrael, Adonai Eloheinu, Adonai echad. Hear, O Israel, the Lord is our God, the Lord is One."),
           st("Echad", 5.0, &[6.0], "Say the last word again, echad, one, and draw it out. As you hold it, feel the One filling the heavens above, the earth below, and the four directions around you. There is nothing else."),
           st("Barukh shem", 2.0, &[6.5], "Now whisper the response: barukh shem kevod malkhuto le'olam va'ed. Blessed be the name of his glorious kingdom for ever and ever."),
           st("Love", 2.0, &[7.0], "Then the next verse: you shall love the Lord your God with all your heart, with all your soul, and with all your might. Rest in that love."),
           st("Rest", 1.0, &[9.0], "Lower your hand, open your eyes, and rest in the oneness for a moment.")]));
    v.push(p(g, "jk_abulafia", "Abulafia — Chanting the Letters (Ecstatic Kabbalah)", "Ḥokhmat ha-tseruf · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Abraham Abulafia</b> (1240–after 1291), founder of prophetic or ecstatic Kabbalah, taught <i>ḥokhmat ha-tseruf</i>, \"the science of letter combination\": chanting Hebrew letters joined to each vowel, one sound to each out-breath with resting breaths between, each vowel paired with a slow movement of the head, in a quiet room, in clean clothes, often by candlelight. He compared it to music, with the letters as notes. Moshe Idel's <i>The Mystical Experience in Abraham Abulafia</i> (1988) describes the method. Abulafia combined the letters of divine names, which his own tradition guarded closely, so this session uses only <b>alef</b>, the silent first letter, with the vowels <i>o, a, e, i, u</i>.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to letter chanting in the way of Abraham Abulafia, the thirteenth century master of ecstatic Kabbalah. Sit in a quiet room, in clean clothes, as he advised. You will chant only the first Hebrew letter, alef, with its five vowels."),
           st("Breath and sound", 4.0, &[8.0], "Breathe in slowly. On a long out breath, sound alef with the first vowel: ohh. Then rest for one breath. Continue: ahh, ehh, eee, ooo, one sound for each out breath, with a quiet breath in between."),
           st("Combining", 8.0, &[6.5], "Keep cycling through the five sounds: oh, ah, eh, ee, oo. Let them become a slow, flowing chant. Abulafia compared this to music: the letters are the notes, and the soul is the instrument."),
           st("Silence", 4.0, &[6.0], "Let the chant fall silent, and see the letter alef in your mind, shining. Abulafia taught that after the chanting, new thoughts and light may flow into the mind. Watch whatever arises, without grasping."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "jk_tree", "The Tree of Life — Drawing Light through the Sefirot", "Ten sefirot, crown to feet · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>In Kabbalah, God's infinite light (<i>Ein Sof</i>) flows into the world through ten <b>sefirot</b>, arranged as the <b>Tree of Life</b>: <b>Keter</b> (crown), <b>Ḥokhmah</b> (wisdom), <b>Binah</b> (understanding), <b>Ḥesed</b> (loving-kindness), <b>Gevurah</b> (strength), <b>Tiferet</b> (beauty), <b>Netzaḥ</b> (endurance), <b>Hod</b> (splendour), <b>Yesod</b> (foundation) and <b>Malkhut</b> (kingdom, the <i>Shekhinah</i> or divine presence). Because humans are made in the divine image, the sefirot are also mapped onto the body. The map used here follows later Kabbalists' reading of the Zohar's <i>Pataḥ Eliyahu</i> passage; teachers differ, especially on where Malkhut sits (the feet or the mouth).</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Tree of Life, the ten sefirot of Kabbalah, through which, the Kabbalists teach, divine light flows into the world. Sit upright and close your eyes. You will draw that light down through your body, from the crown to the feet."),
           st("Keter", 2.0, &[9.0], "Keter, the crown, just above your head: pure will, beyond thought. Picture a brilliant white light resting there, like a crown of light."),
           st("Ḥokhmah · Binah", 3.0, &[8.0], "Let the light divide in two. Hokhmah, wisdom, on the right side of your head, a flash of insight. Binah, understanding, on the left, where insight takes shape."),
           st("Ḥesed · Gevurah", 3.0, &[7.5], "The light flows down into your arms. Hesed, loving kindness, in the right arm, giving without limit. Gevurah, strength and restraint, in the left arm, holding firm."),
           st("Tiferet", 3.0, &[7.0], "They meet in your heart, in Tiferet, beauty and harmony, where kindness and strength are balanced. Breathe there for a while."),
           st("Netzaḥ · Hod", 3.0, &[6.5], "The light flows into your hips and legs. Netzah, endurance, on the right. Hod, splendour and humility, on the left."),
           st("Yesod", 2.0, &[6.0], "Yesod, the foundation, in the pelvis, gathers all the streams of light into one."),
           st("Malkhut", 3.0, &[6.0], "Malkhut, the kingdom, at your feet: the Shekhinah, the divine presence in this world. Feel the light rest in the ground beneath you, and in everything around you."),
           st("Return to the source", 3.0, &[7.0], "Now let the light rise back up the tree, through the foundation, the heart and the crown, returning to its source."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "jk_ayin", "Ayin — Becoming Nothing (Hasidic Bittul)", "Self-nullification · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>Early Hasidic masters, above all <b>Dov Ber, the Maggid of Mezritch</b> (d. 1772), taught that a person must regard themselves as <b>ayin</b>, \"nothing\", to become a vessel for the divine: <i>bittul ha-yesh</i>, the dissolving of the separate \"something\". In Hebrew, <i>ani</i> (\"I\") and <i>ayin</i> (\"nothing\") are spelled with the same three letters. In Kabbalah, Ayin is also a name for the highest divine level, and Hasidism teaches that God renews creation every moment. Daniel Matt's essay <i>Ayin: The Concept of Nothingness in Jewish Mysticism</i> (1990) traces the idea.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the contemplation of ayin, nothingness, as taught by the Maggid of Mezritch and the early Hasidic masters. Sit comfortably and close your eyes. In Hebrew, ani, I, and ayin, nothing, are spelled with the same three letters."),
           st("Something", 4.0, &[8.0], "Notice the sense of being someone: your name, your story, your plans. This is yesh, somethingness. Don't fight it. Just see it clearly, as a thin layer over something far vaster."),
           st("Bittul", 6.0, &[6.5], "Now, little by little, let it go. Think of yourself as nothing, as the Maggid taught, a vessel emptied so the divine can flow through. With each out breath, let the I soften into ayin."),
           st("Ayin", 6.0, &[5.5], "Rest as nothing. No one to pray, nothing to achieve. Only the divine life flowing through everything, renewing the world in every moment."),
           st("Return", 2.0, &[9.0], "Gently let the sense of yourself return, lighter than before. Open your eyes when you are ready.")]));
    v.push(p(g, "jk_merkavah", "Merkavah — Ascent through the Seven Palaces", "Hekhalot visionary ascent · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Merkavah</b> (\"chariot\") mysticism grew from Ezekiel's vision of the divine chariot-throne (Ezekiel 1) and flourished in late antiquity in the <b>Hekhalot</b> (\"palaces\") texts. Its adepts, strangely called \"descenders to the chariot\", recited hymns and names to pass the angelic guards of <b>seven heavenly palaces</b> and behold the throne of glory. Hai Gaon (c. 1000) describes them fasting and sitting with the head between the knees, as Elijah did on Mount Carmel (1 Kings 18:42). Here there is no fasting, the bowed posture lasts only two minutes, and the secret names are replaced by the angels' song from Isaiah 6:3, <i>Kadosh, kadosh, kadosh</i>, which the Hekhalot hymns echo.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the chariot meditation of the merkavah mystics, who ascended through seven heavenly palaces in their visions. Sit and, as they did, lower your head toward your knees, only as far as is comfortable, or simply bow your head."),
           st("Hymn", 4.0, &[8.0], "Slowly sit up. Softly repeat the angels' song from Isaiah: kadosh, kadosh, kadosh. Holy, holy, holy. The whole earth is full of his glory. Let the rhythm carry you."),
           st("Seven palaces", 12.0, &[6.5], "Picture yourself at the gate of the first palace. With each repetition of the hymn, pass through another gate, higher and deeper, seven palaces in all, each one brighter than the last."),
           st("The throne", 5.0, &[6.0], "In the seventh palace, rest before the throne of glory, as Ezekiel saw it, with a radiance like a rainbow in the clouds. Don't try to see more. Simply be present, in awe."),
           st("Descent", 2.0, &[10.0], "Now come back down through the seven palaces, gate by gate, carrying the stillness with you. Feel your body, and open your eyes.")]));
    v.push(p(g, "jk_niggun", "Niggun — Wordless Melody (Hasidic)", "Singing over a drone · 15 min", &[196.0, 293.66], &[0.45, 0.3], "none", "binaural", "Traditional",
      "<p>A <b>niggun</b> is a wordless melody, sung to syllables such as <i>yai-dai-dai</i> or <i>bim-bam</i> and repeated again and again. Since the <b>Baal Shem Tov</b> (c. 1698–1760) it has been at the heart of Hasidic worship, where song is said to reach where words cannot. Some niggunim are joyful dances; others are slow songs of <i>devekut</i>, \"clinging\" to God. A saying attributed to Rabbi Shneur Zalman of Liadi calls melody \"the pen of the soul\". A drone of two notes, G and D, an open fifth, plays here as a ground to sing over.</p>",
      vec![st("Settle", 2.0, &[10.0, 10.0], "Welcome to the niggun, the wordless melody of Hasidic prayer. Sit comfortably. A drone of two notes plays as a ground to sing over. There are no words, only syllables like yai dai dai, or bim bam."),
           st("Humming", 3.0, &[8.0, 8.0], "Begin by humming, quietly, any simple tune that comes, over the drone. Let it repeat. A niggun goes round and round, each time a little deeper."),
           st("Singing", 6.0, &[7.0, 7.0], "Now open the sound into syllables: yai dai dai, yai dai dai. Let the melody rise and fall. Sing with longing, or with joy, whichever is in you today. There is no right way to sound."),
           st("Devekut", 3.0, &[6.0, 6.0], "Let the melody slow down and grow softer, a niggun of devekut, clinging to God. Hum it, then only hear it in your heart."),
           st("Silence", 1.0, &[6.0, 6.0], "Let the melody fall silent. Rest in what it opened.")]));

    // ================= Taoist inner cultivation =================
    let g = "Taoist Inner Cultivation";
    v.push(p(g, "ta_zuowang", "Zuowang — Sitting and Forgetting (Zhuangzi)", "Zhuangzi ch. 6 · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>In chapter 6 of the <b>Zhuangzi</b> (4th–3rd century BCE), Yan Hui tells Confucius that he can now \"sit and forget\" (<b>zuowang</b>): he lets his body fall away, dismisses perception and knowing, leaves form behind and merges with the great pervading Way. Confucius asks to become his student. Later Daoists made this a method, above all <b>Sima Chengzhen</b> (647–735), to whom the <i>Zuowang lun</i> (\"Discourse on Sitting and Forgetting\") is attributed. This session forgets in the same order: the body, the senses, then knowing.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to zuowang, sitting and forgetting, from the Zhuangzi. Yan Hui told Confucius: I let my body fall away, set aside seeing and knowing, and become one with the great Way. Sit comfortably, close your eyes, and let your breathing be natural."),
           st("Forget the body", 5.0, &[8.0], "First, forget the body. Feel it, then let it go: the limbs, the weight, the edges. Let it become so relaxed that it seems to fall away."),
           st("Forget the senses", 5.0, &[7.0], "Next, let the senses rest. Hear without listening, feel without touching. Let sounds and sensations pass through you like wind through an open window."),
           st("Forget knowing", 6.0, &[6.0], "Now let go of knowing: names, opinions, what you know and who you are. When a thought arises, don't follow it and don't fight it. Forget it, and sit."),
           st("The great Way", 5.0, &[5.5], "Rest, merged with the great flow of the Tao, which needs nothing from you. Just sitting, just forgetting."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "ta_xinzhai", "Xinzhai — The Fasting of the Heart-Mind", "Zhuangzi ch. 4 · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>In chapter 4 of the <b>Zhuangzi</b>, Confucius teaches Yan Hui the <b>fasting of the heart-mind</b> (<b>xinzhai</b>), a fast from grasping, not from food: unify your attention; listen not with the ears but with the heart-mind; then not with the heart-mind but with <b>qi</b>. The ears stop at hearing and the mind at matching things to its ideas, but qi is empty and waits on all things. The Way gathers only in emptiness, and that emptiness is the fasting of the heart-mind.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to xinzhai, the fasting of the heart mind, from the Zhuangzi. This is a fast, not from food, but from grasping. Sit upright, close your eyes, and gather your attention into one."),
           st("Ears", 4.0, &[9.0], "First, listen with your ears. Hear the sounds around you, near and far, just as they are."),
           st("Heart-mind", 5.0, &[7.5], "Now stop listening with your ears, and listen with your heart mind. Feel how the mind meets each sound, naming it, matching it, holding on to it."),
           st("Qi", 6.0, &[6.0], "Now stop listening with the heart mind, and listen with your qi, the life breath. Qi is empty and waits on all things. Let yourself become that waiting emptiness."),
           st("Emptiness", 3.0, &[5.5], "The Way gathers only in emptiness, and emptiness is the fasting of the heart mind. Rest there.")]));
    v.push(p(g, "ta_standing", "Zhan Zhuang — Standing like a Tree", "Qigong standing practice · 20 min", &[200.0], &[0.45], "pink", "binaural", "Traditional",
      "<p><b>Zhan zhuang</b> (\"standing like a post\") is a foundation of qigong and of the internal martial arts, made widely known by <b>Wang Xiangzhai</b> (1885–1963), founder of Yiquan. You stand still with soft knees, then hold the arms rounded as if hugging a tree, and let the body settle into deep, quiet effort. Warmth and trembling are common. Keep a chair nearby and your eyes half open. Lower your arms whenever they tire, and sit down if you feel dizzy or have pain. If you have heart problems or high blood pressure, keep the arms low and the holds short.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to zhan zhuang, standing like a tree, the still standing practice of qigong. Keep a chair nearby. Stand with your feet shoulder width apart, knees slightly bent, eyes half open, and let your arms hang loosely."),
           st("Wuji", 4.0, &[9.0], "This is wuji, the empty stance. Let your weight sink into your feet, as if roots were growing from them. Lengthen the back of your neck, and soften your knees. Breathe naturally into the belly."),
           st("Holding the tree", 6.0, &[8.0], "Now raise your arms in front of your chest, rounded, as if gently hugging a large tree, fingers apart, elbows a little low. Keep your shoulders soft. If your arms tire, lower them for a while, then raise them again."),
           st("Stillness", 5.0, &[7.5], "Stay still and quiet, arms raised or lowered. You may feel warmth, tingling, or trembling in the legs; this is common. If you feel dizzy or any pain, sit down. Otherwise, let the stillness deepen."),
           st("Closing", 3.0, &[10.0], "Slowly lower your arms, and rest your hands over your lower belly, the dantian. Feel warmth gather there. Then walk around for a minute before you sit down.")]));
    v.push(p(g, "ta_sixsounds", "Liu Zi Jue — The Six Healing Sounds", "Six whispered breaths · 18 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>six healing sounds</b> (<i>liù zì jué</i>, \"six-word formula\") are an ancient Chinese breathing practice, first recorded in <b>Tao Hongjing</b>'s <i>Yangxing yanming lu</i> (around 500 CE). Each sound, breathed out slowly, is linked in traditional Chinese medicine to an organ: <b>xū</b> the liver, <b>hē</b> the heart, <b>hū</b> the spleen, <b>sī</b> the lungs, <b>chuī</b> the kidneys and <b>xī</b> the \"triple burner\", which spreads warmth through the body. The Chinese Health Qigong Association published a standard form in 2003. The out-breaths are long and gentle, never forced; these links are traditional, not established by research.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the six healing sounds, liu zi jue, an ancient Chinese breathing practice. Sit or stand comfortably. For each sound, breathe in through the nose, then breathe out slowly while whispering it, about six times. Never force the breath."),
           st("Xū · liver", 2.5, &[9.0], "The first sound is shoo, for the liver. Whisper a long shooo on each slow out breath, with your eyes wide, as if gazing far away."),
           st("Hē · heart", 2.5, &[8.5], "The second sound is a soft huhh, for the heart. Let the chest soften as you breathe it out."),
           st("Hū · spleen", 2.5, &[8.0], "The third sound is a hollow hooo, for the spleen and digestion. Feel it in the belly."),
           st("Sī · lungs", 2.5, &[7.5], "The fourth sound is a hissing sss, through lightly closed teeth, for the lungs."),
           st("Chuī · kidneys", 2.5, &[7.0], "The fifth sound is chway, for the kidneys. Feel it low in your back."),
           st("Xī · triple burner", 2.5, &[6.5], "The sixth sound is shee, for the triple burner, the warmth that flows through the whole body."),
           st("Rest", 1.0, &[6.5], "Let the sounds go, breathe naturally, and rest, feeling the whole body calm and even.")]));

    // ================= Shamanic and ancestral rituals =================
    let g = "Shamanic & Ancestral Rituals";
    v.push(with_tex(p(g, "sr_lower", "Lower-World Journey — Meeting a Power Animal", "Core shamanism · 25 min", &[100.0], &[0.25], "none", "binaural", "Traditional",
      "<p>In <b>core shamanism</b> (Michael Harner, <i>The Way of the Shaman</i>, 1980) the classic first journey goes to the <b>lower world</b>, entered through an opening in the earth you know from waking life (a cave, a hollow tree, a spring or a burrow) and down a tunnel. There journeyers seek a <b>power animal</b>, a helping spirit; in Harner's teaching, an animal that shows itself to you four times, from different sides, is your helper. The drum beats at about four and a half times a second, then gives the fast call-back that signals the return.</p>",
      vec![st("Intention", 2.0, &[4.2], "Welcome to the lower world journey of core shamanism. Lie down and cover your eyes with a cloth. Picture an opening into the earth that you know from waking life, a cave, a hollow tree, a spring, or an animal's burrow. Your intention: to meet a power animal.").tex(0.3),
           st("Descent", 5.0, &[4.5], "Go into the opening, and follow the tunnel down. It may be dark, it may twist or slope. Keep going, with curiosity, until you come out into a landscape."),
           st("Lower world", 10.0, &[4.5], "You are in the lower world. Notice the ground, the light, the plants and the sounds. Ask for your power animal to show itself. An animal that shows itself to you four times, from different sides, is your helper. Greet it."),
           st("With the animal", 5.0, &[4.5], "Spend time with your animal. Follow it, listen to it, or ask your question. Accept whatever comes, without judging it."),
           st("Call-back", 2.0, &[8.0], "The drum is calling you back. Thank your animal, and return quickly the way you came, up the tunnel and out of the opening.").ramp(20.0),
           st("Return", 1.0, &[10.0], "You are back. Feel your body, take a deep breath, and open your eyes. You might like to write down what you saw.").tex(0.0)]),
      tx("drum", 1.0, 0.0, 0.0)));
    v.push(with_tex(p(g, "sr_posture", "Ritual Body Postures — Fifteen Minutes of Rattle", "Goodman's method · 22 min", &[200.0], &[0.35], "none", "binaural", "Speculative",
      "<p>The anthropologist <b>Felicitas Goodman</b> (1914–2005) noticed the same body postures in ancient art and carvings from many cultures and proposed that each one, held for exactly <b>fifteen minutes</b> to a rattle or drum at about <b>210 beats a minute</b>, opens a particular kind of trance: healing, divination, spirit journeys (<i>Where the Spirits Ride the Wind</i>, 1990). The rattle part is close to well-documented rhythmic trance, but her claim that each posture gives a specific experience has not been tested in controlled research, so this session is tagged Speculative. It uses the posture she called the <b>Bear Spirit</b>, associated with healing. You can do it seated.</p>",
      vec![st("Settle", 3.0, &[10.0], "Welcome to ritual body postures, as explored by the anthropologist Felicitas Goodman. You will hold one posture for fifteen minutes while a rattle plays. Stand with your feet parallel, a little apart, knees soft, or sit upright on a chair if you prefer.").tex(0.0),
           st("Posture", 2.0, &[8.0], "Breathe calmly. Now take the bear spirit posture: rest your hands on your belly near the navel, fingertips lightly touching, and tilt your head slightly back, with your eyes closed. If you feel unsteady at any time, sit down.").tex(0.0),
           st("Trance", 15.0, &[5.5], "The rattle begins. Hold the posture, relaxed and still, and simply notice what comes: sensations, images, a sense of movement or presence. Don't analyse; just observe."),
           st("Return", 2.0, &[10.0], "The rattle stops. Release the posture, stretch, and open your eyes. Take a moment to recall, or write down, what you experienced.").tex(0.0)]),
      tx("rattle", 0.5, 0.0, 0.2857)));
    v.push(with_tex(p(g, "sr_dance", "Trance Dance — Rising and Falling Drum", "Ecstatic movement · 30 min", &[110.0], &[0.2], "none", "binaural", "Traditional",
      "<p>Dancing to a steady rhythm until the movement seems to take over is one of humanity's oldest ways into trance. Among the <b>Ju/'hoansi San</b> of the Kalahari, healers dance through the night to clapping and song until they enter <i>!kia</i>, a state of \"boiling\" healing energy (Richard Katz, <i>Boiling Energy</i>, 1982), and many modern ecstatic-dance gatherings follow a similar wave that builds to a peak and settles into stillness. Here a frame drum rises from 72 to 156 beats a minute and back down. <b>Clear a space</b>, keep your eyes softly open, <b>never spin</b>, breathe freely, slow down whenever you need to, and drink water afterwards.</p>",
      vec![st("Warm-up", 5.0, &[1.2], "Welcome to the trance dance. Clear a space around you, take off your shoes if the floor allows, and keep your eyes softly open. Begin by swaying with the slow drum. Let your feet, then your knees and hips, find the beat."),
           st("Pulse", 5.0, &[1.6], "The drum picks up. Let the movement spread to your arms, shoulders and head. Move however your body wants to, with no steps to remember and no one watching.").ramp(20.0),
           st("Build", 5.0, &[2.0], "Faster now. Keep breathing freely, and let the drum lead. Shake out your hands and shoulders, stamp your feet, and let the rhythm move you more than you move yourself.").ramp(20.0),
           st("Peak", 5.0, &[2.6], "This is the peak. Let go into the rhythm, as dancers have for thousands of years, and let the body dance itself. Keep your eyes open enough to stay safe, and never spin. Slow down if you feel dizzy or out of breath.").ramp(20.0),
           st("Unwind", 5.0, &[1.8], "The drum slows. Let your movements become smooth, flowing and soft, like water. Feel the warmth and the aliveness in your body.").ramp(20.0),
           st("Stillness", 5.0, &[0.8], "Come to stillness, standing or lying down. Feel your heartbeat and breath settle, and the energy still moving inside you. Rest here, and drink some water when you finish.").ramp(20.0).tex(0.3)]),
      tx("drum", 1.0, 0.0, 0.0)));

    // ================= Western esoteric and classical =================
    let g = "Western Esoteric & Classical";
    {
        const D_DRIFT: &str = "Remember to think of your theme. Let it be there as you drift.";
        const D_REPORT: &str = "Without opening your eyes, what were you just thinking about? Say it softly, or note it, and then let yourself drift again.";
        v.push(p(g, "we_incubation", "Dream Incubation — Temple of Asclepius to the Sleep Lab", "Horowitz 2020, 2023 · 30 min", &[150.0], &[0.35], "brown", "binaural", "Research",
          "<p>In ancient Greece the sick slept in the temples of <b>Asclepius</b>, such as Epidaurus, hoping the god would appear in a dream to heal or advise them; this is <b>incubation</b>, from the Latin for \"lying down in\". At MIT, <b>Haar Horowitz et al. (2020, <i>Consciousness and Cognition</i>)</b> brought it into the lab as <b>targeted dream incubation</b>: just as people drifted into sleep, a voice reminded them of a theme, then woke them briefly to report their imagery, again and again. Their hypnagogic dreams took up the theme far more often, and in a follow-up (<b>2023, <i>Scientific Reports</i></b>) incubated naps improved performance on creative tasks afterwards. The study used a hand-worn sensor to time the prompts; this session approximates it with timed prompts, so keep the volume low.</p>",
          vec![st("Settle", 3.0, &[10.0], "Welcome to dream incubation, the ancient practice of the healing temples of Asclepius, studied today at the edge of sleep. Lie down comfortably and choose a theme: a question, an image, or a problem to solve. Keep a notebook or a voice recorder nearby."),
               st("Theme", 2.0, &[8.0], "Hold your theme in mind. Say it to yourself in a few simple words, and picture it. Then let yourself start to drift toward sleep, keeping it loosely in mind."),
               st("Drift", 5.0, &[6.0], D_DRIFT),
               st("Report", 1.0, &[8.0], D_REPORT),
               st("Drift", 5.0, &[5.5], D_DRIFT),
               st("Report", 1.0, &[8.0], D_REPORT),
               st("Drift", 5.0, &[5.0], D_DRIFT),
               st("Report", 1.0, &[8.0], D_REPORT),
               st("Drift", 5.0, &[5.0], D_DRIFT),
               st("Wake", 2.0, &[10.0], "Gently come back now. Write down everything you can remember, even fragments, then read your theme again, and see what the drifting mind has offered.")]));
    }
    v.push(p(g, "we_plotinus", "Plotinus — Ascent to the One (Neoplatonism)", "Enneads V.8 and VI.9 · 25 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Plotinus</b> (204/5–270), founder of Neoplatonism, taught that the soul can return to its source, <b>the One</b>, by turning inward, from the body to the soul, from the soul to <b>Intellect</b> (<i>nous</i>), and beyond it to the One itself. His student Porphyry wrote that Plotinus reached this union four times while he knew him. In <i>Ennead</i> V.8.9 Plotinus gives an exercise: picture the whole universe as one transparent sphere, then take away its mass and size. Another treatise ends with the words \"take away everything\" (V.3.17), and the <i>Enneads</i> close with the flight \"of the alone to the alone\" (VI.9.11).</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the contemplation of Plotinus, the third century philosopher who taught that the soul can return to its source, the One. Sit upright and close your eyes. You will turn inward, step by step, from the world, to the soul, to the One."),
           st("The sphere", 6.0, &[8.0], "Imagine the whole universe as a single transparent sphere: sun, stars, earth, seas and living things, all seen at once, each one visible through all the others, as Plotinus describes."),
           st("Without mass", 5.0, &[7.0], "Now keep the sphere, but take away its mass, its size and its place. Let it be pure light, held in the mind, no longer made of matter."),
           st("Intellect", 5.0, &[6.0], "Turn from the image to the light that is seeing it. Plotinus called this nous, intellect, where knower and known are one. Rest as that seeing."),
           st("Take away everything", 5.0, &[5.5], "Plotinus ends one treatise with three words: take away everything. Let go of the sphere, the light, even the seeing. Rest in the One, the alone with the alone."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "we_above", "As Above, So Below — Becoming the Cosmos (Hermetic)", "Corpus Hermeticum XI · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Hermetic</b> writings, attributed to <b>Hermes Trismegistus</b>, shaped Western esotericism from the Renaissance to the Golden Dawn. The <b>Emerald Tablet</b>, first known from Arabic sources of around the 9th century, gives the famous principle usually summarised as <b>\"as above, so below\"</b>: what is below mirrors what is above. In the <b>Corpus Hermeticum</b> (book XI, 2nd–3rd century) Mind tells Hermes that to understand God he must make himself as vast as the cosmos, leap beyond every body and all time, and become eternity. This session moves from the small world of the body to the great world of the sky, then expands to hold both.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to a Hermetic contemplation, from the writings attributed to Hermes Trismegistus. Sit upright and close your eyes. The Emerald Tablet teaches: as above, so below. You will expand your awareness until it holds the whole cosmos."),
           st("Below", 4.0, &[8.0], "Begin with the body: the breath, the heartbeat, the warmth. A small world, a microcosm, made of the same elements as the stars."),
           st("Above", 4.0, &[7.0], "Now picture the sky above you, the moon, the planets and the stars. Sense how your heartbeat and the turning of the heavens are one pattern: as above, so below."),
           st("Becoming the cosmos", 6.0, &[6.0], "The Hermetic books say: make yourself as vast as the cosmos, leap beyond all time, become eternity. Let your awareness expand to contain everything, every place and every time, at once."),
           st("The one thing", 3.0, &[5.5], "Rest as that vastness, holding above and below together as one thing."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "we_middlepillar", "The Middle Pillar — Five Spheres of Light (Golden Dawn)", "Regardie 1938 · 22 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Middle Pillar</b> exercise comes from the Hermetic Order of the <b>Golden Dawn</b> (founded 1888) and was published by <b>Israel Regardie</b> in <i>The Middle Pillar</i> (1938). It adapts the central column of the Kabbalistic Tree of Life to the body: five spheres of light, from above the head to the feet, each \"vibrated\" with a Hebrew divine name, the Golden Dawn way of intoning a word so that it resonates through the body. The four-letter name is spelled out letter by letter, as Regardie gives it. The light is then circulated around the body. Many ceremonial magicians practise it daily for energy and balance.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Middle Pillar, the Golden Dawn exercise published by Israel Regardie in 1938. Stand or sit upright with your eyes closed. You will build five spheres of light down the centre of your body, vibrating a divine name at each one."),
           st("Crown", 3.0, &[9.0], "Above the crown of your head, picture a sphere of brilliant white light. Breathe in, and on the out breath vibrate the name eh hey yeh, meaning I am, so that you feel it resonate. Repeat it a few times."),
           st("Throat", 3.0, &[8.0], "Draw a shaft of light down to your throat, where a second sphere of light forms. Vibrate: yod heh vav heh elohim."),
           st("Heart", 3.0, &[7.5], "Let the shaft descend to your heart, forming a sphere of golden light. Vibrate: yod heh vav heh eloah ve-daath."),
           st("Groin", 3.0, &[7.0], "Down to the groin, a third sphere below the heart. Vibrate: shaddai el chai, the almighty living one."),
           st("Feet", 3.0, &[6.5], "Down to your feet, a sphere of light in the ground beneath them. Vibrate: adonai ha-aretz, lord of the earth. Now see the whole pillar of light, from crown to feet."),
           st("Circulation", 4.0, &[6.0], "Now circulate the light. Breathe out and let it flow down your left side; breathe in and let it rise up your right side. Then down the front and up the back. Finally, let it rise up the centre and shower down around you like a fountain."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(p(g, "we_pentagram", "Lesser Banishing Ritual of the Pentagram (Golden Dawn)", "Centring ritual · 15 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The <b>Lesser Banishing Ritual of the Pentagram</b> is the best-known ritual of Western ceremonial magic, taught by the Hermetic Order of the Golden Dawn and published by Israel Regardie in <i>The Golden Dawn</i> (1937–40). Practitioners use it daily to centre themselves and clear their space. It has three parts: the <b>Kabbalistic Cross</b>; a pentagram traced in each of the four directions with a divine name; and the calling of the four archangels, Raphael, Gabriel, Michael and Uriel. Then the cross is repeated. You stand facing east, with room to move your arms.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the lesser banishing ritual of the pentagram, the best known ritual of Western ceremonial magic. Stand facing east, with room to move your arms. Take a few slow breaths, and picture yourself growing tall, your head among the stars and your feet on the earth."),
           st("Kabbalistic Cross", 2.0, &[9.0], "The Kabbalistic cross. Touch your forehead and vibrate: ateh, yours is. Touch your chest: malkuth, the kingdom. Your right shoulder: ve-geburah, and the power. Your left shoulder: ve-gedulah, and the glory. Clasp your hands at your heart: le-olam, forever. Amen."),
           st("Pentagrams", 5.0, &[8.0], "Facing east, trace a large star in the air: from your left hip, up to above your head, down to your right hip, up to your left shoulder, across to your right shoulder, and back to your left hip. Point to its centre and vibrate: yod heh vav heh. Turn to the south and do the same, vibrating: adonai. To the west: eh hey yeh. To the north: agla. Then turn back to the east."),
           st("Archangels", 3.0, &[7.0], "Stretch your arms out like a cross, and say: before me, Raphael. Behind me, Gabriel. On my right hand, Michael. On my left hand, Uriel. For about me flames the pentagram, and within me shines the six rayed star."),
           st("Closing cross", 2.0, &[8.0], "Repeat the Kabbalistic cross: ateh, malkuth, ve-geburah, ve-gedulah, le-olam, amen. Stand in the clear, quiet space you have made."),
           st("Stillness", 1.0, &[10.0], "Rest for a moment, centred and calm, then go on with your day.")]));
    v.push(p(g, "we_bards", "The Bardic Darkness — Composing in the Dark", "Gaelic poets, Martin 1703 · 30 min", &[200.0], &[0.45], "brown", "binaural", "Traditional",
      "<p>In <i>A Description of the Western Islands of Scotland</i> (1703), <b>Martin Martin</b> recorded how the Gaelic poets composed: they shut themselves away in darkness for a day, lying on their backs with a stone on the belly and their plaids wrapped about their heads to cover their eyes, and worked their verses in their minds. Accounts of the Irish bardic schools likewise describe students composing in dark, windowless rooms. The darkness turns attention inward toward the half-dreaming state where images and rhythms arise. Use a light weight, such as a folded blanket, not a stone.</p>",
      vec![st("Settle", 3.0, &[10.0], "Welcome to the bardic darkness. In 1703 Martin Martin wrote that the Gaelic poets composed lying in the dark, with a stone on the belly and a plaid over their eyes. Lie on your back in a dark room, cover your eyes, and rest a light weight, like a folded blanket, on your belly."),
           st("Subject", 2.0, &[8.0], "Choose your subject: a poem, a song, a story, or a question. Say it once in your mind, then let it go into the darkness."),
           st("Darkness", 10.0, &[6.0], "Lie still in the darkness. Feel the weight rise and fall with your breath. Let words, images and rhythms come up from the dark, without forcing them. Turn them over, and let them grow."),
           st("Composing", 12.0, &[5.5], "Begin to shape what comes. Hold the lines in memory, as the bards did, without writing. Repeat them to yourself, change them, and polish them in the dark."),
           st("Into the light", 3.0, &[10.0], "Slowly uncover your eyes and let the light back in. Write down what you composed while it is still fresh.")]));
    v.push(p(g, "we_galdr", "Galdr — Chanting the Runes (Norse)", "Modern reconstruction · 15 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Galdr</b> (Old Norse, \"incantation\", from <i>gala</i>, to chant or crow) was a spoken or sung magic mentioned throughout the Eddas and sagas. In the <i>Hávamál</i>, Odin hangs nine nights on the windswept tree to win the runes, then lists eighteen charms he knows. How galdr actually sounded is unknown. Modern practitioners chant the names of the runes, reconstructed from medieval rune poems, with long, resonant vowels, and this session follows that <b>modern reconstruction</b> with four runes of the Elder Futhark: <b>Ansuz</b>, <b>Laguz</b>, <b>Isa</b> and <b>Dagaz</b>.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to galdr, the chanted magic of the Norse sagas. No one knows exactly how it sounded; this is a modern reconstruction, chanting the names of four runes. Stand or sit upright, and breathe deeply into your belly."),
           st("Ansuz", 3.0, &[8.0], "Ansuz, the rune of the god, of breath and the spoken word. Breathe in, and on the out breath chant: aaan suuuz, letting the sound resonate in your chest. Repeat it again and again."),
           st("Laguz", 3.0, &[7.0], "Laguz, the rune of water and flow. Chant: laaa guuuz, smooth and flowing, like a river."),
           st("Isa", 3.0, &[6.0], "Isa, the rune of ice and stillness. Chant: eeee saaa, steady and clear. Let the mind become as still and clear as ice."),
           st("Dagaz", 3.0, &[7.0], "Dagaz, the rune of day and dawn. Chant: daaa gaaaz, and feel light breaking through, like the sun rising."),
           st("Silence", 1.0, &[8.0], "Let the chanting stop, and listen to the silence ringing after it.")]));

    // ================= Trance and hypnosis techniques =================
    let g = "Trance & Hypnosis Techniques";
    v.push(p(g, "tr_autogenic", "Autogenic Training — The Six Standard Exercises", "Stetter & Kupper 2002 · 25 min", &[200.0], &[0.5], "pink", "binaural", "Research",
      "<p><b>Autogenic training</b> was developed by the German psychiatrist <b>Johannes Heinrich Schultz</b> from his work with hypnosis and published in 1932. You silently repeat six formulas, each several times: <b>heaviness</b> in the limbs, <b>warmth</b> in the limbs, a <b>calm, regular heartbeat</b>, <b>calm breathing</b> (\"it breathes me\"), <b>warmth in the solar plexus</b> and a <b>cool forehead</b>. A meta-analysis of 60 studies (<b>Stetter &amp; Kupper 2002, <i>Applied Psychophysiology and Biofeedback</i></b>) found medium-to-large benefits for tension headache, mild high blood pressure, anxiety, mild depression and sleep problems. Schultz always ended with a brisk \"taking back\". If you have a heart condition, you can skip the heartbeat formula.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to autogenic training, the self relaxation method of Johannes Schultz. Lie down, or sit with your head slightly forward and your forearms resting on your thighs. You will silently repeat six simple formulas, each several times. Begin with: I am completely calm."),
           st("Heaviness", 4.0, &[9.0], "First, heaviness. Repeat silently and slowly: my right arm is heavy. Then my left arm, both arms, and both legs. Let the heaviness happen by itself, without trying."),
           st("Warmth", 4.0, &[8.0], "Second, warmth. Repeat: my right arm is warm. Then the left arm, both arms, and both legs. Picture warm sunlight resting on your limbs."),
           st("Heartbeat", 3.0, &[7.5], "Third, the heart. Repeat: my heartbeat is calm and regular. If attending to your heart is uncomfortable, skip this one and simply rest."),
           st("Breathing", 3.0, &[7.0], "Fourth, the breath. Repeat: my breathing is calm, or simply, it breathes me. Let the breath move by itself."),
           st("Solar plexus", 3.0, &[6.5], "Fifth, the solar plexus. Repeat: my solar plexus is warm, and feel a gentle warmth spread through your upper belly."),
           st("Cool forehead", 3.0, &[6.0], "Sixth, the forehead. Repeat: my forehead is pleasantly cool. Imagine a light, cool breeze on your forehead."),
           st("Taking back", 3.0, &[12.0], "Now take it back, as Schultz taught. Clench your fists, bend your arms firmly, breathe in deeply, and open your eyes. Stretch, and be fully alert.").ramp(60.0)]));
    v.push(p(g, "tr_erickson", "The 3-2-1 Method of Self-Hypnosis (Betty Erickson)", "Sight, sound, feeling · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p><b>Elizabeth \"Betty\" Erickson</b>, wife and collaborator of the psychiatrist <b>Milton H. Erickson</b>, taught a simple self-hypnosis method that many hypnotherapists still use. You first set how long you want to spend in trance and an intention for your unconscious mind. Then, with eyes open, you name three things you see, three you hear and three you feel; then two of each; then one of each. You close your eyes and repeat the count with inner images, sounds and feelings. Narrowing attention this way leads naturally into trance.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to Betty Erickson's three, two, one method of self hypnosis. Sit comfortably with your eyes open. First, decide to spend about fifteen minutes in trance, and set an intention: something you would like your inner mind to work on, put simply and positively."),
           st("Three", 3.0, &[10.0], "With your eyes open, slowly and silently name three things you can see. Then three things you can hear. Then three things you can feel, like your hands resting or the chair beneath you."),
           st("Two", 2.0, &[9.0], "Now two things you can see, two you can hear, and two you can feel. They can be the same or new."),
           st("One", 2.0, &[8.0], "Now one thing you can see, one you can hear, and one you can feel. Then let your eyes close."),
           st("Inner count", 4.0, &[7.0], "With your eyes closed, do the same inside. Imagine three things you see in your mind, three you hear, and three you feel. Then two of each. Then one of each."),
           st("Trance", 6.0, &[6.0], "Now let go, and let your inner mind work on your intention in its own way. There is nothing to do. Drift, daydream, or simply rest."),
           st("Emerge", 1.0, &[12.0], "It's time to come back. Count yourself up from one to five, more alert with each number, and open your eyes, refreshed.").ramp(30.0)]));
    let mut x = p(g, "tr_braid", "Braid's Eye Fixation — The First Hypnotic Induction", "Braid 1843 · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
      "<p>The Scottish surgeon <b>James Braid</b> (1795–1860) coined the word <b>hypnotism</b> in <i>Neurypnology</i> (1843). His induction: fix the eyes and the whole attention on a small bright object held a little above eye level until the eyelids grow heavy and close by themselves. Braid rejected Mesmer's \"animal magnetism\" and explained hypnosis as attention absorbed in a single idea, which he later called <i>monoideism</i>; modern hypnosis research descends from his view. The <b>🔆 gazing aid</b> shows a still bright point. Never strain your eyes; close them whenever you like.</p>",
      vec![st("Settle", 2.0, &[10.0], "Welcome to the eye fixation method of James Braid, the surgeon who named hypnotism in 1843. Open the bright point on this screen, or hold a small shiny object, about an arm's length away and a little above eye level. Sit back comfortably."),
           st("Fixation", 5.0, &[10.0], "Fix your eyes and your whole attention on the point. Keep looking, without straining. Notice how your eyelids begin to feel heavy, and your vision may blur or swim. That's normal."),
           st("Eyes closing", 3.0, &[8.0], "Let the heaviness grow. When your eyelids want to close, let them close, all by themselves. If they haven't closed yet, gently close them now."),
           st("Deepening", 7.0, &[6.0], "With your eyes closed, let your attention rest on a single idea: deeper and calmer with each breath. Braid believed this was hypnosis: the whole mind absorbed in one idea."),
           st("Emerge", 3.0, &[12.0], "Now come back. Take a deep breath, feel your body, and open your eyes, wide awake and alert.").ramp(60.0)]);
    x.visual = "trataka:point".into();
    v.push(x);
    {
        const FR_SURFACE: &str = "Now come up, just a little. Let your eyes open for a moment and notice how relaxed you are. Then let them close again, and go down twice as deep.";
        const FR_DOWN: &str = "Sink down again, twice as deep as before. Let every muscle let go, and let the mind drift.";
        v.push(p(g, "tr_fractionation", "Fractionation — Going Deeper by Rising (Vogt)", "Vogt's deepening method · 20 min", &[200.0], &[0.5], "pink", "binaural", "Traditional",
          "<p><b>Fractionation</b> is a deepening method developed in the 1890s by the German neurologist <b>Oskar Vogt</b> (1870–1959): the subject is hypnotised, brought partly out, asked about the experience, and hypnotised again. Each re-entry tends to go deeper than the last, and hypnotists still use it to deepen trance quickly. Here you sink down, rise a little and open your eyes briefly, three times, then settle into the deepest round.</p>",
          vec![st("Settle", 2.0, &[10.0], "Welcome to fractionation, a deepening method first used by the neurologist Oskar Vogt in the eighteen nineties. Sit or lie comfortably and close your eyes. You will sink down, come up a little, and sink again, going deeper each time."),
               st("Down", 3.0, &[8.0], "Let your body relax from head to toe. Count slowly down from ten to one, sinking a little deeper with each number."),
               st("Surface", 1.0, &[10.0], FR_SURFACE).ramp(20.0),
               st("Deeper", 3.0, &[6.5], FR_DOWN).ramp(20.0),
               st("Surface", 1.0, &[10.0], FR_SURFACE).ramp(20.0),
               st("Deeper", 3.0, &[5.5], FR_DOWN).ramp(20.0),
               st("Surface", 1.0, &[10.0], FR_SURFACE).ramp(20.0),
               st("Deepest", 4.0, &[5.0], "Now sink all the way down, deeper than ever, and rest in deep calm. There is nothing to do and nothing to think about.").ramp(20.0),
               st("Emerge", 2.0, &[12.0], "Now come all the way back. Count up from one to five, more alert with each number. Five: eyes open, fully awake.").ramp(30.0)]));
    }
}

/// Tradition keys for the Tradition filter: a family key, then its branch (if any).
/// Curated by hand: text matching would, for example, file singing bowls under Tibetan Buddhism.
/// Families: abrahamic (christian [mystic, catholic, orthodox, protestant, rosicrucian, masonic],
/// jewish [kabbalah, hasidic, mussar], islamic [sufi], bahai), hindu (vedanta, tantra, kundalini, mantra, yoga),
/// buddhist (tibetan, theravada, zen, eastasian), jain, sikh, taoist, zoroastrian, shinto, shamanic,
/// western (hermetic, rosicrucian, masonic, classical, celtic, norse, fourthway), secular.
fn trad_of(id: &str, group: &str) -> Vec<String> {
    let k: &[&str] = match group {
        "Christian Mysticism" => &["abrahamic", "christian"],
        "Jewish Mysticism & Kabbalah" => &["abrahamic", "jewish"],
        "Sufi Mysticism" => &["abrahamic", "islamic"],
        "Tibetan Buddhism & Bön" => &["buddhist", "tibetan"],
        "Taoist Inner Cultivation" => &["taoist"],
        "Shamanic & Ancestral Rituals" => &["shamanic"],
        "Trance & Hypnosis Techniques" => &["secular"],
        _ => match id {
            "bu_metta" | "bu_brahma" | "bu_anapana" | "bu_noting" | "bu_kasina" | "my_jhana" | "my_vipassana" => &["buddhist", "theravada"],
            "bu_mu" | "bu_shikantaza" | "zen" => &["buddhist", "zen"],
            "bu_nembutsu" | "bu_daimoku" | "bu_ajikan" => &["buddhist", "eastasian"],
            "my_tibetan" | "ap_phowa" | "ap_dreamyoga" | "br_tummo" | "br_vase" | "monks" => &["buddhist", "tibetan"],
            "my_nondual" => &["buddhist", "tibetan", "hindu", "vedanta"],
            "hy_mandukya" | "my_inquiry" => &["hindu", "vedanta"],
            "hy_vbt" => &["hindu", "tantra"],
            "hy_bija" => &["hindu", "tantra", "mantra"],
            "hy_shiva" | "hy_gayatri" | "hy_mahamantra" | "om" | "omdrone" | "my_mantra" => &["hindu", "mantra"],
            "kirtan" => &["hindu", "yoga", "mantra"],
            "hy_trataka" | "hy_soham" | "hy_antarmouna" | "nsdr" | "my_nada" | "br_kapal" | "br_bhastrika" | "br_fire"
            | "br_sky" | "br_bhramari" | "br_nadi" | "br_ratio" | "br_slowbh" => &["hindu", "yoga"],
            "my_centering" | "my_jesus" | "my_vocal" => &["abrahamic", "christian"],
            "mantra6" => &["abrahamic", "christian", "hindu", "mantra"],
            "my_hitbodedut" => &["abrahamic", "jewish"],
            "my_dhikr" => &["abrahamic", "islamic"],
            "my_orbit" => &["taoist"],
            "shaman" | "drumtrance" | "ap_upper" | "ap_middle" => &["shamanic"],
            "ap_rising" | "ap_tattva" | "we_above" | "we_middlepillar" | "we_pentagram" => &["western", "hermetic"],
            "we_plotinus" | "my_psychomanteum" => &["western", "classical"],
            "we_incubation" => &["western", "classical", "secular"],
            "we_bards" => &["western", "celtic"],
            "we_galdr" => &["western", "norse"],
            "hypno" | "ganzfeld" | "my_float" | "my_arc" | "edison" | "ap_hypnotic" | "ap_imagined" | "ap_lab" => &["secular"],
            _ => &[],
        },
    };
    let mut out: Vec<String> = k.iter().map(|s| s.to_string()).collect();
    // Schools within a family (1.8), only ever added after the family keys above:
    // Christian mysticism, Catholic, Orthodox, Protestant; Kabbalah, Hasidic; Sufi; Kundalini.
    let school: &[&str] = match group {
        "Christian Mysticism" => &["mystic"],
        "Sufi Mysticism" => &["sufi"],
        _ => &[],
    };
    let more: &[&str] = match id {
        "cm_lectio" | "cm_ignatian" | "cm_castle" | "cm_cloud" | "cm_rosary" | "cm_maranatha" => &["catholic"],
        "my_centering" => &["catholic", "mystic"],
        "cm_vigil" | "cm_visio" => &["catholic", "orthodox"],
        "my_jesus" => &["orthodox", "mystic"],
        "cm_taize" => &["protestant", "catholic"],
        "cm_quaker" | "cm_bestill" | "my_vocal" => &["protestant"],
        "jk_abulafia" | "jk_tree" | "jk_merkavah" => &["kabbalah"],
        "jk_ayin" => &["hasidic", "kabbalah"],
        "jk_niggun" | "my_hitbodedut" => &["hasidic"],
        "my_dhikr" => &["sufi"],
        "kirtan" | "hy_bija" => &["kundalini"],
        _ => &[],
    };
    for s in school.iter().chain(more.iter()) {
        if !out.iter().any(|x| x == s) { out.push(s.to_string()); }
    }
    out
}

/// Schools inside a Buddhist or Hindu branch, for the third level of the tradition
/// browser (the same depth Christianity has with Catholic, Orthodox and so on).
/// Tibetan: vajrayana (deity & mantra), dzogchen (Dzogchen & Mahāmudrā), sixyogas
/// (dream, sleep, bardo, phowa, inner fire), lojong (compassion), bon.
/// Theravada: samatha (calm), vipassana (insight), brahmavihara (loving-kindness).
/// Zen: rinzai (kōan), soto (just sitting), chan (Chinese Chan & Korean Seon).
/// East Asian: pureland, nichiren, shingon.
/// Yoga: pranayama (breath), raja (Patañjali), hatha (Haṭha & Nāda), nidra (Yoga Nidrā).
/// Mantra: vedic (Om & Gāyatrī), shaiva, vaishnava. Tantra: kashmir, srividya.
fn schools_of(id: &str) -> &'static [&'static str] {
    match id {
        "tb_mani" | "tb_tara" | "tb_medicine" | "tb_vajrasattva" | "tb_manjushri" | "tb_guruyoga" | "tb_vajra"
        | "my_tibetan" | "tb_gyuto" | "tb_chod" => &["vajrayana"],
        "tb_sky" | "tb_phat" | "tb_tilopa" | "my_nondual" => &["dzogchen"],
        "tb_whitea" => &["dzogchen", "bon"],
        "ap_dreamyoga" | "tb_sleepyoga" | "ap_phowa" | "tb_bardo" | "br_tummo" | "br_vase" => &["sixyogas"],
        "tb_tonglen" | "monks" => &["lojong"],
        "my_jhana" | "bu_kasina" | "bu_buddho" | "bu_buddhanussati" | "bu_maranasati" => &["samatha"],
        "bu_anapana" => &["samatha", "vipassana"],
        "my_vipassana" | "bu_noting" | "bu_walking" => &["vipassana"],
        "bu_metta" | "bu_brahma" => &["brahmavihara"],
        "bu_mu" | "bu_softbutter" | "bu_susokukan" => &["rinzai"],
        "bu_shikantaza" => &["soto"],
        "bu_huatou" | "bu_silentillum" | "bu_whatisthis" => &["chan"],
        "bu_nembutsu" | "bu_amituofo" | "bu_guanyin" => &["pureland"],
        "bu_daimoku" => &["nichiren"],
        "bu_ajikan" => &["shingon"],
        "br_kapal" | "br_bhastrika" | "br_fire" | "br_sky" | "br_nadi" | "br_ratio" | "br_slowbh" => &["pranayama"],
        "br_bhramari" => &["pranayama", "hatha"],
        "hy_samyama" | "hy_pratyahara" | "hy_antarmouna" => &["raja"],
        "hy_trataka" | "my_nada" => &["hatha"],
        "nsdr" => &["nidra"],
        "om" | "omdrone" | "hy_gayatri" | "hy_japa" | "hy_mandukya" => &["vedic"],
        "hy_shiva" => &["shaiva"],
        "hy_vbt" => &["kashmir", "shaiva"],
        "hy_mahamantra" => &["vaishnava"],
        "hy_sriyantra" => &["srividya"],
        _ => &[],
    }
}

fn crate_band(hz: f64) -> &'static str {
    if hz < 0.75 { "Epsilon" } else if hz < 4.0 { "Delta" } else if hz < 8.0 { "Theta" } else if hz < 13.0 { "Alpha" } else if hz < 30.0 { "Beta" } else if hz < 90.0 { "Gamma" } else if hz < 150.0 { "Hypergamma" } else { "Lambda" }
}

/// Give every session searchable tags: group synonyms, evidence, bands, beat type,
/// sound texture, carrier tones and topic keywords found in its text.
fn auto_tags(v: &mut [Preset], also: &[String]) {
    const GROUP: &[(&str, &[&str])] = &[
        ("Inspired by the Gateway Experience", &["inspired by", "gateway", "gateway experience", "gateway process", "monroe", "hemi-sync", "hemisync", "focus", "focus level", "cia", "wave i", "obe", "out of body"]),
        ("Inspired by Lifeline (Focus 22–27)", &["inspired by", "lifeline", "monroe", "gateway", "focus", "focus level", "afterlife", "hemi-sync", "spiritual"]),
        ("Research-based", &["research", "science", "scientific", "study", "evidence"]),
        ("Altered States", &["altered states", "altered state", "asc", "consciousness", "trance", "research"]),
        ("Purpose & Tradition", &["purpose", "tradition", "intention"]),
        ("Brainwave Library", &["brainwave", "brain wave", "frequency", "library", "single frequency"]),
        ("Schumann Resonances", &["schumann", "earth", "resonance", "grounding", "7.83"]),
        ("Solfeggio Frequencies", &["solfeggio", "healing tones", "sacred", "frequency"]),
        ("Chakra Series", &["chakra", "chakras", "kundalini", "energy centres", "yoga", "spiritual"]),
        ("Cosmic Octave (Planets)", &["planet", "planets", "cosmic", "cosmic octave", "cousto", "astrology", "astronomy"]),
        ("Tunings", &["tuning", "pitch", "432", "440", "music"]),
        ("Sound Systems Library", &["sound systems", "library", "methods", "binaural", "monaural", "isochronic", "am", "amplitude modulation", "hybrid", "bilateral", "emdr", "8d", "spatial", "hemi-sync", "holosync", "noise", "drum"]),
        ("Light Journeys (Flicker)", &["light", "light journey", "flicker", "strobe", "visual", "visuals", "hallucination", "hallucinations", "closed-eye visuals", "dreamachine", "ganzflicker", "patterns", "geometry", "protocol", "lucia", "lumenate", "mind machine", "audio-visual entrainment"]),
        ("Infrasound (opt-in)", &["infrasound", "low frequency", "rumble", "ghost", "haunted", "18.98", "17 hz", "7 hz", "vibration"]),
        ("Breathwork (opt-in)", &["breathwork", "breathing", "hyperventilation", "wim hof", "holotropic", "rebirthing", "power breathing", "connected breathing", "pranayama", "breath hold"]),
        ("Breathing Techniques", &["breathing", "breath", "breathwork", "pranayama", "slow breathing", "calm", "relaxation", "breathing guide"]),
        ("Spiritual & Mystical Experiences", &["spiritual", "mystical", "mystic", "mysticism", "religious", "prayer", "contemplation", "contemplative", "meditation", "transcendence", "enlightenment", "awakening", "sacred"]),
        ("Christian Mysticism", &["christian", "christianity", "christian mysticism", "mystical", "mysticism", "mystic", "prayer", "contemplative", "contemplation", "spiritual", "abrahamic", "meditation"]),
        ("Tibetan Buddhism & Bön", &["tibetan", "tibetan buddhism", "vajrayana", "bon", "bön", "buddhist", "buddhism", "meditation", "spiritual"]),
        ("Buddhist Meditation", &["buddhist", "buddhism", "dharma", "meditation", "spiritual"]),
        ("Hindu & Yogic Meditation", &["hindu", "hinduism", "yoga", "yogic", "vedic", "meditation", "spiritual"]),
        ("Sufi Mysticism", &["sufi", "sufism", "islam", "islamic", "mystical", "mysticism", "mystic", "abrahamic", "spiritual"]),
        ("Jewish Mysticism & Kabbalah", &["jewish", "judaism", "kabbalah", "kabbala", "cabala", "mystical", "mysticism", "mystic", "abrahamic", "spiritual"]),
        ("Taoist Inner Cultivation", &["taoist", "taoism", "daoist", "daoism", "tao", "dao", "chinese", "inner cultivation", "meditation", "spiritual"]),
        ("Shamanic & Ancestral Rituals", &["shamanic", "shaman", "shamanism", "ritual", "rituals", "ancestral", "trance", "altered states"]),
        ("Western Esoteric & Classical", &["western esoteric", "esoteric", "esotericism", "western", "occult", "mystery tradition", "ritual", "spiritual"]),
        ("Trance & Hypnosis Techniques", &["trance", "hypnosis", "hypnotic", "self-hypnosis", "induction", "relaxation", "altered states"]),
        ("Astral Projection & OBE", &["astral", "astral projection", "obe", "out of body", "out-of-body", "projection", "monroe", "rope", "phase", "wbtb", "lucid", "vibrations", "astral travel", "soul travel"]),
        ("Kundalini Yoga (gentle)", &["kundalini", "kundalini yoga", "yoga", "yogic", "mantra", "chanting", "chakra", "chakras", "energy", "gentle", "meditation", "spiritual"]),
        ("Rosicrucian & Alchemical", &["rosicrucian", "rosicrucianism", "rose cross", "rosy cross", "alchemy", "alchemical", "esoteric", "western esoteric", "hermetic", "mystical", "spiritual"]),
        ("Masonic Contemplation", &["masonic", "freemasonry", "freemason", "mason", "masons", "symbolism", "esoteric", "western esoteric", "contemplation", "spiritual"]),
        ("Jain Meditation", &["jain", "jainism", "jaina", "ahimsa", "equanimity", "meditation", "spiritual"]),
        ("Sikh Simran", &["sikh", "sikhism", "simran", "naam", "naam simran", "gurbani", "kirtan", "meditation", "spiritual"]),
        ("Zoroastrian, Bahá'í & Shinto", &["world religions", "faith", "prayer", "spiritual"]),
    ];
    // (needle in lowercase text, tags to add)
    const TOPICS: &[(&str, &[&str])] = &[
        ("sleep", &["sleep"]), ("insomnia", &["sleep", "insomnia"]), ("nap", &["nap", "sleep"]),
        ("dream", &["dream", "dreams"]), ("lucid", &["lucid", "lucid dreaming"]),
        ("meditat", &["meditation"]), ("zen", &["zen", "meditation"]), ("mindful", &["mindfulness"]),
        ("focus", &["focus", "concentration"]), ("study", &["study", "learning"]), ("vigilance", &["focus", "alertness"]),
        ("attention", &["attention", "focus"]), ("adhd", &["adhd"]), ("memory", &["memory"]),
        ("anxiety", &["anxiety", "calm"]), ("stress", &["stress", "calm"]), ("relax", &["relaxation", "calm"]), ("calm", &["calm"]),
        ("creativ", &["creativity"]), ("insight", &["insight", "creativity"]), ("problem", &["problem solving"]),
        ("energ", &["energy"]), ("morning", &["morning", "energy"]), ("pain", &["pain"]), ("headache", &["headache", "pain"]),
        ("heal", &["healing"]), ("restor", &["restoration"]),
        ("astral", &["astral projection", "obe", "out of body"]), ("out-of-body", &["obe", "out of body", "astral projection"]),
        ("remote viewing", &["remote viewing", "stargate", "psi"]), ("stargate", &["stargate", "cia"]),
        ("shaman", &["shamanic", "shaman", "journey"]), ("journey", &["journey"]), ("trance", &["trance"]),
        ("hypno", &["hypnosis", "hypnotic", "trance"]), ("hypnagog", &["hypnagogic", "twilight"]),
        ("chant", &["chant", "chanting"]), ("mantra", &["mantra"]), ("rosary", &["prayer", "rosary"]), ("prayer", &["prayer"]),
        (" om", &["om", "aum"]), ("kirtan", &["kirtan", "kundalini"]), ("yoga", &["yoga"]), ("nidra", &["yoga nidra", "nsdr"]),
        ("breath", &["breathing", "breath"]), ("hrv", &["hrv", "heart rate variability"]), ("heart", &["heart"]),
        ("visualis", &["visualization", "visualisation"]), ("imagery", &["imagery"]), ("psilocybin", &["psychedelic"]),
        ("third eye", &["third eye", "pineal"]), ("pineal", &["pineal", "third eye", "dmt"]),
        ("compassion", &["compassion", "loving-kindness"]), ("gratitude", &["gratitude"]), ("bliss", &["bliss", "euphoria"]),
        ("ganzfeld", &["ganzfeld", "red light", "sensory"]), ("bowl", &["singing bowl", "tibetan", "sound bath"]),
        ("rain", &["rain", "nature"]), ("nature", &["nature"]), ("drum", &["drum", "drumming", "rhythm"]),
        ("gamma", &["gamma"]), ("40 hz", &["40 hz", "gamma"]), ("alzheimer", &["alzheimer", "genus", "memory"]),
        ("intuition", &["intuition"]), ("intention", &["intention"]), ("manifest", &["manifestation"]),
        ("grounding", &["grounding"]), ("432", &["432"]), ("528", &["528", "love frequency"]),
        ("sufi", &["sufi", "dhikr", "islam"]), ("buddhis", &["buddhist", "buddhism"]), ("tibet", &["tibetan"]), ("christian", &["christian"]),
        ("jesus", &["christian", "prayer"]), (" god", &["god"]), ("tummo", &["tummo", "inner fire"]), ("golden dawn", &["golden dawn", "hermetic", "western esoteric"]),
        ("tattva", &["tattva", "symbol"]), ("vipassana", &["vipassana", "insight meditation"]), ("jhāna", &["jhana", "absorption"]), ("jhana", &["jhana", "absorption"]),
        ("nondual", &["nondual", "non-dual"]), ("advaita", &["advaita"]), ("mirror", &["mirror"]), ("grief", &["grief", "bereavement"]),
        ("floatation", &["float", "floatation", "isolation tank"]), ("mystical", &["mystical"]), ("taoist", &["taoist", "qi"]), ("hasid", &["jewish", "hasidic"]),
        ("pranayama", &["pranayama", "yoga breathing"]), ("hyperventilat", &["hyperventilation"]), ("box breathing", &["box breathing"]),
        ("sighing", &["sighing", "physiological sigh"]), ("humming", &["humming"]), ("nostril", &["alternate nostril"]), ("flashlight", &["flashlight", "torch"]),
        ("eyes open", &["eyes open"]), ("hypnotis", &["hypnosis", "hypnotic"]), ("shamanism", &["shamanic", "shaman"]), ("kriya", &["kriya"]),
        ("kabbal", &["kabbalah"]), ("sefir", &["sefirot", "tree of life"]), ("tree of life", &["tree of life"]), ("merkavah", &["merkavah", "chariot mysticism"]),
        ("shema", &["shema"]), ("niggun", &["niggun", "singing"]), ("dervish", &["dervish"]), ("rumi", &["rumi"]), ("koan", &["koan"]),
        ("nembutsu", &["nembutsu", "pure land"]), ("amida", &["pure land"]), ("nichiren", &["nichiren", "lotus sutra"]), ("shingon", &["shingon"]),
        ("kasina", &["kasina"]), ("kasiṇa", &["kasina"]), ("mettā", &["metta", "loving-kindness"]), ("metta", &["metta", "loving-kindness"]),
        ("loving-kindness", &["loving-kindness"]), ("loving kindness", &["loving-kindness"]), ("tonglen", &["tonglen", "compassion"]),
        ("dzogchen", &["dzogchen"]), ("mahamudra", &["mahamudra"]), ("bardo", &["bardo"]), ("chöd", &["chod"]), ("japa", &["japa", "mantra"]),
        (" mala", &["mala", "prayer beads"]), ("beads", &["prayer beads"]), ("tantra", &["tantra"]), ("upanishad", &["upanishad"]), ("upaniṣad", &["upanishad"]),
        ("shiva", &["shiva"]), ("krishna", &["krishna", "bhakti"]), ("bhakti", &["bhakti"]), ("gayatri", &["gayatri"]), ("gāyatrī", &["gayatri"]),
        ("trataka", &["trataka"]), ("trāṭaka", &["trataka"]), ("candle", &["candle"]), ("gazing", &["gazing"]), ("lectio", &["lectio divina"]),
        ("ignatius", &["ignatian", "jesuit"]), ("teresa", &["teresa of avila", "carmelite"]), ("cloud of unknowing", &["cloud of unknowing"]),
        ("taizé", &["taize"]), ("quaker", &["quaker"]), ("psalm", &["psalm", "bible"]), (" icon", &["icon"]), ("vigil", &["vigil", "night prayer"]),
        ("zhuangzi", &["zhuangzi", "chuang tzu"]), ("qigong", &["qigong"]), ("power animal", &["power animal", "spirit animal"]), ("rattle", &["rattle"]),
        (" dance", &["dance"]), ("dancing", &["dance"]), ("incubat", &["dream incubation"]), ("asclepius", &["asclepius"]),
        ("plotinus", &["plotinus", "neoplatonism"]), ("hermes", &["hermes trismegistus"]), ("pentagram", &["pentagram", "banishing"]),
        ("middle pillar", &["middle pillar"]), ("bardic", &["bardic", "poetry"]), ("rune", &["runes"]), ("galdr", &["galdr"]),
        ("autogenic", &["autogenic training"]), ("erickson", &["ericksonian"]), ("eye fixation", &["eye fixation"]), ("fractionation", &["fractionation"]),
        ("wim hof", &["wim hof"]), ("arrhythmic", &["arrhythmic"]), ("afterimage", &["afterimage"]), ("lucid dream", &["lucid", "lucid dreaming"]),
    ];
    let band = |hz: f64| -> &'static str {
        if hz <= 0.0 { "" } else if hz < 0.75 { "epsilon" } else if hz < 4.0 { "delta" } else if hz < 8.0 { "theta" } else if hz < 13.0 { "alpha" }
        else if hz < 30.0 { "beta" } else if hz < 90.0 { "gamma" } else if hz < 150.0 { "hypergamma" } else { "lambda" }
    };
    for (i, p) in v.iter_mut().enumerate() {
        let mut tags: Vec<String> = Vec::new();
        let mut add = |s: &str| { let s = s.to_lowercase(); if !s.is_empty() && !tags.contains(&s) { tags.push(s); } };
        for (g, ts) in GROUP { if p.group == *g { for t in *ts { add(t); } } }
        add(&p.evidence);
        if p.evidence == "Research" { add("peer-reviewed"); add("peer reviewed"); add("science"); }
        if p.evidence == "Gateway" { add("gateway"); add("monroe"); }
        add(&p.mode);
        if p.mode == "binaural" { add("headphones"); add("binaural beats"); }
        if p.mode == "isochronic" { add("speakers"); add("pulses"); }
        match p.mode.as_str() {
            "am" => { add("speakers"); add("amplitude modulation"); add("modulated"); }
            "hybrid" => { add("headphones"); add("binaural beats"); add("isochronic"); add("pulses"); }
            "bilateral" => { add("headphones"); add("emdr"); add("alternating"); add("left right"); }
            "spatial" => { add("headphones"); add("8d"); add("8d audio"); add("panning"); }
            _ => {}
        }
        let gains_on = p.gains.iter().any(|g| *g > 0.0);
        for s in &p.stages { for b in &s.beats { if gains_on || p.texture.is_some() { add(band(*b)); } } }
        for c in &p.carriers { if *c > 0.0 { add(&format!("{}", (*c * 100.0).round() / 100.0)); } }
        if let Some(tx) = &p.texture {
            let k: &[&str] = match tx.kind.as_str() {
                "drum" => &["drum", "drumming", "shamanic", "rhythm", "trance"],
                "bowls" => &["singing bowl", "tibetan", "sound bath", "bowls"],
                "kirtan" => &["kirtan", "chant", "mantra", "bell"],
                "tlr" => &["lucid", "lucid dreaming", "dream", "sleep", "cue"],
                "breathbell" => &["breathing", "bell", "breath"],
                "pulses" => &["sleep", "deep sleep", "slow wave", "pink noise"],
                "rain" => &["rain", "nature", "soundscape"],
                "mokugyo" => &["wooden fish", "mokugyo", "chanting", "rhythm"],
                "rattle" => &["rattle", "rhythm", "trance", "shamanic"],
                "gong" => &["gong", "gong bath", "sound bath"],
                "heartbeat" => &["heartbeat", "heart", "pulse"],
                "kartal" => &["cymbals", "kartal", "kirtan", "chanting"],
                _ => &[],
            };
            for t in k { add(t); }
        }
        if let Some(m) = &p.music {
            add("music"); add("meditation music");
            for (n, part) in m.name.split('|').enumerate() {
                let spec = part.split('*').next().unwrap_or("").trim();
                let (k, var) = if n == 0 { (m.kind.as_str(), spec) } else { spec.split_once(':').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((spec, "")) };
                let ts: &[&str] = match k {
                    "tanpura" => &["tanpura", "drone", "indian music"],
                    "drone" => &["drone"],
                    "choir" => &["choir", "voices"],
                    "om" | "gyuto" => &["chant", "chanting", "voices"],
                    "flute" => &["flute"],
                    "guqin" => &["guqin", "zither", "chinese music"],
                    "harp" => &["harp"],
                    "daf" => &["daf", "frame drum", "drum"],
                    "damaru" => &["damaru", "hand drum", "drum"],
                    "rin" => &["bell"],
                    "gongbath" => &["gong", "gong bath", "sound bath"],
                    "stream" => &["stream", "water", "nature"],
                    "wind" => &["wind", "nature"],
                    "fire" => &["fire", "crackling fire"],
                    "chant" => &["chant", "chanting", "voices", "group chant"],
                    _ => &[],
                };
                for t in ts { add(t); }
                let vs: &[&str] = match (k, var) {
                    ("drone", "organ") => &["organ"],
                    ("drone", "bowl") => &["singing bowl"],
                    ("drone", "harmonium") => &["harmonium"],
                    ("choir", "organum") => &["organum", "plainchant"],
                    ("choir", "hum") => &["humming"],
                    ("choir", "ah") => &["choir"],
                    ("choir", _) => &["ison", "byzantine chant"],
                    ("om", _) => &["om", "aum"],
                    ("gyuto", _) => &["overtone chanting", "tibetan chant", "throat singing"],
                    ("flute", "shakuhachi") => &["shakuhachi", "japanese"],
                    ("flute", "xiao") => &["xiao", "chinese music"],
                    ("flute", v) if v.starts_with("bansuri") => &["bansuri", "raga"],
                    ("flute", _) => &["ney", "reed flute"],
                    ("rin", "church") => &["church bell"],
                    ("rin", "bell") => &["bell"],
                    ("rin", _) => &["rin", "singing bowl"],
                    _ => &[],
                };
                for t in vs { add(t); }
            }
        }
        if p.visual == "ganzfeld" { add("ganzfeld"); add("red light"); add("visual"); }
        if p.visual.contains("flicker") { add("flicker"); add("light journey"); add("visual"); add("hallucination"); add("strobe"); }
        if p.visual == "tattva" { add("tattva"); add("symbol"); add("afterimage"); add("visual"); }
        if p.visual.starts_with("breath:") { add("breathing guide"); add("breathing"); }
        if p.visual.starts_with("kasina") { add("kasina"); add("gazing"); add("gazing aid"); add("visual"); if p.visual == "kasina:moon" { add("moon"); } }
        if p.visual.starts_with("trataka") {
            add("gazing"); add("gazing aid"); add("visual");
            if p.visual == "trataka:point" { add("eye fixation"); } else { add("candle"); add("flame"); }
        }
        for k in &p.trad {
            let ts: &[&str] = match k.as_str() {
                "abrahamic" => &["abrahamic"],
                "christian" => &["christian", "christianity"],
                "jewish" => &["jewish", "judaism"],
                "islamic" => &["islam", "islamic", "sufi", "sufism"],
                "hindu" => &["hindu", "hinduism"],
                "vedanta" => &["vedanta", "advaita"],
                "tantra" => &["tantra", "tantric"],
                "mantra" => &["mantra", "chanting"],
                "yoga" => &["yoga", "yogic"],
                "buddhist" => &["buddhist", "buddhism", "dharma"],
                "tibetan" => &["tibetan", "tibetan buddhism", "vajrayana"],
                "theravada" => &["theravada", "pali"],
                "zen" => &["zen", "chan"],
                "eastasian" => &["mahayana", "japanese buddhism", "pure land", "chinese buddhism", "east asian buddhism"],
                "taoist" => &["taoist", "taoism", "daoist", "daoism"],
                "shamanic" => &["shamanic", "shaman", "shamanism"],
                "western" => &["western esoteric", "esoteric", "esotericism"],
                "hermetic" => &["hermetic", "hermeticism"],
                "classical" => &["classical", "ancient greek"],
                "celtic" => &["celtic", "gaelic"],
                "norse" => &["norse", "heathen"],
                "secular" => &["secular", "non-religious"],
                "mystic" => &["christian mysticism", "mystic", "mystical", "contemplative"],
                "catholic" => &["catholic", "catholicism", "roman catholic"],
                "orthodox" => &["orthodox", "eastern orthodox", "byzantine"],
                "protestant" => &["protestant", "protestantism"],
                "rosicrucian" => &["rosicrucian", "rosicrucianism", "rose cross"],
                "masonic" => &["masonic", "freemasonry", "mason"],
                "kabbalah" => &["kabbalah", "kabbala", "jewish mysticism"],
                "hasidic" => &["hasidic", "hasidism", "chassidic"],
                "mussar" => &["mussar", "jewish ethics"],
                "sufi" => &["sufi", "sufism", "dhikr"],
                "bahai" => &["bahai", "bahá'í", "baha'i"],
                "kundalini" => &["kundalini", "kundalini yoga"],
                "jain" => &["jain", "jainism"],
                "sikh" => &["sikh", "sikhism"],
                "zoroastrian" => &["zoroastrian", "zoroastrianism", "parsi"],
                "shinto" => &["shinto", "japanese"],
                "fourthway" => &["fourth way", "gurdjieff", "the work"],
                "vajrayana" => &["vajrayana", "deity yoga", "tantric buddhism"],
                "dzogchen" => &["dzogchen", "mahamudra", "great perfection", "rigpa"],
                "sixyogas" => &["six yogas", "naropa", "dream yoga", "bardo"],
                "lojong" => &["lojong", "mind training", "compassion"],
                "bon" => &["bon", "bön", "yungdrung"],
                "samatha" => &["samatha", "shamatha", "calm abiding", "concentration"],
                "vipassana" => &["vipassana", "insight meditation"],
                "brahmavihara" => &["brahmavihara", "loving-kindness", "metta"],
                "rinzai" => &["rinzai", "koan", "kōan"],
                "soto" => &["soto", "sōtō", "shikantaza"],
                "chan" => &["chan buddhism", "seon", "korean buddhism", "huatou"],
                "pureland" => &["pure land", "amida", "amitabha", "nembutsu"],
                "nichiren" => &["nichiren", "lotus sutra"],
                "shingon" => &["shingon", "esoteric buddhism"],
                "pranayama" => &["pranayama", "breath control", "yogic breathing"],
                "raja" => &["raja yoga", "patanjali", "yoga sutras", "ashtanga"],
                "hatha" => &["hatha yoga", "nada yoga"],
                "nidra" => &["yoga nidra", "yogic sleep"],
                "vedic" => &["vedic", "veda", "upanishad"],
                "shaiva" => &["shaiva", "shaivism", "shiva"],
                "vaishnava" => &["vaishnava", "krishna", "bhakti"],
                "kashmir" => &["kashmir shaivism", "trika"],
                "srividya" => &["sri vidya", "shri vidya", "shakta", "shaktism"],
                _ => &[],
            };
            for t in ts { add(t); }
        }
        if p.stages.iter().any(|s| s.breath) { add("breathing"); add("breath pacer"); }
        let text = format!(" {} {} {} {} {}", p.name, p.short, p.about,
            p.stages.iter().map(|s| format!("{} {}", s.name, s.cue)).collect::<Vec<_>>().join(" "),
            also.get(i).map(|s| s.as_str()).unwrap_or("")).to_lowercase();
        for (needle, ts) in TOPICS { if text.contains(needle) { for t in *ts { add(t); } } }
        p.tags = tags;
    }
}
