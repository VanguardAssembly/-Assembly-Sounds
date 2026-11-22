//! More of the world's contemplative traditions (Kundalini, Rosicrucian, Jain, Sikh, Zoroastrian,
//! Bahá'í, Shinto, and more Buddhist, Hindu, Sufi, Jewish, Christian, Taoist, Western and
//! shamanic practice), and the accompaniment every traditional session plays: the drone, chant,
//! flute, drum or bells its practitioners would hear (music.rs, chant/*.ogg).
//! A child module of presets, so it shares the session builders.
use super::{mx, p, st, tx, with_tex, StageExt, C_RETURN_SHORT};
use crate::plan::{Preset, Stage, Texture};

// ---------------------------------------------------------------- accompaniment of earlier sessions
// (id, kind, variant or chant clips, level, root Hz, interval s, music gain for each stage).
// Gains follow the practice: full while the tradition chants or plays, soft while you work
// inwardly, and silent where silence itself is the practice. Beats, tones and stages are untouched.
type M = (&'static str, &'static str, &'static str, f64, f64, f64, &'static [f64]);
const MUSIC: &[M] = &[
    // Christian
    ("cm_lectio", "drone", "organ|rin:church*0.9", 0.8, 0.0, 0.0, &[0.6, 0.35, 0.35, 0.35, 0.25, 0.5]),
    ("cm_maranatha", "rin", "church", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0]),
    ("cm_ignatian", "drone", "organ|rin:church*0.9", 0.8, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.25, 0.35, 0.5]),
    ("cm_castle", "choir", "hum|rin:church*0.9", 0.8, 0.0, 0.0, &[0.5, 0.45, 0.4, 0.3, 0.2, 0.15, 0.4]),
    ("cm_cloud", "choir", "organum", 0.7, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.3, 0.4]),
    ("cm_taize", "chant", "ubicaritas", 1.0, 0.0, 0.0, &[0.4, 1.0, 0.5, 0.0]),
    ("cm_vigil", "chant", "inmanus", 0.9, 0.0, 0.0, &[0.6, 0.25, 0.0, 0.0]),
    ("cm_bestill", "chant", "vacate", 0.9, 0.0, 0.0, &[0.6, 0.5, 0.4, 0.3, 0.2, 0.0]),
    ("cm_visio", "choir", "ison|rin:church*0.8", 0.7, 0.0, 0.0, &[0.5, 0.35, 0.3, 0.35, 0.3]),
    ("cm_rosary", "chant", "avemaria", 0.9, 0.0, 0.0, &[0.3, 0.8, 0.8, 0.8, 0.8, 0.8, 0.0]),
    // Tibetan Buddhism & Bön
    ("tb_shamatha", "rin", "bell", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0]),
    ("tb_tonglen", "drone", "bowl|rin:bell*0.8", 0.8, 0.0, 0.0, &[0.5, 0.4, 0.35, 0.3, 0.35, 0.4]),
    ("tb_mani", "chant", "mani", 1.0, 0.0, 0.0, &[0.3, 0.3, 1.0, 0.2, 0.0]),
    ("tb_tara", "chant", "tara", 1.0, 0.0, 0.0, &[0.3, 0.3, 1.0, 0.2, 0.0]),
    ("tb_medicine", "chant", "medicine", 1.0, 0.0, 0.0, &[0.3, 0.3, 1.0, 0.3, 0.0]),
    ("tb_vajra", "gyuto", "", 0.8, 0.0, 0.0, &[0.4, 0.35, 0.25, 0.2]),
    ("tb_phat", "drone", "bowl", 0.8, 0.0, 0.0, &[0.4, 0.3, 0.0, 0.0, 0.2, 0.3]),
    ("tb_tilopa", "rin", "bell", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
    ("tb_chod", "damaru", "|rin:bell*0.7", 0.9, 0.0, 0.0, &[0.4, 0.5, 0.6, 0.6, 0.7, 0.2, 0.3]),
    ("tb_bardo", "gyuto", "", 0.7, 0.0, 0.0, &[0.3, 0.3, 0.3, 0.3, 0.3, 0.25, 0.15, 0.3]),
    ("tb_gyuto", "gyuto", "", 1.0, 0.0, 0.0, &[0.3, 0.7, 1.0, 0.0]),
    ("tb_sleepyoga", "chant", "white_a", 0.8, 0.0, 0.0, &[0.4, 0.3, 0.15, 0.0]),
    ("tb_sky", "wind", "", 0.8, 0.0, 0.0, &[0.5, 0.5, 0.4, 0.4]),
    // Buddhist
    ("bu_metta", "chant", "metta", 0.8, 0.0, 0.0, &[0.6, 0.3, 0.25, 0.25, 0.2, 0.4]),
    ("bu_brahma", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
    ("bu_anapana", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
    ("bu_noting", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0]),
    ("bu_mu", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0]),
    ("bu_shikantaza", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0]),
    ("bu_nembutsu", "chant", "nembutsu", 1.0, 0.0, 0.0, &[0.0, 0.0, 1.0, 0.15]),
    ("bu_daimoku", "chant", "daimoku", 1.0, 0.0, 0.0, &[0.0, 1.0, 0.8, 0.0]),
    ("bu_kasina", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0]),
    ("bu_ajikan", "chant", "white_a", 0.9, 0.0, 0.0, &[0.4, 0.8, 0.25, 0.2, 0.0]),
    // Hindu & yogic
    ("hy_trataka", "tanpura", "", 0.7, 0.0, 0.0, &[0.5, 0.4, 0.35, 0.4, 0.35]),
    ("hy_soham", "tanpura", "", 0.7, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.2]),
    ("hy_vbt", "tanpura", "", 0.7, 0.0, 0.0, &[0.4, 0.35, 0.0, 0.3, 0.3, 0.15, 0.3]),
    ("hy_bija", "chant", "bija_lam@1,bija_vam@2,bija_ram@3,bija_yam@4,bija_ham@5,bija_om@6|tanpura*0.5", 1.0, 128.0, 0.0, &[0.4, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.3]),
    ("hy_shiva", "chant", "shiva|drone:harmonium*0.6", 1.0, 138.59, 0.0, &[0.3, 1.0, 0.8, 0.0]),
    ("hy_gayatri", "chant", "gayatri|tanpura*0.6", 1.0, 138.59, 0.0, &[0.3, 0.6, 1.0, 0.2]),
    ("hy_mahamantra", "chant", "mahamantra|drone:harmonium*0.6", 1.0, 138.59, 0.0, &[0.0, 0.35, 1.0, 0.0]),
    ("hy_antarmouna", "tanpura", "", 0.7, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.3, 0.25, 0.2, 0.3]),
    ("hy_mandukya", "chant", "aum_a@1,aum_u@2,aum_m@3|tanpura*0.5", 1.0, 136.1, 0.0, &[0.3, 1.0, 1.0, 1.0, 0.0, 0.3]),
    // Sufi
    ("su_sema", "flute", "ney:hijaz|daf*0.5", 0.9, 0.0, 0.0, &[0.4, 0.6, 0.9, 1.0, 0.0]),
    ("su_muraqaba", "flute", "ney", 0.7, 0.0, 0.0, &[0.4, 0.3, 0.2, 0.2, 0.3]),
    ("su_lataif", "flute", "ney", 0.6, 0.0, 0.0, &[0.4, 0.25, 0.25, 0.2, 0.2, 0.15, 0.3]),
    ("su_breaths", "flute", "ney:rast", 0.6, 0.0, 0.0, &[0.4, 0.3, 0.3, 0.3, 0.3, 0.2]),
    ("su_names", "chant", "yalatif@1,yarahman@2,yanur@3,yawadud@4,hu@5", 1.0, 0.0, 0.0, &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
    // Jewish
    ("jk_shema", "chant", "shema@1-2,baruch@3,niggun_hum@4", 1.0, 0.0, 0.0, &[0.0, 1.0, 0.8, 1.0, 0.4, 0.0]),
    ("jk_abulafia", "chant", "aleph", 0.9, 0.0, 0.0, &[0.3, 1.0, 0.6, 0.0, 0.0]),
    ("jk_tree", "chant", "niggun_hum", 0.7, 0.0, 0.0, &[0.5, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.25, 0.4]),
    ("jk_ayin", "chant", "niggun_hum", 0.7, 0.0, 0.0, &[0.5, 0.3, 0.2, 0.0, 0.3]),
    ("jk_merkavah", "chant", "kadosh", 1.0, 0.0, 0.0, &[0.3, 1.0, 0.4, 0.3, 0.3]),
    ("jk_niggun", "chant", "niggun_hum@1,niggun@2-3", 1.0, 0.0, 0.0, &[0.0, 0.8, 1.0, 0.6, 0.0]),
    // Taoist
    ("ta_zuowang", "guqin", "", 0.8, 0.0, 0.0, &[0.5, 0.4, 0.35, 0.3, 0.25, 0.4]),
    ("ta_xinzhai", "guqin", "", 0.8, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.25, 0.15]),
    ("ta_standing", "guqin", "|stream*0.7", 0.8, 0.0, 0.0, &[0.5, 0.5, 0.5, 0.4, 0.4]),
    ("ta_sixsounds", "chant", "six_xu@1,six_he@2,six_hu@3,six_si@4,six_chui@5,six_xi@6", 1.0, 0.0, 0.0, &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0]),
    // Shamanic (the drum or rattle is already the session's texture)
    ("sr_lower", "fire", "", 0.7, 0.0, 0.0, &[0.6, 0.0, 0.0, 0.0, 0.0, 0.5]),
    ("sr_posture", "wind", "", 0.6, 0.0, 0.0, &[0.5, 0.3, 0.0, 0.4]),
    // Western esoteric & classical
    ("we_incubation", "stream", "", 0.6, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.2, 0.3, 0.2, 0.3, 0.2, 0.3, 0.4]),
    ("we_plotinus", "harp", "", 0.7, 0.0, 0.0, &[0.5, 0.35, 0.3, 0.3, 0.2, 0.4]),
    ("we_above", "harp", "", 0.7, 0.0, 0.0, &[0.5, 0.4, 0.35, 0.3, 0.3, 0.4]),
    ("we_middlepillar", "chant", "eheieh@1,yhvh_elohim@2,yhvh_eloah@3,shaddai@4,adonai_haaretz@5", 1.0, 0.0, 0.0, &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
    ("we_pentagram", "chant", "qcross@1.4,lbrp_names@2,archangels@3", 1.0, 0.0, 0.0, &[0.0, 1.0, 1.0, 1.0, 1.0, 0.0]),
    ("we_bards", "harp", "", 0.7, 0.0, 0.0, &[0.5, 0.3, 0.0, 0.0, 0.5]),
    ("we_galdr", "chant", "ansuz@1,laguz@2,isa@3,dagaz@4", 1.0, 0.0, 0.0, &[0.0, 1.0, 1.0, 1.0, 1.0, 0.0]),
    // Spiritual & mystical experiences
    ("my_centering", "rin", "church", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0]),
    ("my_tibetan", "drone", "bowl", 0.7, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.25, 0.4]),
    ("my_vocal", "drone", "organ", 0.7, 0.0, 0.0, &[0.5, 0.6, 0.6, 0.4, 0.0]),
    ("my_jhana", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
    ("my_nondual", "rin", "bell", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0]),
    ("my_inquiry", "tanpura", "", 0.6, 0.0, 0.0, &[0.5, 0.4, 0.3, 0.25, 0.4]),
    ("my_dhikr", "chant", "lailaha_slow", 1.0, 0.0, 0.0, &[0.0, 1.0, 0.4, 0.0, 0.0]),
    ("my_jesus", "chant", "jesus@1,jesus_w@2", 1.0, 0.0, 0.0, &[0.0, 1.0, 1.0, 0.0, 0.0]),
    ("my_nada", "drone", "shruti", 0.6, 0.0, 0.0, &[0.5, 0.3, 0.1, 0.0, 0.3]),
    ("my_vipassana", "rin", "rin", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
    ("my_orbit", "guqin", "", 0.7, 0.0, 0.0, &[0.5, 0.4, 0.35, 0.35, 0.3, 0.4]),
    ("my_hitbodedut", "wind", "", 0.7, 0.0, 0.0, &[0.6, 0.4, 0.3, 0.5]),
    // Altered states, purpose & breathing sessions that come from a practice
    ("kirtan", "chant", "satanama@0.4,satanama_w@1.3", 1.0, 0.0, 0.0, &[1.0, 1.0, 1.0, 1.0, 1.0, 0.0]),
    ("omdrone", "chant", "aum10", 1.0, 0.0, 0.0, &[1.0]),
    ("om", "chant", "aum10", 1.0, 0.0, 0.0, &[0.5, 1.0]),
    ("mantra6", "chant", "avemaria", 0.9, 0.0, 0.0, &[1.0]),
    ("br_bhramari", "tanpura", "", 0.5, 0.0, 0.0, &[0.4, 0.3, 0.4]),
    ("br_nadi", "tanpura", "", 0.5, 0.0, 0.0, &[0.4, 0.3, 0.4]),
    ("br_ratio", "tanpura", "", 0.5, 0.0, 0.0, &[0.4, 0.3, 0.4]),
    ("br_slowbh", "tanpura", "", 0.5, 0.0, 0.0, &[0.4, 0.3, 0.4]),
    ("br_vase", "drone", "bowl", 0.5, 0.0, 0.0, &[0.4, 0.3, 0.4]),
];

/// Give each session its accompaniment (a session's own choice, if any, wins). Gains are only
/// set where a stage has none of its own.
pub(super) fn music(v: &mut [Preset]) {
    for pr in v.iter_mut() {
        if pr.music.is_some() { continue; }
        let Some(m) = MUSIC.iter().find(|m| m.0 == pr.id) else { continue };
        if m.6.len() != pr.stages.len() { continue; } // (checked by a test)
        pr.music = mx(m.1, m.2, m.3, m.4, m.5);
        for (s, g) in pr.stages.iter_mut().zip(m.6.iter()) {
            if s.mus.is_none() { s.mus = Some(*g); }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_music_entry_matches_its_session() {
        let v = super::super::all();
        for m in super::MUSIC {
            let p = v.iter().find(|p| p.id == m.0).unwrap_or_else(|| panic!("no session {}", m.0));
            assert_eq!(m.6.len(), p.stages.len(), "stage gains for {}", m.0);
            assert!(p.music.is_some(), "music for {}", m.0);
        }
        for p in &v {
            if p.music.is_some() {
                assert!(p.stages.iter().all(|s| s.mus.map(|g| (0.0..=1.0).contains(&g)).unwrap_or(true)), "{}", p.id);
            }
        }
    }
}

// ---------------------------------------------------------------- new traditions and sessions
const C_DEDICATE: &str = "Dedicate whatever good came from this practice to the happiness of all beings.";
const C_STILL: &str = "Let the practice go, and sit quietly for a moment in the stillness it leaves. Then slowly open your eyes.";

/// A traditional session: one 200 Hz carrier at a gentle level under its own accompaniment.
#[allow(clippy::too_many_arguments)]
fn n(g: &str, id: &str, name: &str, short: &str, gain: f64, noise: &str, about: &str, t: &[&str], m: Option<Texture>, stages: Vec<Stage>) -> Preset {
    let mut x = p(g, id, name, short, &[200.0], &[gain], noise, "binaural", "Traditional", about, stages);
    x.trad = t.iter().map(|s| s.to_string()).collect();
    x.music = m;
    x
}

/// Sessions added in 1.8: Kundalini (gentle), Rosicrucian & alchemical, Masonic, Jain, Sikh,
/// Zoroastrian, Bahá'í and Shinto practice, and more Buddhist, Hindu, Sufi, Jewish, Christian,
/// Taoist, Western and shamanic meditations. Nothing here needs a warning: no fast breathing,
/// no breath holds, no flicker.
pub(super) fn more(v: &mut Vec<Preset>) {
    let mut add: Vec<Preset> = Vec::new();
    kundalini(&mut add);
    rosicrucian(&mut add);
    masonic(&mut add);
    buddhist(&mut add);
    hindu(&mut add);
    jain_sikh(&mut add);
    abrahamic(&mut add);
    taoist(&mut add);
    western(&mut add);
    other(&mut add);
    // Each session joins the end of its group, and each new group follows its nearest relative,
    // so groups stay together and every earlier session keeps its place in the list.
    const AFTER: &[(&str, &str)] = &[
        ("Kundalini Yoga (gentle)", "Hindu & Yogic Meditation"),
        ("Jain Meditation", "Kundalini Yoga (gentle)"),
        ("Sikh Simran", "Jain Meditation"),
        ("Rosicrucian & Alchemical", "Western Esoteric & Classical"),
        ("Masonic Contemplation", "Rosicrucian & Alchemical"),
        ("Zoroastrian, Bahá'í & Shinto", "Shamanic & Ancestral Rituals"),
    ];
    for x in add {
        let anchor = if v.iter().any(|q| q.group == x.group) { x.group.clone() } else {
            AFTER.iter().find(|a| a.0 == x.group).map(|a| a.1.to_string()).unwrap_or_default()
        };
        match v.iter().rposition(|q| q.group == anchor) {
            Some(i) => v.insert(i + 1, x),
            None => v.push(x),
        }
    }
}

fn kundalini(v: &mut Vec<Preset>) {
    let g = "Kundalini Yoga (gentle)";
    let t: &[&str] = &["hindu", "kundalini"];
    v.push(n(g, "kn_adi", "Tuning In — Ong Namo Guru Dev Namo (Adi Mantra)", "Chant to open practice · 12 min", 0.35, "none",
      "<p><b>Kundalini Yoga</b> as taught by <b>Yogi Bhajan</b> (Harbhajan Singh Khalsa, 1929–2004), who brought it to the West in 1968, opens every class by \"tuning in\" with the <b>Adi Mantra</b>, <i>Ong Namo Guru Dev Namo</i>: \"I bow to the creative wisdom, I bow to the divine teacher within.\" It is chanted at least three times on long breaths, palms pressed together at the chest. The words are Gurmukhi, the script of Sikh scripture. In 2020 an investigation commissioned by the organisations he founded found allegations of sexual abuse against Yogi Bhajan more likely than not to be true; many teachers keep the practices while rejecting his authority. This session uses only gentle elements: no rapid breathing and no breath holds.</p>",
      t, mx("chant", "adi|tanpura*0.4", 1.0, 146.83, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to tuning in, the opening of a Kundalini Yoga class. Sit cross-legged or on a chair, spine straight, and press your palms together at the centre of your chest.").mus(0.3),
           st("Adi mantra", 4.0, &[8.0], "Breathe in deeply, and chant with the group on one long breath: ong namo, guru dev namo. I bow to the creative wisdom, I bow to the teacher within.").mus(1.0),
           st("Resonance", 4.0, &[7.0], "Keep chanting softly, or join in silently. Feel the ong vibrate at the roof of your mouth and the back of your nose, and let your chest hum with the sound.").mus(0.5),
           st("Sat Nam", 2.0, &[8.0], "Let the chant end. Breathe in deeply, and as you breathe out say a long sat and a short nam. Rest your hands on your knees and sit quietly.").mus(0.0)]));
    v.push(n(g, "kn_satnam", "Sat Nam — Truth Is My Identity", "Seed mantra with the breath · 15 min", 0.35, "none",
      "<p><b>Sat Nam</b> (\"truth is my identity\", or \"true name\") is the <i>bīj</i> (seed) mantra of Kundalini Yoga and a phrase from the opening of the Sikh <i>Mūl Mantar</i>. Students greet each other with it and chant it as a long <i>Saaat</i> and a short <i>Naam</i>. Here you chant it along with a group, then hear it silently with the breath, the closed eyes turned gently up toward the brow point.</p>",
      t, mx("chant", "satnam|tanpura*0.4", 1.0, 146.83, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to Sat Nam meditation. Sit tall and close your eyes, then gently turn them up toward the point between your eyebrows. Sat means truth, and nam means name, or identity.").mus(0.3),
           st("Chanting", 6.0, &[8.0], "Chant along with the group: a long saaat, and a short naam. Keep the sound steady and unhurried, and let each round begin on a fresh breath.").mus(1.0),
           st("Silent", 5.0, &[6.5], "Now hear sat silently as you breathe in, and nam as you breathe out. Let each breath grow long and slow, without forcing it.").mus(0.25),
           st("Rest", 2.0, &[8.0], C_STILL).mus(0.0)]));
    v.push(n(g, "kn_ramadasa", "Ra Ma Da Sa — Healing Meditation (Siri Gaitri Mantra)", "One round per breath · 16 min", 0.35, "none",
      "<p><b>Ra Ma Da Sa, Sa Say So Hung</b>, the <b>Siri Gaitri Mantra</b>, is the best-known healing meditation of Kundalini Yoga, taught by Yogi Bhajan in 1973. Its eight sounds are read as sun, moon, earth, infinity; infinity, totality, I am Thou. It is chanted for 11 minutes, one round per breath, sitting with the elbows tucked against the ribs and the forearms angled out, palms flat and facing up. Traditionally it ends with a held breath while you picture someone to be healed; this version uses a slow breath instead.</p>",
      t, mx("chant", "ramadasa|tanpura*0.4", 1.0, 146.83, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Ra Ma Da Sa healing meditation. Sit with a straight spine. Tuck your elbows against your sides, angle your forearms out, and turn your palms flat toward the sky.").mus(0.3),
           st("Chanting", 11.0, &[7.0], "Chant one full round on each breath: ra ma da sa, sa say so hung. Sun, moon, earth, infinity; infinity, totality, I am thou. Let the hung ring at the back of your throat.").mus(1.0),
           st("Healing", 2.0, &[6.0], "Stop chanting. Breathe in slowly and picture someone who needs healing, yourself or another, held in warm light. Breathe out, and let the image go.").mus(0.0),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT).mus(0.0)]));
    v.push(n(g, "kn_gong", "Gong Relaxation — Deep Rest After Practice", "Gong bath in savasana · 15 min", 0.3, "none",
      "<p>Kundalini Yoga classes end with deep relaxation lying on the back (<b>śavāsana</b>), and many teachers play a large gong over the group, a custom Yogi Bhajan introduced in the 1970s. The gong's dense, slowly shifting overtones are said to clear the mind; most people simply find them absorbing. Lie down, cover yourself, and let the sound do the work.</p>",
      t, mx("gongbath", "", 0.9, 0.0, 0.0),
      vec![st("Settle", 2.0, &[8.0], "Welcome to gong relaxation. Lie flat on your back, arms by your sides, palms up. Cover yourself with a blanket, because the body cools as it rests deeply.").mus(0.4),
           st("Gong", 10.0, &[4.0], "Let the waves of the gong wash through you. There is nothing to do. When thoughts come, let the sound carry them away.").mus(1.0),
           st("Stillness", 2.0, &[5.0], super::C_REST).mus(0.2),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT).mus(0.0)]));
    v.push(n(g, "kn_serpent", "Kundalini Rising — Gentle Breath Through the Chakras", "Light attention, no force · 20 min", 0.45, "pink",
      "<p>In tantric and yogic texts <b>kuṇḍalinī</b> (\"the coiled one\") is the dormant energy pictured as a serpent at the base of the spine, which rises through the chakras to the crown in awakening, as described in the <i>Ṣaṭ-cakra-nirūpaṇa</i> (1577), translated by Arthur Avalon in <i>The Serpent Power</i> (1919). Teachers in every lineage warn against forcing it. This session is deliberately gentle: slow natural breathing and light visualisation only, without the rapid breathing, locks or breath holds of vigorous kriyas. If you feel heat, shaking, rushes of energy or agitation, stop and ground yourself.</p>",
      t, mx("tanpura", "", 0.7, 128.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to gentle kundalini awareness. Sit tall and relaxed. This practice uses only slow, natural breathing and light attention. Nothing is forced, and you can stop at any time.").mus(0.5),
           st("Root", 3.0, &[7.5], "Rest your attention at the base of your spine. Picture a soft, warm glow there, like a coiled thread of light, quiet and patient.").mus(0.4),
           st("Rising breath", 8.0, &[6.5], "As you breathe in, imagine the light rising gently up the centre of your spine. As you breathe out, let it settle back down. Slow and easy, never pushing.").mus(0.35),
           st("Crown", 4.0, &[6.0], "Let the rising breath reach the crown of your head. Rest your attention there, open and spacious, as if the top of your head were filled with sky.").mus(0.3),
           st("Grounding", 3.0, &[8.0], "Now bring your attention down to your heart, then your belly, then the ground beneath you. Feel heavy and steady before you open your eyes.").mus(0.5)]));
}

fn rosicrucian(v: &mut Vec<Preset>) {
    let g = "Rosicrucian & Alchemical";
    let t: &[&str] = &["abrahamic", "christian", "rosicrucian", "western"];
    v.push(n(g, "ro_rosecross", "The Rose Cross Meditation (Rudolf Steiner)", "Black cross, seven red roses · 20 min", 0.5, "pink",
      "<p>The Rosicrucian movement began with three anonymous manifestos, the <i>Fama Fraternitatis</i> (1614), the <i>Confessio</i> (1615) and <i>The Chymical Wedding of Christian Rosenkreutz</i> (1616), which told of a secret brotherhood working for the spiritual renewal of Christian Europe. Its emblem is the <b>rose cross</b>. <b>Rudolf Steiner</b> (1861–1925), founder of Anthroposophy, gave this meditation in <i>An Outline of Esoteric Science</i> (1910). You build the image of a black cross ringed by seven red roses out of feelings about the plant and the blood, so the symbol carries a living meaning: the lower nature purified, desire transformed into love.</p>",
      t, mx("choir", "hum", 0.6, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Rose Cross meditation, as Rudolf Steiner taught it. Sit upright and still, and close your eyes. We will build the image step by step, with feeling as well as thought.").mus(0.5),
           st("The plant", 4.0, &[8.0], "Picture a green plant growing out of the earth, leaf after leaf. Feel how pure it is, growing by its own laws, without desire or passion.").mus(0.4),
           st("The blood", 4.0, &[7.0], "Now think of red human blood. It carries desire and passion, but also the power to rise higher than the plant, through the striving of the self.").mus(0.35),
           st("The cross", 7.0, &[6.0], "See a black cross before you: the lower nature burned away. Where its beams meet, seven glowing red roses form a ring: passion purified into love. Hold the whole image calmly.").mus(0.3),
           st("Close", 3.0, &[9.0], "Let the image fade, and stay with the feeling it leaves. Then breathe deeply, and come back to the room.").mus(0.5)]));
    v.push(n(g, "ro_retrospection", "Evening Retrospection — The Day in Reverse (Max Heindel)", "In bed, before sleep · 15 min", 0.4, "pink",
      "<p><b>Max Heindel</b> (1865–1919) founded the Rosicrucian Fellowship in Oceanside, California, and set out its teaching in <i>The Rosicrucian Cosmo-Conception</i> (1909). Its two daily exercises are <b>retrospection</b> at night and <b>concentration</b> in the morning. In retrospection you review the day <b>backwards</b>, from bedtime to waking, as an impartial witness, sincerely regretting what you did wrong and feeling thankful for the good. Heindel taught that this digests the day's experience in advance; Rudolf Steiner taught a similar backward review.</p>",
      t, mx("harp", "", 0.5, 0.0, 0.0),
      vec![st("Settle", 2.0, &[8.0], "Welcome to evening retrospection, as taught by Max Heindel of the Rosicrucian Fellowship. Lie in bed, relaxed but awake. You will review your day in reverse, from now back to the morning.").mus(0.5),
           st("Reverse review", 9.0, &[6.0], "Begin with the last thing you did before lying down, then go backwards, scene by scene. Watch yourself like a fair witness. Where you did wrong, feel sincere regret; where you did well, feel thankful.").mus(0.4),
           st("Resolve", 2.0, &[5.0], "When you reach the morning, gather what you learned into one quiet resolve for tomorrow.").mus(0.3),
           st("Sleep", 2.0, &[3.0], "Now let the day go completely. Let your breathing slow, and drift toward sleep.").mus(0.2)]));
    v.push(n(g, "ro_concentration", "Morning Concentration — One High Ideal (Max Heindel)", "On waking · 12 min", 0.45, "pink",
      "<p>Heindel's morning exercise is <b>concentration</b>: holding one high ideal or image steadily before the mind, then letting it unfold in <b>meditation</b>, <b>observation</b>, <b>discrimination</b> and finally <b>contemplation</b>, the steps he lists in <i>The Rosicrucian Cosmo-Conception</i>. It is done soon after waking, before the business of the day fills the mind. Choose an ideal that matters to you: love, patience, service, or a sacred image such as the rose cross.</p>",
      t, mx("drone", "organ", 0.45, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to morning concentration, the companion of evening retrospection. Sit up in bed or on a chair, and close your eyes before the day's business fills your mind.").mus(0.5),
           st("One ideal", 6.0, &[8.0], "Choose one high ideal, such as love, patience or a sacred image, and hold it at the centre of attention. When other thoughts come, return to it calmly, without strain.").mus(0.4),
           st("Observation", 3.0, &[7.5], "Now let the ideal unfold: what it means, and how it could live in your actions today. Let insight come by itself.").mus(0.35),
           st("Into the day", 1.0, &[12.0], "Take that one ideal with you into the day, and open your eyes.").mus(0.5)]));
    v.push(n(g, "ro_seed", "The Seed Meditation — Growing and Decaying (Rudolf Steiner)", "From Knowledge of the Higher Worlds · 18 min", 0.5, "pink",
      "<p>In <i>Knowledge of the Higher Worlds and Its Attainment</i> (1904–05) Rudolf Steiner gives a meditation on a <b>seed</b>: look closely at a seed, then feel the invisible force that will make it grow into a whole plant; later, contemplate a full-grown plant and the forces by which it will wither. He calls these feelings of <b>growing and decaying</b>, and taught that, practised patiently, they awaken a new kind of perception. Hold a real seed if you can.</p>",
      t, mx("harp", "", 0.5, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to Rudolf Steiner's seed meditation. Hold a real seed in your hand if you have one, or picture one clearly. Sit quietly and look at it closely.").mus(0.5),
           st("The seed", 4.0, &[8.0], "Notice its shape, colour and surface. Then think: hidden in this small thing is a whole plant, waiting. Feel wonder at that invisible force.").mus(0.4),
           st("Growing", 6.0, &[6.5], "Close your eyes and picture the seed sprouting, rooting and rising: stem, leaves, flower, and new seeds. Follow the growth slowly, and feel its living force.").mus(0.35),
           st("Decaying", 4.0, &[6.0], "Now picture a plant in full bloom slowly withering, its leaves falling back to the earth. Feel the quiet force of dying, and the renewal hidden within it.").mus(0.35),
           st("Close", 2.0, &[10.0], C_STILL).mus(0.5)]));
    v.push(n(g, "ro_vitriol", "V.I.T.R.I.O.L. — Visit the Interior of the Earth (Alchemy)", "Descent to the hidden stone · 20 min", 0.5, "brown",
      "<p><b>V.I.T.R.I.O.L.</b> stands for <i>Visita Interiora Terrae Rectificando Invenies Occultum Lapidem</i>: \"visit the interior of the earth; by rectifying, you will find the hidden stone\". The motto appears in the alchemical treatise <i>Azoth</i> attributed to Basil Valentine (1613), and later in Rosicrucian and Masonic symbolism. Read inwardly, the earth is oneself, rectifying is purification, and the stone is the philosopher's stone, the true self. This session is a guided inner descent.</p>",
      t, mx("drone", "bowl", 0.5, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to vitriol, the alchemists' motto: visit the interior of the earth, and by rectifying you will find the hidden stone. Sit still, and close your eyes.").mus(0.5),
           st("Descent", 5.0, &[6.5], "Imagine a stairway leading down into the earth, and into yourself. Walk down slowly, step by step, into the cool, quiet dark.").mus(0.4),
           st("Rectifying", 6.0, &[5.5], "In the inner chamber, look honestly at what you find: fears, resentments, habits. Don't judge them. Offer each one to the fire of attention, and let it be purified.").mus(0.35),
           st("The stone", 5.0, &[5.0], "At the very centre, find the hidden stone: a small, steady light that was always there. Rest in it, and let it warm you.").mus(0.3),
           st("Ascent", 2.0, &[10.0], "Carry the light back up the stairway, step by step, into the daylight of the room. Then open your eyes.").mus(0.5)]));
    v.push(n(g, "ro_chymical", "The Chymical Wedding of Christian Rosenkreutz (1616)", "A seven-day inner journey · 26 min", 0.45, "pink",
      "<p><b>The Chymical Wedding of Christian Rosenkreutz</b> (1616), by the Lutheran theologian <b>Johann Valentin Andreae</b>, is the third Rosicrucian manifesto: an alchemical allegory told over seven days. An old hermit is invited to a royal wedding at a mysterious castle; the guests are weighed against the virtues; the royal couple die and are brought back to life by an alchemical work in a seven-storey tower; and Christian is made a Knight of the Golden Stone. This session walks you through the story's main stages as an inner journey.</p>",
      t, mx("harp", "|choir:hum*0.5", 0.6, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Chymical Wedding, the Rosicrucian story of 1616. Sit or lie comfortably and close your eyes. You will travel through the story's seven days in your imagination.").mus(0.5),
           st("The invitation", 3.0, &[8.0], "It is the eve of Easter. A wind rises, and a winged messenger in blue hands you a letter sealed with a cross: you are invited to the royal wedding. Ask yourself if you are ready.").mus(0.45),
           st("The journey", 4.0, &[7.0], "At dawn you set out, a red band across your shoulder and roses in your hat. The road divides into four ways. Choose one, trust it, and walk until you see the castle.").mus(0.4),
           st("The weighing", 4.0, &[6.5], "Inside, each guest is weighed on golden scales against the virtues. Step onto the scales, and let your faults and your strengths be seen honestly, without fear.").mus(0.35),
           st("The tower", 6.0, &[5.5], "Now climb the tower of the great work, seven floors. On each one, something old in you is dissolved, refined and made new.").mus(0.3),
           st("Rebirth", 4.0, &[6.0], "At the top, the king and queen awaken, alive and renewed. Feel that renewal in yourself: opposites joined in a quiet, golden wholeness.").mus(0.35),
           st("The return", 3.0, &[10.0], "On the seventh day you are made a knight of the golden stone and sent home with a task. Bring that task back with you as you open your eyes.").mus(0.5)]));
    v.push(n(g, "ro_stages", "The Great Work — Blackening, Whitening, Yellowing, Reddening", "Alchemy as inner change · 22 min", 0.5, "brown",
      "<p>Western alchemy described the <b>Great Work</b> in colour stages: <b>nigredo</b> (blackening, decomposition), <b>albedo</b> (whitening, purification), <b>citrinitas</b> (yellowing, dawning) and <b>rubedo</b> (reddening, the finished stone). The Swiss psychiatrist <b>Carl Jung</b> read them as stages of inner transformation in <i>Psychology and Alchemy</i> (1944) and <i>Mysterium Coniunctionis</i> (1955–56). Bring to mind a real situation that is asking you to change.</p>",
      t, mx("drone", "organ", 0.45, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the great work of alchemy, read as inner change, as Carl Jung read it. Sit quietly, close your eyes, and bring to mind something in your life that is asking to change.").mus(0.5),
           st("Nigredo", 5.0, &[5.5], "Nigredo, the blackening. Let yourself feel the difficulty fully, the confusion or loss, without trying to fix it. Old forms dissolve in the dark.").mus(0.35),
           st("Albedo", 5.0, &[6.5], "Albedo, the whitening. As if washed by moonlight, the situation grows clearer and calmer. Notice what is left when the heat of reaction has cooled.").mus(0.4),
           st("Citrinitas", 4.0, &[7.5], "Citrinitas, the yellowing. A dawn rises inside you: insight, meaning, a new understanding of what this has been teaching you.").mus(0.45),
           st("Rubedo", 4.0, &[8.0], "Rubedo, the reddening. Feel the new understanding become warm and alive in your body, ready to be lived. The opposites in you are joined.").mus(0.5),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT).mus(0.5)]));
    v.push(n(g, "ro_rosecrossritual", "The Rose Cross Ritual — Yeheshuah (Golden Dawn)", "Six crosses of peace · 14 min", 0.4, "none",
      "<p>The <b>Hermetic Order of the Golden Dawn</b> (founded in London in 1888) called its inner order the <i>Ordo Rosae Rubeae et Aureae Crucis</i>, the Order of the Red Rose and the Golden Cross. Its <b>Rose Cross ritual</b>, published by Israel Regardie in the 1930s, traces a cross in each of six directions while vibrating <b>Yeheshuah</b>, the name of Jesus as the Golden Dawn spelled it: the four letters of the divine name with the letter shin, spirit, at their centre. It is used for peace and protection. This session describes it in plain words.</p>",
      &["abrahamic", "christian", "rosicrucian", "western", "hermetic"], mx("chant", "yeheshuah", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Rose Cross ritual of the Golden Dawn's inner order, a ritual of peace and protection. Stand if you can, facing the southeast, and breathe slowly.").mus(0.0),
           st("Six crosses", 6.0, &[8.0], "Trace a large cross in the air before you, and vibrate the name Yeheshuah into its centre. Turn and repeat at each quarter, then above you and below you, joining the crosses with a line of light.").mus(1.0),
           st("The rose", 4.0, &[6.5], "Return to the centre. Picture a golden cross in your heart with a red rose at its centre, and feel peace spreading out through the six crosses around you.").mus(0.3),
           st("Close", 2.0, &[9.0], "Let the image settle into a feeling of quiet protection. Breathe deeply, and return to ordinary awareness.").mus(0.0)]));
}

fn masonic(v: &mut Vec<Preset>) {
    let g = "Masonic Contemplation";
    let t: &[&str] = &["abrahamic", "christian", "masonic", "western"];
    let organ = || mx("drone", "organ", 0.4, 0.0, 0.0);
    v.push(n(g, "ma_ashlar", "From Rough to Perfect Ashlar — Shaping the Inner Stone", "Masonic symbol of self-work · 16 min", 0.45, "pink",
      "<p><b>Freemasonry</b>, which took its modern form with the founding of the Grand Lodge of England in 1717, teaches morality through the tools and symbols of stonemasons. Its members come from many faiths, and most lodges ask them to believe in a Supreme Being. Two stones lie in every lodge: the <b>rough ashlar</b>, a stone fresh from the quarry, for the human being in a rough and imperfect state, and the <b>perfect ashlar</b>, squared by the working tools, for a life made fit by education and self-discipline. This is a private contemplation on publicly known symbols, not a lodge ritual.</p>",
      t, organ(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the contemplation of the two ashlars, stones found in every Masonic lodge. Sit upright, feet flat on the floor, hands resting on your thighs, and close your eyes.").mus(0.5),
           st("The rough ashlar", 4.0, &[8.0], "Picture a rough stone, just taken from the quarry, uneven and unshaped: the human being as we begin. Notice one rough edge in yourself, honestly.").mus(0.4),
           st("The working tools", 5.0, &[7.0], "Take up the gavel and the chisel: your will, and steady attention. In your mind, strike gently at that rough edge, a habit you want to shape. Feel the patience the work needs.").mus(0.35),
           st("The perfect ashlar", 4.0, &[6.5], "Picture the finished stone, square and true, ready to take its place in the temple. See yourself as you could be, and the place you could fill among others.").mus(0.4),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT).mus(0.5)]));
    v.push(n(g, "ma_point", "The Point Within a Circle", "Masonic emblem of right conduct · 14 min", 0.45, "pink",
      "<p>The <b>point within a circle</b> is explained in the Masonic monitors of the 18th and 19th centuries: the point stands for the individual, the circle for the boundary line of conduct beyond which passions and prejudices should never carry us. The circle is drawn between two parallel lines, said to stand for St John the Baptist and St John the Evangelist, with the volume of sacred law resting on top. Kept within them, the old lecture says, one cannot materially err.</p>",
      t, organ(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the point within a circle, an emblem explained to every new Mason. Sit still, and picture a single point with a circle drawn evenly around it.").mus(0.5),
           st("The point", 4.0, &[8.0], "The point is you. The circle is the boundary of your conduct, beyond which your passions should not carry you. Feel the steadiness of a life kept within its circle.").mus(0.4),
           st("Two lines", 4.0, &[7.0], "Picture two parallel lines touching the circle on either side, and above them the book of sacred law. Think of the teachers and principles that keep you true.").mus(0.35),
           st("Reflection", 3.0, &[6.5], "Hold the whole emblem quietly. Ask where, lately, you have stepped outside your circle, and how you will return to it.").mus(0.35),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT).mus(0.5)]));
    v.push(n(g, "ma_gauge", "The Twenty-Four-Inch Gauge — Dividing the Day", "Daily review with a working tool · 12 min", 0.45, "pink",
      "<p>The <b>twenty-four-inch gauge</b> is the first working tool given to an Entered Apprentice. Masonic lectures explain it as the twenty-four hours of the day, to be divided into three parts: one for the service of God and of a worthy person in distress, one for our usual work, and one for refreshment and sleep. Used as a daily review, it is a simple way to look at how you spend your time.</p>",
      t, organ(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the twenty-four-inch gauge, the first working tool of an Entered Apprentice. It stands for the twenty-four hours of the day. Sit comfortably, and close your eyes.").mus(0.5),
           st("Three parts", 6.0, &[8.0], "Masons divide the day into three parts: service to God and to people in need, your usual work, and refreshment and sleep. Look at your own day, and see how it is divided.").mus(0.4),
           st("Adjust", 3.0, &[7.5], "Where is the balance off? Choose one small change for tomorrow: an hour given to service, to rest, or to better work.").mus(0.4),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT).mus(0.5)]));
    v.push(n(g, "ma_stairs", "The Winding Stairs — Ascent to the Middle Chamber", "Fellowcraft contemplation · 18 min", 0.45, "pink",
      "<p>In the <b>Fellowcraft</b> (second) degree, the candidate climbs, in symbol, a <b>winding staircase</b> of three, five and seven steps to the <b>middle chamber</b> of King Solomon's Temple, where wages are paid. The lecture links the steps to wisdom, strength and beauty, to the five orders of architecture and the five senses, and to the <b>seven liberal arts and sciences</b>: grammar, rhetoric, logic, arithmetic, geometry, music and astronomy. It is a picture of lifelong learning.</p>",
      t, organ(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the winding stairs, a Masonic picture of the search for knowledge. Sit upright and close your eyes. See a spiral staircase rising before you toward a lighted chamber.").mus(0.5),
           st("Three steps", 3.0, &[8.0], "Climb the first three steps: wisdom, strength and beauty. Pause on each one, and feel its quality.").mus(0.4),
           st("Five steps", 4.0, &[7.0], "Climb five more: the five senses, through which we learn about the world. Notice each one now: hearing, sight, touch, smell and taste.").mus(0.35),
           st("Seven steps", 5.0, &[6.5], "Climb the last seven: grammar, rhetoric, logic, arithmetic, geometry, music and astronomy. Feel your mind widen with each step.").mus(0.35),
           st("The middle chamber", 3.0, &[6.0], "Enter the middle chamber, and rest in its quiet light. Receive what you have earned: understanding, and the wish to use it well.").mus(0.4),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT).mus(0.5)]));
}

fn buddhist(v: &mut Vec<Preset>) {
    let g = "Buddhist Meditation";
    let thera: &[&str] = &["buddhist", "theravada"];
    let zen: &[&str] = &["buddhist", "zen"];
    let rin = || mx("rin", "rin", 1.0, 0.0, 0.0);
    let bell = || mx("rin", "bell", 1.0, 0.0, 0.0);
    v.push(n(g, "bu_buddho", "Buddho — The Thai Forest Meditation Word", "Bud on the in-breath, dho on the out · 20 min", 0.5, "pink",
      "<p>The <b>Thai Forest tradition</b>, revived by <b>Ajahn Mun</b> (1870–1949) and spread by students such as <b>Ajahn Chah</b> (1918–1992), often begins meditation with the word <b>buddho</b>, \"awake\" or \"the one who knows\": <i>bud</i> on the in-breath, <i>dho</i> on the out-breath. As concentration grows the word falls away by itself, and the mind rests in knowing.</p>",
      thera, bell(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to buddho meditation, as taught in the Thai forest tradition by Ajahn Mun and Ajahn Chah. Sit cross-legged or on a chair, right hand resting in the left, and close your eyes."),
           st("Buddho", 10.0, &[7.0], "Breathing in, think bud. Breathing out, think dho. Buddho means awake, the one who knows. Let the word ride on the breath, unhurried."),
           st("Knowing", 6.0, &[6.0], "As the mind calms, the word may fade by itself. Let it. Rest in simple knowing, aware of the breath and of awareness itself."),
           st("Close", 2.0, &[9.0], C_DEDICATE)]));
    v.push(n(g, "bu_maranasati", "Maranasati — Recollecting Death, Valuing Life", "The five remembrances · 18 min", 0.45, "pink",
      "<p><b>Maraṇasati</b>, mindfulness of death, is taught in the Pāli suttas (in AN 6.19 the Buddha urges monks to recall death even between one breath and the next) and in Buddhaghosa's <i>Visuddhimagga</i>. The <b>Five Remembrances</b> (AN 5.57) are recited daily in many monasteries: I am of the nature to grow old, to become ill and to die; everything dear to me will change; my actions are my only true belongings. The aim is not gloom but urgency, gratitude and less fear. If you are grieving or feel fragile, choose another session.</p>",
      thera, bell(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to maranasati, the Buddha's recollection of death, practised to value life and loosen fear. Sit comfortably. If you are grieving or feel fragile today, choose another session."),
           st("Five remembrances", 6.0, &[7.0], "Reflect slowly: I will grow old. I will become ill. I will die. Everyone and everything I love will change. My actions are my only true belongings."),
           st("With the breath", 6.0, &[6.0], "Now, with each breath, remember how uncertain life is, and how precious this breath is. Let any fear soften into appreciation."),
           st("Living fully", 3.0, &[7.0], "Ask yourself: if this were my last month, what would matter most? Let the answer guide you gently."),
           st("Close", 1.0, &[10.0], C_DEDICATE)]));
    v.push(n(g, "bu_walking", "Walking Meditation — Lifting, Moving, Placing", "Slow walking, eyes open · 20 min", 0.4, "pink",
      "<p>Walking meditation (<i>caṅkama</i>) is as old as Buddhism: the suttas describe monks pacing back and forth on a path. The Burmese teacher <b>Mahāsi Sayādaw</b> (1904–1982) taught it in stages, noting \"left, right\", then \"lifting, moving, placing\". Zen keeps a slower, more formal walk called <b>kinhin</b> between periods of sitting. Walk barefoot if you can, on a clear, level path of ten to twenty steps. Keep your eyes open, and keep the volume low enough to hear your surroundings.</p>",
      &["buddhist", "theravada", "zen"], bell(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to walking meditation. Find a clear path of ten to twenty steps. Stand at one end, hands clasped in front of you, eyes resting softly on the ground ahead."),
           st("Left, right", 5.0, &[9.0], "Walk slowly, noting each step: left, right. Feel each sole touch the ground. At the end of the path, stop, note standing, turn slowly, and walk back."),
           st("Three parts", 8.0, &[8.0], "Slow down further, and note three parts of every step: lifting, moving, placing. Know each movement as it happens, with your attention in the foot."),
           st("Standing", 3.0, &[8.0], "Come to a stop. Feel the whole body standing: the weight in the feet, the breath moving. Then walk on slowly until the bell."),
           st("Close", 2.0, &[10.0], "Stop, and stand quietly for a few breaths. Carry this careful attention into your next ordinary steps.")]));
    v.push(n(g, "bu_buddhanussati", "Buddhānussati — Recollecting the Buddha (Itipiso)", "Pāli chant and reflection · 18 min", 0.35, "none",
      "<p><b>Buddhānussati</b>, recollection of the Buddha, is one of the ten recollections of the Pāli canon and a classic way to inspire and steady the mind. Theravāda Buddhists begin with the homage <i>Namo tassa bhagavato arahato sammāsambuddhassa</i> three times, then chant the <b>Itipiso</b>, the Buddha's nine qualities: worthy, perfectly awakened, accomplished in knowledge and conduct, well gone, knower of the worlds, unsurpassed trainer of those to be tamed, teacher of gods and humans, awakened, blessed. Here the chanting leads into quiet reflection.</p>",
      thera, mx("chant", "namo@1,itipiso@2-3", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the recollection of the Buddha. Sit with your palms together at your chest. The community first chants the homage, namo tassa, three times. Listen, or chant along.").mus(0.0),
           st("Homage", 3.0, &[8.0], "Namo tassa bhagavato arahato samma sambuddhassa: homage to the blessed one, the worthy one, the perfectly self-awakened one.").mus(1.0),
           st("Itipiso", 6.0, &[7.0], "Now the itipiso, the nine qualities of the Buddha. Let your heart warm to each one: worthy, awakened, perfect in knowledge and conduct, well gone, knower of the worlds.").mus(1.0),
           st("Reflection", 5.0, &[6.5], "Let the chant fade into the background. Recall one quality that inspires you, perhaps his compassion or his peace, and let it fill your mind with quiet joy.").mus(0.2),
           st("Close", 2.0, &[9.0], C_DEDICATE).mus(0.0)]));
    v.push(n(g, "bu_susokukan", "Susokukan — Counting the Breath (Zen)", "One to ten, then begin again · 20 min", 0.45, "pink",
      "<p><b>Susokukan</b> (\"contemplation by counting breaths\") is the first practice given to beginners in Japanese Zen. You count each out-breath from one to ten and start again; when you lose count, you start again at one. It descends from the Chinese breath-counting methods taught by <b>Zhiyi</b> (538–597). Sit facing a wall or the floor, eyes half open and lowered.</p>",
      zen, rin(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to susokukan, the Zen practice of counting breaths. Sit upright on a cushion or chair, left hand resting in the right, thumbs lightly touching. Eyes half open, gazing down."),
           st("Counting", 10.0, &[7.5], "Count each out-breath silently: one, two, three, up to ten, then begin again at one. If you lose count, or find yourself at fifteen, smile and return to one."),
           st("Just breathing", 6.0, &[6.5], "Now drop the numbers. Simply be with each breath as it comes and goes, the whole body breathing."),
           st("Close", 2.0, &[9.0], "The bell rings. Rest a moment, sway gently from side to side, and stand up slowly.")]));
    v.push(n(g, "bu_huatou", "Huatou — Who Is Reciting the Buddha's Name? (Chan)", "The great doubt · 22 min", 0.45, "pink",
      "<p>A <b>huatou</b> (\"head of the word\") is the point before a thought becomes words. Chinese Chan masters from <b>Dahui Zonggao</b> (1089–1163) to <b>Xuyun</b> (Empty Cloud, 1840–1959) taught students to hold a short question, such as \"Who is reciting the Buddha's name?\", until a deep sense of doubt and searching gathers: the <b>great doubt</b>, which they said shatters in awakening. You don't answer the question with thought; you look into it.</p>",
      zen, rin(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to huatou practice, the Chan method taught by Master Xuyun. Sit upright and still. You will hold one question, not to answer it with thought, but to look into it."),
           st("The question", 8.0, &[7.0], "Ask silently: who is reciting the Buddha's name? Or simply: who? Don't analyse. Look toward the place the question points to, before any words."),
           st("Great doubt", 8.0, &[6.0], "Let a sense of not-knowing grow, the great doubt. Keep it alive gently, like a cat watching a mouse hole: alert and patient."),
           st("Close", 4.0, &[8.0], C_STILL)]));
    v.push(n(g, "bu_whatisthis", "What Is This? — Korean Seon Questioning", "Hwadu meditation · 20 min", 0.45, "pink",
      "<p>Korean <b>Seon</b> Buddhism, the meditation school of the Jogye Order, is built on the <i>hwadu</i>, the Korean form of huatou. The most common one is simply <b>\"What is this?\"</b> (<i>imwotko</i>), taught by masters such as <b>Kusan Sunim</b> (1909–1983) at Songgwangsa monastery. It points to the one who sees, hears and thinks. Asking it with sincere not-knowing is the whole practice.</p>",
      zen, rin(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to Korean Seon, the tradition of Master Kusan. Sit upright, hands in your lap, eyes lowered. Your question is simple: what is this? In Korean, imwotko."),
           st("Asking", 8.0, &[7.0], "Ask slowly and sincerely: what is this? What is it that sees, hears and thinks right now? Stay with the not-knowing that follows the question."),
           st("Breath and question", 7.0, &[6.0], "Ask what is this on a long out-breath, then rest in the silence of the in-breath. Let the question grow deeper, not louder."),
           st("Close", 3.0, &[9.0], "Let the question go, and notice the fresh, clear quality of the mind. Bow inwardly, and open your eyes.")]));
    v.push(n(g, "bu_silentillum", "Silent Illumination — Clear, Open Awareness (Chan)", "Master Hongzhi, 12th century · 20 min", 0.45, "pink",
      "<p><b>Silent illumination</b> (<i>mozhao</i>) was taught by the Chinese Caodong master <b>Hongzhi Zhengjue</b> (1091–1157): \"silently and serenely, forgetting words, clarity appears before you\". It is the root of Japanese Sōtō Zen's <i>shikantaza</i>. In the 20th century <b>Master Sheng Yen</b> (1931–2009) taught it in stages: relax the body, be aware of the whole body sitting, then of the whole environment, open and without choosing.</p>",
      zen, rin(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to silent illumination, taught in twelfth-century China by Master Hongzhi, and in our time by Sheng Yen. Sit upright and relaxed, with your eyes half open."),
           st("Whole body", 6.0, &[7.0], "First be aware of the whole body sitting, all at once rather than part by part. Relax every muscle that isn't needed to stay upright."),
           st("Silent and clear", 10.0, &[6.0], "Now let awareness include everything, sounds, sensations and thoughts, without choosing among them. Silent: not grasping. Illuminating: clearly aware."),
           st("Close", 2.0, &[9.0], "Slowly come back. Rub your palms together, place them over your eyes, and open your eyes into your palms.")]));
    v.push(with_tex(n(g, "bu_heartsutra", "The Heart Sutra — Gate Gate Pāragate", "Chanting with the wooden fish · 18 min", 0.3, "none",
      "<p>The <b>Heart Sutra</b> (<i>Prajñāpāramitā Hṛdaya</i>) is the most chanted text of Mahāyāna Buddhism, recited daily in Chinese, Japanese, Korean, Vietnamese and Tibetan temples. In a few lines Avalokiteśvara teaches that \"form is emptiness, emptiness is form\", and it ends with the mantra <b>Gate gate pāragate pārasaṃgate bodhi svāhā</b>: gone, gone, gone beyond, gone completely beyond, awakening, so be it. Here it is chanted in the Japanese reading, one syllable to each knock of the wooden fish.</p>",
      &["buddhist", "zen", "eastasian"], mx("chant", "heart", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Heart Sutra. Sit upright with your palms together. Form is emptiness, emptiness is form. Its mantra means gone, gone beyond, completely beyond: awakening!").mus(0.0).tex(0.0),
           st("Chanting", 10.0, &[8.0], "Chant with the wooden fish, one syllable to each knock: gya tei, gya tei, ha ra gya tei, hara so gya tei, bo ji sowaka. Let voice and rhythm become one.").mus(1.0),
           st("Emptiness", 4.0, &[6.0], "Let the chanting stop. Sit in the silence it leaves. Notice how sounds, thoughts and feelings arise and pass, open and empty, like clouds through sky.").mus(0.0).tex(0.0),
           st("Close", 2.0, &[9.0], C_DEDICATE).mus(0.0).tex(0.0)]),
      tx("mokugyo", 0.55, 460.0, 0.5)));
    v.push(n(g, "bu_softbutter", "Hakuin's Soft Butter Method (Zen Healing)", "Melting relaxation, 1757 · 18 min", 0.45, "pink",
      "<p>The Zen master <b>Hakuin Ekaku</b> (1686–1769) fell ill from over-strenuous practice and, by his own account in <i>Yasenkanna</i> (\"Idle Talk on a Night Boat\", 1757), cured himself with methods learned from a hermit named Hakuyū. One is the <b>soft butter method</b> (<i>nanso no hō</i>): imagining a lump of fragrant soft butter melting from the crown of the head down through the whole body, dissolving tension and illness. It remains a well-loved Zen relaxation.</p>",
      zen, mx("drone", "bowl", 0.45, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the soft butter method of the Zen master Hakuin, who healed his own exhaustion with it. Sit or lie down comfortably, and close your eyes."),
           st("The butter", 4.0, &[7.0], "Imagine a lump of soft, fragrant butter, the size of a duck egg, resting on the crown of your head. Feel its coolness and its lovely scent."),
           st("Melting", 8.0, &[6.0], "Slowly it melts and flows down over your head, face, neck and shoulders, soaking through your chest, belly and back, dissolving tension wherever it goes."),
           st("The bath", 2.0, &[5.5], "Let it pool around your hips and legs like warm water in a bath, soaking you in comfort from below."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    let ea: &[&str] = &["buddhist", "eastasian"];
    v.push(with_tex(n(g, "bu_amituofo", "Nianfo — Namo Amituofo (Chinese Pure Land)", "Chanting with the wooden fish · 18 min", 0.3, "none",
      "<p>In Chinese Pure Land Buddhism, <b>nianfo</b> (\"recollecting the Buddha\") means reciting <b>Namo Amituofo</b>, \"homage to Amitābha Buddha\", often with a wooden fish keeping time and the recitations counted on prayer beads. Masters such as <b>Yinguang</b> (1862–1940) taught it as a complete path open to everyone, and Chan teachers joined it to the question \"who is reciting?\". In monasteries it is chanted walking and sitting, sometimes for days at a time.</p>",
      ea, mx("chant", "amituofo", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to nianfo, the Chinese Pure Land practice of calling the Buddha of infinite light: namo amituofo. Sit upright, holding prayer beads if you have them.").mus(0.0).tex(0.0),
           st("Chanting", 10.0, &[7.5], "Chant along with the wooden fish: na mo a mi tuo fo. Move one bead with each recitation. Let each word be clear, and hear it as you say it.").mus(1.0),
           st("Silent recitation", 4.0, &[6.0], "Let the voice fall silent and continue in the mind: amituofo, amituofo. Every thought that comes, turn it into the Buddha's name.").mus(0.3).tex(0.0),
           st("Close", 2.0, &[9.0], C_DEDICATE).mus(0.0).tex(0.0)]),
      tx("mokugyo", 0.55, 460.0, 0.5)));
    v.push(n(g, "bu_guanyin", "Namo Guanshiyin Pusa — Calling on Compassion", "Chinese chant to Guanyin · 18 min", 0.35, "none",
      "<p><b>Guanyin</b> (Avalokiteśvara, \"the one who hears the cries of the world\") is the bodhisattva of compassion, loved throughout East Asia. Chapter 25 of the <i>Lotus Sutra</i> promises that whoever calls her name single-mindedly is heard. Reciting <b>Namo Guanshiyin Pusa</b> (\"homage to the bodhisattva who perceives the sounds of the world\") is a daily practice of millions of Chinese Buddhists. In Tibet the same bodhisattva is Chenrezig.</p>",
      ea, mx("chant", "guanyin", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the recitation of Guanyin's name, the bodhisattva who hears the cries of the world. Sit comfortably, palms together, and bring to mind anyone who is suffering.").mus(0.3),
           st("Chanting", 9.0, &[7.5], "Chant with the community: namo guan shi yin pu sa. Let compassion fill each word, as if the sound itself were care flowing out to the world.").mus(1.0),
           st("Listening", 5.0, &[6.0], "Now stop chanting, and listen. Guanyin's way is listening deeply. Turn your hearing inward, and rest in the quiet behind all sounds.").mus(0.25),
           st("Close", 2.0, &[9.0], C_DEDICATE).mus(0.0)]));

    let g = "Tibetan Buddhism & Bön";
    let tb: &[&str] = &["buddhist", "tibetan"];
    v.push(n(g, "tb_vajrasattva", "Vajrasattva Purification — Om Benza Satto Hung", "Nectar of purification · 20 min", 0.35, "none",
      "<p><b>Vajrasattva</b> (Tibetan Dorje Sempa) is the buddha of purification. His practice is one of the four \"foundational practices\" (<i>ngöndro</i>) of Tibetan Buddhism: with regret and resolve, you visualise him white and radiant above your head while nectar flows down through you, washing away harm and obscuration. The full mantra has 100 syllables; this session uses the short form, <b>Om Benza Satto Hung</b> (Sanskrit <i>Oṃ Vajrasattva Hūṃ</i>).</p>",
      tb, mx("chant", "vajrasattva", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to Vajrasattva purification. Sit upright. Recall with honest regret something you have done that caused harm, and resolve not to repeat it.").mus(0.2),
           st("Visualisation", 4.0, &[8.0], "Above your head, on a lotus and moon, picture Vajrasattva: white as snow, radiant, smiling, holding a vajra and a bell. Feel his complete acceptance.").mus(0.4),
           st("Nectar", 10.0, &[6.5], "Chant om benza satto hung. With each round, white nectar flows from his heart into your crown, washing harm and heaviness out through the soles of your feet.").mus(1.0),
           st("Purified", 2.0, &[6.0], "Vajrasattva melts into light and dissolves into you. Rest, clean and clear, like a crystal.").mus(0.2),
           st("Close", 2.0, &[9.0], C_DEDICATE).mus(0.0)]));
    v.push(n(g, "tb_manjushri", "Mañjuśrī — Om A Ra Pa Tsa Na Dhih (Wisdom)", "Mantra of wisdom · 18 min", 0.35, "none",
      "<p><b>Mañjuśrī</b> (Tibetan Jampelyang) is the bodhisattva of wisdom, holding a flaming sword that cuts through ignorance and a book of the Perfection of Wisdom. His mantra, <b>Om A Ra Pa Tsa Na Dhih</b>, is made from the first letters of the <i>Arapacana</i> syllabary, which Mahāyāna texts link to insight into emptiness. Tibetan students recite it, ending with a run of <i>dhih</i>, to sharpen understanding and memory before study and debate.</p>",
      tb, mx("chant", "manjushri", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the practice of Manjushri, the bodhisattva of wisdom. Sit upright. In front of you, picture him golden-orange and youthful, a flaming sword raised in his right hand.").mus(0.2),
           st("Mantra", 10.0, &[8.0], "Chant om a ra pa tsa na dhih. Light streams from Manjushri's heart into yours, clearing dullness and confusion like sun burning off fog.").mus(1.0),
           st("Dhih", 3.0, &[10.0], "Now repeat the seed syllable dhih, again and again, quick and clear. Feel your mind sharpen, bright and awake.").mus(0.4),
           st("Rest", 2.0, &[7.0], "Manjushri dissolves into light and merges with you. Rest in that clarity.").mus(0.0),
           st("Close", 1.0, &[10.0], C_DEDICATE).mus(0.0)]));
    v.push(n(g, "tb_guruyoga", "Guru Rinpoche — The Vajra Guru Mantra", "Guru yoga of Padmasambhava · 20 min", 0.35, "none",
      "<p><b>Padmasambhava</b>, called <b>Guru Rinpoche</b> (\"precious teacher\"), brought Vajrayāna Buddhism to Tibet in the 8th century and helped found its first monastery, Samye, with King Trisong Detsen and the abbot Śāntarakṣita. In <b>guru yoga</b> you visualise the teacher, receive his blessing as light from three syllables at his forehead, throat and heart, and merge your mind with his. His mantra, the <b>Vajra Guru mantra</b>, is <i>Om Ah Hung Benza Guru Pema Siddhi Hung</i>.</p>",
      tb, mx("chant", "guru", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the guru yoga of Guru Rinpoche, who brought Buddhism to Tibet. Sit upright. In the sky before you, picture him seated in rainbow light, smiling and full of power.").mus(0.2),
           st("Three lights", 5.0, &[8.0], "White light from his forehead enters yours, red light from his throat enters your throat, and blue light from his heart enters your heart: body, speech and mind blessed.").mus(0.5),
           st("Mantra", 9.0, &[6.5], "Chant the Vajra Guru mantra: om ah hung, benza guru pema siddhi hung. Let devotion and the sound carry you closer to him.").mus(1.0),
           st("Merging", 3.0, &[6.0], "Guru Rinpoche melts into light and dissolves into you. Your mind and his are one. Rest in that open awareness.").mus(0.0),
           st("Close", 1.0, &[10.0], C_DEDICATE).mus(0.0)]));
    v.push(n(g, "tb_whitea", "The White A — Dzogchen Guardian of Awareness (Bön)", "Sound, light and space · 18 min", 0.35, "none",
      "<p>In <b>Dzogchen</b>, the \"Great Perfection\" taught in the Nyingma school and in <b>Bön</b>, Tibet's pre-Buddhist religion, the Tibetan letter <b>A</b> stands for the unborn nature of mind. Students visualise a white A in a sphere of five-coloured light at the heart, sound a long \"Ahh\", and let the mind dissolve into open space. The Bön teacher <b>Tenzin Wangyal Rinpoche</b> teaches it as the basis of meditation and of dream practice.</p>",
      tb, mx("chant", "white_a", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the practice of the white A, from Dzogchen and Bon. Sit upright, eyes open and gazing softly into the space in front of you.").mus(0.0),
           st("Ahh", 6.0, &[7.5], "Breathe in, and on the out-breath sound a long, open ahh with the chanters. Feel the sound spread out from your heart in every direction.").mus(1.0),
           st("The white A", 5.0, &[6.0], "Picture a shining white letter A at your heart, inside a sphere of rainbow light. With each ahh, its light fills your body and the whole of space.").mus(0.5),
           st("Open space", 4.0, &[5.5], "Let the A dissolve into light, and the light into space. Rest as open awareness, without centre or edge.").mus(0.15),
           st("Close", 1.0, &[10.0], C_DEDICATE).mus(0.0)]));
}

fn hindu(v: &mut Vec<Preset>) {
    let g = "Hindu & Yogic Meditation";
    let tan = || mx("tanpura", "", 0.6, 0.0, 0.0);
    v.push(n(g, "hy_netineti", "Neti Neti — Not This, Not This (Advaita)", "Negation to the witness · 20 min", 0.45, "pink",
      "<p><b>Neti neti</b> (\"not this, not this\") comes from the <i>Bṛhadāraṇyaka Upaniṣad</i> (about 700 BCE), where the sage <b>Yājñavalkya</b> describes the Self (<i>ātman</i>) only by negation: it can never be grasped as an object. In <b>Advaita Vedānta</b>, from Śaṅkara (8th century) to modern teachers, the method sets aside the body, sensations, feelings and thoughts as \"not me\", one by one, until only the witnessing awareness remains.</p>",
      &["hindu", "vedanta"], tan(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to neti neti, the Upanishads' way of finding the self by saying: not this, not this. Sit comfortably and close your eyes."),
           st("Not the body", 4.0, &[8.0], "Feel your body: its weight, warmth and sensations. They are known, so they are objects. Say inwardly: not this, not this. I am that which knows them."),
           st("Not the mind", 6.0, &[6.5], "Now notice feelings and thoughts as they pass. They come and go before you. Not this, not this. You are the one in whom they appear."),
           st("The witness", 6.0, &[6.0], "What remains, that you can't set aside? Rest as the silent witness, aware of everything, itself not an object. Simply be that."),
           st("Close", 2.0, &[9.0], C_STILL)]));
    v.push(n(g, "hy_japa", "Japa with a Mālā — 108 Repetitions of Om", "Bead by bead · 18 min", 0.35, "none",
      "<p><b>Japa</b> is repeating a mantra, softly aloud, whispered or silently, often counted on a <b>mālā</b> of 108 beads with a larger \"guru\" bead that is not crossed. In the <i>Bhagavad Gītā</i> (10.25) Krishna says that of offerings he is the offering of japa. Patañjali's <i>Yoga Sūtras</i> (1.27–28) name <b>Om</b> as the sound of the Lord and say that repeating it while reflecting on its meaning turns the mind inward. Hold the mālā in the right hand over the middle finger, and move one bead per repetition with the thumb, without using the index finger.</p>",
      &["hindu", "mantra", "yoga"], mx("chant", "aum|tanpura*0.5", 1.0, 136.1, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to japa, mantra repetition on a mala of 108 beads. Sit upright, holding the mala in your right hand, draped over the middle finger, starting at the bead next to the large guru bead.").mus(0.3),
           st("Aloud", 5.0, &[8.0], "Chant om with the group, and move one bead toward you with your thumb on each om. Let the a, the u and the m each sound fully.").mus(1.0),
           st("Whispered", 5.0, &[7.0], "Now let your om become a whisper, still one bead each time. Feel the meaning: the sound of the whole, from waking to dream to deep sleep.").mus(0.5),
           st("Silent", 5.0, &[6.0], "Continue silently, in the mind alone. When you reach the guru bead, don't cross it: turn the mala around and continue.").mus(0.2),
           st("Close", 1.0, &[10.0], C_STILL).mus(0.0)]));
    v.push(n(g, "hy_samyama", "Saṃyama — Concentration, Meditation, Absorption (Patañjali)", "Yoga Sūtras, Book III · 22 min", 0.45, "pink",
      "<p>Book III of Patañjali's <i>Yoga Sūtras</i> (about 400 CE) describes <b>saṃyama</b>, the three inner limbs practised together on one object: <b>dhāraṇā</b> (concentration, holding the mind to one place), <b>dhyāna</b> (meditation, an unbroken flow of attention toward it) and <b>samādhi</b> (absorption, when only the object shines, as if empty of oneself). Choose one simple object: a flame, a sound, the breath at the nostrils, or a point of light at the heart.</p>",
      &["hindu", "yoga"], tan(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to samyama, the three inner limbs of Patanjali's yoga. Sit upright and still. Choose one simple object: the breath at the nostrils, a flame, or a point of light at the heart."),
           st("Dharana", 6.0, &[7.5], "Dharana, concentration. Hold your attention on the object. Each time the mind wanders, bring it back, kindly and at once."),
           st("Dhyana", 7.0, &[6.0], "Dhyana, meditation. Let the returning become a flow, attention streaming toward the object like oil poured from one vessel into another."),
           st("Samadhi", 5.0, &[5.0], "Samadhi, absorption. Let the sense of a meditator fade, until only the object remains, shining by itself. Don't force it; simply allow."),
           st("Close", 2.0, &[9.0], C_STILL)]));
    v.push(n(g, "hy_pratyahara", "Pratyāhāra — Withdrawing the Senses", "The fifth limb of yoga · 18 min", 0.45, "pink",
      "<p><b>Pratyāhāra</b> (\"drawing back\") is the fifth of the eight limbs of yoga in the <i>Yoga Sūtras</i> (2.54–55): the senses withdraw from their objects, \"as a tortoise draws in its limbs\" (<i>Bhagavad Gītā</i> 2.58), and follow the mind inward. Modern teachers, such as Swami Satyananda in <i>Yoga Nidra</i> (1976), teach it by moving attention from distant sounds to nearby sounds, then to the body and the breath.</p>",
      &["hindu", "yoga"], tan(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to pratyahara, the withdrawal of the senses, the fifth limb of yoga. Sit or lie comfortably, and close your eyes."),
           st("Far sounds", 3.0, &[8.0], "Listen to the most distant sounds you can hear. Don't name them; just hear them. Then let them go."),
           st("Near sounds", 3.0, &[7.0], "Now listen to sounds nearby: the room, your clothing, your breath. Hear them, and let them go."),
           st("The body", 5.0, &[6.0], "Draw attention inside the skin. Feel the body from within, its warmth and weight, as a tortoise draws in its limbs."),
           st("Inner space", 4.0, &[5.0], "Now rest in the dark space behind your closed eyes. The senses are quiet, and the mind turns inward."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(n(g, "hy_sriyantra", "Śrī Yantra — Journey to the Bindu (Śrī Vidyā)", "Visualising the yantra · 22 min", 0.45, "pink",
      "<p>The <b>Śrī Yantra</b> is the central symbol of <b>Śrī Vidyā</b>, the tantric worship of the goddess Lalitā Tripurasundarī: nine interlocking triangles (four pointing up for Śiva, five pointing down for Śakti) that form 43 smaller triangles, ringed by lotus petals and a square enclosure with four gates, around a central point, the <b>bindu</b>. Practitioners travel inward, enclosure by enclosure, from the outer square to the bindu, the union of Śiva and Śakti. Use a printed image if you have one.</p>",
      &["hindu", "tantra"], tan(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Sri Yantra meditation of the Sri Vidya tradition. If you have an image of the yantra, gaze at it softly; if not, picture it with your eyes closed."),
           st("The square", 4.0, &[8.0], "See the outer square with its four gates: the world of the senses. Enter through the gate in front of you."),
           st("The lotuses", 5.0, &[7.0], "Pass through the rings of lotus petals, sixteen, then eight. Feel the mind growing quieter with each ring."),
           st("The triangles", 6.0, &[6.0], "Now move through the interlocking triangles, upward for Shiva, downward for Shakti, circuit after circuit, toward the centre."),
           st("The bindu", 4.0, &[5.0], "Arrive at the bindu, the single point at the centre, where Shiva and Shakti are one. Rest there, in pure, still awareness."),
           st("Close", 1.0, &[10.0], C_STILL)]));
}

fn jain_sikh(v: &mut Vec<Preset>) {
    let g = "Jain Meditation";
    let jn: &[&str] = &["jain"];
    v.push(n(g, "jn_navkar", "Navkār Mantra — Bowing to the Five Supreme Beings", "The universal Jain prayer · 16 min", 0.35, "none",
      "<p>The <b>Navkār</b> (or Namokār) <b>Mantra</b> is the most important prayer of <b>Jainism</b>, recited by Jains of every sect. It bows to the five supreme beings: the <i>arihants</i> (enlightened teachers), the <i>siddhas</i> (liberated souls), the <i>ācāryas</i> (heads of orders), the <i>upādhyāyas</i> (teachers) and all <i>sādhus</i> (monks and nuns). It asks for nothing and names no one: it honours qualities, not persons. Its last lines say that this fivefold bow destroys all obstacles and is the first of all blessings.</p>",
      jn, mx("chant", "navkar|tanpura*0.4", 1.0, 138.59, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Navkar Mantra, the universal prayer of the Jains. Sit upright with your hands folded. It honours qualities, not persons: the awakened, the liberated, the teachers and the monks and nuns.").mus(0.3),
           st("Chanting", 9.0, &[7.5], "Chant along, line by line: namo arihantanam, namo siddhanam, namo ayariyanam, namo uvajjhayanam, namo loe savva sahunam. With each line, bow inwardly.").mus(1.0),
           st("Silent", 4.0, &[6.0], "Continue the mantra silently. Picture each of the five as a light of a different colour: white, red, yellow, blue and dark green.").mus(0.2),
           st("Close", 1.0, &[10.0], "May all living beings be at peace. May I forgive all beings, and may all beings forgive me.").mus(0.0)]));
    v.push(n(g, "jn_preksha", "Prekṣā Dhyāna — Perceiving the Breath and Body", "Ācārya Mahāprajña, 1970s · 20 min", 0.45, "pink",
      "<p><b>Prekṣā dhyāna</b> (\"perceptive meditation\") was developed in the 1970s by <b>Ācārya Mahāprajña</b> of the Jain Śvetāmbara Terāpanthī order, under Ācārya Tulsī, from Jain texts such as the <i>Ācārāṅga Sūtra</i>. Its core steps are relaxation (<i>kāyotsarga</i>), perception of the breath (<i>śvāsa prekṣā</i>), perception of the body (<i>śarīra prekṣā</i>) and of psychic centres (<i>caitanya kendra</i>). Its key instruction: perceive, without liking or disliking.</p>",
      jn, mx("tanpura", "", 0.5, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to preksha dhyana, the Jain meditation of perceiving. Sit upright, eyes closed. The one rule is: perceive what is there, without liking or disliking it."),
           st("Relaxation", 4.0, &[7.5], "Relax the body part by part, from the toes to the head, and feel it become still. Let the body be at ease while the mind stays awake."),
           st("The breath", 6.0, &[6.5], "Perceive the breath: slow, deep, through the belly. Then feel it at the nostrils, cool coming in, warm going out, without controlling it."),
           st("The body", 6.0, &[6.0], "Move attention slowly through the body, perceiving every sensation: warmth, pressure, tingling, pain, ease. Neither cling nor push away."),
           st("Close", 2.0, &[9.0], "May all living beings be at peace. May I forgive all beings, and may all beings forgive me.")]));
    v.push(n(g, "jn_samayika", "Sāmāyika — Forty-Eight Minutes of Equanimity", "The Jain vow of stillness · 48 min", 0.4, "pink",
      "<p><b>Sāmāyika</b> is the daily Jain vow of <b>equanimity</b>: for one <i>muhūrta</i>, 48 minutes, you sit apart, renounce all harmful activity, and rest in sameness of mind toward friend and foe, pleasure and pain, gain and loss. Lay Jains often keep it at dawn or dusk, with the Navkār Mantra, scripture study and silent meditation. It is a foretaste of the equanimity of the liberated soul.</p>",
      jn, mx("rin", "bell", 1.0, 0.0, 0.0),
      vec![st("The vow", 3.0, &[10.0], "Welcome to samayika, forty-eight minutes of equanimity. Sit on a clean mat. Make the vow: for this time I give up every harmful act of body, speech and mind."),
           st("Recitation", 10.0, &[8.0], "Recite the Navkar Mantra slowly, or read a passage of scripture you love. Let the mind settle in what is good."),
           st("Equanimity", 25.0, &[6.0], "Rest in equanimity. Whatever comes, pleasant or painful, praise or blame, see it as passing, and stay even. Friend and foe, gain and loss: the same."),
           st("Forgiveness", 8.0, &[7.0], "Bring to mind anyone you have hurt, and anyone who has hurt you. Silently say: I forgive all beings; may all beings forgive me. I have friendship with all."),
           st("Close", 2.0, &[10.0], "The time of samayika is complete. Carry its evenness into the day, and open your eyes.")]));
    v.push(n(g, "jn_kayotsarga", "Kāyotsarga — Abandoning the Body", "Deep stillness with awareness · 18 min", 0.4, "pink",
      "<p><b>Kāyotsarga</b> (\"abandoning the body\") is the Jain practice of standing or sitting completely still while letting go of identification with the body. The Jain tīrthaṅkaras are shown in this standing posture, arms hanging straight, and the great statue of Bāhubali at Shravanabelagola shows him so still that vines climb his limbs. In Prekṣā dhyāna it is the first step: deep relaxation with full awareness.</p>",
      jn, mx("tanpura", "", 0.45, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to kayotsarga, the Jain practice of letting go of the body. Lie on your back, or stand with your arms hanging loosely, and close your eyes."),
           st("Stillness", 4.0, &[7.5], "Become completely still. Decide not to move for the rest of the session, as still as a statue, but soft and relaxed."),
           st("Letting go", 8.0, &[5.5], "Move through the body, and tell each part to relax: feet, legs, belly, chest, arms, neck, face. Feel it released, as if it were not yours."),
           st("Pure awareness", 3.0, &[5.0], "Rest as the awareness that knows the body, separate from it, calm and clear: the soul, Jains say, shining by itself."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));

    let g = "Sikh Simran";
    let sk: &[&str] = &["sikh"];
    v.push(n(g, "sk_waheguru", "Waheguru Simran — Remembering the Wondrous Lord", "Naam simran with harmonium · 18 min", 0.35, "none",
      "<p><b>Sikhism</b>, founded by <b>Guru Nānak</b> (1469–1539) in the Punjab, teaches <b>naam simran</b>, the loving remembrance of God's name, as the heart of spiritual life. <b>Waheguru</b> (\"wondrous Lord\", or \"wondrous enlightener\") is the name most often repeated: aloud in kirtan with harmonium and tabla, or silently with the breath, <i>Wahe</i> on the in-breath and <i>Guru</i> on the out-breath. The Gurus taught simran together with a full householder's life of honest work and sharing.</p>",
      sk, mx("chant", "waheguru|drone:harmonium*0.5", 1.0, 138.59, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to Waheguru simran, the Sikh remembrance of God's name. Cover your head if that is your custom, sit cross-legged or on a chair, and close your eyes.").mus(0.3),
           st("Kirtan", 8.0, &[7.5], "Sing along with the sangat, the congregation: waheguru, waheguru. Let your heart go into each word: wondrous Lord, wondrous enlightener.").mus(1.0),
           st("With the breath", 6.0, &[6.0], "Now continue silently: wahe as you breathe in, guru as you breathe out. Let the name and the breath become one.").mus(0.25),
           st("Close", 2.0, &[9.0], "Rest in the name. Waheguru ji ka khalsa, waheguru ji ki fateh. Slowly open your eyes.").mus(0.0)]));
    v.push(n(g, "sk_mulmantar", "Mūl Mantar — The Root Verse of the Guru Granth Sahib", "Recitation and reflection · 16 min", 0.35, "none",
      "<p>The <b>Mūl Mantar</b> (\"root verse\") opens the <i>Guru Granth Sahib</i>, the Sikh scripture, and sums up Guru Nānak's teaching: <i>Ik Onkār</i> (one creator), <i>Sat Nām</i> (truth is the name), <i>Karatā Purakh</i> (the creative being), <i>Nirbhau</i> (without fear), <i>Nirvair</i> (without hatred), <i>Akāl Mūrat</i> (timeless form), <i>Ajūnī</i> (beyond birth), <i>Saibhaṅ</i> (self-existent), <i>Gur Prasād</i> (known by the Guru's grace). Sikhs recite it every day.</p>",
      sk, mx("chant", "mulmantar|drone:harmonium*0.5", 1.0, 138.59, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Mool Mantar, the root verse of the Sikh scripture. Sit upright, hands folded, and listen to its opening words: ik onkar, one creator, present in all.").mus(0.3),
           st("Recitation", 8.0, &[7.5], "Recite along with the sangat: ik onkar, sat naam, karta purakh, nirbhau, nirvair, akaal moorat, ajooni, saibhang, gur prasaad.").mus(1.0),
           st("Reflection", 5.0, &[6.0], "Stay with one quality: nirbhau, without fear, or nirvair, without hatred. Ask how the one who is fearless and without hatred could live through you today.").mus(0.2),
           st("Close", 1.0, &[10.0], C_STILL).mus(0.0)]));
}

fn abrahamic(v: &mut Vec<Preset>) {
    let g = "Sufi Mysticism";
    let su: &[&str] = &["abrahamic", "islamic", "sufi"];
    v.push(n(g, "su_jahri", "Jahrī Dhikr — Lā ilāha illā-llāh with the Daf", "Voiced remembrance · 18 min", 0.3, "none",
      "<p>Sufi orders distinguish <b>silent dhikr</b> (<i>khafī</i>), kept above all by the Naqshbandī order, from <b>voiced dhikr</b> (<i>jahrī</i>), kept by orders such as the Qādirī, Rifāʿī and Chishtī. In a <i>ḥaḍra</i> (\"presence\") gathering the circle chants <b>Lā ilāha illā-llāh</b> together, often with a frame drum (<i>daf</i>), swaying, the rhythm slowly building. The first half of the phrase negates (\"no god\"), the second affirms (\"but God\").</p>",
      su, mx("chant", "lailaha|daf*0.5", 1.0, 0.0, 0.3),
      vec![st("Settle", 2.0, &[10.0], "Welcome to jahri dhikr, the voiced remembrance of the Sufi circle. Sit on the floor or a chair, back straight, hands on your knees, and close your eyes.").mus(0.2),
           st("Dhikr", 10.0, &[7.5], "Chant with the circle and the drum: la ilaha illallah. On la ilaha, turn your head gently to the right; on illallah, bring it down toward your heart.").mus(1.0),
           st("In the heart", 4.0, &[6.0], "Let your voice fall silent while the circle continues. Feel the phrase beating in your heart: no god, but God.").mus(0.4),
           st("Close", 2.0, &[9.0], C_STILL).mus(0.0)]));
    v.push(n(g, "su_hu", "Hū — The Breath of the Divine Name", "Allāh Hū · 16 min", 0.3, "none",
      "<p><b>Hū</b> (\"He\") is the last breath of the name <b>Allāh</b> and, for Sufis, the most intimate name: the pronoun of the divine essence, beyond every attribute. In many orders the <i>ḥaḍra</i> ends with <i>Allāh Hū</i>, the sound growing breathier until it is only the breath itself. The poet <b>Rūmī</b> (1207–1273) wrote that the Hū is in every breath.</p>",
      su, mx("chant", "hu", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the dhikr of Hu, the last breath of the divine name. Sit comfortably, close your eyes, and place your attention in the centre of your chest.").mus(0.2),
           st("Allah Hu", 8.0, &[7.0], "Chant with the circle: allah hu. Let the hu be long and soft, falling down into your chest like a deep sigh.").mus(1.0),
           st("The breath", 4.0, &[5.5], "Let the sound become only breath. Hear the hu in every out-breath, quiet as wind, the name breathing itself.").mus(0.3),
           st("Close", 2.0, &[9.0], C_STILL).mus(0.0)]));
    v.push(n(g, "su_tasbih", "Tasbīḥ — Glory, Praise and Greatness (33 × 3)", "Remembrance after prayer · 14 min", 0.3, "none",
      "<p>After each of the five daily prayers, many Muslims recite the <b>tasbīḥ</b> taught in the ḥadīth: <i>Subḥānallāh</i> (glory be to God) 33 times, <i>Al-ḥamdu lillāh</i> (praise be to God) 33 times and <i>Allāhu akbar</i> (God is greatest) 33 or 34 times, counted on the fingers or on prayer beads (<i>misbaḥa</i>). The Prophet Muḥammad is reported to have taught it to his daughter <b>Fāṭima</b>, so it is often called the tasbīḥ of Fāṭima.</p>",
      &["abrahamic", "islamic", "sufi"], mx("chant", "tasbih", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the tasbih, the remembrance that follows prayer. Sit facing the qibla if you like, holding prayer beads, or count on the joints of your right hand.").mus(0.2),
           st("Tasbih", 9.0, &[7.5], "Murmur along: subhanallah, glory be to God, thirty-three times; alhamdulillah, praise be to God, thirty-three times; allahu akbar, God is greatest.").mus(1.0),
           st("Gratitude", 2.0, &[6.0], "Sit quietly, and let gratitude fill the heart for everything you have been given.").mus(0.2),
           st("Close", 1.0, &[10.0], C_STILL).mus(0.0)]));
    v.push(n(g, "su_ney", "The Song of the Reed — Listening to the Ney (Rūmī)", "Samāʿ, mystical listening · 16 min", 0.3, "none",
      "<p>Rūmī's great poem, the <i>Masnavī</i>, opens with the <b>song of the reed</b>: the reed flute (<i>ney</i>), cut from its reed bed, cries out with longing to return to its origin, as the soul longs for God. For the Mevlevī order, the \"whirling dervishes\", the ney is the sacred instrument of <b>samāʿ</b>, spiritual listening. This session is a practice of listening: let the sound draw your own longing inward.</p>",
      su, mx("flute", "ney", 0.9, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to sama, Sufi listening. Rumi's poem begins: listen to the reed, how it tells a tale, complaining of separation. Sit comfortably and close your eyes.").mus(0.6),
           st("Listening", 8.0, &[6.5], "Listen to the ney, the reed flute, with your whole being. Let its longing touch your own: for home, for love, for the source of everything.").mus(1.0),
           st("Within", 4.0, &[5.5], "Hear the sound as if it rose from inside your own chest, the breath of the beloved passing through you, as through the reed.").mus(0.8),
           st("Close", 2.0, &[9.0], C_STILL).mus(0.3)]));

    let g = "Jewish Mysticism & Kabbalah";
    let hum = || mx("chant", "niggun_hum", 0.7, 0.0, 0.0);
    v.push(n(g, "jk_hitbonenut", "Hitbonenut — There Is Nothing But God (Chabad)", "Hasidic contemplation · 20 min", 0.45, "pink",
      "<p><b>Hitbonenut</b> (\"contemplation\") is the meditation of <b>Chabad Hasidism</b>, taught by its founder <b>Rabbi Shneur Zalman of Liadi</b> (1745–1812) in the <i>Tanya</i> and by his son <b>Rabbi Dovber</b> in his <i>Tract on Ecstasy</i>. You think a concept through, deeply and in detail, until it moves the heart. A classic theme is <i>ein od milvado</i>, \"there is nothing else besides Him\" (Deuteronomy 4:35): every thing exists only through the divine word that creates it anew at every moment.</p>",
      &["abrahamic", "jewish", "hasidic"], hum(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to hitbonenut, the contemplation of Chabad Hasidism. Sit comfortably and close your eyes. Today's theme: ein od milvado, there is nothing else besides Him.").mus(0.5),
           st("Thinking it through", 7.0, &[7.0], "Think slowly: the world was not made once and left. At every moment the divine word speaks each thing into being. Picture a stone, a tree, your hand, held in being right now.").mus(0.35),
           st("Into the heart", 7.0, &[6.0], "Now let the idea sink from the mind into the heart. If all is held by God, what is there to fear? Let awe and love rise by themselves.").mus(0.3),
           st("Nothing else", 3.0, &[6.0], "Rest in the feeling: there is nothing else. Only the one, appearing as everything.").mus(0.3),
           st("Close", 1.0, &[10.0], C_STILL).mus(0.5)]));
    v.push(n(g, "jk_cheshbon", "Cheshbon HaNefesh — Accounting of the Soul (Mussar)", "Evening review of a trait · 15 min", 0.45, "pink",
      "<p><b>Cheshbon ha-nefesh</b> (\"accounting of the soul\") is the nightly self-review of the <b>Mussar</b> movement, founded by <b>Rabbi Yisrael Salanter</b> (1810–1883). Its handbook, <i>Cheshbon ha-Nefesh</i> (1812) by <b>Rabbi Menachem Mendel Lefin</b>, lists thirteen <i>middot</i> (traits), among them equanimity, patience, order, decisiveness, cleanliness, humility, justice, frugality, diligence, silence, calmness, truth and separation from vice. You work on one trait a week and note each evening how you lived it.</p>",
      &["abrahamic", "jewish", "mussar"], hum(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to cheshbon hanefesh, the Mussar accounting of the soul. Sit quietly in the evening, perhaps with a small notebook. Choose this week's trait, such as patience, humility or truth.").mus(0.5),
           st("The day", 7.0, &[7.0], "Go through your day. Where did you live your trait well? Where did you fall short? Look honestly, as a loving teacher would, without harshness.").mus(0.3),
           st("The lesson", 4.0, &[6.0], "Choose one moment to learn from. What would living the trait have looked like there? Picture yourself doing it next time.").mus(0.3),
           st("Close", 2.0, &[8.0], "Note one line in your notebook, if you have one. Then let the day go, and rest.").mus(0.5)]));
    v.push(n(g, "jk_omer", "Counting the Omer — Seven Qualities of the Heart", "Sefirot of the forty-nine days · 20 min", 0.45, "pink",
      "<p>Between Passover and Shavuot, Jews count the 49 days of the <b>Omer</b> (Leviticus 23:15). The Kabbalists gave each week, and each day within it, one of the seven lower <i>sefirot</i>: <i>chesed</i> (loving-kindness), <i>gevurah</i> (strength and restraint), <i>tiferet</i> (harmony and compassion), <i>netzach</i> (endurance), <i>hod</i> (humility and splendour), <i>yesod</i> (bonding) and <i>malkhut</i> (sovereignty, presence), so that each day blends two qualities, such as \"chesed within gevurah\". This session works through all seven; in the Omer season, dwell on the day's pair.</p>",
      &["abrahamic", "jewish", "kabbalah"], hum(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the seven qualities of the Omer count. Sit comfortably and close your eyes. We will touch each of the seven lower sefirot in turn, as qualities of your own heart.").mus(0.5),
           st("Chesed, gevurah", 5.0, &[7.5], "Chesed, loving-kindness: feel warmth flowing out freely. Gevurah, strength: feel the healthy limit that gives that warmth its shape.").mus(0.3),
           st("Tiferet, netzach, hod", 6.0, &[6.5], "Tiferet, harmony: kindness and strength in balance, as compassion. Netzach, endurance: the will to keep going. Hod, humility: gratitude and yielding.").mus(0.3),
           st("Yesod, malkhut", 5.0, &[6.0], "Yesod, bonding: the connection that carries all these qualities to others. Malkhut, presence: the dignity of bringing them into the world.").mus(0.3),
           st("Close", 2.0, &[9.0], "Choose the one quality you most need now, and carry it into tomorrow. Then open your eyes.").mus(0.5)]));

    let g = "Christian Mysticism";
    v.push(n(g, "cm_lawrence", "The Practice of the Presence of God (Brother Lawrence)", "Prayer in everyday life · 13 min", 0.45, "pink",
      "<p><b>Brother Lawrence of the Resurrection</b> (Nicolas Herman, about 1614–1691) was a lay brother of the Carmelite monastery in Paris, who worked in the kitchen and repaired sandals. His letters and conversations, gathered in <i>The Practice of the Presence of God</i> (1692), teach a continual, simple conversation with God in the middle of ordinary work: for him the time of work did not differ from the time of prayer. This session builds the habit so you can carry it into the day.</p>",
      &["abrahamic", "christian", "catholic", "mystic"], mx("rin", "church", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the practice of the presence of God, as Brother Lawrence lived it among the pots and pans of a monastery kitchen. Sit simply, and close your eyes."),
           st("Presence", 4.0, &[8.0], "Become aware that God is here, closer than your breath. No special words are needed. Simply turn toward him, as you would toward a friend in the room."),
           st("Little words", 5.0, &[7.5], "Speak to him in short, simple words, from the heart: my God, I am yours. Thank you. Help me. Let each one be a glance of love."),
           st("Into the day", 2.0, &[10.0], "Now open your eyes, and choose one ordinary task to do today in his company. When you forget, simply return, without fuss.")]));
    v.push(n(g, "cm_examen", "The Daily Examen (St Ignatius of Loyola)", "Five steps of evening review · 15 min", 0.45, "pink",
      "<p>St <b>Ignatius of Loyola</b> asked the Jesuits never to omit the <b>Examen</b>, a daily review set out in his <i>Spiritual Exercises</i> (1548). Its classic five steps are: give thanks; ask for light; review the day hour by hour, noticing where you felt drawn toward God (consolation) or away from him (desolation); ask forgiveness; and resolve for tomorrow. It takes about fifteen minutes, usually in the evening.</p>",
      &["abrahamic", "christian", "catholic", "mystic"], mx("drone", "organ", 0.4, 0.0, 0.0),
      vec![st("Gratitude", 2.0, &[10.0], "Welcome to the daily examen of Saint Ignatius. Sit quietly and become aware of God's presence. First, give thanks for three gifts of today, however small."),
           st("Light", 1.0, &[9.0], "Ask for light: to see your day as God sees it, honestly and with love."),
           st("Review", 7.0, &[7.0], "Now replay the day, hour by hour. Where did you feel alive, peaceful, close to God? Where did you feel restless, empty, or far away? Notice, without judging."),
           st("Forgiveness", 3.0, &[6.5], "Choose one moment you regret. Ask for forgiveness, and feel it given."),
           st("Tomorrow", 2.0, &[9.0], "Look ahead to tomorrow. Ask for the grace you will need, and end with the Our Father.")]));
    v.push(n(g, "cm_compline", "Compline — Night Prayer: In Manus Tuas", "The last office of the day · 16 min", 0.35, "none",
      "<p><b>Compline</b> (Latin <i>completorium</i>, \"completion\") is the night prayer of the Liturgy of the Hours, set out in the 6th-century Rule of St Benedict and still sung in monasteries before the night's great silence. It includes an examination of conscience, a psalm (4, 91 or 134), the responsory <b>In manus tuas, Domine, commendo spiritum meum</b> (\"Into your hands, Lord, I commend my spirit\", Luke 23:46 and Psalm 31:5), the Song of Simeon and a hymn to Mary.</p>",
      &["abrahamic", "christian", "catholic", "mystic"], mx("chant", "inmanus|rin:church*0.6", 1.0, 0.0, 0.0),
      vec![st("Examination", 3.0, &[9.0], "Welcome to compline, the night prayer of monasteries. Sit or kneel quietly. Look back on the day, and ask forgiveness for what was not done in love.").mus(0.0),
           st("Psalm", 4.0, &[7.5], "Pray the words of psalm four: in peace I will lie down and sleep, for you alone, Lord, make me dwell in safety.").mus(0.3),
           st("In manus tuas", 5.0, &[6.0], "The monks sing in manus tuas: into your hands, Lord, I commend my spirit. Let your own spirit rest in those hands.").mus(1.0),
           st("Nunc dimittis", 3.0, &[5.0], "Now, Lord, you let your servant go in peace. Protect us, Lord, as we stay awake; watch over us as we sleep.").mus(0.4),
           st("Great silence", 1.0, &[4.0], super::C_REST).mus(0.0)]));
    v.push(n(g, "cm_kyrie", "Kyrie Eleison — Chanting Mercy", "Lord, have mercy · 15 min", 0.35, "none",
      "<p><b>Kyrie eleison</b> (Greek, \"Lord, have mercy\") is among the oldest Christian prayers: sung in the Greek East since at least the 4th century, and kept in Greek even in the Latin Mass. In the Orthodox liturgy it answers each petition of the litanies, sometimes forty times over. Repeated slowly it becomes a prayer of the heart, close to the Jesus Prayer. Here a men's choir chants it in plainchant style.</p>",
      &["abrahamic", "christian", "catholic", "orthodox", "mystic"], mx("chant", "kyrie", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Kyrie, one of the oldest Christian prayers: kyrie eleison, Lord, have mercy. Sit or stand still, and let your heart open.").mus(0.3),
           st("Chant", 8.0, &[7.0], "Listen to the chant, or sing along: kyrie eleison, christe eleison, kyrie eleison. Bring before God yourself, and everyone who needs mercy.").mus(1.0),
           st("Mercy", 4.0, &[6.0], "Let the chant continue softly. Feel mercy, not as judgement lifted, but as love that holds you just as you are.").mus(0.4),
           st("Close", 1.0, &[10.0], C_STILL).mus(0.0)]));
    v.push(n(g, "cm_eckhart", "Gelassenheit — Meister Eckhart's Letting-Be", "Into the ground of the soul · 20 min", 0.45, "pink",
      "<p><b>Meister Eckhart</b> (about 1260–1328), a German Dominican, preached that God is born in the <b>ground of the soul</b> when we become empty of images and of self-will. His word <b>Gelassenheit</b> (\"letting-be\", releasement) means leaving self and things and letting God be God in us; his <b>Abgeschiedenheit</b> (detachment) is the stillness in which this happens. Some of his teachings were condemned in 1329; today he is read across Christianity and far beyond.</p>",
      &["abrahamic", "christian", "mystic"], mx("choir", "hum", 0.55, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to Gelassenheit, the letting-be of Meister Eckhart. Sit still and close your eyes. Eckhart said: God is at home; it is we who have gone out for a walk."),
           st("Letting go of things", 5.0, &[7.0], "Let go of the things that occupy you: plans, worries, possessions. Not by force: simply leave them be, like coats hung by the door."),
           st("Letting go of self", 6.0, &[6.0], "Now let go even of wanting: wanting to pray well, to feel something, to get somewhere. Let the self-will become quiet and empty."),
           st("The ground", 5.0, &[5.0], "In that emptiness, rest in the ground of the soul, where, Eckhart says, God's ground and my ground are one. Nothing to do. Only be."),
           st("Close", 2.0, &[9.0], C_STILL)]));
}

fn taoist(v: &mut Vec<Preset>) {
    let g = "Taoist Inner Cultivation";
    let ta: &[&str] = &["taoist"];
    let qin = || mx("guqin", "", 0.7, 0.0, 0.0);
    v.push(n(g, "ta_goldenflower", "The Secret of the Golden Flower — Circulating the Light", "Taoist inner alchemy · 22 min", 0.45, "pink",
      "<p><b>The Secret of the Golden Flower</b> (<i>Taiyi Jinhua Zongzhi</i>), a Taoist inner-alchemy text received in spirit-writing sessions in 17th-century China and attributed to the immortal <b>Lü Dongbin</b>, teaches <b>circulating the light</b> (<i>huiguang</i>): turning the gaze inward and letting attention rest between the eyes until a light seems to circle. Richard Wilhelm's German translation (1929), with a commentary by Carl Jung, made it famous in the West.</p>",
      ta, qin(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the circulation of the light, from The Secret of the Golden Flower. Sit upright, relaxed, and lower your eyelids until only a little light comes in."),
           st("Turning the light", 6.0, &[7.5], "Look down along the tip of your nose, then let your attention rest gently between the eyes. Turn the light of your awareness back, toward its own source."),
           st("Circulating", 8.0, &[6.0], "Let the breath become soft and silent. Feel the light, or simply the attention, settle in the centre, and circle there quietly, like a slow, bright wheel."),
           st("The golden flower", 4.0, &[5.5], "Don't chase any light or image. If one appears, let it be. Rest in the stillness where, the text says, the golden flower blooms."),
           st("Close", 2.0, &[9.0], "Rub your palms together until warm, and cover your eyes with them. Then open your eyes slowly.")]));
    v.push(n(g, "ta_innersmile", "The Inner Smile — Smiling to the Organs", "Taoist qigong meditation · 18 min", 0.45, "pink",
      "<p>The <b>Inner Smile</b> is a Taoist qigong meditation taught widely in the West by <b>Mantak Chia</b> from the 1980s (<i>Taoist Ways to Transform Stress into Vitality</i>, 1985), drawn from older Chinese practices of nourishing the organs. You let a genuine smile, felt first in the eyes, flow down to the heart, lungs, liver, kidneys and the rest of the body, as if thanking each part for its work.</p>",
      ta, qin(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the inner smile. Sit near the edge of a chair, feet flat, hands resting in your lap. Picture someone or something that makes you smile."),
           st("The eyes", 3.0, &[8.0], "Let that smile come into your eyes, soft and warm. Feel your face relax around it."),
           st("Heart and lungs", 5.0, &[7.0], "Let the smile flow down to your heart: thank it for beating. Then to your lungs: thank them for every breath. Feel them soften and warm."),
           st("The organs", 5.0, &[6.0], "Smile down to your liver on the right, your stomach, your kidneys in the lower back. Thank each one, and let the warmth spread."),
           st("The whole body", 2.0, &[6.0], "Now smile to the whole body at once, then gather the warmth into your belly, just below the navel."),
           st("Close", 1.0, &[10.0], C_STILL)]));
    v.push(n(g, "ta_embryonic", "Embryonic Breathing (Taixi)", "Breathing like a child in the womb · 18 min", 0.45, "pink",
      "<p><b>Embryonic breathing</b> (<i>taixi</i>) appears in Taoist writings from the 4th-century <i>Baopuzi</i> of <b>Ge Hong</b> onward: breathing so softly and slowly, centred in the lower belly (the <i>dantian</i>), that a feather held at the nose would not stir, \"like a child in the womb\". Later inner-alchemy teachers saw it as a return to the source. Never strain or hold the breath; let it grow fine by itself.</p>",
      ta, mx("guqin", "|stream*0.6", 0.7, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to embryonic breathing, from the Taoist classics. Sit or lie comfortably. Place your hands one over the other, just below your navel."),
           st("Belly breath", 4.0, &[7.5], "Breathe through the nose, slowly and quietly, letting the belly rise under your hands as you breathe in, and fall as you breathe out."),
           st("Finer", 7.0, &[6.0], "Let each breath become softer, longer and finer, so soft that a feather at your nose would not move. Never strain, and never hold the breath."),
           st("Like a child", 4.0, &[5.0], "Rest in the feeling of being held and breathed, like a child in the womb, the breath almost forgotten, centred in the belly."),
           st("Return", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(n(g, "ta_baduanjin", "Baduanjin — The Eight Pieces of Brocade", "Standing qigong · 16 min", 0.35, "pink",
      "<p><b>Baduanjin</b> (\"eight pieces of brocade\") is one of the oldest and most popular qigong sets, recorded in the 12th century and practised in parks across China. Its eight movements stretch and turn the body in time with slow breathing: holding up the heavens; drawing the bow; separating heaven and earth; looking back; swaying the head and tail; touching the toes; punching with angry eyes; and rising on the toes. Repeat each six to eight times, slowly, within your comfort. Clear a space around you, and keep the volume low.</p>",
      ta, qin(),
      vec![st("Stand", 1.0, &[10.0], "Welcome to the eight pieces of brocade. Stand with your feet shoulder-width apart, knees soft, arms relaxed. Move slowly, breathe with each movement, and never push into pain."),
           st("Heaven and the bow", 4.0, &[10.0], "One: interlace your fingers and push your palms up to the sky, then lower them. Two: step wide, and draw an imaginary bow to the left and to the right."),
           st("Separating and looking back", 4.0, &[10.0], "Three: push one palm up and the other down, then switch. Four: turn your head slowly to look behind you, left and right, shoulders relaxed."),
           st("Swaying and touching", 3.0, &[10.0], "Five: in a gentle horse stance, sway your upper body from side to side. Six: run your hands down your legs toward your toes, then roll up slowly."),
           st("Punching and rising", 3.0, &[10.0], "Seven: make soft fists and punch slowly forward, eyes wide. Eight: rise up onto your toes, then drop gently onto your heels, seven times."),
           st("Close", 1.0, &[10.0], "Stand still, hands resting over your lower belly. Feel the warmth and tingling, and breathe naturally.")]));
    v.push(with_tex(n(g, "ta_shouyi", "Shouyi — Guarding the One (Purity and Stillness)", "Qingjing Jing chant · 18 min", 0.3, "none",
      "<p><b>Shouyi</b> (\"guarding the One\") is one of the oldest Taoist meditations, foreshadowed in the <i>Daodejing</i> (\"can you embrace the One and not let go?\", ch. 10) and described by Ge Hong in the 4th century. The <b>Qingjing Jing</b> (Classic of Purity and Stillness, Tang dynasty), chanted daily in Quanzhen monasteries, teaches that if you can be constantly pure and still, heaven and earth return to you. Here its opening lines are chanted to the wooden fish, then you rest in stillness.</p>",
      ta, mx("chant", "qingjing", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to guarding the One. Sit upright and still. The Classic of Purity and Stillness says: the great Tao has no form, yet it gives birth to heaven and earth.").mus(0.0).tex(0.0),
           st("Chanting", 6.0, &[8.0], "Listen to the monks chanting the classic with the wooden fish, or chant along. Let each syllable fall evenly, like drops of water.").mus(1.0),
           st("Guarding the One", 8.0, &[6.0], "Now let the chant end. Gather your attention into one point, just below the navel, and guard it gently, like a mother hen sitting on her egg.").mus(0.0).tex(0.0),
           st("Close", 2.0, &[9.0], "If you can be constantly pure and still, heaven and earth return to you. Rest in that stillness, then slowly open your eyes.").mus(0.0).tex(0.0)]),
      tx("mokugyo", 0.5, 460.0, 0.5)));
}

fn western(v: &mut Vec<Preset>) {
    let g = "Western Esoteric & Classical";
    let stoic: &[&str] = &["western", "classical", "secular"];
    let harp = || mx("harp", "", 0.5, 0.0, 0.0);
    v.push(n(g, "we_viewabove", "The View from Above (Stoic, Marcus Aurelius)", "Seeing life from a height · 14 min", 0.45, "pink",
      "<p>The <b>Stoics</b> practised rising above the earth in imagination and looking down on human affairs from a great height. <b>Marcus Aurelius</b>, Roman emperor (121–180), returns to it in his <i>Meditations</i> (for example 7.48 and 9.30): crowds, armies, farms, weddings and funerals, all tiny and passing. The historian of philosophy <b>Pierre Hadot</b> called it \"the view from above\". It puts worries in proportion and kindles a sense of belonging to the whole.</p>",
      stoic, harp(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the view from above, a Stoic meditation of Marcus Aurelius. Sit comfortably and close your eyes. See yourself sitting exactly where you are."),
           st("Rising", 5.0, &[8.0], "Now rise slowly, as if lifted on a calm wind: above your room, your building, your street. See the houses, the roads, the people going about their lives."),
           st("The whole earth", 4.0, &[7.0], "Rise higher, over cities, mountains and seas, until you see the whole earth turning in space. Every life, every worry you have, is down there, small and passing."),
           st("Return", 3.0, &[9.0], "Slowly come back down, to your country, your town, your room, your body. Bring the wider view with you: what really matters today?")]));
    v.push(n(g, "we_evening", "The Evening Review (Seneca & the Pythagoreans)", "Three questions before sleep · 12 min", 0.4, "pink",
      "<p>The Roman Stoic <b>Seneca</b> (about 4 BCE–65 CE) describes in <i>On Anger</i> (3.36) how, when the lamp was out, he examined his whole day: What bad habit have I cured today? What fault have I resisted? In what way am I better? He learned it from the philosopher Sextius, and it goes back to the <b>Golden Verses</b> of the Pythagoreans: do not let sleep fall on your eyes until you have gone over each act of the day three times.</p>",
      stoic, harp(),
      vec![st("Settle", 2.0, &[8.0], "Welcome to the evening review of Seneca and the Pythagoreans. Lie down with the lights out, and let the day come back to you."),
           st("Three questions", 6.0, &[6.0], "Ask yourself Seneca's three questions. What did I do badly? What did I do well? What did I leave undone? Answer honestly, as a kind but fair judge."),
           st("Peace", 2.0, &[4.5], "Forgive yourself for the faults, and resolve to do better tomorrow. Seneca said: how peaceful is the sleep that follows such a review."),
           st("Sleep", 2.0, &[3.0], "Now let the day go, and drift toward sleep.")]));
    v.push(n(g, "we_premeditatio", "Premeditatio Malorum — Rehearsing Hardship Calmly (Stoic)", "Calm in advance · 14 min", 0.45, "pink",
      "<p><b>Premeditatio malorum</b> (\"rehearsal of misfortunes\") is a Stoic exercise described by <b>Seneca</b> and <b>Epictetus</b>: imagining calmly, in advance, losses and setbacks that could happen, so that if they come they find you prepared, and while they don't, you value what you have. Modern cognitive therapy uses related ideas. The Stoic version ends by recalling what is always in your control: your judgments and your choices. Start with small setbacks, not with your worst fears.</p>",
      stoic, harp(),
      vec![st("Settle", 2.0, &[10.0], "Welcome to premeditatio malorum, the Stoic rehearsal of hardship. Sit upright and breathe slowly. Choose one realistic setback for today, small rather than catastrophic."),
           st("Rehearsal", 5.0, &[8.0], "Picture it happening: the delay, the criticism, the loss. Notice your first reaction, and let it pass. This, too, is part of life."),
           st("Response", 4.0, &[7.5], "Now see yourself meeting it well: calm, fair and steady. Epictetus said: it is not things that disturb us, but our judgments about them."),
           st("Gratitude", 2.0, &[8.0], "Come back to the present. None of it has happened. Feel grateful for what you have right now, while you have it."),
           st("Close", 1.0, &[10.0], C_RETURN_SHORT)]));
    v.push(n(g, "we_gurdjieff", "Self-Remembering — I Am (the Fourth Way)", "Gurdjieff & Ouspensky · 16 min", 0.45, "pink",
      "<p><b>G. I. Gurdjieff</b> (about 1866–1949) taught what his pupil <b>P. D. Ouspensky</b> called the <b>Fourth Way</b>: inner work done in the middle of ordinary life rather than in a monastery. Its central practice is <b>self-remembering</b>: being aware of yourself at the same moment as you are aware of what you are doing, which, Gurdjieff said, we almost never are. A common exercise is to sense the body part by part, then the whole, while silently saying \"I am\". Gurdjieff himself played a small harmonium for his pupils.</p>",
      &["western", "fourthway"], mx("drone", "harmonium", 0.35, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to self-remembering, the central practice of the Fourth Way. Sit upright, and notice that you are here, now: not lost in thought, but present."),
           st("Sensing", 6.0, &[8.0], "Sense your right arm from inside, its weight and warmth, then your right leg, your left leg, your left arm. Then the whole body at once."),
           st("I am", 5.0, &[7.0], "Keeping the sensation of the whole body, say silently: I, on the in-breath, and am, on the out-breath. Be aware of yourself being aware."),
           st("Divided attention", 2.0, &[9.0], "Open your eyes, and look at an object while still sensing your body. Attention points two ways: out to the world, and back to yourself."),
           st("Close", 1.0, &[10.0], "Carry this into the day: whenever you remember, remember yourself. Sense your body, and say: I am.")]));
    v.push(n(g, "we_resh", "Solar Adorations — Greeting the Sun at Four Stations (Thelema)", "After Liber Resh · 12 min", 0.45, "pink",
      "<p>In <b>Liber Resh vel Helios</b> (1911), the occultist <b>Aleister Crowley</b> set out four daily adorations of the sun for followers of <b>Thelema</b>: at dawn, noon, sunset and midnight, facing the sun's direction and greeting it under four Egyptian god-forms, Ra, Hathoor, Tum and Khephra. It is meant to keep the practitioner mindful, all through the day, of the sun's constant light. This session gives the outline in its own words, as a meditation on light at each station.</p>",
      &["western", "hermetic"], mx("harp", "", 0.45, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the solar adorations of Liber Resh. Stand or sit facing the direction of the sun at this hour: east at dawn, south at noon, west at sunset, north at midnight."),
           st("Dawn and noon", 4.0, &[8.0], "At dawn the sun rises as Ra, a falcon in the boat of morning. At noon it is Hathoor, at the height of its strength. Feel its light on your face, whatever the hour."),
           st("Sunset and midnight", 4.0, &[7.0], "At sunset it is Tum, descending in peace. At midnight it is Khephra, the scarab, hidden under the earth, yet shining still."),
           st("The hidden sun", 2.0, &[6.5], "The sun never truly sets; only our side of the earth turns away. Rest in that constant light within you.")]));
    v.push(n(g, "we_awen", "Awen — The Druid Chant of Inspiration", "Three drawn-out syllables · 14 min", 0.35, "none",
      "<p><b>Awen</b> is a Welsh word for poetic inspiration, the gift of the bards. Modern Druid orders such as the <b>Order of Bards, Ovates and Druids</b> (founded 1964) open and close their ceremonies by chanting it together, three times, as one long, drawn-out sound, <i>Aaa-ooo-ennn</i>, under the symbol of three rays of light, /|\\. It is sung to call inspiration and unity.</p>",
      &["western", "celtic"], mx("chant", "awen|wind*0.4", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the awen, the Druid chant of inspiration. Stand or sit outdoors if you can, or picture an oak grove around you. Breathe deeply.").mus(0.3),
           st("Chant", 6.0, &[8.0], "Chant with the grove, one long breath for each awen: aaah, oooh, ennn. Three times, then rest, then again. Let the sound rise from the ground through you.").mus(1.0),
           st("Three rays", 4.0, &[6.5], "In the silence, picture three rays of light descending: one for love, one for wisdom, one for truth. Let inspiration pour in.").mus(0.3),
           st("Close", 2.0, &[9.0], "Thank the land, the sky and the ancestors. Then slowly open your eyes.").mus(0.0)]));
    v.push(n(g, "we_pathwork", "Pathworking — The 32nd Path, from Malkuth to Yesod", "A Tree of Life journey · 22 min", 0.45, "pink",
      "<p><b>Pathworking</b> is a guided-imagination practice of the Hermetic Qabalah: travelling in vision along the 22 paths that join the ten <i>sefirot</i> of the Tree of Life. Golden Dawn members used tarot cards as doorways; later writers such as <b>Dion Fortune</b> and <b>W. E. Butler</b> developed it into a method. The path walked first is usually the 32nd, given to the Tarot's World card and the Hebrew letter Tau, from <b>Malkuth</b> (the kingdom, the physical world) to <b>Yesod</b> (the foundation, the realm of the moon and dreams).</p>",
      &["western", "hermetic"], mx("drone", "organ", 0.4, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to pathworking on the Tree of Life. Sit upright and close your eyes. You will walk the thirty-second path, from the kingdom of earth to the foundation of the moon."),
           st("Malkuth", 4.0, &[8.0], "You stand in a temple with a floor of black and white squares. Its walls glow with the colours of earth: citrine, olive, russet and black. Feel solid, grounded and safe."),
           st("The door", 4.0, &[7.0], "Before you hangs a curtain marked with the letter tau, a simple cross. On it is the image of the World card: a dancer within a wreath. Part the curtain, and step through."),
           st("The path", 6.0, &[6.0], "A violet path rises through the night under a silver crescent moon. Walk it slowly. Notice what you see, hear and meet along the way, without forcing anything."),
           st("Yesod", 4.0, &[5.5], "You arrive in a violet temple full of soft moonlight, the realm of dreams and images. Rest here, and receive whatever comes."),
           st("Return", 2.0, &[10.0], "Now walk back down the path, through the curtain, into the earth temple, and into your body. Stamp your feet lightly, and open your eyes.")]));
}

fn other(v: &mut Vec<Preset>) {
    let g = "Shamanic & Ancestral Rituals";
    v.push(n(g, "sr_ancestors", "Honouring the Ancestors", "Remembrance and gratitude · 18 min", 0.4, "pink",
      "<p>Honouring the ancestors is among the most widespread religious practices on earth: offerings at family altars in China, Japan, Korea and Vietnam, the Day of the Dead in Mexico, libations in West African traditions, Samhain in Celtic lands. The common thread is remembering those who came before, with gratitude, and asking for their blessing. This is a simple, general remembrance you can adapt to your own family and tradition. If grief is raw, go gently, or choose another session.</p>",
      &["shamanic"], mx("fire", "", 0.6, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to honouring the ancestors. Light a candle if you like, perhaps beside a photograph. Sit comfortably, and close your eyes."),
           st("Those you knew", 5.0, &[7.0], "Bring to mind relatives or elders you knew who have died. See their faces. Thank them for one thing each: a gift, a lesson, a moment of love."),
           st("The long line", 5.0, &[6.0], "Now feel the long line behind them: parents behind parents, hundreds of generations, each one surviving and passing life on, so that you could be here."),
           st("Blessing", 4.0, &[6.0], "Imagine them standing behind you, wishing you well. Ask for their blessing on your life, and offer your own: to live in a way that honours them."),
           st("Return", 2.0, &[10.0], C_RETURN_SHORT)]));
    v.push(with_tex(n(g, "sr_fire", "Fire Ceremony — Releasing into the Flame", "Letting go · 18 min", 0.4, "pink",
      "<p>Fire is at the centre of ritual in many cultures: the Vedic <i>agni</i>, the Zoroastrian sacred fire, the bonfires of Beltane and Midsummer, the hearths of countless homes. A widespread practice is naming what you want to release, or writing it on paper, and offering it to the flame. Sit by a real fire, a candle in a safe holder, or picture one. Never leave a flame unattended.</p>",
      &["shamanic"], mx("fire", "", 0.9, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the fire ceremony. Sit before a fire or a candle, or picture one. Keep a slip of paper nearby if you want to write. Watch the flame, and breathe.").tex(0.0),
           st("Naming", 4.0, &[8.0], "Bring to mind something you are ready to let go of: a habit, a resentment, a fear. Name it clearly, aloud or on the paper."),
           st("Offering", 6.0, &[6.0], "Offer it to the flame, in your mind or for real, safely. Watch it turn to light and smoke and rise away. Feel the space it leaves."),
           st("Calling in", 4.0, &[6.5], "Now name what you wish to grow in that space: courage, peace, love. Breathe it in with the warmth of the fire."),
           st("Close", 2.0, &[9.0], "Thank the fire. When you are ready, put out any real flame completely, and open your eyes.").tex(0.0)]),
      tx("rattle", 0.3, 0.0, 0.5)));

    let g = "Zoroastrian, Bahá'í & Shinto";
    v.push(n(g, "zo_ashem", "Ashem Vohu — Facing the Light (Zoroastrian)", "Prayer of truth · 14 min", 0.35, "none",
      "<p><b>Zoroastrianism</b>, founded by the prophet <b>Zarathustra</b> in ancient Iran (dates are debated; perhaps 1500–1000 BCE), is among the oldest living religions. Its followers pray five times a day facing a source of light, the sun or a sacred fire, as a symbol of God, <b>Ahura Mazda</b>. The short Avestan prayer <b>Ashem Vohu</b> praises <i>asha</i>, truth and right order: truth is the best good, and happiness comes to the one who is truthful for the sake of the best truth. Its ethic is <i>humata, hukhta, hvarshta</i>: good thoughts, good words, good deeds.</p>",
      &["zoroastrian"], mx("chant", "ashem|fire*0.5", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the Ashem Vohu, the Zoroastrian prayer of truth. Face the sun, a lamp or a candle, as a sign of the divine light. Stand or sit upright.").mus(0.3),
           st("The prayer", 6.0, &[7.5], "Listen to the priest murmuring the prayer at the fire, or join in: ashem vohu vahishtem asti. Truth is the best good.").mus(1.0),
           st("Good thoughts", 4.0, &[6.5], "Reflect on the threefold path: humata, good thoughts; hukhta, good words; hvarshta, good deeds. Choose one of each for today.").mus(0.4),
           st("Close", 2.0, &[9.0], "May truth and light grow in the world through you. Slowly open your eyes.").mus(0.0)]));
    v.push(n(g, "ba_abha", "Allāh-u-Abhá — Ninety-Five Repetitions (Bahá'í)", "God is Most Glorious · 14 min", 0.35, "none",
      "<p>The <b>Bahá'í Faith</b>, founded by <b>Bahá'u'lláh</b> (1817–1892) in 19th-century Persia, teaches the oneness of God, of religion and of humanity. In the <i>Kitáb-i-Aqdas</i> Bahá'u'lláh asked his followers to repeat the Greatest Name, <b>Alláh-u-Abhá</b> (\"God is Most Glorious\"), <b>95 times each day</b>, after washing the hands and face and sitting facing the Qiblih. Bahá'ís often count on a string of 95 beads, or on their fingers.</p>",
      &["abrahamic", "bahai"], mx("chant", "abha", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to the repetition of the Greatest Name. Wash your hands and face if you can, then sit facing the Qiblih, or simply sit quietly. Hold a string of beads, or count on your fingers.").mus(0.2),
           st("Repetition", 9.0, &[7.5], "Repeat with the gathering: allah-u-abha, God is most glorious, ninety-five times. Let the name lift your heart toward the source of all glory.").mus(1.0),
           st("Oneness", 2.0, &[6.0], "Sit in silence. Bahaullah wrote: the earth is but one country, and mankind its citizens. Feel your oneness with all people.").mus(0.2),
           st("Close", 1.0, &[10.0], C_STILL).mus(0.0)]));
    v.push(n(g, "sh_harae", "Harae — Shinto Purification of the Heart", "Harae tamae, kiyome tamae · 14 min", 0.35, "none",
      "<p><b>Harae</b> (or <i>harai</i>) is purification, the foundation of <b>Shinto</b>, the native religion of Japan: removing <i>kegare</i> (impurity) so that the heart's original brightness (<i>akaki kokoro</i>) can shine. At shrines a priest waves a wand of paper streamers (<i>ōnusa</i>) over worshippers; at the entrance people rinse their hands and mouth at a stone basin. A short purification prayer used by many is <b>Harae tamae, kiyome tamae</b>: please purify, please cleanse.</p>",
      &["shinto"], mx("chant", "harae|stream*0.6", 1.0, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to harae, Shinto purification. Rinse your hands with water if you can. Then sit in seiza, kneeling, or on a chair, with your back straight.").mus(0.3),
           st("Purification words", 5.0, &[7.5], "Chant with the priests: harae tamae, kiyome tamae. Please purify, please cleanse. Feel each word like clear water poured over you.").mus(1.0),
           st("The bright heart", 5.0, &[6.0], "Picture a mountain stream washing through you, carrying away dust and heaviness, until your heart shines clear and bright, as it was in the beginning.").mus(0.5),
           st("Close", 2.0, &[9.0], "Bow twice, clap twice, and bow once more, as at a shrine. Then sit quietly for a moment, and open your eyes.").mus(0.2)]));
    v.push(n(g, "sh_chinkon", "Chinkon — Calming the Soul (Shinto)", "Stillness at the centre · 15 min", 0.4, "pink",
      "<p><b>Chinkon</b> (\"calming the soul\") is an ancient Shinto rite for settling the spirit firmly in the body, once performed for the emperor at the winter solstice (<i>chinkonsai</i>). Some modern Shinto lineages teach it as meditation: kneeling in <i>seiza</i>, the hands held together before the lower belly, breathing quietly and gathering the spirit at the body's centre. Joined to <i>kishin</i> (\"returning to the kami\") it is used for deep stillness.</p>",
      &["shinto"], mx("stream", "|wind*0.5", 0.5, 0.0, 0.0),
      vec![st("Settle", 2.0, &[10.0], "Welcome to chinkon, the Shinto calming of the soul. Kneel in seiza or sit upright. Rest your hands together in front of your lower belly, right hand over left."),
           st("Gathering", 5.0, &[7.0], "Breathe slowly through the nose, deep into the belly. With each out-breath, feel your spirit settle downward, gathering at your centre, below the navel."),
           st("Stillness", 6.0, &[5.5], "Rest there, still as a lake on a windless day. Sounds of water and wind pass by; you remain settled at the centre."),
           st("The kami", 2.0, &[6.0], "Feel your stillness joined to the stillness of nature all around you: the mountains, the forests, the sea. Then breathe deeply, and open your eyes.")]));
}
