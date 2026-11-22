//! “How to practise”: a beginner's guide for every session — how to prepare, the technique,
//! every stage step by step, what you may notice, what to do when it's hard, safety, how to
//! finish, and what you will hear. A child of info.rs; the text is trusted HTML written here.
use crate::plan::{Guide, Preset, Texture};

#[path = "guide_tips.rs"]
mod tips;

#[derive(Clone, Copy, PartialEq, Eq)]
enum F {
    Gateway, Lifeline, Listen, Astral, Lucid, Hypno, Mystic, Christian, Buddhist, Zen, Tibetan, Hindu,
    Kundalini, Jain, Sikh, Sufi, Jewish, Taoist, Shamanic, Esoteric, Rosicrucian, Masonic, Stoic,
    Shinto, Zoroastrian, Bahai, Flicker, Infra, BreathOpt, Breath,
}

struct T { prep: &'static str, tech: &'static str, notice: &'static str, trouble: &'static str, after: &'static str, safety: &'static str }

pub(super) fn make(p: &Preset) -> Guide {
    let f = fam(p);
    let t = text(f, main_beat(p));
    let mut prep = String::from("<ul>");
    prep.push_str(t.prep);
    prep.push_str(gear(p));
    if let Some(x) = visual_prep(p) { prep.push_str(x); }
    prep.push_str("<li>Put your phone on Do Not Disturb, and read the steps below once before you begin.</li></ul>");
    let mut technique = String::new();
    if let Some(x) = tips::get(&p.id) { technique.push_str(x); }
    if let Some(x) = listen_extra(p) { technique.push_str(x); }
    technique.push_str(t.tech);
    if p.music.as_ref().map_or(false, |m| m.kind == "chant") {
        technique.push_str("<p><b>Chanting with the group:</b> the recorded voices chant with you. Join in at a comfortable pitch, or simply listen; your voice doesn't need to be loud or perfect. When the voices fall silent, carry the chant on inwardly.</p>");
    }
    let mut safety = String::from(t.safety);
    if let Some(x) = special_safety(&p.id) { safety.push_str(x); }
    safety.push_str("<p>Never listen while driving, cycling or doing anything that needs your full attention. This app is not a medical treatment; if you have a health condition, follow your doctor's advice.</p>");
    Guide {
        prep, technique, steps: steps(p), notice: t.notice.into(), trouble: t.trouble.into(), safety, after: t.after.into(), sound: sound(p),
    }
}

// ---------------------------------------------------------------- which family

fn fam(p: &Preset) -> F {
    let id = p.id.as_str();
    let has = |k: &str| p.trad.iter().any(|x| x == k);
    if p.visual.contains("flicker") { return F::Flicker; }
    if p.visual.starts_with("infra") { return F::Infra; }
    match p.group.as_str() {
        "Breathwork (opt-in)" => return F::BreathOpt,
        "Breathing Techniques" => return F::Breath,
        "Inspired by the Gateway Experience" => return F::Gateway,
        "Inspired by Lifeline (Focus 22–27)" => return F::Lifeline,
        "Trance & Hypnosis Techniques" => return F::Hypno,
        "Rosicrucian & Alchemical" => return F::Rosicrucian,
        "Masonic Contemplation" => return F::Masonic,
        _ => {}
    }
    if p.visual.starts_with("breath:") { return F::Breath; }
    match id {
        "lucid" | "tlr" | "ap_ssild" | "ap_wbtb" | "ap_dreamyoga" | "we_incubation" | "tb_sleepyoga" => return F::Lucid,
        "hypno" | "ap_hypnotic" => return F::Hypno,
        "ap_rising" | "ap_tattva" | "we_plotinus" => return F::Esoteric,
        "ap_phowa" => return F::Tibetan,
        "ap_upper" | "ap_middle" | "shaman" | "drumtrance" | "sys_drum" => return F::Shamanic,
        "ap_ground" | "rv" | "nsdr" | "kn_gong" | "bowls" => return F::Listen,
        "kirtan" => return F::Kundalini,
        "bu_walking" => return F::Buddhist,
        "my_nondual" | "mantra6" | "my_psychomanteum" | "ganzfeld" => return F::Mystic,
        "we_viewabove" | "we_evening" | "we_premeditatio" => return F::Stoic,
        "zen" => return F::Zen,
        _ => {}
    }
    if p.group == "Kundalini Yoga (gentle)" { return F::Kundalini; }
    if p.group == "Astral Projection & OBE" || id == "obe" { return F::Astral; }
    if has("christian") { return F::Christian; }
    if has("jewish") { return F::Jewish; }
    if has("islamic") { return F::Sufi; }
    if has("bahai") { return F::Bahai; }
    if has("tibetan") { return F::Tibetan; }
    if has("zen") { return F::Zen; }
    if has("buddhist") { return F::Buddhist; }
    if has("jain") { return F::Jain; }
    if has("sikh") { return F::Sikh; }
    if has("hindu") { return F::Hindu; }
    if has("taoist") { return F::Taoist; }
    if has("shamanic") { return F::Shamanic; }
    if has("shinto") { return F::Shinto; }
    if has("zoroastrian") { return F::Zoroastrian; }
    if has("western") { return F::Esoteric; }
    if p.group == "Spiritual & Mystical Experiences" { return F::Mystic; }
    F::Listen
}

/// The beat of the longest stage (the session's "home" rhythm); 0 when there is no beat.
fn main_beat(p: &Preset) -> f64 {
    if !p.gains.iter().any(|g| *g > 0.0) { return 0.0; }
    let mut best = (0.0f64, 0.0f64);
    for s in &p.stages {
        if let Some(b) = s.beats.first() { if s.min > best.0 { best = (s.min, *b); } }
    }
    best.1
}

fn band(hz: f64) -> &'static str {
    if hz < 0.75 { "epsilon" } else if hz < 4.0 { "delta" } else if hz < 8.0 { "theta" } else if hz < 13.0 { "alpha" }
    else if hz < 30.0 { "beta" } else if hz < 90.0 { "gamma" } else if hz < 150.0 { "hypergamma" } else { "lambda" }
}

// ---------------------------------------------------------------- shared text by family

const L_NOTICE: &str = "<ul><li>A gradual shift in how alert or relaxed you feel. Effects are usually subtle, and they build with regular use.</li><li>The beat may sound like a slow wobble, a soft pulse or a gentle rotation inside your head.</li><li>Some people notice nothing in particular; sitting or lying quietly with a steady sound is still restful.</li></ul>";
const L_TROUBLE: &str = "<ul><li><b>The tone is irritating:</b> turn the <i>Tone level</i> down and the <i>Background noise</i> up, or try another noise colour.</li><li><b>Restless or fidgety:</b> breathe out more slowly than you breathe in, and let your shoulders drop.</li><li><b>Headache, ear discomfort or unease:</b> lower the volume, or stop.</li></ul>";
const L_AFTER: &str = "<p>Take a few deeper breaths, stretch, and notice how you feel compared with before you started. Get up slowly. If you used it for sleep, simply let the session end on its own.</p>";
const L_SAFETY: &str = "<p>Keep the volume moderate. If you have epilepsy or a heart condition, ask your doctor before using rhythmic sound.</p>";

const BUD_PREP: &str = "<li><b>Posture:</b> sit on a cushion with your legs crossed (both shins on the floor in the ‘Burmese’ position is easiest), kneel on a cushion or a bench, or sit on a chair with your feet flat. Keep your back upright but not stiff, your chin slightly tucked, your shoulders soft.</li><li><b>Hands:</b> rest them in your lap, right hand on left, thumbs lightly touching, or palms down on your thighs.</li><li><b>Eyes:</b> closed, or open and lowered to the floor about a metre in front of you.</li><li><b>When:</b> the same time every day if you can. Regular short sittings help more than occasional long ones.</li>";
const BUD_NOTICE: &str = "<ul><li>Restlessness and a wandering mind at first, then stretches of calm.</li><li>Pleasant tingling, lightness or waves of joy (<i>pīti</i>), or a quiet happiness (<i>sukha</i>).</li><li>Old memories or emotions sometimes surface. Let them pass through, like weather.</li></ul>";
const BUD_TROUBLE: &str = "<p>The Buddha named five hindrances, each with a remedy:</p><ul><li><b>Wanting</b> (daydreams, plans): note ‘wanting’ and return to the object.</li><li><b>Ill will</b> (irritation, impatience): soften, and spend a minute wishing yourself and others well.</li><li><b>Dullness</b> (drowsiness): open your eyes, sit taller, or practise standing or walking.</li><li><b>Restlessness</b> (agitation, worry): relax your body and lengthen the out-breath.</li><li><b>Doubt</b> (‘am I doing this right?’): if you keep coming back to the object, you are.</li></ul>";
const BUD_AFTER: &str = "<p>End by dedicating whatever good came from the practice to the happiness of all beings. Get up slowly and carry a little of the mindfulness into whatever you do next.</p>";
const BUD_SAFETY: &str = "<p>Long or intense meditation can occasionally bring up anxiety, low mood, strange perceptions or memories of trauma. If that happens, sit for shorter times with your eyes open, try walking meditation, and talk to a teacher or a therapist who knows meditation (Cheetah House, for example, helps people with meditation-related difficulties).</p>";

const MYS_SAFETY: &str = "<p>Deep states can sometimes stir up strong emotions or old memories. Stop whenever you feel overwhelmed: open your eyes and feel your feet on the floor. Avoid these sessions if you have a history of psychosis or dissociation.</p>";
const ESO_SAFETY: &str = "<p>If a practice leaves you fearful or confused, stop, and come back to simple breathing and grounding. Avoid intense ritual and pathworking if you are prone to psychosis, or during a mental-health crisis.</p>";

fn text(f: F, beat: f64) -> T {
    match f {
        F::Gateway | F::Lifeline => T {
            prep: "<li><b>Where:</b> lie on your back on a bed or a reclining chair in a dark, quiet room where you won't be disturbed. A pillow under your knees eases the lower back.</li><li><b>Before you start:</b> use the bathroom, loosen tight clothing, take off glasses, and avoid a heavy meal, alcohol or caffeine for an hour or two beforehand. An eye mask helps.</li><li><b>Have ready:</b> a notebook and pen to write down your impressions afterwards.</li><li><b>When:</b> choose a time when you are rested. If you tend to fall asleep, practise earlier in the day or sit semi-upright.</li>",
            tech: if f == F::Gateway {
                "<p>These sessions follow the preparation Robert Monroe taught, then deepen step by step:</p><ul><li><b>Energy Conversion Box:</b> imagine a strong box with a heavy lid. Put into it every worry, task and bodily discomfort, close the lid, and leave it there for the session.</li><li><b>Resonant Tuning:</b> breathe in deeply; as you breathe out, hum or sound a long ‘ahhh’ or ‘ommm’ along with the tones, and feel the vibration in your chest and head.</li><li><b>Energy Balloon:</b> imagine energy flowing up from your feet, out of the top of your head and down around you like a fountain, filling a protective balloon around your body.</li><li><b>Intention:</b> in your own words, tell yourself that you are more than your physical body, what you would like to explore, and that you will come back safely and calmly.</li><li><b>Focus levels:</b> Focus 10 is ‘mind awake, body asleep’: the body so relaxed you lose track of it while the mind stays clear. Focus 12 widens awareness beyond the body; Focus 15 is a still, timeless state; Focus 21 is the edge of other kinds of experience, in Monroe's model.</li></ul><p>Don't try to make anything happen. Let the sound guide you, and treat whatever comes as interesting.</p>"
            } else {
                "<p>Lifeline begins with the same preparation as Gateway (box, resonant tuning, energy balloon, intention) and then goes beyond Focus 21. In Monroe's teaching, Focus 22 to 27 are levels of non-physical existence; here they are offered as calm guided imagery. Let images come and simply observe them. You don't need to believe or disbelieve anything.</p>"
            },
            notice: "<ul><li>Heaviness, lightness, tingling or a pleasant buzzing in the body.</li><li>Losing track of where your body ends, or of how much time has passed.</li><li>Drifting images, colours, memories or voices, like the edge of a dream.</li><li>‘Clicking out’: a gap you can't remember. This is common and harmless.</li></ul>",
            trouble: "<ul><li><b>Falling asleep:</b> very common at first. Practise when you're rested, sit semi-upright, or keep one forearm raised on its elbow so it drops if you drift off.</li><li><b>Restless or itchy:</b> it's fine to move or scratch, then settle again. A longer resonant tuning helps.</li><li><b>Nothing seems to happen:</b> also common. Monroe's own course repeats each level many times, and the relaxation is worthwhile in itself.</li><li><b>Anxious or uneasy:</b> open your eyes, breathe slowly, and stop if you wish. You are always in control.</li></ul>",
            after: "<p>When the return begins, bring your awareness back to the body: feel its weight, wiggle your fingers and toes, take a deep breath, then open your eyes. Lie still for a minute before you sit up. Write down anything you noticed before it fades, and drink some water.</p>",
            safety: if f == F::Gateway {
                "<p>If you have epilepsy, a heart condition, or a history of psychosis or dissociation, ask your doctor first. Stop whenever you feel uneasy.</p>"
            } else {
                "<p>These sessions use imagery of death and the afterlife. Skip them if you are recently bereaved or the themes upset you. If you have a history of psychosis or dissociation, choose a different session.</p>"
            },
        },
        F::Listen => {
            let (prep, tech) = if beat > 0.0 && beat < 4.0 {
                ("<li><b>Where:</b> lie in bed or on a sofa, lights low, warm enough. It's fine to fall asleep; the session ends by itself.</li>",
                 "<p>Just listen and let go. You don't need to concentrate. If thoughts come, let them drift by like clouds and come back to the sound. Let each out-breath be a little longer and slower than the in-breath.</p>")
            } else if beat >= 4.0 && beat < 8.0 {
                ("<li><b>Where:</b> lie down or recline in a quiet, dim room, with your eyes closed or covered.</li>",
                 "<p>Close your eyes and listen. Let your body grow heavy. Images, memories or ideas may drift up; watch them without holding on. If you get lost in thought, come back to the sound.</p>")
            } else if beat >= 13.0 && beat < 30.0 {
                ("<li><b>Where:</b> sit at your desk with the task you want to focus on in front of you. Eyes open is fine.</li>",
                 "<p>Start your task once the tone has settled. Keep the volume so low that you almost forget it's there, and take a short break every 25 to 50 minutes.</p>")
            } else if beat >= 30.0 {
                ("<li><b>Where:</b> sit upright at a desk or on a cushion.</li>",
                 "<p>Either work on a task that needs focus, or meditate with your eyes open or closed, resting your attention lightly on the sound.</p>")
            } else {
                ("<li><b>Where:</b> sit or recline comfortably with your eyes closed, or play it softly in the background while you read or rest.</li>",
                 "<p>Rest your attention lightly on the sound and on your breathing. Let your face, jaw and shoulders soften. The aim is a calm, unhurried mind, not an empty one.</p>")
            };
            T { prep, tech, notice: L_NOTICE, trouble: L_TROUBLE, after: L_AFTER, safety: L_SAFETY }
        }
        F::Astral => T {
            prep: "<li><b>Where:</b> lie on your back on a bed, arms by your sides, in a dark, quiet room. Use an eye mask.</li><li><b>When:</b> these techniques work best when you are drowsy but awake: early in the morning after 4 to 6 hours of sleep (set an alarm, stay up for a few minutes, then lie down again), or during an afternoon nap.</li><li><b>Have ready:</b> a notebook to write down what happens.</li>",
            tech: "<p>Lie completely still and let your body fall asleep while you keep a thread of awareness. The voice and the sound give you something to hold on to; come back to them whenever you drift.</p>",
            notice: "<ul><li><b>Vibrations:</b> buzzing, humming or rushing, sometimes like electricity. They are a sign you are close to the edge of sleep; stay calm and let them spread.</li><li>Loud inner sounds (buzzing, roaring, voices), flashes of light, or vivid pictures.</li><li>Feelings of floating, sinking or rocking, or of a second body that can move.</li><li><b>Sleep paralysis:</b> being awake but unable to move, sometimes with a sense that someone is present. It is part of normal sleep and harmless, although it can be frightening.</li></ul>",
            trouble: "<ul><li><b>You keep falling asleep:</b> try in the early morning or during a nap, or keep one forearm upright on its elbow.</li><li><b>You're wide awake:</b> try later in the night, or after a short sleep.</li><li><b>Fear when the vibrations start:</b> remind yourself it is a normal state at the edge of sleep. Breathe, and let it pass, or stop.</li><li><b>Nothing happens:</b> most people need many attempts. Relaxation and dream recall improve along the way.</li></ul>",
            after: "<p>Move your fingers and toes, take a deep breath and sit up slowly. Write down everything you remember: sensations, sounds, images. If you feel spacey, use the After-Practice Grounding session.</p>",
            safety: "<p>To end sleep paralysis, concentrate on moving one finger or toe, or on slow breathing; it passes within a minute or two. Scientists explain out-of-body experiences as vivid states of the brain at the edge of sleep, so don't treat anything you experience as a guide to real-world risks. Skip these sessions if you have a history of psychosis, dissociation, PTSD or seizures, or if sleep paralysis distresses you.</p>",
        },
        F::Lucid => T {
            prep: "<li><b>Where:</b> in bed, lights off, at bedtime or during the night.</li><li><b>Best timing:</b> ‘wake back to bed’. Set an alarm for about 4½ to 6 hours after you fall asleep, stay up for 10 to 30 minutes, then go back to bed with this session. Dreams are longest and most vivid in the second half of the night.</li><li><b>Have ready:</b> a dream journal and pen by the bed. Writing down your dreams every morning is the best way to remember more of them.</li><li>Use comfortable earbuds or a sleep headband, and keep the volume very low.</li>",
            tech: "<p>As you drift off, hold a gentle intention: ‘Next time I'm dreaming, I'll notice that I'm dreaming.’ During the day, ask yourself a few times ‘Am I dreaming?’ and check (read some text twice, or try to push a finger through your palm). The habit carries over into dreams.</p>",
            notice: "<ul><li>Images and sounds as you fall asleep (hypnagogia).</li><li>Dreams that are more vivid or easier to remember.</li><li>False awakenings: dreaming that you have woken up. Do a reality check.</li><li>Lucidity: realising you are dreaming. Stay calm; rub your dream hands together or look at the ground to steady the dream.</li></ul>",
            trouble: "<ul><li><b>Can't get back to sleep:</b> stay up for less time, and don't use the alarm before work days.</li><li><b>You wake up from excitement when you become lucid:</b> keep calm and focus on touch in the dream.</li><li><b>No results yet:</b> keep the dream journal going for a few weeks. Lucidity often arrives suddenly.</li></ul>",
            after: "<p>When you wake, lie still and replay the dream before you move, then write it down, even fragments.</p>",
            safety: "<p>Don't trade away too much sleep: use the alarm method only a few nights a week. Avoid it if you have insomnia, epilepsy (losing sleep can trigger seizures), bipolar disorder (disrupted sleep can trigger episodes) or another condition where regular sleep matters.</p>",
        },
        F::Hypno => T {
            prep: "<li><b>Where:</b> sit in a comfortable chair with your head supported, or recline; feet flat, hands resting on your thighs.</li><li><b>Before you start:</b> choose one simple, positive suggestion in the present tense, for example ‘I feel calm and steady when I speak’ or ‘I fall asleep easily’.</li>",
            tech: "<p>Hypnosis is focused attention plus relaxation and suggestion. Go along with the guidance as you would with a good story; you don't have to force anything. When the suggestion comes, imagine it as vividly as you can, as if it were already true.</p>",
            notice: "<ul><li>Heaviness, warmth or tingling; fluttering eyelids.</li><li>Time feeling shorter or longer than it was.</li><li>A narrow, absorbed focus, as when you're lost in a book.</li><li>You stay aware throughout. Many people say ‘I don't think I was hypnotised’; that is normal.</li></ul>",
            trouble: "<ul><li><b>You keep analysing:</b> notice the thought, then return to the words and the sound.</li><li><b>Falling asleep:</b> sit more upright.</li><li><b>Feeling uneasy:</b> open your eyes; you can stop at any moment.</li></ul>",
            after: "<p>Count up from one to five, take a deep breath, open your eyes and stretch. Wait a minute before standing up, and a few minutes before driving.</p>",
            safety: "<p>You stay in control and can open your eyes at any time. Don't use self-hypnosis for serious medical or psychiatric problems without professional help. If you have a history of psychosis, dissociation or trauma, work with a qualified clinician.</p>",
        },
        F::Mystic => T {
            prep: "<li><b>Where:</b> a quiet, dim room where you won't be disturbed.</li><li><b>Posture:</b> sit upright on a chair or cushion, or lie down if the guidance says so.</li><li><b>Begin:</b> set an intention in your own words: what you would like to open to or understand.</li>",
            tech: "<p>Follow the guidance at each stage with a relaxed, open attitude. Let experiences come without grasping at them or pushing them away.</p>",
            notice: "<ul><li>Deep calm, absorption, a sense of unity or timelessness, insights, emotion, or simply rest.</li></ul>",
            trouble: "<ul><li><b>Too intense:</b> open your eyes and feel your feet on the floor.</li><li><b>Nothing happens:</b> that's fine. These states can't be forced; the practice itself is the point.</li></ul>",
            after: "<p>Sit quietly for a few minutes before you get up. Writing a few lines about what happened helps you take it into everyday life.</p>",
            safety: MYS_SAFETY,
        },
        F::Christian => T {
            prep: "<li><b>Where and when:</b> a quiet corner and a regular time; early morning or evening is traditional.</li><li><b>Posture:</b> sit on a chair with your back upright and feet flat, hands resting open on your lap. Kneeling, or sitting on a prayer stool, are traditional alternatives.</li><li><b>Optional:</b> light a candle, place a cross or an icon in front of you, have a Bible or prayer beads at hand.</li><li><b>Begin:</b> pause, remember that you are in God's presence, and, if it is your custom, make the sign of the cross or say a short prayer.</li>",
            tech: "<p>Let the words be few and the attention gentle. Christian contemplatives speak of consenting to God's presence rather than achieving a state.</p>",
            notice: "<ul><li>Stillness, a sense of being held, warmth in the chest, tears, or deep peace.</li><li>Just as often: distractions, dryness, nothing special. The tradition values faithfulness, not feelings.</li></ul>",
            trouble: "<ul><li><b>Distracting thoughts:</b> Thomas Keating's advice was to resist none, hold on to none, react to none, and return ever so gently to the prayer.</li><li><b>Sleepiness:</b> sit more upright, or pray at a different time of day.</li><li><b>Feeling nothing:</b> St John of the Cross wrote of dryness as part of the path. Keep your time of prayer faithfully.</li></ul>",
            after: "<p>Close with the Lord's Prayer or a few words of thanks, and rest for a minute before getting up. Take one word or phrase with you into the day.</p>",
            safety: "<p>If prayer stirs up painful memories, scrupulous guilt or anxiety, talk with a priest, pastor or spiritual director. These practices don't replace medical or psychological care.</p>",
        },
        F::Buddhist => T {
            prep: BUD_PREP,
            tech: "<p>Rest your attention on the object the guidance gives you. When the mind wanders, and it will again and again, notice that it has wandered and come back, kindly. That moment of noticing and returning is the practice.</p>",
            notice: BUD_NOTICE, trouble: BUD_TROUBLE, after: BUD_AFTER, safety: BUD_SAFETY,
        },
        F::Zen => T {
            prep: "<li><b>Posture (zazen):</b> sit on the front third of a firm cushion in the Burmese, half-lotus or kneeling (seiza) position, or on a chair. Knees down, pelvis tilted slightly forward, spine upright, head as if lifted by a thread, chin slightly in.</li><li><b>Hands:</b> the cosmic mudrā: left hand resting in the right, palms up, thumb tips lightly touching to make an oval just below the navel.</li><li><b>Eyes:</b> half open, gaze resting on the floor about a metre ahead. Sōtō students often face a wall.</li><li><b>Breath:</b> natural, through the nose, settling low in the belly (<i>hara</i>).</li>",
            tech: "<p>Zen practice is simple and demanding: sit still, keep the posture alive, and come back wholeheartedly to the practice each time you notice you have drifted.</p>",
            notice: BUD_NOTICE,
            trouble: "<ul><li><b>Pain in the knees or back:</b> adjust the posture, use a higher cushion or a chair. Pain from strain is not a virtue.</li><li><b>Sleepiness:</b> open your eyes a little wider and straighten your spine.</li><li><b>Thinking:</b> don't fight thoughts. Let them come and go, and return to the practice.</li></ul>",
            after: "<p>At the end, bow with your palms together, sway gently from side to side before standing, and move slowly. Many Zen students close with a dedication of merit.</p>",
            safety: BUD_SAFETY,
        },
        F::Tibetan => T {
            prep: "<li><b>Posture:</b> sit cross-legged on a cushion or on a chair, back straight, shoulders relaxed, chin slightly tucked, hands resting in your lap, eyes gently lowered or closed.</li><li><b>Optional:</b> a mālā of 108 beads to count mantras, and a picture of the deity.</li><li><b>Begin in the traditional way:</b> take refuge in the Buddha, the Dharma and the Sangha, and set the intention to practise for the benefit of all beings (<i>bodhicitta</i>). End with a dedication.</li>",
            tech: "<p><b>Visualisation</b> doesn't need to be vivid. Start with the felt sense that the figure is present, the way you know someone is in the room with you; add details lightly, and if the image wobbles, come back to the feeling. <b>Mantras</b> are recited at a natural pace and clearly, first aloud, then in a whisper. Tibetans pronounce Sanskrit their own way (Benza for Vajra, Peme for Padme); either is fine.</p>",
            notice: BUD_NOTICE, trouble: BUD_TROUBLE, after: BUD_AFTER,
            safety: "<p>Many of these practices are traditionally received from a qualified teacher, with an empowerment or a reading transmission. If you take them up seriously, look for a teacher.</p><p>Long or intense meditation can occasionally bring up anxiety, low mood or memories of trauma; if so, practise for shorter times with your eyes open and talk to a teacher or therapist.</p>",
        },
        F::Hindu => T {
            prep: "<li><b>Posture:</b> sit cross-legged on a folded blanket (<i>sukhāsana</i>), kneel sitting on your heels (<i>vajrāsana</i>), or sit on a chair, spine straight, head level.</li><li><b>Hands:</b> on your knees, palms up, thumb and index fingertip touching (<i>jñāna mudrā</i>), unless the guidance says otherwise.</li><li><b>Optional:</b> a lamp or candle, incense, a picture of your teacher or chosen deity, and a mālā of 108 beads for japa.</li><li><b>When:</b> traditionally at sunrise or sunset (<i>sandhyā</i>), on an empty stomach.</li>",
            tech: "<p><b>Using a mālā:</b> hold it in your right hand, draped over the middle (or ring) finger, and with your thumb draw one bead towards you for each repetition. Don't use the index finger, and don't cross the large guru (<i>meru</i>) bead: at the end of a round, turn the mālā around and go back.</p><p><b>Sanskrit sounds:</b> a as in ‘about’, ā long as in ‘father’, i as in ‘bit’, ī as in ‘machine’, u as in ‘put’, ū as in ‘rule’, e as in ‘they’, o as in ‘go’; ṃ is a nasal hum.</p>",
            notice: "<ul><li>Calm, warmth, tingling, lightness.</li><li>The mantra carrying on by itself, or gaps of silence between thoughts.</li><li>Emotions rising and passing.</li></ul>",
            trouble: "<ul><li><b>You lose count:</b> don't start again; just carry on.</li><li><b>A wandering mind:</b> come back to the mantra or the object without criticising yourself. Even Arjuna, in the Bhagavad Gītā (6.34), complains that the mind is as hard to hold as the wind.</li><li><b>Stiff legs:</b> use a higher cushion or a chair.</li></ul>",
            after: "<p>Sit quietly for a minute. Rub your palms together and rest them over your eyes before opening them. A traditional close is ‘Om śānti, śānti, śāntiḥ’: peace, peace, peace.</p>",
            safety: "<p>Keep the breathing soft and natural. If you feel dizzy, anxious or overwhelmed, breathe normally, open your eyes and ground yourself: feel your feet, have something to eat.</p>",
        },
        F::Kundalini => T {
            prep: "<li><b>Posture:</b> sit cross-legged on a mat or folded blanket, or on a chair with your feet flat. Keep the spine straight with a light neck lock: lengthen the back of the neck and draw the chin slightly in.</li><li><b>Eyes:</b> closed, gently focused at the brow point between the eyebrows, as if looking up and in, without strain.</li><li><b>Hands:</b> <i>gyan mudrā</i> (index fingertip to thumb tip), unless the guidance says otherwise.</li><li><b>Traditional extras:</b> loose cotton clothes and a head covering; practise on an empty stomach, ideally before sunrise.</li>",
            tech: "<p>Kundalini Yoga sessions open by ‘tuning in’ with the Adi Mantra, <i>Ong Namo Guru Dev Namo</i>, three times, and close with a long <i>Sat Nam</i>. Chant from the navel with a steady, comfortable voice. The sessions in this app are deliberately gentle: natural breathing, no breath of fire, no breath holds, no strain.</p>",
            notice: "<ul><li>Warmth, tingling or a sense of energy along the spine or in the hands.</li><li>Emotional release, lightness, deep calm.</li><li>The mantra seeming to vibrate through the body.</li></ul>",
            trouble: "<ul><li><b>Arms or back get tired:</b> rest them for a moment, then carry on. A chair is fine.</li><li><b>Spacey or light-headed:</b> breathe normally, press your feet into the floor and open your eyes.</li><li><b>Pronunciation:</b> don't worry about perfection; the rhythm matters most.</li></ul>",
            after: "<p>Close with a long ‘Sat Nam’ (a long Saaat, a short Nam). Lie on your back for a few minutes, then drink some water. Eat something light if you feel spacey.</p>",
            safety: "<p>Stop if you feel dizzy, overheated, pressure in the head, panic or strong agitation; walk, eat and rest. Intense practice has been linked with distressing ‘kundalini’ experiences in a small number of people. If you have a history of psychosis, bipolar disorder, epilepsy, heart disease or high blood pressure, or are pregnant, ask a doctor and a qualified teacher first.</p>",
        },
        F::Jain => T {
            prep: "<li><b>Posture:</b> sit cross-legged on a woollen mat or a clean cloth, or on a chair; for <i>kāyotsarga</i> you can also stand with your feet slightly apart and your arms hanging loosely.</li><li><b>Place:</b> clean and quiet; many Jains face north or east.</li><li><b>Traditional extras:</b> plain white clothes, and a rosary of 108 beads for counting the Navkār.</li>",
            tech: "<p>Jain meditation aims at equanimity (<i>sāmāyika</i>): meeting every experience with neither attraction nor aversion, and with non-harm (<i>ahiṃsā</i>) towards all living beings, even in thought.</p>",
            notice: "<ul><li>Steadiness, lightness, a cool, clear calm.</li><li>Seeing reactions (liking, disliking) arise and fade.</li></ul>",
            trouble: "<ul><li><b>Restlessness:</b> return to the breath and the body as they are.</li><li><b>Discomfort in stillness:</b> move mindfully, then settle again.</li></ul>",
            after: "<p>Close by asking and offering forgiveness, ‘Micchāmi dukkaḍaṃ’ (may my faults be forgiven), and wish well-being to every living being.</p>",
            safety: "<p>If standing still makes you dizzy, sit or lie down. Keep the breath natural.</p>",
        },
        F::Sikh => T {
            prep: "<li><b>Respect:</b> cover your head with a scarf or turban and take off your shoes, as Sikhs do when reciting gurbani.</li><li><b>Posture:</b> sit cross-legged on the floor or on a chair, back upright, hands resting in your lap or on your knees.</li><li><b>When:</b> many Sikhs do simran in <i>amrit vela</i>, the ‘ambrosial hours’ before dawn.</li><li><b>Optional:</b> a <i>simarnā</i> (prayer beads).</li>",
            tech: "<p><i>Naam simran</i> means remembering the Name. Let ‘Waheguru’ ride on the breath, ‘Vaahe’ as you breathe in and ‘Guru’ as you breathe out, first aloud with the kirtan, then silently. It is pronounced ‘vaa-hay-goo-roo’.</p>",
            notice: "<ul><li>Peace, warmth, gratitude, a sense of the Name continuing by itself.</li></ul>",
            trouble: "<ul><li><b>A wandering mind:</b> come back to the Name with the next breath.</li><li><b>Unfamiliar Gurmukhi:</b> listen first, then join in.</li></ul>",
            after: "<p>If it is your custom, close with ‘Waheguru ji ka Khalsa, Waheguru ji ki Fateh’ (the Khalsa belongs to Waheguru; victory belongs to Waheguru), and sit for a moment in gratitude.</p>",
            safety: "<p>No special precautions. Keep the breath natural.</p>",
        },
        F::Sufi => T {
            prep: "<li><b>Traditional preparation:</b> many Sufis make ablution (<i>wuḍūʾ</i>) first and sit facing the qibla, the direction of Mecca.</li><li><b>Posture:</b> kneel sitting on your heels, sit cross-legged, or sit on a chair, back upright.</li><li><b>Optional:</b> prayer beads (a <i>tasbīḥ</i> of 33 or 99 beads), dim light, a quiet room.</li>",
            tech: "<p><i>Dhikr</i> means remembrance. Let the Divine Name ride on the breath and settle into the heart; sincerity matters more than the number of repetitions. Arabic sounds: ā is long, as in ‘father’; the double ‘ll’ in Allāh is held; ḥ is a breathy h from the throat; Hū is a soft, breathed ‘hoo’.</p>",
            notice: "<ul><li>Warmth or softening in the chest, tears, longing, quiet joy.</li><li>A rhythmic absorption in which the repetition seems to carry itself.</li></ul>",
            trouble: "<ul><li><b>Counting distracts you:</b> use beads, or simply follow the rhythm.</li><li><b>The movements feel awkward:</b> make them smaller, or keep still and let the movement be inward.</li></ul>",
            after: "<p>Sit in silence for a while. Many close by reciting al-Fātiḥa or sending blessings on the Prophet (<i>ṣalawāt</i>), if it is their custom.</p>",
            safety: "<p>Keep any movement of the head or body small and gentle; stop if your neck hurts or you feel dizzy. If you are a Muslim and unsure about a practice, ask a teacher you trust.</p>",
        },
        F::Bahai => T {
            prep: "<li><b>Traditional preparation:</b> wash your hands and face, and if you can, sit facing the Qiblih (the Shrine of Bahá'u'lláh near Acre, in Israel).</li><li><b>Posture:</b> sit comfortably upright in a quiet place.</li><li><b>Optional:</b> a string of 95 beads (or 19) to count with.</li>",
            tech: "<p>Repeat the Greatest Name at an even, unhurried pace, with attention to its meaning: God is Most Glorious.</p>",
            notice: "<ul><li>Calm, devotion, a sense of joy or nearness.</li></ul>",
            trouble: "<ul><li><b>Losing count:</b> beads help; or simply continue to the end of the recording.</li></ul>",
            after: "<p>Sit in silence for a moment. Some read a short passage from the Bahá'í writings.</p>",
            safety: "<p>No special precautions.</p>",
        },
        F::Jewish => T {
            prep: "<li><b>Place:</b> a quiet room. Some people face Jerusalem, light a candle, or wear a tallit or kippah, as is their custom.</li><li><b>Posture:</b> sit comfortably upright. For hitbodedut any posture is fine, even walking outdoors.</li><li><b>Have ready:</b> the Hebrew text with a transliteration, if you'd like to follow along.</li><li><b>Note:</b> observant Jews don't use electronic devices on Shabbat and festivals, so use these sessions on other days.</li>",
            tech: "<p><b>Hebrew sounds:</b> ḥ and kh are a throaty sound like the ‘ch’ in Scottish ‘loch’; e as in ‘bed’; ai as in ‘aisle’. Jewish meditation joins thought, feeling and words: dwell on the idea until it moves the heart (<i>kavvanah</i>).</p>",
            notice: "<ul><li>A sense of closeness (<i>devekut</i>), joy, awe, tears, or simple quiet.</li></ul>",
            trouble: "<ul><li><b>Unfamiliar Hebrew:</b> say it slowly, or use the English. Intention matters more than accent.</li><li><b>Wandering thoughts:</b> come back to the words or the image.</li></ul>",
            after: "<p>Close with a moment of gratitude, the Shema, or a personal prayer. Some write a line in a notebook about what they learned.</p>",
            safety: "<p>Traditionally, mystical techniques were learned step by step with a teacher. If a practice leaves you unsettled, come back to simpler prayer and to grounding.</p>",
        },
        F::Taoist => T {
            prep: "<li><b>Seated:</b> sit on the front half of a chair, feet flat and hip-width apart, hands resting on your thighs or folded over the lower belly; or sit cross-legged on a cushion.</li><li><b>Standing practices:</b> feet parallel and shoulder-width apart, knees soft (never locked, never past the toes), tailbone dropping, the crown of the head lifted as if hung from a thread.</li><li><b>Tongue:</b> rest the tip lightly on the palate just behind the upper teeth, as Taoist practice teaches.</li><li><b>Breath:</b> through the nose, soft, slow and even.</li>",
            tech: "<p>The lower <i>dantian</i>, the ‘field of elixir’, lies about three finger-widths below the navel, inside the body. Taoist practice is effortless (<i>wu wei</i>): guide your attention lightly and let the breath and the <i>qi</i> follow. Never force.</p>",
            notice: "<ul><li>Warmth in the belly or hands, tingling, a feeling of fullness or flow.</li><li>More saliva (traditionally swallowed slowly, in three sips, down to the dantian).</li><li>Small, spontaneous movements.</li></ul>",
            trouble: "<ul><li><b>Heat or pressure in the head:</b> bring your attention down to the belly and the soles of your feet.</li><li><b>Tired legs when standing:</b> straighten up a little, rest, or sit.</li><li><b>The breath feels forced:</b> let go of control; natural breathing is always right.</li></ul>",
            after: "<p>Close by ‘gathering’: rest both palms over the dantian and feel the energy settle there for a few breaths. Rub your palms until they are warm and smooth them over your face, then walk a little before you go on.</p>",
            safety: "<p>Stop if you feel dizzy, pain, or pressure in the head. Never hold the breath, and never try to force energy up the spine.</p>",
        },
        F::Shamanic => T {
            prep: "<li><b>Where:</b> lie on your back on a mat or bed, or sit; cover your eyes with a scarf or mask and dim the lights.</li><li><b>Intention:</b> before you start, decide on one clear, simple aim, such as ‘to meet a helper’ or ‘to see what I need to know about…’, and say it silently.</li><li><b>Entrance:</b> for the lower world, picture an opening into the earth that you know in real life (a cave, a hollow tree, a spring, an animal's burrow); for the upper world, a tall tree, a mountain, a ladder or rising smoke.</li><li><b>Have ready:</b> a notebook to write down the journey.</li>",
            tech: "<p>The steady beat carries the imagery. Let what comes come, rather than steering it. When you hear the call-back (a change of rhythm and then a fast roll), thank whatever you met and come back the way you went, into your body.</p>",
            notice: "<ul><li>Vivid imagery: landscapes, animals, people; a feeling of moving or flying.</li><li>Or just fragments, feelings and colours. Michael Harner taught that all of it counts.</li></ul>",
            trouble: "<ul><li><b>Seeing nothing:</b> ‘feel’, ‘hear’ or simply ‘know’ your way; imagination is the doorway.</li><li><b>The drum distracts you:</b> turn the <i>Texture</i> level down a little.</li><li><b>Something frightening:</b> leave the way you came, open your eyes and ground yourself.</li></ul>",
            after: "<p>Sit up slowly, write the journey down, and eat or drink something. Walk around or touch the ground to come fully back.</p>",
            safety: "<p>These are adaptations of ‘core shamanism’ for personal practice, not indigenous ceremonies, which belong to their own peoples. Don't use them if you have a history of psychosis or dissociation, and stop if you feel overwhelmed.</p>",
        },
        F::Esoteric => T {
            prep: "<li><b>Place:</b> a clean, quiet space. For rituals, leave room to stand and turn; many practitioners begin facing east.</li><li><b>Read first:</b> go through the steps below once so you know the words, names and gestures.</li><li><b>Have ready:</b> a notebook as a practice diary: the date, the time, how you felt and what happened.</li>",
            tech: "<p><b>Vibrating a name:</b> breathe in deeply, imagine the name filling your chest, then intone it slowly on the out-breath, low and resonant, so that you feel it buzz in your chest and face. <b>Visualising:</b> a felt sense is enough; keep the images bright and simple.</p>",
            notice: "<ul><li>Calm focus, warmth, tingling, a sense of presence or of the room ‘clearing’.</li><li>Vivid inner images, or nothing much. Both are fine.</li></ul>",
            trouble: "<ul><li><b>Forgetting the words:</b> keep the text in front of you; precision comes with repetition.</li><li><b>Feeling self-conscious:</b> vibrate softly at first.</li></ul>",
            after: "<p>Ground yourself: stamp one foot lightly, eat or drink something, and write in your diary.</p>",
            safety: ESO_SAFETY,
        },
        F::Rosicrucian => T {
            prep: "<li><b>Place and time:</b> a quiet room and a regular time. Steiner and Heindel recommend the morning for concentration and the evening for review.</li><li><b>Posture:</b> sit upright and relaxed, or lie in bed for the evening retrospection.</li><li><b>Have ready:</b> a notebook.</li>",
            tech: "<p>Rudolf Steiner asked for calm, clear thinking without force: build each image deliberately, then let the feelings it awakens rise by themselves. The aim is inner development through patient, regular practice, not visions.</p>",
            notice: "<ul><li>Quiet focus, warmth, a sense of meaning, or images that keep unfolding.</li></ul>",
            trouble: "<ul><li><b>The image keeps slipping:</b> rebuild it calmly, piece by piece.</li><li><b>Falling asleep during the evening review:</b> that is all right. Simply begin again the next night.</li></ul>",
            after: "<p>Sit quietly for a minute, then write any insight in your notebook.</p>",
            safety: ESO_SAFETY,
        },
        F::Masonic => T {
            prep: "<li><b>Place:</b> a quiet room; sit upright.</li><li><b>Optional:</b> a picture of the symbol in front of you (they are printed in many public Masonic books), or simply imagine it.</li><li><b>Have ready:</b> a notebook for one practical resolution.</li>",
            tech: "<p>Masonic contemplation is moral reflection on public symbols: look at the symbol, think about what it teaches, apply it to your own life, and finish with one practical step. Freemasonry welcomes men (and, in some lodges, women) of any faith who believe in a Supreme Being; nothing here is secret lodge ritual.</p>",
            notice: "<ul><li>Clarity about a habit or a value; a calm resolve.</li></ul>",
            trouble: "<ul><li><b>It feels too abstract:</b> pick one real situation from this week and apply the symbol to it.</li></ul>",
            after: "<p>Write your resolution down and look at it again tomorrow.</p>",
            safety: "<p>No special precautions.</p>",
        },
        F::Stoic => T {
            prep: "<li><b>Posture:</b> sit upright with your feet flat, or lie in bed for the evening review.</li><li><b>Have ready:</b> a journal. The Stoics wrote their exercises down, as Marcus Aurelius did in his <i>Meditations</i>.</li>",
            tech: "<p>Stoic exercises train judgement: separate what is in your control (your choices, attitudes and actions) from what is not, and look at events calmly, from a wider view.</p>",
            notice: "<ul><li>Problems shrinking to their real size; calm resolve; gratitude.</li></ul>",
            trouble: "<ul><li><b>You get caught up in worry:</b> step back to the question ‘What is in my control here?’</li><li><b>Harsh self-judgement:</b> Seneca reviewed his day as a fair judge, not a prosecutor.</li></ul>",
            after: "<p>Write a sentence or two: what went well, what you'll do differently, and one action for tomorrow.</p>",
            safety: "<p>If imagining setbacks makes you more anxious rather than calmer, keep it brief and focus on your response, or choose another session.</p>",
        },
        F::Shinto => T {
            prep: "<li><b>Purify first:</b> if you can, rinse your hands and mouth with clean water (<i>temizu</i>), as at a shrine entrance.</li><li><b>Posture:</b> kneel in seiza, sit cross-legged, or sit on a chair, back upright, hands folded in front of the lower belly.</li><li><b>Place:</b> clean and quiet; a household shrine (<i>kamidana</i>) or a window facing nature is traditional.</li>",
            tech: "<p>Shinto practice values sincerity (<i>makoto</i>) and a bright, clear heart. At a shrine, worshippers bow twice, clap twice and bow once; you can do this at the start or the end if you wish.</p>",
            notice: "<ul><li>Freshness, clarity, a feeling of being cleansed.</li></ul>",
            trouble: "<ul><li><b>A busy mind:</b> come back to the image of clear water and the sound of the words.</li></ul>",
            after: "<p>Bow once and sit for a moment in gratitude.</p>",
            safety: "<p>No special precautions.</p>",
        },
        F::Zoroastrian => T {
            prep: "<li><b>Light:</b> face a source of light (the sun, a lamp or a candle); Zoroastrians pray facing light, a symbol of truth and goodness (<i>asha</i>).</li><li><b>Posture:</b> stand or sit upright. Many Zoroastrians first wash their face and hands (<i>padyab</i>) and retie the sacred cord (<i>kusti</i>) if they wear one.</li>",
            tech: "<p>The prayer is in Avestan, the language of the oldest Zoroastrian texts. Recite it slowly; the meaning matters more than perfect pronunciation.</p>",
            notice: "<ul><li>Clarity, uprightness, warmth.</li></ul>",
            trouble: "<ul><li><b>Unfamiliar words:</b> listen once, then join in.</li></ul>",
            after: "<p>Close by reflecting on good thoughts, good words and good deeds (<i>humata, hūkhta, hvarshta</i>) for the day ahead.</p>",
            safety: "<p>Use a candle only in a safe holder, and never leave it burning unattended.</p>",
        },
        F::Flicker => T {
            prep: "<li><b>First:</b> read and accept the warning screen; this session needs it.</li><li><b>Where:</b> sit or recline in a dim room, in a stable position, so that you can't fall if you feel strange.</li><li><b>Eyes:</b> closed, unless the session says eyes open. Face the screen, or hold the phone's flashlight about 10 to 30 cm from your closed eyes, as the app shows you.</li>",
            tech: "<p>Keep your eyes relaxed and closed, face the light, and simply watch what appears: colours, patterns, movement. Don't try to steer it. If it becomes too intense, move the light further away or turn your head.</p>",
            notice: "<ul><li>Bright colours, kaleidoscopes, spirals, grids and pulsing shapes; sometimes dreamlike scenes.</li><li>A sense of movement, time distortion, deep relaxation.</li></ul>",
            trouble: "<ul><li><b>Too bright:</b> move further from the light, or turn the brightness down.</li><li><b>Eye strain or headache:</b> stop and rest your eyes.</li><li><b>Seeing very little:</b> make the room darker and try a session around 10 Hz.</li></ul>",
            after: "<p>Keep your eyes closed for a minute after the light stops, then open them slowly. Sit for a few minutes before standing up, and don't drive straight afterwards.</p>",
            safety: "<p><b>Stop at once</b> if you feel unwell, dizzy, sick or confused, get a headache, see an aura or notice twitching. Never use flicker if you or a close relative have epilepsy or have had a seizure, if light triggers your migraines, if you have a heart or mental-health condition that intense experiences could worsen, or for children. The light stops as soon as you lock the screen or tap the black screen.</p>",
        },
        F::Infra => T {
            prep: "<li><b>First:</b> read and accept the warning screen.</li><li><b>Speakers:</b> infrasound is felt more than heard. Good speakers or a subwoofer play it best; phone speakers and most earbuds can't.</li><li>Keep the volume low to moderate.</li>",
            tech: "<p>Sit or lie still, and notice your body's response (pressure, vibration, mood) with curiosity rather than expectation.</p>",
            notice: "<ul><li>Faint pressure or vibration; sometimes unease, chills or a sense of presence. Expectation plays a large part in these feelings.</li></ul>",
            trouble: "<ul><li><b>You feel nothing:</b> your device may not be able to play such low tones.</li></ul>",
            after: "<p>Sit quietly for a minute and notice how you feel.</p>",
            safety: "<p>Stop if you feel pressure in your ears or chest, nausea, dizziness, anxiety or a headache. Don't use these sessions if you are pregnant or have a heart condition or an anxiety disorder.</p>",
        },
        F::BreathOpt => T {
            prep: "<li><b>First:</b> read and accept the warning screen.</li><li><b>Where:</b> sit or lie on a bed, a sofa or the floor. <b>Never</b> in or near water, in the bath, standing up, or while driving.</li><li><b>When:</b> on an empty stomach, and not just before sleep, as it is energising.</li><li>Follow the on-screen breathing guide.</li>",
            tech: "<p>Follow the pacer and the voice. Breathe fully but don't strain. If a round includes a hold, never push it past comfort: breathe whenever you need to.</p>",
            notice: "<ul><li>Tingling in the hands, feet and face; light-headedness; cramping or stiff ‘claw’ hands from low carbon dioxide.</li><li>Heat, cold, strong emotions, or euphoria.</li></ul>",
            trouble: "<ul><li><b>Tingling or cramping becomes too much:</b> slow down and breathe normally; it fades within a minute or two.</li><li><b>Anxiety:</b> stop, and breathe slowly with long out-breaths.</li></ul>",
            after: "<p>Breathe normally and rest for several minutes before standing, then get up slowly.</p>",
            safety: "<p>Don't practise if you are pregnant or have epilepsy, heart disease, high blood pressure, a history of stroke, glaucoma, panic disorder or serious mental illness. Stop if you feel faint. Never combine breathing exercises or breath holds with water: people have drowned this way.</p>",
        },
        F::Breath => T {
            prep: "<li><b>Posture:</b> sit upright on a chair or cushion, or lie on your back, shoulders relaxed.</li><li><b>Breathe</b> through the nose unless the guidance says otherwise, low into the belly. One hand on the belly and one on the chest helps you feel it.</li><li>Follow the on-screen breathing guide.</li>",
            tech: "<p>Match your breath to the pacer: smooth, quiet and unforced. If a count feels too long, make it shorter; comfort matters more than the numbers.</p>",
            notice: "<ul><li>Calm, warm hands, a slower heartbeat, yawning.</li></ul>",
            trouble: "<ul><li><b>Air hunger:</b> make the counts shorter.</li><li><b>Dizziness:</b> breathe normally for a while.</li></ul>",
            after: "<p>Let your breathing return to normal and notice how you feel.</p>",
            safety: "<p>If you feel light-headed, return to normal breathing. Skip any breath holds if you are pregnant or have heart or lung disease.</p>",
        },
    }
}

// ---------------------------------------------------------------- preparation extras

fn gear(p: &Preset) -> &'static str {
    let on = p.gains.iter().any(|g| *g > 0.0);
    match p.mode.as_str() {
        _ if !on => "<li><b>Sound:</b> headphones or speakers both work. Keep the volume low.</li>",
        "binaural" | "hybrid" => "<li><b>Headphones:</b> use stereo headphones or earbuds. Binaural beats only form when each ear hears its own tone, so speakers won't do. Keep the volume low, just loud enough to hear clearly; louder is not stronger.</li>",
        "bilateral" | "spatial" => "<li><b>Headphones:</b> use stereo headphones, so the sound can move between your ears. Keep the volume low.</li>",
        _ => "<li><b>Sound:</b> headphones or speakers both work for this beat type. Keep the volume low, just loud enough to hear clearly.</li>",
    }
}

fn visual_prep(p: &Preset) -> Option<&'static str> {
    let v = p.visual.as_str();
    if v == "ganzfeld" {
        Some("<li><b>Ganzfeld:</b> use the red-light button in this session, or tape halved ping-pong balls over your closed eyes and face a soft red lamp, and listen to the noise through headphones.</li>")
    } else if v.starts_with("kasina") || v.starts_with("trataka") || v == "tattva" {
        Some("<li><b>Gazing image:</b> set the screen at eye level, about an arm's length away, and turn the brightness down. Blink whenever you need to, and never strain your eyes.</li>")
    } else {
        None
    }
}

fn listen_extra(p: &Preset) -> Option<&'static str> {
    Some(match p.group.as_str() {
        "Chakra Series" => "<p><b>Where to rest your attention:</b> root, at the base of the spine; sacral, just below the navel; solar plexus, between the navel and the breastbone; heart, the centre of the chest; throat, at the throat; third eye, between the eyebrows; crown, at the top of the head. You can picture a soft light there, in the chakra's modern colour (red, orange, yellow, green, blue, indigo, violet, from root to crown).</p>",
        "Solfeggio Frequencies" => "<p>Listen with a calm, open mind. Some listeners pair each tone with an intention, such as 528 Hz with love or 396 Hz with letting go of fear; that is optional.</p>",
        "Cosmic Octave (Planets)" => "<p>If you like, picture the planet, or reflect on the qualities tradition gives it (Venus, love and beauty; Mars, courage; Saturn, patience and discipline). That is optional; simply listening is enough.</p>",
        "Schumann Resonances" => "<p>If you can, sit with your bare feet on the floor, and with each out-breath imagine roots reaching down into the earth.</p>",
        "Tunings" => "<p>Listen as you would to music, and compare how the different tunings feel to you.</p>",
        "Sound Systems Library" => "<p>These sessions use the same rhythm with different beat types, so you can compare them. Notice which feels most comfortable and absorbing; that is the one to use.</p>",
        _ => return None,
    })
}

fn special_safety(id: &str) -> Option<&'static str> {
    Some(match id {
        "bu_maranasati" | "tb_bardo" | "ap_phowa" | "sr_ancestors" | "my_psychomanteum" =>
            "<p>This session reflects on death or on people who have died. If you are grieving, or the theme makes you anxious or low, keep it short, practise with support, or choose another session.</p>",
        "my_nada" => "<p>If you have tinnitus and listening for inner sounds makes it more noticeable or distressing, choose a different session.</p>",
        "hy_trataka" | "cm_visio" => "<p>Stop if your eyes water strongly or hurt. If you use a real candle, keep it in a safe holder and never leave it unattended.</p>",
        "sr_fire" => "<p>Fire safety: use a candle in a holder or a fireproof bowl, away from curtains and anything flammable, with water nearby, and never leave a flame unattended. Doing the ceremony in imagination works just as well.</p>",
        "ta_baduanjin" | "ta_standing" | "jn_kayotsarga" | "sr_dance" | "bu_walking" =>
            "<p>Move only within a painless range. If you have back or joint problems, high blood pressure or poor balance, keep movements small, skip deep bends, and do the practice sitting if you need to.</p>",
        "tb_chod" => "<p>Choose a ‘demon’ (fear, habit, difficulty) that feels manageable at first. If you are working with trauma, do this practice with a therapist's support.</p>",
        "cm_vigil" | "we_bards" => "<p>Keep the room safe to move around in the dark, and don't let night practice cut into the sleep you need.</p>",
        _ => return None,
    })
}

// ---------------------------------------------------------------- step by step

fn steps(p: &Preset) -> String {
    let mut s = String::from("<ol>");
    for st in &p.stages {
        let mins = if st.min > 0.0 { format!("{} min", num(st.min)) } else { "open-ended".to_string() };
        let cue = if st.cue.trim().is_empty() {
            "No voice in this part. Rest in the silence and let the sound carry you.".to_string()
        } else {
            esc(&st.cue)
        };
        s.push_str(&format!("<li><b>{}</b> · {}<br>{}", esc(&st.name), mins, cue));
        if st.breath { s.push_str(" <i>Follow the on-screen breathing guide.</i>"); }
        s.push_str("</li>");
    }
    s.push_str("</ol>");
    s
}

fn num(x: f64) -> String {
    if (x - x.round()).abs() < 1e-9 { format!("{}", x.round() as i64) } else { format!("{:.1}", x) }
}

fn hz(x: f64) -> String {
    let r = (x * 100.0).round() / 100.0;
    if (r - r.round()).abs() < 1e-9 { format!("{}", r.round() as i64) } else { format!("{}", r) }
}

fn esc(s: &str) -> String { s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;") }

// ---------------------------------------------------------------- what you'll hear

fn sound(p: &Preset) -> String {
    let mut s = String::from("<ul>");
    let on = p.gains.iter().any(|g| *g > 0.0);
    if on {
        let mut lo = f64::INFINITY;
        let mut hi: f64 = 0.0;
        let mut layered = false;
        for st in &p.stages {
            if st.beats.len() > 1 { layered = true; }
            for b in &st.beats { if *b > 0.0 { lo = lo.min(*b); hi = hi.max(*b); } }
        }
        let what = match p.mode.as_str() {
            "binaural" => "<b>Binaural beats</b>: each ear hears a slightly different tone, and your brain hears the difference as a slow beat",
            "monaural" => "<b>Monaural beats</b>: two close tones mixed together, swelling and fading at the beat rate",
            "isochronic" => "<b>Isochronic pulses</b>: one tone switching smoothly on and off at the beat rate",
            "am" => "<b>Amplitude-modulated tones</b>: a tone that swells and fades smoothly at the beat rate",
            "hybrid" => "<b>Hybrid beats</b>: binaural beats that are also gently pulsed",
            "bilateral" => "<b>Bilateral tones</b>: a tone moving between your left and right ears",
            "spatial" => "<b>Spatial (8D) sound</b>: a tone circling slowly around your head",
            _ => "<b>Beats</b>",
        };
        let rate = if !lo.is_finite() {
            String::new()
        } else if (hi - lo).abs() < 0.01 {
            format!(" at {} Hz ({})", hz(lo), band(lo))
        } else {
            format!(", between {} Hz ({}) and {} Hz ({})", hz(lo), band(lo), hz(hi), band(hi))
        };
        let mut cars: Vec<String> = Vec::new();
        for c in &p.carriers { let t = hz(*c); if *c > 0.0 && !cars.contains(&t) { cars.push(t); } }
        let car = if cars.is_empty() { String::new() } else { format!(", on a {} Hz carrier tone", cars.join(" / ")) };
        s.push_str(&format!("<li>{}{}{}{}. The <i>Tone level</i> slider sets their volume, and <i>Beat type</i> lets you switch to another kind of beat.</li>",
            what, rate, if layered { ", in layers of several beats at once" } else { "" }, car));
    } else {
        s.push_str("<li><b>No beat:</b> this session has no beat tone.</li>");
    }
    let noise = match p.noise.as_str() { "pink" => "Pink", "brown" => "Brown", "white" => "White", "surf" => "Ocean-surf", _ => "" };
    if !noise.is_empty() {
        s.push_str(&format!("<li><b>{} noise</b> in the background, to mask other sounds (the <i>Background noise</i> slider; change the colour under <i>Noise type</i>).</li>", noise));
    }
    if let Some(t) = &p.texture {
        let k = match t.kind.as_str() {
            "drum" => "<b>Frame drum</b> keeping a steady trance rhythm",
            "bowls" => "<b>Singing bowls</b>, struck and left to ring",
            "kirtan" => "<b>A bell</b> on each syllable of the chant, to keep time",
            "tlr" => "<b>Soft cue tones</b> that remind you to notice whether you are dreaming",
            "breathbell" => "<b>A breathing bell</b> marking each breath",
            "pulses" => "<b>Soft pulses</b> of pink noise timed for deep sleep",
            "rain" => "<b>Rain</b>",
            "mokugyo" => "<b>Wooden fish</b> (<i>mokugyo</i>) keeping the chant's rhythm",
            "rattle" => "<b>Rattle</b> keeping a steady trance rhythm",
            "gong" => "<b>Gong</b>",
            "heartbeat" => "<b>A slow heartbeat</b>",
            "kartal" => "<b>Kartal</b> (small hand cymbals) keeping time",
            _ => "<b>A sound texture</b>",
        };
        s.push_str(&format!("<li>{} (the <i>Texture</i> slider).</li>", k));
    }
    if let Some(m) = &p.music {
        let varies = p.stages.iter().any(|x| x.mus.unwrap_or(1.0) <= 0.0) && p.stages.iter().any(|x| x.mus.unwrap_or(1.0) > 0.0);
        for line in music_lines(m) {
            s.push_str(&format!("<li>{} (the <i>Music &amp; chant</i> slider).</li>", line));
        }
        if varies { s.push_str("<li>The music follows the stages: it comes in where it helps and falls silent where the practice needs quiet.</li>"); }
    }
    if p.stages.iter().any(|x| !x.cue.trim().is_empty()) {
        s.push_str("<li><b>Voice guidance</b> speaks each step aloud (the <i>Voice level</i> slider; you can also switch the voice off and follow the steps above).</li>");
    }
    let v = p.visual.as_str();
    if v.contains("flicker") { s.push_str("<li><b>Light:</b> the screen or the phone's flashlight flickers in rhythm with the beat, once you have accepted the warning.</li>"); }
    if v == "ganzfeld" { s.push_str("<li><b>Red light:</b> an optional even red screen for the Ganzfeld.</li>"); }
    if v.starts_with("breath:") { s.push_str("<li><b>Breathing guide:</b> an on-screen pacer showing when to breathe in, hold and breathe out.</li>"); }
    if v.starts_with("kasina") || v.starts_with("trataka") || v == "tattva" { s.push_str("<li><b>Gazing image</b> on screen.</li>"); }
    if v.starts_with("infra") { s.push_str("<li><b>Infrasound:</b> a very low tone below the range of hearing, felt more than heard.</li>"); }
    s.push_str("</ul>");
    s
}

fn note_name(f: f64) -> Option<String> {
    if f <= 0.0 { return None; }
    const N: [&str; 12] = ["C", "C♯", "D", "E♭", "E", "F", "F♯", "G", "A♭", "A", "B♭", "B"];
    let x = 12.0 * (f / 440.0).log2() + 57.0;
    let n = x.round();
    if (x - n).abs() > 0.2 { return None; }
    let i = n as i64;
    Some(N[(i.rem_euclid(12)) as usize].to_string())
}

fn music_lines(m: &Texture) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (n, part) in m.name.split('|').enumerate() {
        let spec = part.split('*').next().unwrap_or("").trim();
        let (k, var) = if n == 0 { (m.kind.as_str(), spec) } else { spec.split_once(':').map(|(a, b)| (a.trim(), b.trim())).unwrap_or((spec, "")) };
        let key = |default: f64| note_name(if m.root > 0.0 { m.root } else { default }).map(|x| format!(" in {}", x)).unwrap_or_default();
        let line = match k {
            "tanpura" => format!("<b>Tanpura</b>, the Indian drone lute, gently plucked{}", key(138.59)),
            "drone" => match var {
                "organ" => "<b>Organ</b>: a soft, sustained chord".to_string(),
                "bowl" => "<b>Singing-bowl drone</b>".to_string(),
                "harmonium" => "<b>Harmonium</b> drone".to_string(),
                _ => "<b>A soft drone</b> underneath the practice".to_string(),
            },
            "choir" => match var {
                "organum" => "<b>Plainchant organum</b>: voices holding a chant in fifths".to_string(),
                "hum" => "<b>A humming choir</b>".to_string(),
                "ah" => "<b>A choir</b> singing on ‘ah’".to_string(),
                _ => "<b>Ison</b>: voices holding a steady drone note, as in Byzantine chant".to_string(),
            },
            "om" => "<b>A low AUM chant</b>".to_string(),
            "gyuto" => "<b>Deep-voice chant</b> in the Tibetan Gyuto style, one voice sounding a chord".to_string(),
            "flute" => {
                if var.starts_with("shakuhachi") { "<b>Shakuhachi</b>, the Japanese bamboo flute of Zen".to_string() }
                else if var.starts_with("xiao") { "<b>Xiao</b>, the Chinese vertical bamboo flute".to_string() }
                else if var.starts_with("bansuri") {
                    let raga = if var.contains("bhupali") { "Bhupali" } else if var.contains("bhairav") { "Bhairav" } else { "Yaman" };
                    format!("<b>Bansuri</b>, the Indian bamboo flute, improvising in raga {}", raga)
                } else {
                    let maqam = if var.contains("rast") { "Rast" } else if var.contains("bayati") { "Bayati" } else { "Hijaz" };
                    format!("<b>Ney</b>, the Sufi reed flute, in maqam {}", maqam)
                }
            }
            "guqin" => "<b>Guqin</b>, the Chinese seven-string zither".to_string(),
            "harp" => "<b>Harp</b>".to_string(),
            "daf" => "<b>Daf</b>, the Sufi frame drum".to_string(),
            "damaru" => "<b>Damaru</b>, the Tibetan hand drum".to_string(),
            "rin" => match var {
                "church" => "<b>A church bell</b> marking each new stage".to_string(),
                "bell" => "<b>A bell</b> marking each new stage".to_string(),
                _ => "<b>A rin</b> (Japanese bowl-bell) marking each new stage".to_string(),
            },
            "gongbath" => "<b>Gong bath</b>: large gongs played in slow waves".to_string(),
            "stream" => "<b>A mountain stream</b>".to_string(),
            "wind" => "<b>Soft wind</b>".to_string(),
            "fire" => "<b>A crackling fire</b>".to_string(),
            "chant" => {
                let mut names: Vec<&str> = Vec::new();
                for c in spec.split(',') {
                    let nm = c.split('@').next().unwrap_or("").trim();
                    if let Some(l) = chant_label(nm) { if !names.contains(&l) { names.push(l); } }
                }
                if names.is_empty() { "<b>Group chant</b>: a group of voices chanting with you".to_string() }
                else { format!("<b>Group chant</b>: voices chanting {}", names.join(", then ")) }
            }
            _ => continue,
        };
        out.push(line);
    }
    out
}

fn chant_label(n: &str) -> Option<&'static str> {
    Some(match n {
        "abha" => "<i>Alláh-u-Abhá</i>",
        "adi" => "<i>Ong Namo Guru Dev Namo</i>",
        "adonai_haaretz" | "eheieh" | "shaddai" | "yhvh_eloah" | "yhvh_elohim" => "the divine names of the Middle Pillar",
        "aleph" => "the letter Alef with its vowels",
        "amituofo" => "<i>Namo Amituofo</i>",
        "ansuz" | "laguz" | "isa" | "dagaz" => "the rune names",
        "archangels" | "lbrp_names" | "qcross" => "the names of the Pentagram ritual",
        "ashem" => "the <i>Ashem Vohu</i>",
        "aum" | "aum10" | "aum_a" | "aum_u" | "aum_m" => "<i>AUM</i>",
        "avemaria" => "the <i>Ave Maria</i>",
        "awen" => "<i>Awen</i>",
        "baruch" => "<i>Barukh shem kevod malkhuto</i>",
        "bija_lam" | "bija_vam" | "bija_ram" | "bija_yam" | "bija_ham" | "bija_om" => "the chakra seed sounds",
        "daimoku" => "<i>Nam-myoho-renge-kyo</i>",
        "gayatri" => "the <i>Gāyatrī</i> mantra",
        "guanyin" => "<i>Namo Guanshiyin Pusa</i>",
        "guru" => "the Vajra Guru mantra",
        "harae" => "<i>Harae tamae, kiyome tamae</i>",
        "heart" => "<i>Gate gate pāragate</i>",
        "hu" => "<i>Allāh Hū</i>",
        "inmanus" => "<i>In manus tuas</i>",
        "itipiso" => "the <i>Itipiso</i>",
        "jesus" | "jesus_w" => "the Jesus Prayer",
        "kadosh" => "<i>Kadosh, kadosh, kadosh</i>",
        "kyrie" => "<i>Kyrie eleison</i>",
        "lailaha" | "lailaha_slow" => "<i>Lā ilāha illā-llāh</i>",
        "mahamantra" => "the Hare Krishna mahā-mantra",
        "mani" => "<i>Om Mani Padme Hum</i>",
        "manjushri" => "<i>Om A Ra Pa Tsa Na Dhih</i>",
        "medicine" => "the Medicine Buddha mantra",
        "metta" => "the Pāli mettā verses",
        "mulmantar" => "the <i>Mūl Mantar</i>",
        "namo" => "<i>Namo tassa</i>",
        "navkar" => "the <i>Navkār Mantra</i>",
        "nembutsu" => "<i>Namu Amida Butsu</i>",
        "niggun" | "niggun_hum" => "a wordless niggun",
        "qingjing" => "the <i>Qingjing Jing</i>",
        "ramadasa" => "<i>Ra Ma Da Sa, Sa Say So Hung</i>",
        "satanama" | "satanama_w" => "<i>Sa Ta Na Ma</i>",
        "satnam" => "<i>Sat Nam</i>",
        "shema" => "the <i>Shema</i>",
        "shiva" => "<i>Om Namah Shivaya</i>",
        "six_xu" | "six_he" | "six_hu" | "six_si" | "six_chui" | "six_xi" => "the six healing sounds",
        "tara" => "<i>Om Tare Tuttare Ture Soha</i>",
        "tasbih" => "<i>Subḥānallāh, Al-ḥamdu lillāh, Allāhu akbar</i>",
        "ubicaritas" => "<i>Ubi caritas</i>",
        "vacate" => "<i>Vacate et videte</i> (Be still, and know)",
        "vajrasattva" => "<i>Om Benza Satto Hung</i>",
        "waheguru" => "<i>Waheguru</i>",
        "white_a" => "a long <i>Ah</i>",
        "yalatif" | "yarahman" | "yanur" | "yawadud" => "the Beautiful Names of God",
        "yeheshuah" => "<i>Yeheshuah</i>",
        _ => return None,
    })
}
