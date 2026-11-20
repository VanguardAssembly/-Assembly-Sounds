//! The session library. Every preset carries an honest evidence tag:
//!   Gateway     – interpretation of the Monroe Institute Focus-level model
//!   Research    – based on a published study (cited in `about`)
//!   Traditional – popular / historical association, not scientifically established
//!   Speculative – fringe claim; included for completeness, labelled as such
use crate::plan::{cue_id, Preset, Stage};

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
    }
}
trait StageExt {
    fn breath(self) -> Self;
    fn car(self, c: &[f64]) -> Self;
    fn ramp(self, s: f64) -> Self;
}
impl StageExt for Stage {
    fn breath(mut self) -> Self { self.breath = true; self }
    fn car(mut self, c: &[f64]) -> Self { self.carriers = Some(c.to_vec()); self }
    fn ramp(mut self, s: f64) -> Self { self.ramp = Some(s); self }
}
fn with_min(mut s: Stage, min: f64) -> Stage { s.min = min; s }

#[allow(clippy::too_many_arguments)]
fn p(group: &str, id: &str, name: &str, short: &str, carriers: &[f64], gains: &[f64], noise: &str,
     mode: &str, evidence: &str, about: &str, stages: Vec<Stage>) -> Preset {
    Preset {
        id: id.into(), group: group.into(), name: name.into(), short: short.into(),
        carriers: carriers.to_vec(), gains: gains.to_vec(), noise: noise.into(), mode: mode.into(),
        evidence: evidence.into(), about: about.into(), stages, live: false,
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
    let g = "Gateway Experience";
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
    let g = "Lifeline (Focus 22–27)";
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
