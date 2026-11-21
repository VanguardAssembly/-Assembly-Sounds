//! The session library. Every preset carries an honest evidence tag:
//!   Gateway     – interpretation of the Monroe Institute Focus-level model
//!   Research    – based on a published study (cited in `about`)
//!   Traditional – popular / historical association, not scientifically established
//!   Speculative – fringe claim; included for completeness, labelled as such
use crate::plan::{cue_id, Preset, Stage, Texture};

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
    }
}
trait StageExt {
    fn breath(self) -> Self;
    fn car(self, c: &[f64]) -> Self;
    fn ramp(self, s: f64) -> Self;
    fn tex(self, g: f64) -> Self;
}
impl StageExt for Stage {
    fn breath(mut self) -> Self { self.breath = true; self }
    fn car(mut self, c: &[f64]) -> Self { self.carriers = Some(c.to_vec()); self }
    fn ramp(mut self, s: f64) -> Self { self.ramp = Some(s); self }
    fn tex(mut self, g: f64) -> Self { self.tex = Some(g); self }
}
fn with_min(mut s: Stage, min: f64) -> Stage { s.min = min; s }
fn tx(kind: &str, level: f64, root: f64, interval: f64) -> Option<Texture> {
    Some(Texture { kind: kind.into(), level, root, interval })
}
fn with_tex(mut p: Preset, t: Option<Texture>) -> Preset { p.texture = t; p }

#[allow(clippy::too_many_arguments)]
fn p(group: &str, id: &str, name: &str, short: &str, carriers: &[f64], gains: &[f64], noise: &str,
     mode: &str, evidence: &str, about: &str, stages: Vec<Stage>) -> Preset {
    Preset {
        id: id.into(), group: group.into(), name: name.into(), short: short.into(),
        carriers: carriers.to_vec(), gains: gains.to_vec(), noise: noise.into(), mode: mode.into(),
        evidence: evidence.into(), about: about.into(), stages, live: false,
        texture: None, visual: String::new(), tags: Vec::new(), info: None,
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
    v.push(p(g, "ap_ground", "After-Practice Grounding", "Integration · 10 min", &[194.18], &[0.8], "surf", "binaural", "Traditional",
      "<p>A short alpha-to-beta return for after any deep or out-of-body practice: feel your weight, the ground, and come back fully alert. Many traditions stress \"grounding\" after inner journeys; physiologically it simply brings you out of drowsy, dreamy states before you stand up or drive.</p>",
      vec![st("Ground", 4.0, &[10.0], C_GROUND), st("Weight", 3.0, &[12.0], "Feel the weight of your body supported by the earth beneath you."),
           st("Awake", 3.0, &[16.0], C_COUNT)]));

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

    crate::info::fill(&mut v);
    auto_tags(&mut v);

    // assign cue ids
    for pr in v.iter_mut() {
        for s in pr.stages.iter_mut() {
            s.cue_id = if s.cue.is_empty() { String::new() } else { cue_id(&s.cue) };
        }
    }
    v
}

fn crate_band(hz: f64) -> &'static str {
    if hz < 0.75 { "Epsilon" } else if hz < 4.0 { "Delta" } else if hz < 8.0 { "Theta" } else if hz < 13.0 { "Alpha" } else if hz < 30.0 { "Beta" } else if hz < 90.0 { "Gamma" } else if hz < 150.0 { "Hypergamma" } else { "Lambda" }
}

/// Give every session searchable tags: group synonyms, evidence, bands, beat type,
/// sound texture, carrier tones and topic keywords found in its text.
fn auto_tags(v: &mut [Preset]) {
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
        ("Astral Projection & OBE", &["astral", "astral projection", "obe", "out of body", "out-of-body", "projection", "monroe", "rope", "phase", "wbtb", "lucid", "vibrations"]),
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
    ];
    let band = |hz: f64| -> &'static str {
        if hz <= 0.0 { "" } else if hz < 0.75 { "epsilon" } else if hz < 4.0 { "delta" } else if hz < 8.0 { "theta" } else if hz < 13.0 { "alpha" }
        else if hz < 30.0 { "beta" } else if hz < 90.0 { "gamma" } else if hz < 150.0 { "hypergamma" } else { "lambda" }
    };
    for p in v.iter_mut() {
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
                _ => &[],
            };
            for t in k { add(t); }
        }
        if p.visual == "ganzfeld" { add("ganzfeld"); add("red light"); add("visual"); }
        if p.stages.iter().any(|s| s.breath) { add("breathing"); add("breath pacer"); }
        let text = format!(" {} {} {} {}", p.name, p.short, p.about,
            p.stages.iter().map(|s| format!("{} {}", s.name, s.cue)).collect::<Vec<_>>().join(" ")).to_lowercase();
        for (needle, ts) in TOPICS { if text.contains(needle) { for t in *ts { add(t); } } }
        p.tags = tags;
    }
}
