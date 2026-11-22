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
    let first: Vec<String> = v.iter().map(|p| p.stages.first().map(|s| s.cue.clone()).unwrap_or_default()).collect();
    crate::settle::apply(&mut v);
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
        ("Astral Projection & OBE", &["astral", "astral projection", "obe", "out of body", "out-of-body", "projection", "monroe", "rope", "phase", "wbtb", "lucid", "vibrations", "astral travel", "soul travel"]),
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
                _ => &[],
            };
            for t in k { add(t); }
        }
        if p.visual == "ganzfeld" { add("ganzfeld"); add("red light"); add("visual"); }
        if p.visual.contains("flicker") { add("flicker"); add("light journey"); add("visual"); add("hallucination"); add("strobe"); }
        if p.visual == "tattva" { add("tattva"); add("symbol"); add("afterimage"); add("visual"); }
        if p.visual.starts_with("breath:") { add("breathing guide"); add("breathing"); }
        if p.stages.iter().any(|s| s.breath) { add("breathing"); add("breath pacer"); }
        let text = format!(" {} {} {} {} {}", p.name, p.short, p.about,
            p.stages.iter().map(|s| format!("{} {}", s.name, s.cue)).collect::<Vec<_>>().join(" "),
            also.get(i).map(|s| s.as_str()).unwrap_or("")).to_lowercase();
        for (needle, ts) in TOPICS { if text.contains(needle) { for t in *ts { add(t); } } }
        p.tags = tags;
    }
}
