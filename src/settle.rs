//! A tailored "settle in" cue for every session.
//!
//! The first stage of a session is where the listener arrives, so it should say what
//! *this* session is and how to get ready for it. Many older sessions shared one generic
//! line; `apply` gives each of them its own opening (hand-written for individual sessions,
//! generated from the session's own parameters for the large frequency families).
//! Only the first stage's spoken text changes: names, timing, beats and tones stay as they were.
//! Sessions whose opening was already specific to them keep it.
use crate::plan::Preset;

pub fn apply(v: &mut [Preset]) {
    for p in v.iter_mut() {
        let text = match hand(&p.id) {
            Some(t) => Some(t.to_string()),
            None => family(p),
        };
        if let (Some(t), Some(s)) = (text, p.stages.first_mut()) {
            s.cue = t;
        }
    }
}

/// Hand-written openings, one per session.
fn hand(id: &str) -> Option<&'static str> {
    Some(match id {
        // ---- Gateway & Lifeline
        "f3" => "Welcome to Focus 3, the first step. Lie back or sit comfortably, let your eyes close, and simply listen as the three layers of tone begin to blend into one.",
        "f10" => "Welcome to Focus 10, mind awake, body asleep. Lie back and loosen anything tight. Let your eyes close. Your body will drift toward sleep while your mind stays clear, so let the tones do the work.",
        "f12" => "Welcome to Focus 12, expanded awareness. Lie back and let your eyes close. After the preparation you will pass through Focus 10, then let your awareness widen beyond the edges of your body. For now, just listen.",
        "f15" => "Welcome to Focus 15, the state of no time. Lie back, loosen anything tight, and let your eyes close. After the preparation we pass through Focus 10 and 12 into deep stillness. Let go of the clock.",
        "f21" => "Welcome to Focus 21, the bridge. This is the longest Gateway session, so make sure you are warm and comfortable. Let your eyes close. We will move step by step, through Focus 10, 12 and 15, to the edge of the bridge.",
        "f22" => "Welcome to Focus 22, the dreamlike edge. Lie back and let your eyes close. We travel through the familiar Focus levels first, then rest where waking and dreaming overlap. Let images come and go.",
        "f23" => "Welcome to Focus 23, quiet reflection. Lie back, let your eyes close, and allow yourself to feel safe and held. This is a gentle session for memories and feelings. You can open your eyes at any time.",
        "f25" => "Welcome to Focus 24 to 26, the belief system levels. Lie back and let your eyes close. This session invites you to look at your own beliefs with curiosity. First, let the tones carry you through the familiar levels.",
        "f27" => "Welcome to Focus 27, the park. Lie back and let your eyes close. At the end of this journey you will create a peaceful place of your own. For now, let the tones begin to slow you down.",
        "ap_monroe" => "Welcome to Monroe's vibrational state and roll out. Lie flat on your back if you can, and loosen anything tight. Let your eyes close. If buzzing or tingling comes later, that is expected. Stay calm and curious.",

        // ---- Research-based
        "alpha" => "Welcome to Alpha Calm. Sit or lie back and let your eyes close. Let your breathing slow a little, and let your shoulders drop. The beat will ease you into the relaxed alpha rhythm of ten hertz.",
        "theta" => "Welcome to Theta Meditation. Sit comfortably, eyes closed, and let your attention rest on your breathing. Over the next minutes the beat slows toward six hertz, the rhythm of deep meditation.",
        "delta" => "Welcome to Deep Sleep. Lie down in bed, turn the volume very low, and let the day go. Over the next half hour the beat slows toward three hertz as you drift into sleep.",
        "beta" => "Welcome to Beta Focus. Sit upright and open the task you are about to work on. Put your phone out of reach. In three minutes the beat rises to sixteen hertz to help you stay alert.",
        "gamma" => "Welcome to Gamma. Sit comfortably with your eyes closed, and let your breathing slow. In a few minutes the beat rises to forty hertz, a bright, fast rhythm studied for creative thinking. Stay relaxed and alert.",
        "genus" => "Welcome to the forty hertz session. These are quick, even pulses of sound, the kind used in the GENUS studies. Keep the volume comfortable and low. Sit or lie back, and simply let the pulses run.",
        "anx" => "Welcome to Anxiety Ease. You are safe, and there is nothing to fix right now. Breathe in for five seconds, and out for five. Let the slow rhythm of your breath do the calming.",
        "alphatheta" => "Welcome to Alpha Theta training. Sit back and let your eyes close. For the first part, simply rest in calm alpha. Later the beat slips toward the border of theta, where dreamlike images begin.",
        "nap" => "Welcome to the Power Nap. Lie down, close your eyes, and let your body grow heavy. You will rest for about ten minutes, then the beat brings you gently back up, awake and refreshed.",
        "smr" => "Welcome to SMR Calm Focus. Sit upright and bring your attention to the task in front of you. The beat settles at thirteen and a half hertz, the calm, steady focus rhythm. Breathe evenly, and begin.",
        "monks" => "Welcome to Compassion Gamma. Sit comfortably, eyes closed, and rest a hand on your heart if you like. Let your breathing slow. Soon you will bring to mind someone you love.",

        // ---- Purpose & Tradition
        "obe" => "Welcome to the astral projection session. Lie flat on your back, arms by your sides, and let your eyes close. Let your breathing slow. First we relax the body completely. Then comes the rope.",
        "rv" => "Welcome to remote viewing practice. Have a pen and paper ready, and ask a friend to choose a hidden picture or place as your target. Then sit back, close your eyes, and cool down.",
        "shaman" => "Welcome to the shamanic journey. Lie down and cover your eyes. Set your intention for the journey: a question, or a place you wish to visit. Hold it simply, in a few words.",
        "pineal" => "Welcome to the third eye session. Sit upright and comfortable, and let your eyes close. Let your breathing slow. Soon you will rest your attention lightly at the point between your eyebrows.",
        "om" => "Welcome to Om meditation. Sit upright, with your spine tall and relaxed, and let your eyes close. Take a few slow breaths. Soon you will chant, or silently hear, Om on each breath out, matched to the tone.",
        "heal" => "Welcome to Deep Restoration. Lie down where you can fall asleep, and keep the volume very low. Let your body sink into the bed. This long, slow session glides down into deep delta sleep.",
        "vis" => "Welcome to Visualisation. Sit or lie back and close your eyes. Bring to mind one goal that matters to you, and hold it loosely for now. In a few minutes you will picture it in rich detail.",
        "study" => "Welcome to Study and Memory. Open your book or notes, and clear your desk of distractions. Sit upright and take three slow breaths. Then begin reading, and let the steady tone hold your focus.",

        // ---- Sound Systems Library
        "sys_binaural" => "This is the binaural beats demo. Wear stereo headphones: each ear hears a slightly different tone, and your brain creates the slow six hertz wobble. Close your eyes and listen for it.",
        "sys_monaural" => "This is the monaural beats demo. The two tones are mixed before they reach you, so the beat is a real pulse in the air, and speakers work. Close your eyes and listen to the throb.",
        "sys_isochronic" => "This is the isochronic tones demo: one tone switching cleanly on and off, six times a second. Speakers are fine. Close your eyes and let the even pulse hold your attention.",
        "sys_am" => "This is the amplitude modulation demo. The tone swells and fades smoothly, like a slow tremolo, gentler than a pulse. Speakers are fine. Close your eyes and ride the waves of sound.",
        "sys_hybrid" => "This is the hybrid demo: a binaural pair that is also pulsed. Headphones work best. Close your eyes and notice two things at once: the wobble between your ears, and the pulse within the tone.",
        "sys_bilateral" => "This is the bilateral tones demo. Wear headphones. The tone moves from your left ear to your right and back, about once a second. Close your eyes and let your attention follow it, side to side.",
        "sys_spatial" => "This is the spatial audio demo. Wear headphones. The tone drifts slowly from side to side around your head, about once every ten seconds. Close your eyes and follow it as it travels.",
        "sys_layered" => "This is the layered beats demo, in the Hemi-Sync style. Three pairs of tones play at once, each with its own beat. Close your eyes and listen to the chord of rhythms settle into one.",
        "sys_descend" => "This is the descending carriers demo. Layered beats sink in stages, from alpha down to delta, over half an hour. Lie back, close your eyes, and let each step take you a little deeper.",
        "sys_noise" => "This is the coloured noise demo: no tones and no beats, just steady pink noise. Use it to compare how you feel with and without beats. Close your eyes and rest in the soft rush of sound.",
        "sys_drum" => "This is the rhythmic drumming demo, the oldest sound system of all. No tones, only the drum. Close your eyes, and let its steady beat, about four times a second, carry your attention inward.",

        // ---- Astral Projection & OBE
        "ap_rope" => "Welcome to Robert Bruce's rope technique. Lie on your back with your arms by your sides, and let your eyes close. The pulsing beats help you relax deeply first. Keep your mind awake as your body lets go.",
        "ap_muldoon" => "Welcome to the target technique. Before you relax, choose a familiar room as your target, and picture it clearly for a moment. Now lie back, close your eyes, and let your breathing slow.",
        "ap_phase" => "This is the phase technique. Use it right after waking, ideally after about six hours of sleep. Do not move, and do not open your eyes. Stay still, relaxed and quiet, with the volume very low.",
        "ap_wbtb" => "Welcome back to bed. You have been up for a little while; now lie down comfortably. Each time you hear the chime, repeat silently: next time I am dreaming, I will remember that I am dreaming.",
        "ap_theta_gamma" => "Welcome to the theta gamma stack. Lie back and let your eyes close. Two beats play together: a slow theta beat for depth, and a quiet forty hertz beat for clarity. Let your breathing slow.",
        "ap_float" => "Welcome to floating. Wear headphones and keep the volume low. Lie back, close your eyes, and let your body grow heavy. Soon the sound begins to drift slowly around your head.",
        "ap_bilateral" => "Welcome. This session calms you before you try to leave the body. Wear headphones. Lie back, close your eyes, and let the tone sway gently from ear to ear. With each sway, let a little fear go.",
        "ap_ground" => "Welcome to grounding, for after any deep practice. Sit up slowly. Place both feet flat on the floor, and press them down. Feel how solid the floor is, and how awake your body is becoming.",

        // ---- Light Journeys
        "lj_alpha" => "Welcome to the ten hertz light journey. Dim the room, sit facing the light, and close your eyes. When you are ready, press Light journey and accept the warnings. The flashes lock to the ten hertz beat.",
        "lj_sweep" => "Welcome to the sweet spot sweep. Close your eyes and face the light. The flicker starts at eight hertz and climbs one step every two minutes. Notice which speed gives you the brightest colours and patterns.",
        "lj_geometry" => "Welcome to the kaleidoscope. Close your eyes and face the light. At twelve hertz, the fastest rate here, look for crisp shapes: checkerboards, spirals, honeycombs, and lines radiating from the centre.",
        "lj_descent" => "Welcome to the imagery descent. Close your eyes and face the light. The journey begins with bright ten hertz patterns, then slows toward theta, where scenes can become soft and dreamlike.",
        "lj_dream" => "Welcome to the Dreamachine wander. Sit with your eyes closed and your face close to the warm light, as Gysin's guests once sat by the spinning cylinder. The rhythm drifts between eight and twelve hertz.",
        "lj_ganzflicker" => "Welcome to Ganzflicker, eyes closed. The light alternates between red and black, about ten times a second. Close your eyes, face the screen, and let the red glow fill your whole field of view.",

        // ---- Infrasound
        "inf_ghost" => "Welcome to the ghost frequency. Keep the volume low. If you wish, press Infrasound and accept the warning to add the 18.98 hertz layer. Sit in a dim room, close your eyes, and notice how you feel, without expectation.",
        "inf_concert" => "Welcome to Soundless Music, the seventeen hertz experiment. Keep the volume low, and add the infrasound layer if you choose. Then close your eyes, and notice any change in mood, with curiosity rather than fear.",
        "inf_7" => "Welcome to the deep body rumble. Keep the volume low. Seven hertz is felt more than heard, and most headphones barely reproduce it. Lie back, close your eyes, and notice any sense of vibration in your chest or belly.",
        "inf_schumann" => "Welcome to the Schumann resonance, played as sound. Keep the volume low, and add the infrasound layer if you wish. Lie back, close your eyes, and let the slow 7.83 hertz rhythm settle under the tone.",

        // ---- Breathwork
        "br_power" => "Welcome to power breathing. Sit or lie down somewhere safe, never in or near water. When you are ready, press Breathwork and start the guide. Begin with a few calm breaths, and never force a breath hold.",
        "br_connected" => "Welcome to connected breathing. Lie down on a soft surface somewhere safe, never in water. Press Breathwork to start the guide. Breathe fully, with no pause between in and out, and slow down if tingling gets strong.",

        // ---- Frequency families with one-of-a-kind sessions
        "schstack" => "Welcome to the Schumann stack. All five resonance modes play at once, each on its own tone, like a chord from the sky. Lie back, close your eyes, and rest in the layered sound.",
        "soljourney" => "Welcome to the full Solfeggio journey: nine tones, three minutes each, rising from 174 to 963 hertz. Close your eyes. We begin at 174 hertz, the tone of foundation and grounding.",
        "chjourney" => "Welcome to the chakra journey, from root to crown. Sit tall and close your eyes. We begin at the base of the spine. Feel your connection to the ground beneath you, stable and safe.",
        "t432" => "Welcome to the 432 hertz tuning. Sit back and close your eyes. Compare how this pitch feels with the 440 hertz session. A gentle eight hertz alpha beat rides on the tone.",
        "t440" => "Welcome to concert pitch, 440 hertz, the international tuning standard. Sit back and close your eyes, and use it as a comparison for 432. An eight hertz alpha beat rides on the tone.",
        "t256" => "Welcome to scientific pitch, with middle C at 256 hertz. Sit back and close your eyes. A ten hertz alpha beat rides on the tone. Simply rest, and listen.",
        _ => return None,
    })
}

/// Openings generated from each family member's own parameters.
fn family(p: &Preset) -> Option<String> {
    let id = p.id.as_str();
    if let Some(phrase) = lib_phrase(id) {
        let hz = p.stages.get(1).and_then(|s| s.beats.first()).copied().unwrap_or(10.0);
        return Some(format!("Tune in to {} {} After this minute, the beat glides to {} hertz and holds there.", phrase, posture(hz), spoken(hz)));
    }
    if let Some(n) = id.strip_prefix("sch") {
        let (hz, what) = match n {
            "1" => ("7.83", "the fundamental resonance between the Earth and the sky"),
            "2" => ("14.3", "the second harmonic of the Earth's resonance"),
            "3" => ("20.8", "the third harmonic, brighter and faster"),
            "4" => ("27.3", "the fourth harmonic, high in the beta range"),
            "5" => ("33.8", "the fifth harmonic, at the edge of gamma"),
            _ => return None,
        };
        let pose = if n == "1" { "Close your eyes and let your breathing slow." } else { "Sit comfortably and stay softly alert." };
        return Some(format!("Welcome to Schumann mode {}, {} hertz: {}. {} After this minute, the beat glides into place and holds.", n, hz, what, pose));
    }
    if let Some(n) = id.strip_prefix("sol") {
        let what = match n {
            "174" => "foundation and grounding",
            "285" => "restoration",
            "396" => "releasing fear and guilt",
            "417" => "facilitating change",
            "528" => "transformation; it is often called the love frequency",
            "639" => "connection and relationships",
            "741" => "expression and problem solving",
            "852" => "intuition",
            "963" => "the crown, and oneness",
            _ => return None,
        };
        return Some(format!("Welcome to Solfeggio {} hertz, traditionally linked to {}. Sit comfortably and close your eyes. Let your breathing slow; in two minutes a gentle six hertz theta beat begins.", n, what));
    }
    if let Some(n) = id.strip_prefix("ch") {
        let (name, place) = match n {
            "396" => ("root", "the base of your spine"),
            "417" => ("sacral", "your lower belly"),
            "528" => ("solar plexus", "just above your navel"),
            "639" => ("heart", "the centre of your chest"),
            "741" => ("throat", "your throat"),
            "852" => ("third eye", "the space between your eyebrows"),
            "963" => ("crown", "the top of your head"),
            _ => return None,
        };
        return Some(format!("Welcome to the {} chakra meditation. Sit with your spine tall and your eyes closed, and let your breathing slow. Soon your attention will rest at {}.", name, place));
    }
    if let Some(n) = id.strip_prefix("pl") {
        let (name, hz) = match n {
            "13610" => ("the Earth year, the traditional tone of Om", "136.1"),
            "19418" => ("the Earth day", "194.18"),
            "12622" => ("the Sun", "126.22"),
            "21042" => ("the Moon", "210.42"),
            "14127" => ("Mercury", "141.27"),
            "22123" => ("Venus", "221.23"),
            "14472" => ("Mars", "144.72"),
            "18358" => ("Jupiter", "183.58"),
            "14785" => ("Saturn", "147.85"),
            "20736" => ("Uranus", "207.36"),
            "21144" => ("Neptune", "211.44"),
            "14025" => ("Pluto", "140.25"),
            _ => return None,
        };
        return Some(format!("Welcome to the tone of {}, {} hertz, from Hans Cousto's cosmic octave. Lie back and close your eyes. Let your breathing slow; in two minutes a six hertz theta beat begins.", name, hz));
    }
    None
}

fn posture(hz: f64) -> &'static str {
    if hz < 4.0 { "Lie down, keep the volume low, and let your body grow heavy." }
    else if hz < 8.0 { "Close your eyes and let your breathing slow." }
    else if hz < 13.0 { "Close your eyes and let your shoulders soften." }
    else if hz < 30.0 { "Sit upright, comfortable and alert." }
    else if hz < 50.0 { "Sit comfortably, with your eyes closed or softly open." }
    else { "Keep the volume comfortable and low." }
}

/// A number the way it should be read aloud (no trailing zeros).
fn spoken(hz: f64) -> String {
    let s = format!("{:.2}", hz);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// One spoken phrase per Brainwave Library frequency.
fn lib_phrase(id: &str) -> Option<&'static str> {
    Some(match id {
        "lib50" => "epsilon, half a hertz, below delta: a rhythm some link to very deep meditation.",
        "lib100" => "delta, one hertz: the slow rhythm of deep, dreamless sleep.",
        "lib150" => "delta, 1.5 hertz: restorative sleep, and Monroe's body layer.",
        "lib200" => "delta, two hertz: deep sleep and detachment.",
        "lib250" => "delta, 2.5 hertz, a rate popularly linked to easing pain.",
        "lib300" => "delta, three hertz: the deep sleep rhythm studied in the lab.",
        "lib350" => "delta, 3.5 hertz, popularly linked to receptivity and unity.",
        "lib400" => "theta, four hertz: the threshold between waking and sleep.",
        "lib450" => "theta, 4.5 hertz: the rate of the shamanic drum.",
        "lib500" => "theta, five hertz, popularly linked to intuition and problem solving.",
        "lib550" => "theta, 5.5 hertz, popularly linked to inner guidance.",
        "lib600" => "theta, six hertz: the meditation rhythm.",
        "lib630" => "theta, 6.3 hertz, the rate popularly claimed for astral projection.",
        "lib700" => "theta, seven hertz: mental imagery and visualisation.",
        "lib750" => "theta, 7.5 hertz: the creative border between alpha and theta.",
        "lib783" => "7.83 hertz, the Schumann resonance of the Earth.",
        "lib800" => "alpha, eight hertz: relaxed, receptive learning.",
        "lib850" => "alpha, 8.5 hertz: relaxed awareness.",
        "lib900" => "alpha, nine hertz: calm body awareness.",
        "lib1000" => "alpha, ten hertz: the classic relaxed alpha peak.",
        "lib1050" => "alpha, 10.5 hertz, popularly linked to mind and body healing.",
        "lib1100" => "alpha, eleven hertz: relaxed alertness.",
        "lib1200" => "alpha, twelve hertz: centred and mentally steady.",
        "lib1300" => "SMR, thirteen hertz: calm, steady focus.",
        "lib1400" => "SMR, fourteen hertz: focused attention.",
        "lib1500" => "low beta, fifteen hertz: active thinking.",
        "lib1600" => "beta, sixteen hertz: vigilance.",
        "lib1800" => "beta, eighteen hertz: alertness and engagement.",
        "lib2000" => "beta, twenty hertz: energy and arousal.",
        "lib2400" => "beta, twenty four hertz, used in early vigilance research.",
        "lib2500" => "beta, twenty five hertz: high alertness.",
        "lib3000" => "high beta, thirty hertz: strong stimulation, best used sparingly.",
        "lib3300" => "gamma, thirty three hertz, at the low edge of gamma.",
        "lib3500" => "gamma, thirty five hertz, in low gamma.",
        "lib3800" => "gamma, thirty eight hertz, just below the classic forty.",
        "lib4000" => "gamma, forty hertz: the most studied gamma rhythm, linked to perceptual binding.",
        "lib4500" => "gamma, forty five hertz, just above the classic forty.",
        "lib5000" => "gamma, fifty hertz, played as quick isochronic pulses.",
        "lib6300" => "high gamma, sixty three hertz, as isochronic pulses.",
        "lib8000" => "high gamma, eighty hertz, as rapid isochronic pulses.",
        "lib10000" => "hypergamma, one hundred hertz, a speculative band from popular sources.",
        "lib20000" => "lambda, two hundred hertz, a speculative band from popular sources.",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    /// Every session must open with its own words.
    #[test]
    fn every_first_cue_is_unique() {
        let v = crate::presets::all();
        let mut seen = std::collections::HashMap::new();
        for p in &v {
            let c = p.stages[0].cue.clone();
            assert!(!c.is_empty(), "{} has an empty opening cue", p.id);
            if let Some(other) = seen.insert(c, p.id.clone()) {
                panic!("{} and {} share the same opening cue", other, p.id);
            }
        }
    }
}
