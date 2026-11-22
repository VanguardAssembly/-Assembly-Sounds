//! Long-form background for every session: what it is, where it comes from,
//! when it began, what is claimed, and what research actually found.
//! Text is trusted HTML authored here (never user input).
use crate::plan::{Info, Preset};

#[path = "info_more.rs"]
mod info_more;

fn i(what: impl Into<String>, origin: impl Into<String>, era: impl Into<String>, claims: impl Into<String>, research: impl Into<String>) -> Info {
    Info { what: what.into(), origin: origin.into(), era: era.into(), claims: claims.into(), research: research.into() }
}

pub fn fill(v: &mut [Preset]) {
    for p in v.iter_mut() {
        p.info = Some(info_for(p));
    }
}

// ---------------------------------------------------------------- shared text
const BB_WHAT: &str = "<p><b>Binaural beats</b> happen when each ear hears a slightly different pure tone, for example 200 Hz on the left and 206 Hz on the right. No 6 Hz sound exists in the air. The brainstem (the superior olivary complex, which compares the timing of sound at the two ears) combines the two and you hear a slow \"wobble\" at the 6 Hz difference. <b>Monaural beats</b> mix the two tones before they reach the ear, so the beat is physically present. <b>Isochronic tones</b> switch a single tone on and off at the target rate. They work on speakers and give the strongest brain response at high rates.</p>";
const BB_HISTORY: &str = "<p>Heinrich Wilhelm Dove, a Prussian physicist, described binaural beats in <b>1839</b>. They stayed a curiosity until <b>Gerald Oster</b>'s 1973 <i>Scientific American</i> article \"Auditory Beats in the Brain\", which proposed them as a research tool. <b>Robert Monroe</b> patented a method of using them to induce sleep stages (US Patent 3,884,218, <b>1975</b>), and his \"Hemi-Sync\" tapes made them popular. Consumer CDs, software and apps spread from the 1990s onward.</p>";
const BB_RESEARCH: &str = "<p><b>Overall evidence for binaural beats:</b> a 2019 meta-analysis of 22 studies (Garcia-Argibay, Santed &amp; Reales, <i>Psychological Research</i>) found small-to-moderate effects on anxiety, memory, attention and pain perception. Effects were larger when people listened <i>before</i> a task and for about 10+ minutes. But the main idea, that the brain \"entrains\" to the beat frequency, is weakly supported. A 2023 systematic review of EEG studies (Ingendoh, Posny &amp; Heine, <i>PLOS ONE</i>) found that 5 of 14 studies supported entrainment, 8 contradicted it and 1 was mixed. Orozco Perez et al. (2020, <i>eNeuro</i>) found the EEG followed the beat only weakly and found no change in mood or state beyond a control tone. Many studies are small, short and not blinded.</p>";

const GW_ORIGIN: &str = "<p><b>Robert A. Monroe</b> (1915–1995) was a New York radio-programme producer and executive. In <b>1958</b>, while experimenting with sleep-learning tapes, he began having spontaneous \"out-of-body\" experiences, which he described in <i>Journeys Out of the Body</i> (<b>1971</b>). He set up a research lab to find sound patterns that could reliably produce such states, later called <b>Hemi-Sync</b> (\"hemispheric synchronisation\"). This became <b>The Monroe Institute</b> in Faber, Virginia. Its residential <b>Gateway Voyage</b> course and the home <b>Gateway Experience</b> tapes (Waves I–VIII, from the late 1970s and 1980s) structure consciousness into numbered <b>Focus levels</b>.</p>";
const GW_ERA: &str = "<p>Monroe's sound experiments ran from the late 1950s to the 1960s. The Hemi-Sync patent was filed in 1973 and granted in 1975. The Gateway Voyage program began in the late 1970s, and the Gateway Experience home series was released in the 1980s. In <b>1983</b>, U.S. Army Lt. Col. Wayne McDonnell wrote <i>Analysis and Assessment of Gateway Process</i> for Army intelligence (INSCOM), which sent personnel to train at the Institute. The CIA declassified it in <b>2003</b>, and it went viral online in <b>2021</b>.</p>";
const GW_CLAIMS: &str = "<p>The Institute says Hemi-Sync brings the two brain hemispheres into coherent, synchronised activity. Each Focus level is presented as a distinct, reproducible state of consciousness. McDonnell's report explained the method using Itzhak Bentov's model of consciousness as a <i>holographic</i> energy field and physicist ideas about resonance. It described advanced states as allowing the mind to move beyond time and space. These are claims made by the Institute and the report, not established findings.</p>";
const GW_RESEARCH: &str = "<p>Independent research on Hemi-Sync is limited. Kliempt et al. (1999, <i>Anaesthesia</i>) found that patients who heard Hemi-Sync sounds during surgery under general anaesthesia needed less fentanyl than patients who heard a blank tape. Lane et al. (1998) used beta/theta beats similar to Hemi-Sync and found effects on vigilance. Studies of hemispheric \"synchronisation\" itself are few and mostly unpublished or Institute-funded. The McDonnell report is an intelligence analyst's assessment, not peer-reviewed science. The exact Focus-level frequency mixes are <b>proprietary</b>, so this app's versions are <b>interpretations</b>: layered delta, theta and alpha/beta beats in the published Hemi-Sync style.</p>";

const LL_ORIGIN: &str = "<p><b>Lifeline</b> is the Monroe Institute's follow-on course to Gateway. Monroe described it in his third book, <i>Ultimate Journey</i> (<b>1994</b>). It maps the levels <b>Focus 22–27</b> as regions beyond ordinary physical life. Focus 22 is people in altered or delirious states. Focus 23 is the newly deceased. Focus 24–26 are the \"Belief System Territories\". Focus 27 is \"the Park\", a constructed place of rest and transition. The course is framed as \"retrieval\": helping souls who are \"stuck\".</p>";
const LL_ERA: &str = "<p>The Lifeline programme was developed at the Monroe Institute in the <b>early 1990s</b>, after Monroe's own late-life explorations. It is still taught as a residential course.</p>";
const LL_RESEARCH: &str = "<p>There is <b>no scientific research</b> on the existence of Focus 22–27 or on the afterlife claims. Reports of these levels are personal and subjective. This app offers them as <b>speculative</b> guided meditations: deep delta/theta and gamma layers with calm imagery. The descriptions are Monroe's teachings, not facts. If death- or grief-related imagery is upsetting for you, skip these sessions.</p>";

const BREATH_RES: &str = "<p>Slow breathing at about <b>6 breaths per minute</b> (0.1 Hz) is one of the best-supported calming techniques. It boosts heart-rate variability and the baroreflex (Lehrer &amp; Gevirtz, 2014, <i>Frontiers in Psychology</i>; Zaccaro et al., 2018 systematic review, <i>Frontiers in Human Neuroscience</i>). In many of these sessions the breathing probably does more than the beat does.</p>";

// ---------------------------------------------------------------- main lookup
fn info_for(p: &Preset) -> Info {
    let id = p.id.as_str();
    if let Some(x) = info_more::get(id) {
        return x;
    }
    if let Some(x) = ib_info(id) {
        return x;
    }
    if let Some(x) = lj_info(id) {
        return x;
    }
    if let Some(x) = my_info(id) {
        return x;
    }
    if let Some(x) = med_info(id) {
        return x;
    }
    if let Some(x) = extra(id) {
        return x;
    }
    if let Some(x) = specific(id) {
        return x;
    }
    if let Some(rest) = id.strip_prefix("lib") {
        let hz = rest.parse::<f64>().unwrap_or(1000.0) / 100.0;
        return library(hz, &p.name);
    }
    if let Some(n) = id.strip_prefix("sch") {
        return schumann(n.parse().unwrap_or(1));
    }
    if let Some(n) = id.strip_prefix("sol") {
        return solfeggio(n.parse().unwrap_or(528));
    }
    if let Some(n) = id.strip_prefix("ch") {
        return chakra(n.parse().unwrap_or(396), &p.name);
    }
    if let Some(n) = id.strip_prefix("pl") {
        return planet(n.parse::<f64>().unwrap_or(13610.0) / 100.0, &p.name);
    }
    // generic fallback (should not normally be reached)
    i(format!("<p>{}</p>{}", p.short, BB_WHAT), BB_HISTORY, BB_HISTORY, "<p>See the session description above.</p>", BB_RESEARCH)
}

fn gw(what: &str, extra_claim: &str) -> Info {
    i(format!("{}{}", what, BB_WHAT), GW_ORIGIN, GW_ERA, format!("{}{}", extra_claim, GW_CLAIMS), GW_RESEARCH)
}
fn ll(what: &str) -> Info {
    i(format!("{}{}", what, BB_WHAT), LL_ORIGIN, LL_ERA,
      "<p>Monroe and Lifeline participants describe meeting deceased people, \"helpers\" and belief-based realities, and helping lost souls cross over. Participants say these experiences relieve fear of death. These are spiritual claims and cannot be tested.</p>",
      LL_RESEARCH)
}
fn bb(what: &str, origin: &str, era: &str, claims: &str, study: &str) -> Info {
    i(format!("{}{}", what, BB_WHAT), format!("{}{}", origin, BB_HISTORY), era, claims, format!("{}{}", study, BB_RESEARCH))
}

fn specific(id: &str) -> Option<Info> {
    Some(match id {
        // ================= Gateway =================
        "f3" => gw("<p><b>Focus 3</b> is the opening exercise of Gateway Wave I (\"Discovery\"). It introduces the listener to Hemi-Sync sound and the feeling of the brain's two halves working in step, while staying fully awake.</p>",
                   "<p>Monroe taught that Focus 3 is the first step: the brain becomes more balanced and \"coherent\", and this coherence makes the deeper levels possible.</p>"),
        "f10" => gw("<p><b>Focus 10: \"Mind Awake, Body Asleep\"</b>. The body relaxes to the edge of sleep while the mind stays fully alert. It is the foundation of the whole Gateway series. This version builds it from three layers: a delta \"body\" layer, a theta depth layer and a light alpha/beta \"keep awake\" layer.</p>",
                    "<p>Focus 10 is said to be the launch point for every later exercise. Gateway tapes pair it with preparatory tools: the \"Energy Conversion Box\", the \"Resonant Tuning\" hum and the \"Gateway Affirmation\". McDonnell's report described it as the state in which the brain's hemispheres reach coherence.</p>"),
        "f12" => gw("<p><b>Focus 12: \"Expanded Awareness\"</b>. Attention feels as if it reaches beyond the body. Gateway Wave II and III exercises use it for problem-solving, asking questions and receiving non-verbal answers.</p>",
                    "<p>Monroe taught that in Focus 12 your perception extends beyond the physical senses. McDonnell's 1983 report described it as a state for intuition, problem-solving and \"patterning\": deliberately imagining a desired outcome.</p>"),
        "f15" => gw("<p><b>Focus 15: \"No Time\"</b>. A deep, still state in which the sense of time drops away. Gateway Wave IV uses it to explore what lies \"beyond time\" and to set intentions.</p>",
                    "<p>Gateway teaches that Focus 15 lets you step outside linear time and access past and future possibilities. McDonnell connected this to the idea of consciousness as a field that exists outside space-time.</p>"),
        "f21" => gw("<p><b>Focus 21: \"The Bridge\"</b>. In Monroe's model, the edge of physical reality, where other \"energy systems\" begin. Later Gateway material presents it as the bridge into non-physical levels.</p>",
                    "<p>Monroe described Focus 21 as the border between time-space and other realities, and as the gateway to the Lifeline levels (22–27).</p>"),

        // ================= Lifeline =================
        "f22" => ll("<p><b>Focus 22</b> in Monroe's map is the level of people who are still physically alive but not fully conscious: dreaming, delirious, sedated or in a coma. This version is a dreamlike drift from deep delta into theta.</p>"),
        "f23" => ll("<p><b>Focus 23</b> in Monroe's map is where people who have recently died are said to remain for a while, sometimes unaware they have died. This version is a quiet reflective meditation in low delta/theta.</p>"),
        "f25" => ll("<p><b>Focus 24–26</b> are the \"Belief System Territories\". In Monroe's account these are realities built from shared human beliefs, where people go after death to the afterlife they expected. This version adds a 40 Hz gamma layer over deep delta.</p>"),
        "f27" => ll("<p><b>Focus 27</b>, \"the Park\" or \"Reception Centre\", is in Monroe's account a peaceful, created place where the newly arrived are welcomed, rest and choose their next step. This version is the calmest of the Lifeline sessions.</p>"),

        // ================= Research-based =================
        "alpha" => bb("<p><b>Alpha (8–13 Hz)</b> is the rhythm of relaxed wakefulness. It is strongest at the back of the head when the eyes are closed and the mind is calm. This session uses a 10 Hz beat, the typical adult alpha peak.</p>",
            "<p>Alpha was the first brain rhythm ever recorded. German psychiatrist <b>Hans Berger</b> found it in 1924 and published it in <b>1929</b>, which is why it was once called the \"Berger rhythm\". Adrian &amp; Matthews confirmed it at Cambridge in 1934. In the 1960s, Joe Kamiya's <b>alpha biofeedback</b> experiments started the \"alpha craze\" and the self-help Silva Method (1966).</p>",
            "<p>Alpha was discovered in the 1920s. Alpha relaxation became a pop trend in the 1960s–70s, and alpha binaural beats were sold from the 1990s.</p>",
            "<p>Alpha beats are said to relax you, lift mood, reduce stress and make learning easier.</p>",
            "<p>The 2019 meta-analysis supports a modest calming effect. Evidence that a 10 Hz beat raises EEG alpha is mixed: some studies saw small increases and others none.</p>"),
        "theta" => bb("<p><b>Theta (4–8 Hz)</b> appears in drowsiness, deep meditation and memory processing. This session uses a 6 Hz beat on a 250 Hz carrier, the exact setup of a published study.</p>",
            "<p>Theta was named in the 1940s by <b>W. Grey Walter</b>'s group (after the thalamus). The hippocampal theta rhythm was described in animals in the 1950s. Frontal-midline theta, seen during focused meditation and mental arithmetic, was described by Japanese researchers in the 1970s (Ishihara &amp; Yoshii, 1972).</p>",
            "<p>Theta was named in the 1940s. Meditation EEG studies followed from the 1960s, and this protocol comes from 2017.</p>",
            "<p>Theta beats are said to deepen meditation, increase creativity and open access to the subconscious.</p>",
            "<p><b>Jirakittayakorn &amp; Wongsawat (2017, <i>Frontiers in Neuroscience</i>)</b> found that 10 minutes of a 6 Hz beat increased frontal theta and activity linked to meditative states in about 20 participants, compared with a control. The sample was small and the result has not been widely replicated.</p>"),
        "delta" => bb("<p><b>Delta (0.5–4 Hz)</b> dominates deep, dreamless slow-wave sleep (stage N3). This session plays a 3 Hz beat at a very low volume while you fall asleep.</p>",
            "<p>W. Grey Walter named delta waves in <b>1936</b>, recording them over brain tumours. Loomis, Harvey &amp; Hobart (1937) linked slow waves to deep sleep stages. Slow-wave sleep is when the body releases most of its growth hormone (Van Cauter and colleagues).</p>",
            "<p>Delta was described in the 1930s. Sleep binaural beats became popular from the 1990s, and the study this session copies dates from 2018.</p>",
            "<p>Delta beats are said to deepen sleep, speed up physical recovery and release growth hormone.</p>",
            "<p><b>Jirakittayakorn &amp; Wongsawat (2018, <i>Frontiers in Human Neuroscience</i>)</b> reported that a 3 Hz beat lengthened N3 deep sleep and shortened light N2 sleep in a small sample of 24. No study has shown more growth hormone from beats.</p>"),
        "beta" => bb("<p><b>Beta (13–30 Hz)</b> is the active, alert, task-focused rhythm. This session uses 16 Hz and moves toward 24 Hz.</p>",
            "<p>Hans Berger named beta in the 1920s for the faster waves that appeared when the eyes opened or the mind was busy. Lane et al. compared beta with theta/delta beats in 1998, and Beauchene et al. tested 15 Hz beats on working memory in 2016.</p>",
            "<p>Beta was described in the 1920s and studied as binaural beats from the 1990s.</p>",
            "<p>Beta beats are said to improve concentration, alertness and mood.</p>",
            "<p><b>Lane, Kasian, Owens &amp; Marsh (1998, <i>Physiology &amp; Behavior</i>)</b> found that beta beats (16 and 24 Hz) improved vigilance-task performance and lowered negative mood compared with theta/delta beats. The study was small (n = 29). <b>Beauchene et al. (2016, <i>PLOS ONE</i>)</b> found that 15 Hz beats improved working-memory accuracy compared with other frequencies.</p>"),
        "gamma" => bb("<p><b>Gamma (30–100 Hz)</b> is fast activity linked to attention, perception and binding information together. This session uses 40 Hz, the most-studied gamma rate.</p>",
            "<p>Jasper &amp; Andrews used the term \"gamma\" in 1938. Gray &amp; Singer (1989, <i>Nature</i>) linked 40 Hz synchrony to visual \"binding\". Galambos et al. (1981) discovered the 40 Hz auditory steady-state response. Colzato and Hommel's group studied gamma beats and creativity in 2015.</p>",
            "<p>The term dates from 1938, the binding theory from 1989, and gamma beat studies from the 2010s.</p>",
            "<p>Gamma beats are said to sharpen focus, insight and creativity, and to bring \"peak\" states.</p>",
            "<p><b>Reedijk, Bolders, Colzato &amp; Hommel (2015, <i>Frontiers in Psychology</i>)</b> found that gamma beats changed divergent-thinking (creativity) scores, but only in people with low baseline dopamine. Effects across studies are inconsistent.</p>"),
        "genus" => i("<p><b>GENUS</b> (Gamma ENtrainment Using Sensory stimuli) delivers sound clicks or pulses at exactly 40 Hz. This app plays <b>isochronic</b> 40 Hz pulses, which work without headphones. There is no flashing light: flicker carries a seizure risk and is deliberately not included.</p>",
            "<p>The method comes from <b>Li-Huei Tsai</b>'s lab at <b>MIT</b>. Galambos had already shown in 1981 that the brain produces a strong 40 Hz auditory steady-state response. MIT spin-off Cognito Therapeutics is testing a GENUS headset in human trials.</p>",
            "<p>Iaccarino et al. (2016, <i>Nature</i>) used light flicker in mice. Martorell et al. (2019, <i>Cell</i>) added sound. Human feasibility trials began around 2019–2022, and a larger phase-3 trial (HOPE) started in the mid-2020s.</p>",
            "<p>Supporters hope 40 Hz stimulation can slow Alzheimer's disease by activating the brain's immune cells (microglia) and its waste-clearing (glymphatic) system.</p>",
            "<p>In mouse models, <b>Martorell et al. (2019)</b> found that a week of 40 Hz tones reduced amyloid and improved memory. Murdock et al. (2024, <i>Nature</i>) linked this to glymphatic clearance. Small human studies (Chan et al., 2022, <i>PLOS ONE</i>) reported it was safe and suggested some benefits. Other labs failed to reproduce the amyloid effect in mice (Soula et al., 2023, <i>Nature Neuroscience</i>). <b>It is not a treatment.</b> If you are concerned about memory, see a doctor.</p>"),
        "anx" => bb("<p>A calming session that combines an alpha-to-theta binaural glide with paced breathing at about 6 breaths per minute.</p>",
            "<p>Hospital researchers began using binaural beats to reduce anxiety before surgery in the 2000s (Padmanabhan, Hildreth &amp; Laws, 2005).</p>",
            "<p>The surgical studies date from the 2000s, and the meta-analysis from 2019.</p>",
            "<p>Beats are said to reduce anxiety and stress quickly.</p>",
            &format!("<p><b>Padmanabhan et al. (2005, <i>Anaesthesia</i>)</b> found that patients who heard binaural beats had less anxiety before surgery than patients who heard a blank tape. Anxiety was one of the most consistent effects in the 2019 meta-analysis.</p>{}", BREATH_RES)),
        "zen" => bb("<p>A meditation session modelled on the EEG of Zen monks during <i>zazen</i>: alpha first, then slower alpha, then rhythmic theta.</p>",
            "<p><b>Akira Kasamatsu and Tomio Hirai</b> at the University of Tokyo recorded the EEG of Zen priests of the Sōtō and Rinzai schools. Their study \"An electroencephalographic study on the Zen meditation (Zazen)\" was published in <b>1966</b>. Zen itself developed in China as Chan from the 6th century and reached Japan in the 12th–13th centuries.</p>",
            "<p>Zen practice is many centuries old. The EEG study is from 1966.</p>",
            "<p>Following the monks' brainwave path is said to help you meditate more deeply, faster.</p>",
            "<p>Kasamatsu &amp; Hirai found four stages: alpha appeared, alpha grew stronger, alpha slowed, and rhythmic theta trains appeared, most in the most experienced monks. Later meditation EEG research broadly confirms more alpha and frontal theta (Cahn &amp; Polich, 2006 review). No study shows that beats reproduce the monks' state.</p>"),
        "edison" => bb("<p>A <b>hypnagogia</b> session: drift into the first, lightest stage of sleep (N1), where dreamlike images appear, then come back with your insight. Hold a light object; if it drops, you've gone too deep.</p>",
            "<p>The method is linked to <b>Thomas Edison</b> and <b>Salvador Dalí</b>, who reportedly napped holding steel balls or a key over a plate. In 2021, Delphine Oudiette's team in Paris tested it scientifically.</p>",
            "<p>The anecdote is from the late 19th to early 20th century (Dalí described it in 1948). The study is from 2021.</p>",
            "<p>A short stay in N1 sleep is said to unlock creative insight.</p>",
            "<p><b>Lacaux et al. (2021, <i>Science Advances</i>)</b> found that people who spent at least 15 seconds in N1 were nearly <b>three times as likely</b> (83% vs 30%) to discover a hidden shortcut in a maths task. The effect disappeared if they went into deeper sleep. The beats here only support relaxation.</p>"),
        "alphatheta" => bb("<p>A binaural version of <b>alpha-theta training</b>: hovering at the 7–8 Hz border between relaxed wakefulness and the edge of sleep, where imagery is vivid.</p>",
            "<p>Elmer and Alyce Green at the Menninger Foundation (Kansas) studied alpha-theta \"twilight\" states in the 1960s–70s. <b>Eugene Peniston and Paul Kulkosky</b> used alpha-theta neurofeedback to treat alcoholism (1989) and PTSD (1991).</p>",
            "<p>The research began in the 1960s–70s, and the clinical protocol dates from 1989.</p>",
            "<p>Alpha-theta states are said to heal trauma, reduce addictive craving and boost creativity.</p>",
            "<p>Peniston &amp; Kulkosky's studies were small, and replications have been mixed. A review (Sokhadze et al., 2008) rated alpha-theta neurofeedback for addiction as \"efficacious\" only when combined with other treatment. Gruzelier (2009) linked it to better music performance. <b>Binaural beats are not neurofeedback</b>: there is no feedback loop.</p>"),
        "hrv" => bb("<p><b>Resonance-frequency breathing</b>: about 6 breaths per minute (0.1 Hz), following the on-screen pacer, with a soft 10 Hz alpha beat in the background.</p>",
            "<p>Russian and American researchers (Vaschillo, Lehrer) developed HRV biofeedback in the 1980s–90s. Slow breathing is also part of yoga pranayama and of prayer traditions.</p>",
            "<p>The breathing practices are ancient. HRV biofeedback dates from the 1990s, and the main review is from 2014.</p>",
            "<p>Slow breathing is said to calm the nervous system, lower blood pressure and improve emotional control.</p>",
            &format!("<p><b>Lehrer &amp; Gevirtz (2014)</b> reviewed the evidence. A meta-analysis (Goessl, Curtiss &amp; Hofmann, 2017, <i>Psychological Medicine</i>) found that HRV biofeedback had a <b>large</b> effect on self-reported stress and anxiety.</p>{}", BREATH_RES)),
        "lucid" => bb("<p>An experimental lucid-dreaming prompt: a theta base with a quiet 40 Hz layer, plus reminders to check whether you are dreaming. Best used after waking early, then going back to sleep (\"WBTB\").</p>",
            "<p>The term <b>lucid dream</b> was coined in 1913 by Dutch psychiatrist Frederik van Eeden. Keith Hearne (1975) and <b>Stephen LaBerge</b> (Stanford, 1981) proved lucid dreams are real, using agreed eye-movement signals made from inside REM sleep.</p>",
            "<p>The term dates from 1913 and lab proof from the 1970s–80s. The stimulation study is from 2014.</p>",
            "<p>40 Hz is said to \"switch on\" lucidity in dreams.</p>",
            "<p><b>Voss et al. (2014, <i>Nature Neuroscience</i>)</b> found that 25 and 40 Hz electrical stimulation (tACS) during REM sleep increased lucidity ratings. Later attempts to replicate this were mixed. Audio beats are very different from electrical stimulation, so treat this session as an experiment. Better-supported methods are reality testing, MILD (Stumbrys 2012 review; Aspy 2020) and cue reactivation (see the TLR session).</p>"),
        "nap" => bb("<p>A <b>10-minute power nap</b>: settle, theta rest, then a gentle rise through alpha into beta to avoid grogginess.</p>",
            "<p>Napping is universal. NASA's 1995 pilot study (Rosekind) made the \"power nap\" famous, and Brooks &amp; Lack at Flinders University compared nap lengths in 2006.</p>",
            "<p>NASA's study was in 1995 and the comparison of nap lengths in 2006.</p>",
            "<p>A short nap is said to restore alertness without leaving you groggy.</p>",
            "<p><b>Brooks &amp; Lack (2006, <i>Sleep</i>)</b> found that a 10-minute nap improved alertness and performance almost at once and for up to 2.5 hours. A 5-minute nap did little, and naps of 20–30 minutes caused grogginess (sleep inertia) first. NASA found that a 26-minute cockpit nap improved pilot alertness by about 54%.</p>"),
        "nsdr" => bb("<p><b>Yoga Nidra</b> (\"yogic sleep\"), also called <b>NSDR</b> (non-sleep deep rest): guided body-scan relaxation while you stay aware.</p>",
            "<p>The roots are in Indian tantra and yoga. The modern method was systematised by <b>Swami Satyananda Saraswati</b> of the Bihar School of Yoga, whose book <i>Yoga Nidra</i> appeared in <b>1976</b>. Neuroscientist Andrew Huberman popularised the term \"NSDR\" around 2021.</p>",
            "<p>The method has ancient roots, was formalised in the 1960s–70s, and was rebranded as NSDR in the 2020s.</p>",
            "<p>Practitioners say it gives deep restoration, less stress and better sleep, and that it can partly replace lost sleep.</p>",
            "<p><b>Kjaer et al. (2002, <i>Cognitive Brain Research</i>)</b> used PET scans and found a 65% increase in dopamine release in the striatum during yoga nidra. Small trials suggest it reduces anxiety and improves sleep (Moszeik et al., 2022; Datta et al., 2017). The claim that it \"replaces sleep\" is not proven.</p>"),
        "smr" => bb("<p><b>Sensorimotor rhythm</b> (SMR, about 12–15 Hz) is a calm-but-alert rhythm over the motor cortex that appears when the body is still. This session uses 13.5 Hz.</p>",
            "<p><b>Barry Sterman</b> at UCLA discovered SMR in cats in the late 1960s. Cats trained to produce SMR later resisted seizures caused by exposure to rocket fuel (hydrazine) in a NASA-funded study. Joel Lubar used SMR training for ADHD from <b>1976</b>.</p>",
            "<p>SMR was discovered in the late 1960s and used clinically from the 1970s. The meta-analysis is from 2009.</p>",
            "<p>SMR beats are said to give calm, focused attention, like neurofeedback for ADHD.</p>",
            "<p><b>Arns et al. (2009)</b> meta-analysis: neurofeedback (including SMR) had large effects on inattention and impulsivity in ADHD, though later blinded trials were weaker. A meta-analysis of seizure studies (Tan et al., 2009) supported SMR training for epilepsy. <b>Kennel et al. (2010)</b> found binaural beats gave no significant ADHD benefit. Beats are not neurofeedback.</p>"),
        "pain" => bb("<p>A <b>pain-coping</b> session: theta beat with slow breathing and attention-shifting cues.</p>",
            "<p>Using sound and music to relieve pain goes back to the \"audio-analgesia\" of 1960 (Gardner &amp; Licklider, dentistry). Binaural beat pain studies followed in the 2010s.</p>",
            "<p>Audio-analgesia dates from 1960 and beat studies from the 2010s.</p>",
            "<p>Beats are said to reduce pain and the amount of painkillers needed.</p>",
            &format!("<p>The 2019 meta-analysis found a moderate reduction in how much pain people perceived. Zampi (2016) found modest effects in chronic pain. Kliempt et al. (1999) found patients needed less fentanyl. Music in general reduces pain (Lee, 2016 meta-analysis of 97 trials). This is not medical treatment.</p>{}", BREATH_RES)),
        "monks" => bb("<p><b>Compassion gamma</b>: a 40 Hz layer over alpha, inspired by the EEG of long-term Tibetan Buddhist meditators.</p>",
            "<p>Richard Davidson's lab at the University of Wisconsin worked with monks recommended by the Dalai Lama, including Matthieu Ricard. The practice was <i>non-referential compassion</i> from Tibetan Buddhism, a tradition dating back to roughly the 8th century in Tibet.</p>",
            "<p>The practice is centuries old. The study was published in 2004.</p>",
            "<p>Compassion meditation is said to produce extraordinary gamma synchrony and lasting changes in the brain.</p>",
            "<p><b>Lutz, Greischar, Rawlings, Ricard &amp; Davidson (2004, <i>PNAS</i>)</b> recorded gamma activity in monks with 10,000–50,000 hours of practice that was far higher than in novices. Some critics have asked whether muscle activity contributed. The beat cannot recreate decades of training. Use the session as a compassion practice.</p>"),

        // ================= Purpose & Tradition =================
        "obe" => bb("<p>An <b>out-of-body experience (OBE)</b> or \"astral projection\" practice: the Gateway \"mind awake, body asleep\" approach, a 6.3 Hz theta beat and the \"rope\" visualisation.</p>",
            "<p>Ideas of a soul that travels appear in Egyptian (<i>ka/ba</i>), Indian, Tibetan and shamanic traditions. In the West, <b>Theosophy</b> (late 1800s) popularised \"astral projection\". Sylvan Muldoon &amp; Hereward Carrington's <i>The Projection of the Astral Body</i> (<b>1929</b>) and Monroe's <i>Journeys Out of the Body</i> (1971) made it a practice. Robert Bruce's \"rope technique\" comes from <i>Astral Dynamics</i> (1999).</p>",
            "<p>The idea is ancient. It was named \"astral projection\" in the 19th century and studied in labs from the 2000s.</p>",
            "<p>Practitioners claim that consciousness can leave the body and observe distant places. The 6.3 Hz \"OBE frequency\" is an internet claim.</p>",
            "<p>About 10% of people report at least one OBE in their life. Neuroscience links OBEs to the <b>temporo-parietal junction</b>. <b>Blanke et al. (2002, <i>Nature</i>)</b> triggered OBE-like feelings by electrically stimulating the right angular gyrus. <b>Ehrsson (2007, <i>Science</i>)</b> and Lenggenhager et al. (2007) created them with virtual-reality illusions. No controlled test has shown that people can see real information while \"out\" (e.g. Tart, 1968, was a single uncontrolled case). There is no evidence for 6.3 Hz.</p>"),
        "rv" => bb("<p>A <b>remote viewing</b> practice in the style of the U.S. Stargate program: relax into Focus 10/12, then sketch impressions of a hidden target.</p>",
            "<p>Remote viewing began at the <b>Stanford Research Institute</b> in <b>1972</b>. Physicists Russell Targ and Harold Puthoff worked with artist <b>Ingo Swann</b> (who coined the term) and ex-police officer Pat Price. The CIA, the Army and the Defense Intelligence Agency (DIA) funded it under code names <b>Gondola Wish, Grill Flame, Center Lane, Sun Streak</b> and finally <b>Stargate</b>. Army viewers were trained at the Monroe Institute.</p>",
            "<p>It ran from 1972 to 1995 and was declassified in 1995.</p>",
            "<p>Viewers claimed they could describe distant places and objects with their minds alone, and that the skill could be trained.</p>",
            "<p>In <b>1995</b> the American Institutes for Research reviewed the program for the CIA. Statistician <b>Jessica Utts</b> argued the lab results were statistically significant. Psychologist <b>Ray Hyman</b> argued the methods had flaws and nobody had replicated the results independently. Both agreed it had <b>never produced actionable intelligence</b>, and the program was closed. Mainstream science does not accept remote viewing. Treat this session as an imagination exercise.</p>"),
        "shaman" => bb("<p>A <b>shamanic journey</b> session: isochronic pulses at about 4.5 per second, the rate of a classic shamanic frame drum, guiding imagery of travelling to a \"lower world\".</p>",
            "<p>Drum-driven trance is found among Siberian (Evenki, the source of the word <i>šaman</i>), Sámi, Mongolian, Native American and many other peoples. Anthropologist <b>Michael Harner</b>'s <i>The Way of the Shaman</i> (<b>1980</b>) turned it into \"core shamanism\" for Western students, with steady drumming at about 4–4.5 beats per second.</p>",
            "<p>The practices may be prehistoric. Ethnographers recorded them from the 17th–19th centuries. Harner's method dates from 1980.</p>",
            "<p>Steady drumming at theta rates is said to \"drive\" the brain into theta and allow visionary journeys.</p>",
            "<p>Neher (1962) proposed \"auditory driving\", but his methods were criticised. Maxfield's 1990 dissertation reported theta increases at 4–4.5 beats per second. <b>Hove et al. (2016, <i>Cerebral Cortex</i>)</b> scanned experienced shamanic practitioners with fMRI and found drumming changed connectivity in the default-mode network and the posterior cingulate. This happened in trance, not when people simply listened. See also the Rhythmic Drumming Trance session.</p>"),
        "pineal" => bb("<p>A <b>third eye</b> meditation: attention between the eyebrows, a 6.3 Hz theta beat and a 963 Hz-range carrier.</p>",
            "<p>The <i>ajna</i> chakra, the \"command\" centre between the brows, appears in Hindu tantric texts. <b>René Descartes</b> (1640s) called the pineal gland the \"principal seat of the soul\". Madame Blavatsky's Theosophy (1888) linked it to the third eye. Modern internet culture added frequencies and \"decalcification\".</p>",
            "<p>The tantric sources are medieval. Descartes wrote in the 1600s and Theosophy in the 1800s. The frequency claims are 21st century.</p>",
            "<p>It is claimed to activate the pineal gland, release DMT and open psychic sight.</p>",
            "<p>Aaron Lerner isolated <b>melatonin</b> from the pineal gland in 1958; the gland regulates sleep and day–night rhythm. Rick Strassman's DMT studies (1990–95) speculated that the pineal gland produces DMT. <b>Dean et al. (2019, <i>Scientific Reports</i>)</b> found DMT-making enzymes and DMT in rat brains, at levels far too low to cause visions. No study shows that sound affects the pineal gland.</p>"),
        "om" => bb("<p>An <b>Om</b> meditation on 136.1 Hz, the pitch widely used for chanting Om in modern sound healing, with a 7.83 Hz beat.</p>",
            "<p><b>Om (Aum)</b> is the sacred syllable of Hinduism, Buddhism and Jainism. The <i>Chandogya Upanishad</i> and the <i>Mandukya Upanishad</i> (c. 800–200 BCE) explain it. The 136.1 Hz pitch comes from <b>Hans Cousto</b>'s <i>Die kosmische Oktave</i> (1978), which raises Earth's orbital year by 32 octaves.</p>",
            "<p>The syllable is over 2,500 years old. The 136.1 Hz tuning dates from 1978.</p>",
            "<p>Om is said to be the primordial sound of the universe, and the 136.1 Hz pitch to be grounding, opening the heart.</p>",
            "<p><b>Kalyani et al. (2011, <i>International Journal of Yoga</i>)</b> found in fMRI that chanting Om aloud deactivated limbic regions (amygdala, hippocampus), similar to vagus-nerve stimulation, compared with a \"ssss\" sound. It was a tiny study (n = 12). There is no research on the 136.1 Hz pitch itself.</p>"),
        "heal" => bb("<p>A long <b>delta</b> session for deep rest while falling asleep.</p>",
            "<p>Delta was discovered by W. Grey Walter in 1936. The link between deep sleep and growth hormone was shown from 1968 (Takahashi) and later by Van Cauter's group.</p>",
            "<p>The sleep science is from the 1930s–60s; delta beats have been sold since the 1990s.</p>",
            "<p>It is popularly claimed to speed up healing and boost growth hormone and the immune system.</p>",
            "<p>Deep sleep really does support physical restoration, memory consolidation and brain waste clearance (Xie et al., 2013, <i>Science</i>). Whether beats deepen sleep is weakly supported (Jirakittayakorn 2018, small). No research shows faster healing.</p>"),
        "vis" => bb("<p>A <b>visualisation / goal rehearsal</b> session: relaxed alpha-theta while you imagine reaching a goal in detail.</p>",
            "<p>Mental rehearsal comes from sports psychology (Soviet and East German Olympic programmes from the 1960s–70s) and from self-help such as the Silva Method (1966). It is also Gateway's \"patterning\".</p>",
            "<p>It was studied scientifically from the 1960s and in a meta-analysis in 1994.</p>",
            "<p>Visualising in a relaxed state is said to \"program\" the subconscious mind for success.</p>",
            "<p><b>Driskell, Copper &amp; Moran (1994, <i>Journal of Applied Psychology</i>)</b> meta-analysis: mental practice reliably improves performance, though less than physical practice. Imagining the <i>process</i> beats fantasising about the result (Oettingen's research shows pure positive fantasy can lower effort). The alpha-theta beat is a gentle extra.</p>"),
        "wake" => bb("<p>A <b>morning energiser</b> rising from alpha through beta to a short gamma burst.</p>",
            "<p>It builds on beta research (Lane 1998) and the popular use of fast beats as an \"audio coffee\".</p>",
            "<p>Beta beat studies date from the 1990s and later.</p>",
            "<p>It is said to wake you up, lift your energy and sharpen your focus.</p>",
            "<p>Lane et al. (1998) found beta beats improved vigilance. Any rhythmic, rising, brighter sound tends to raise arousal, and music research shows faster tempo increases alertness. The effect is modest; light and movement do more.</p>"),
        "study" => bb("<p><b>Study &amp; memory</b>: a short alpha settle, then SMR/low-beta for reading and learning.</p>",
            "<p>It grew out of the \"Mozart effect\" craze and beta/SMR beat studies.</p>",
            "<p>Rauscher, Shaw &amp; Ky published in <i>Nature</i> in 1993. Beta/memory studies followed in the 2010s.</p>",
            "<p>Beats or music are said to make you smarter and help you remember more.</p>",
            "<p>The Mozart effect turned out to be a small, short-lived boost from arousal and mood (<b>Pietschnig, Voracek &amp; Formann, 2010</b> meta-analysis: \"Mozart effect–Shmozart effect\"). Beta beats improved some memory measures (Beauchene 2016), but the 2019 meta-analysis found mixed memory effects. Listen <i>before</i> studying, not during difficult reading.</p>"),
        "create" => bb("<p><b>Creative flow</b>: swinging between 10 Hz alpha and the 7.5 Hz alpha-theta border, where imagery and loose associations increase.</p>",
            "<p>The Greens at the Menninger Foundation studied \"theta reverie\" in creative people (1970s). Mihaly Csikszentmihalyi described <i>flow</i> (1975 and 1990).</p>",
            "<p>The work dates from the 1970s–90s.</p>",
            "<p>The alpha-theta border is said to be the \"creative state\".</p>",
            "<p>Higher alpha is linked to creative ideas (Fink &amp; Benedek, 2014 review). <b>Egner &amp; Gruzelier (2003)</b> found alpha-theta neurofeedback improved music students' performance. Gamma beats changed creativity only in some people (Reedijk 2015). The evidence for beats here is thin.</p>"),
        "stress" => bb("<p><b>Stress release</b>: slow breathing does most of the work, with an alpha beat as a companion.</p>",
            "<p>It combines HRV breathing and Stanford's 2023 work on \"cyclic sighing\" (a double inhale, then a long exhale).</p>",
            "<p>The breathing techniques are traditional; the comparison study is from 2023.</p>",
            "<p>It is said to release stress quickly.</p>",
            &format!("<p><b>Balban et al. (2023, <i>Cell Reports Medicine</i>)</b> found that 5 minutes of daily cyclic sighing improved mood and lowered breathing rate more than mindfulness meditation did.</p>{}", BREATH_RES)),
        "insomnia" => bb("<p>An <b>insomnia wind-down</b>: a long, slow glide from alpha into deep delta over brown noise.</p>",
            "<p>It combines sleep-beat research with noise masking. White and pink noise machines have been used for sleep since the 1960s.</p>",
            "<p>Noise machines date from the 1960s and sleep-beat studies from the 2010s.</p>",
            "<p>It is said to help you fall asleep faster and sleep more deeply.</p>",
            "<p>Steady noise can reduce sleep disruption in noisy places (e.g. Ebben 2021 review, mixed quality). Pink-noise studies suggest small benefits (Zhou et al., 2012). A 2023 review of auditory stimulation for sleep found low certainty of evidence. For chronic insomnia, <b>CBT-I</b> is the best-supported treatment.</p>"),
        "headache" => bb("<p>A <b>tension-headache</b> relaxation session: low-volume alpha with body-relaxation cues.</p>",
            "<p>Relaxation therapy and biofeedback have been used for tension headache since the 1970s (Budzynski, Stoyva).</p>",
            "<p>Relaxation therapy and biofeedback date from the 1970s.</p>",
            "<p>Beats are said to relieve headaches.</p>",
            "<p>Relaxation training and EMG biofeedback have reasonable evidence for tension headaches (Nestoriuc et al., 2008 meta-analysis). There are no good trials of binaural beats for headaches. Keep the volume low. See a doctor about sudden, severe or unusual headaches.</p>"),
        "euphoria" => bb("<p>An uplifting <b>gratitude</b> meditation. Online sources market a \"euphoria\" beat; this session uses an ordinary alpha/beta mix.</p>",
            "<p>\"Digital drug\" sites such as <b>I-Doser</b> (around 2007) sold beat tracks named after recreational drugs, which prompted media panics.</p>",
            "<p>The trend started in the mid-2000s.</p>",
            "<p>Specific beats are claimed to release endorphins or mimic drugs.</p>",
            "<p><b>Barratt et al. (2018, <i>Drug and Alcohol Review</i>)</b> surveyed 30,000+ people for the Global Drug Survey. About 5% had tried \"digital drugs\", and most reported little effect. No study shows endorphin release. Gratitude practices have modest evidence for mood (Davis et al., 2016 meta-analysis).</p>"),
        "grounding" => bb("<p><b>Grounding</b>: the 7.83 Hz Schumann beat on Cousto's 194.18 Hz \"Earth day\" tone.</p>",
            "<p>It combines the Schumann resonance (Winfried Otto Schumann, 1952) with Cousto's Cosmic Octave (1978) and the modern \"earthing\" movement (Ober, 2010).</p>",
            "<p>The physics is from the 1950s; the wellness use is from the 1990s–2010s.</p>",
            "<p>Matching Earth's frequency is said to ground you, balance you and counter \"electrosmog\".</p>",
            "<p>The Schumann resonance is real but extremely weak and electromagnetic, not sound. There is no evidence that hearing a 7.83 Hz beat affects the body. Earthing studies are small and low quality. Enjoy it as a calming theta-alpha session.</p>"),

        // ================= Altered States =================
        "hypno" => i("<p><b>Self-hypnosis</b>: a guided induction (focused attention, progressive relaxation, a \"deepener\" counting down), then a suggestion phase and a safe return. Hypnosis is defined as <i>\"a state of consciousness involving focused attention and reduced peripheral awareness, characterized by an enhanced capacity for response to suggestion\"</i> (APA Division 30, 2014).</p>",
            "<p>It traces back to <b>Franz Anton Mesmer</b>'s \"animal magnetism\" in 1770s Vienna and Paris. A 1784 French royal commission that included <b>Benjamin Franklin</b> and Lavoisier concluded that the effects came from imagination. Scottish surgeon <b>James Braid</b> coined \"hypnotism\" (1841–43). Charcot, Bernheim, Freud and later Milton Erickson developed it, and Hilgard and Weitzenhoffer created the Stanford scales (1959).</p>",
            "<p>Mesmerism dates from the 1770s and the word \"hypnotism\" from the 1840s. The British and American Medical Associations approved it in 1955 and 1958, and modern brain imaging began in the 2000s.</p>",
            "<p>Hypnosis is claimed to relieve pain and anxiety, help with IBS and quitting smoking, and boost performance. Pop culture adds mind control and recovering memories. Those are false, and hypnosis can create false memories.</p>",
            "<p>Hypnosis is <b>one of the best-researched altered states</b>. <b>Thompson et al. (2019, <i>Neuroscience &amp; Biobehavioral Reviews</i>)</b> meta-analysed 85 controlled trials and found meaningful pain relief, especially in highly hypnotisable people. Gut-directed hypnotherapy for IBS has strong trial support (Whorwell, 1984 onward). <b>Jiang et al. (2017, <i>Cerebral Cortex</i>)</b> showed reduced dorsal anterior cingulate activity and changed prefrontal–insula connectivity. Rosendahl et al. (2024) umbrella review: benefits across many conditions, with moderate certainty. About 10–15% of people are highly hypnotisable. Recorded self-hypnosis is usually weaker than live sessions with a clinician.</p>"),
        "ganzfeld" => i("<p>The <b>Ganzfeld</b> (German for \"whole field\") is a mild sensory deprivation: an even, featureless field of view (here a <b>steady</b> red screen, never flickering) plus steady noise. After a few minutes the brain, lacking input, starts producing its own imagery, and some people see vivid pictures.</p>",
            "<p>Gestalt psychologist <b>Wolfgang Metzger</b> described it in <b>1930</b>: people staring into a uniform field reported fog, darkening and even temporary \"blankout\" of vision. In the 1970s parapsychologists (<b>Charles Honorton</b>, William Braud) adopted it for telepathy tests using halved ping-pong balls, red light and pink noise.</p>",
            "<p>The perception research began in 1930. Telepathy tests ran from the 1970s to the 2010s, and imagery research from the 2000s onward.</p>",
            "<p>Parapsychologists claim the Ganzfeld strengthens telepathy. Others use it for meditation and creativity because it produces hallucination-like imagery without drugs.</p>",
            "<p>Bem &amp; Honorton (1994, <i>Psychological Bulletin</i>) reported above-chance telepathy hit rates. Milton &amp; Wiseman's 1999 meta-analysis found <b>no effect</b>, and the debate continues with no accepted proof. The imagery is well established. <b>Wackermann, Pütz &amp; Allefeld (2008)</b> reviewed it. <b>Pütz et al. (2006)</b> found that about a third of people had vivid imagery, linked to EEG alpha changes. People who are more absorbed and suggestible see more (Schmidt &amp; Prein, 2019).</p>"),
        "drumtrance" => i("<p><b>Rhythmic drumming trance</b>: synthesised frame-drum hits at a steady theta rate (about 4 per second), with slight human timing variation. Listeners use them with imagery of a journey.</p>",
            "<p>Steady drumming for trance is found worldwide: Siberian and Mongolian shamans, Sámi <i>noaidi</i>, Afro-Brazilian Candomblé, West African and Native American ceremonies. In 1980 Michael Harner adapted it for Westerners in <i>The Way of the Shaman</i>.</p>",
            "<p>Drumming trance may be prehistoric. Lab research began in 1962 (Neher) and brain imaging in 2016 (Hove).</p>",
            "<p>Steady drumming is claimed to drive brainwaves toward theta, produce visionary journeys and give healing insight.</p>",
            "<p><b>Neher (1962)</b> claimed drumming caused \"auditory driving\" of the EEG, but his study was flawed. <b>Maxfield (1990)</b> found theta increases and imagery with 4–4.5 beats per second. <b>Hove et al. (2016, <i>Cerebral Cortex</i>)</b> studied 15 experienced practitioners: drumming-induced trance increased connectivity between the posterior cingulate, dorsal anterior cingulate and insula, and decoupled the auditory system. <b>Rhythm alone does not entrain the brain strongly</b>; expectation and practice matter.</p>"),
        "tlr" => i("<p><b>Targeted Lucidity Reactivation (TLR)</b>: before sleep you learn to link a sound cue with \"Am I dreaming?\" awareness. During sleep the cue replays (here, a soft three-note chime every few minutes) to bring that awareness into a dream.</p>",
            "<p>It builds on LaBerge's Stanford lucid-dream research (1980s) and <b>targeted memory reactivation</b> (Rudoy, Voss, Westerberg &amp; Paller, 2009, <i>Science</i>). Ken Paller's lab at <b>Northwestern University</b> (Carr, Konkoly, Mallett) developed the method.</p>",
            "<p>Lucid dream verification dates from 1975–81 and cue reactivation from 2009. The TLR studies are from 2020 to the 2020s.</p>",
            "<p>A trained cue is said to make lucid dreams more likely.</p>",
            "<p><b>Carr et al. (2020, <i>Consciousness and Cognition</i>)</b> reported that 50% of participants had lucid dreams in a nap with TLR, compared with 17% without cues. <b>Konkoly et al. (2021, <i>Current Biology</i>)</b> showed two-way communication with lucid dreamers (answering maths questions with eye signals). Field studies at home showed smaller effects. This version is not linked to sleep tracking, so it works best on mornings when you go back to sleep, with cues timed for REM.</p>"),
        "swspulses" => i("<p><b>Slow-wave sleep pulses</b>: brief 50 ms bursts of pink noise at about <b>0.8 Hz</b>, the rhythm of deep-sleep slow oscillations. Keep the volume low.</p>",
            "<p>The method comes from <b>Jan Born</b>'s lab in Tübingen. Hongviet Ngo used pink-noise clicks timed to the peaks of the sleeper's own slow waves (\"closed-loop\" stimulation). Northwestern (Papalambros, Zee) and Tononi's lab in Wisconsin extended it.</p>",
            "<p>Open-loop rhythmic sound was tried from 2010 (Ngo); the key studies are from 2013–2017.</p>",
            "<p>The pulses are claimed to deepen slow-wave sleep and improve overnight memory.</p>",
            "<p><b>Ngo et al. (2013, <i>Neuron</i>)</b> found that closed-loop pulses increased slow oscillations and improved word-pair memory. <b>Papalambros et al. (2017, <i>Frontiers in Human Neuroscience</i>)</b> reported benefits in older adults. Important: Ngo et al. (2013, <i>J Neurosci</i>) found that <b>open-loop</b> pulses (like this app's fixed 0.8 Hz rhythm) were less effective, and badly timed pulses can disturb sleep. Consumer headbands that follow your brainwaves do this properly. This version is a simplified experiment.</p>"),
        "kirtan" => i("<p><b>Kirtan Kriya</b>: a 12-minute singing meditation. You chant <b>Sa-Ta-Na-Ma</b> (birth, life, death, rebirth) while touching each fingertip to the thumb in turn. The chant moves from aloud to whisper to silent and back. The app sounds one bell note per syllable, one per second.</p>",
            "<p>It comes from <b>Kundalini Yoga</b> as taught by <b>Yogi Bhajan</b>, who brought it to the U.S. in <b>1968</b>, drawing on the Sikh and Indian yogic traditions. The <b>Alzheimer's Research &amp; Prevention Foundation</b> (Dharma Singh Khalsa) promoted it for brain health from the 1990s.</p>",
            "<p>The mantra roots are traditional. The practice came to the West in 1968, and the clinical studies date from 2010 to 2017.</p>",
            "<p>It is claimed to improve memory, reduce stress and support brain health, possibly slowing cognitive decline.</p>",
            "<p><b>Newberg et al. (2010, <i>Journal of Alzheimer's Disease</i>)</b> used SPECT scans and found 8 weeks of practice increased frontal blood flow and improved memory in people with memory loss. <b>Lavretsky et al. (2013, <i>International Journal of Geriatric Psychiatry</i>)</b> found that dementia caregivers had less depression and 43% higher telomerase activity than with relaxation music. <b>Innes et al. (2017, <i>Journal of Alzheimer's Disease</i>)</b> found memory and mood improvements in older adults with memory complaints. The studies are small, but the results agree.</p>"),
        "bowls" => i("<p>A <b>singing-bowl sound bath</b>. Synthesised bowls are struck in turn; each has paired, slightly detuned partials that beat acoustically at a theta rate, like real hand-hammered bowls.</p>",
            "<p>Metal bowls from the Himalayan region (Nepal, Tibet, northern India) were used as household and ritual objects. Historians debate how much \"singing\" ritual use is old. Bowls became famous in the West after the Tibetan diaspora (from <b>1959</b>), and Western sound healers built \"sound baths\" around them in the 1970s–80s.</p>",
            "<p>The bowls are centuries old. Sound-healing use dates from the 1970s, and research from 2017 onward.</p>",
            "<p>Bowls are said to relax you, balance the chakras and \"heal at the cellular level\" through vibration.</p>",
            "<p><b>Goldsby et al. (2017, <i>Journal of Evidence-Based Complementary &amp; Alternative Medicine</i>)</b> found that 62 people had less tension, anxiety and depression after a sound bath (no control group). <b>Stanhope &amp; Weinstein (2020, <i>Complementary Therapies in Medicine</i>)</b> systematic review: some promising results, but low quality; benefits may come from relaxation and expectation. Small controlled studies found lower blood pressure and heart rate than with silence. There is no evidence for chakra or cellular claims.</p>"),
        "mantra6" => i("<p><b>Mantra / rosary rhythm</b>: you recite a short prayer or mantra once per slow breath, timed by a bell every 10 seconds, giving about <b>6 breaths per minute</b>.</p>",
            "<p>Repetitive prayer appears in almost every tradition. The Catholic <b>rosary</b> took shape in the 12th–15th centuries (popularised by Alan de la Roche in the 1460s). Hindu and Buddhist <i>japa</i> with <i>mala</i> beads is older still. Orthodox Christians use the \"Jesus Prayer\" on prayer ropes, and Sufis practise <i>dhikr</i>.</p>",
            "<p>The traditions are centuries old; the physiology study is from 2001.</p>",
            "<p>Repetitive prayer is said to calm the mind and bring peace and a sense of the sacred.</p>",
            &format!("<p><b>Bernardi et al. (2001, <i>BMJ</i>)</b> found that reciting the Ave Maria in Latin or the mantra <i>om mani padme hum</i> naturally slowed breathing to about 6 per minute. This increased heart-rate variability and baroreflex sensitivity. The authors suggested this is partly <i>why</i> these practices spread.</p>{}", BREATH_RES)),
        "rain" => i("<p>A <b>natural rain soundscape</b>: synthesised rainfall with droplets and a faint 10 Hz beat. Nature sounds are among the most-studied relaxing sounds.</p>",
            "<p>It builds on environmental psychology: Rachel and Stephen Kaplan's <b>Attention Restoration Theory</b> (1989) and Roger Ulrich's stress-recovery theory (1991), along with modern fMRI research.</p>",
            "<p>The theories date from 1989–91; the experiments run from 2010 to 2021.</p>",
            "<p>Nature sounds are said to restore attention, reduce stress and help sleep.</p>",
            "<p><b>Gould van Praag et al. (2017, <i>Scientific Reports</i>)</b> found that natural sounds shifted default-mode connectivity toward outward attention and raised parasympathetic activity compared with artificial sounds. <b>Alvarsson, Wiens &amp; Nilsson (2010)</b> found faster stress recovery with nature sounds. <b>Buxton et al. (2021, <i>PNAS</i>)</b> meta-analysis: natural sounds improved health outcomes and reduced stress and annoyance. Water sounds were best for mood.</p>"),
        "omdrone" => i("<p><b>Om chanting</b>: chant Om aloud on each long out-breath along with a 136.1 Hz monaural drone. Saying the sound aloud is what was tested, not just hearing it.</p>",
            "<p>Om (Aum) is described in the <i>Mandukya Upanishad</i> as the whole of existence (waking, dreaming, deep sleep and the \"fourth\", <i>turiya</i>). It is chanted in Hindu, Buddhist and Jain practice. The 136.1 Hz pitch comes from Cousto (1978).</p>",
            "<p>Upanishadic texts date from about 800–200 BCE. The pitch is from 1978 and the fMRI study from 2011.</p>",
            "<p>Om is claimed to calm the mind, open the heart and connect you with universal vibration.</p>",
            "<p><b>Kalyani et al. (2011, <i>International Journal of Yoga</i>)</b>, n = 12: chanting Om aloud deactivated limbic regions (amygdala, hippocampus, parahippocampal gyrus, insula, orbitofrontal and anterior cingulate cortex) compared with a \"ssss\" control. The pattern resembles vagus-nerve stimulation. Gurjar et al. (2009) found that Om meditation slowed breathing. Long exhalation alone also calms the body.</p>"),

        // ================= Stacks / journeys / tunings =================
        "schstack" => {
            let mut x = schumann(0);
            x.what = format!("<p>All <b>five Schumann modes</b> (7.83, 14.3, 20.8, 27.3, 33.8 Hz) play together as binaural beats, each on its own carrier.</p>{}", x.what);
            x
        }
        "soljourney" => {
            let mut x = solfeggio(0);
            x.what = format!("<p>A climb through <b>all nine Solfeggio tones</b> (174 → 963 Hz), three minutes each, with a steady 6 Hz theta beat.</p>{}", x.what);
            x
        }
        "chjourney" => {
            let mut x = chakra(0, "Chakra Journey");
            x.what = format!("<p>Rises through all <b>seven chakras</b> from root to crown, four minutes each. The carrier glides upward (396 → 963 Hz) and the beat rises from 4 to 12 Hz.</p>{}", x.what);
            x
        }
        "t432" => i("<p>A session tuned to <b>A = 432 Hz</b> instead of the standard 440 Hz, which is about a third of a semitone lower, with an 8 Hz alpha beat.</p>",
            "<p>Tuning standards varied widely before the 20th century: A ranged from about 400 to 460 Hz. France fixed A = 435 Hz in 1859 (<i>diapason normal</i>). <b>Giuseppe Verdi</b> argued in 1884 for A = 432 for Italian orchestras. Modern \"432 Hz\" advocacy grew online in the 2000s–2010s, sometimes with false claims that 440 was a Nazi or Rockefeller plot.</p>",
            "<p>Verdi's proposal was in 1884. The internet movement began in the 2000s, and the pilot study is from 2019.</p>",
            "<p>432 Hz is claimed to be \"mathematically consistent with the universe\", heal DNA and resonate with the heart and with nature.</p>",
            "<p><b>Calamassi &amp; Pomponi (2019, <i>Explore</i>)</b>: a double-blind pilot with 33 people found slightly lower heart rate with 432 Hz music than with 440 Hz, and no significant differences in other measures. Most listeners cannot reliably tell the two tunings apart. The cosmic claims rest on numerology, not science.</p>"),
        "t440" => i("<p>The <b>international standard concert pitch</b>: A above middle C at 440 Hz, offered here for comparison with 432 Hz.</p>",
            "<p>In 1939 an international conference in London recommended A = 440 Hz. It was confirmed in 1955 and adopted as <b>ISO 16</b> in <b>1975</b>. It settled the \"pitch wars\" that had seen orchestras drift sharper and sharper for brilliance.</p>",
            "<p>It was recommended in 1939 and standardised in 1955 and 1975.</p>",
            "<p>440 is a practical standard that lets instruments play together. Conspiracy claims that it is \"disharmonious\" have no basis.</p>",
            "<p>No credible research shows 440 Hz is harmful. Calamassi &amp; Pomponi (2019) found only small differences compared with 432 Hz. Many orchestras actually tune slightly higher (442–443 Hz).</p>"),
        "t256" => i("<p><b>Scientific pitch</b>: middle C = 256 Hz (2<sup>8</sup>), which puts every C at a power of two (1, 2, 4 … 256, 512 Hz), with a 10 Hz alpha beat.</p>",
            "<p>French physicist <b>Joseph Sauveur</b> proposed it in <b>1713</b> because the numbers are neat. It was used in physics and acoustics textbooks and for tuning forks. It is sometimes called \"Verdi tuning\" because C = 256 corresponds to roughly A = 430.5 in equal temperament, close to Verdi's 432.</p>",
            "<p>It was proposed in 1713 and used in science through the 19th–20th centuries. It had a political revival in the 1980s (the Schiller Institute).</p>",
            "<p>Advocates say it is the \"natural\" tuning, better for singers' voices.</p>",
            "<p>There is no research showing health or listening benefits. The choice of 256 is mathematically neat because of binary octaves, but nature has no preferred tuning.</p>"),
        _ => return None,
    })
}

// ---------------------------------------------------------------- Brainwave library
fn library(hz: f64, name: &str) -> Info {
    let (band, what, origin, era, claims, research): (&str, &str, &str, &str, &str, &str) = if hz < 0.75 {
        ("Epsilon", "Very slow \"infraslow\" activity below 1 Hz.",
         "<p>\"Epsilon\" is a term from popular brainwave and neurofeedback culture. Scientists instead speak of <b>infraslow oscillations</b> (ISO, below 0.1–1 Hz), recorded with special DC-coupled EEG. Vanhatalo et al. (2004, <i>PNAS</i>) showed that these rhythms organise faster activity.</p>",
         "<p>Infraslow EEG was studied from the 1950s–70s (Aladjalova in the USSR) and in modern work from 2004. The name \"epsilon\" came from 2000s pop sources.</p>",
         "<p>Epsilon is popularly linked to extremely deep meditation, yogic \"suspended animation\" and spiritual insight.</p>",
         "<p>Infraslow fluctuations are real and linked to attention and resting-state networks. No study shows that a 0.5 Hz beat can be perceived well or produce these states. At this rate the \"beat\" is a very slow swell. <b>Speculative.</b></p>")
    } else if hz < 4.0 {
        ("Delta", "Delta (0.5–4 Hz) is the large, slow wave of deep, dreamless sleep (N3), and also of infancy and anaesthesia.",
         "<p><b>W. Grey Walter</b> named delta waves in 1936 while locating brain tumours. In 1937 Loomis, Harvey &amp; Hobart linked slow waves to deep sleep. Dement &amp; Kleitman described the sleep cycle in 1957.</p>",
         "<p>Delta was discovered in the 1930s and the sleep cycle mapped in the 1950s. Delta beats have been sold since Monroe's 1975 patent.</p>",
         "<p>Popular claims: deep healing, growth-hormone release, immunity, \"unity consciousness\" and pain relief at certain rates (e.g. 2.5 Hz).</p>",
         "<p>Deep sleep really does support tissue repair, growth hormone, memory and brain waste clearance. <b>Jirakittayakorn &amp; Wongsawat (2018)</b> found a 3 Hz beat lengthened N3 sleep in a small study. Other specific delta-rate claims (e.g. 2.5 Hz for pain) are untested.</p>")
    } else if hz < 8.0 {
        ("Theta", "Theta (4–8 Hz) appears in drowsiness and the hypnagogic \"twilight\", deep meditation, REM sleep and memory work.",
         "<p>Grey Walter's group named theta in the 1940s. Hippocampal theta (key to memory and navigation) was studied in animals from the 1950s (Green &amp; Arduini, 1954). Frontal-midline theta in meditation was found from the 1970s. The Greens at Menninger studied \"theta reverie\".</p>",
         "<p>Theta was named in the 1940s, studied in meditation from the 1960s–70s, and used in beat protocols from the 2010s.</p>",
         "<p>Popular claims: intuition, creativity, access to the subconscious, astral projection (6.3 Hz), shamanic journeys (4.5 Hz) and vivid imagery.</p>",
         "<p>Frontal theta rises in focused meditation (Cahn &amp; Polich, 2006). <b>Jirakittayakorn &amp; Wongsawat (2017)</b> found a 6 Hz beat increased frontal theta. Specific claims such as \"intuition\" or \"OBE\" frequencies have no research behind them.</p>")
    } else if hz < 13.0 {
        ("Alpha", "Alpha (8–13 Hz) is the rhythm of relaxed, eyes-closed wakefulness, strongest at the back of the head.",
         "<p><b>Hans Berger</b> discovered alpha in 1924 and published it in 1929; it was the first human brain rhythm recorded. Adrian &amp; Matthews confirmed it in 1934. Joe Kamiya's 1960s <b>alpha biofeedback</b> launched the alpha-relaxation movement and the Silva Method (1966).</p>",
         "<p>Alpha was discovered in the 1920s and became popular in the 1960s–70s. Alpha beats have been sold from the 1990s.</p>",
         "<p>Popular claims: relaxation, positive mood, \"super-learning\" (8 Hz), mind–body healing (10.5 Hz) and centring (12 Hz).</p>",
         "<p>Alpha increases with relaxation and with meditation. The 2019 meta-analysis supports modest calming effects. EEG \"entrainment\" by alpha beats is inconsistent across studies. Claims tied to exact frequencies (10.5, 12 Hz) are untested folklore.</p>")
    } else if hz < 15.0 {
        ("SMR / low beta", "SMR (about 12–15 Hz) is the sensorimotor rhythm over the motor strip: calm, still, alert.",
         "<p><b>Barry Sterman</b> (UCLA) discovered SMR in cats in the late 1960s. Cats trained in SMR resisted seizures caused by rocket fuel in a NASA project. <b>Joel Lubar</b> applied SMR neurofeedback to ADHD in 1976.</p>",
         "<p>SMR was discovered in the late 1960s and became a clinical protocol in the 1970s.</p>",
         "<p>Popular claims: calm focus, attention and better sleep.</p>",
         "<p>SMR <b>neurofeedback</b> has decent evidence for ADHD and epilepsy (Arns et al., 2009; Tan et al., 2009). <b>Binaural beats are not neurofeedback</b>, and beats for ADHD showed no clear benefit (Kennel et al., 2010).</p>")
    } else if hz < 30.0 {
        ("Beta", "Beta (13–30 Hz) is the active, alert, thinking rhythm of everyday waking.",
         "<p>Hans Berger named beta in the 1920s. Beta beats were tested by Lane et al. (1998) and Beauchene et al. (2016).</p>",
         "<p>Beta was discovered in the 1920s and tested as beats from the 1990s.</p>",
         "<p>Popular claims: focus, energy, alertness and motivation. High beta (25–30 Hz) is said to be \"stimulating\".</p>",
         "<p><b>Lane et al. (1998)</b>: 16 and 24 Hz beats improved vigilance and mood compared with theta/delta. <b>Beauchene et al. (2016)</b>: 15 Hz improved working memory. Excess high beta is linked to anxiety and rumination, so use the 30 Hz session sparingly.</p>")
    } else if hz < 90.0 {
        ("Gamma", "Gamma (30–90 Hz) is fast activity linked to attention, perception, memory and binding sensations together.",
         "<p>Jasper &amp; Andrews coined \"gamma\" in 1938. <b>Galambos et al. (1981)</b> found the 40 Hz auditory steady-state response. <b>Gray &amp; Singer (1989, <i>Nature</i>)</b> proposed gamma \"binding\". <b>Lutz et al. (2004)</b> found high gamma in meditating monks.</p>",
         "<p>The term dates from 1938, binding theory from 1989, and 40 Hz therapy research from 2016.</p>",
         "<p>Popular claims: peak awareness, insight, compassion, cognitive enhancement and Alzheimer's protection (40 Hz).</p>",
         "<p><b>40 Hz</b> is by far the most-studied: MIT mouse studies (Iaccarino 2016; Martorell 2019) and early human trials, though replication is debated (Soula 2023). Above about 30–40 Hz, binaural beats become hard to hear, so rates of 50 Hz and above use <b>isochronic</b> pulses, which produce a strong steady-state response. Other gamma frequencies have very little research.</p>")
    } else if hz < 150.0 {
        ("Hypergamma", "\"Hypergamma\" (about 100 Hz) is a label from popular sound-healing sources, not a standard EEG band.",
         "<p>The terms \"hypergamma\" and \"lambda\" spread through sound-healing CDs (e.g. Jeffrey Thompson's Center for Neuroacoustic Research) and websites in the 2000s. In neuroscience, very fast activity is called <b>high-frequency oscillations</b> or hippocampal <b>ripples</b> (about 80–200 Hz, Buzsáki et al., 1992). Ripples help consolidate memory.</p>",
         "<p>Hippocampal ripples were described in 1992. The popular labels date from the 2000s.</p>",
         "<p>It is claimed to bring higher consciousness, ESP and transcendental states.</p>",
         "<p>No research supports these claims. Scalp EEG barely picks up such fast activity. The 100 Hz isochronic pulse will produce a measurable auditory steady-state response, but nothing more is known. <b>Speculative.</b></p>")
    } else {
        ("Lambda", "\"Lambda\" (about 200 Hz) is a popular-culture band label. (In real EEG, \"lambda waves\" are a different thing: occipital waves during visual scanning.)",
         "<p>The name came from 2000s sound-healing and brainwave websites. In neuroscience, 150–250 Hz activity is hippocampal \"ripples\" and \"fast ripples\" (the latter can mark epilepsy). Clinical EEG uses \"lambda waves\" for unrelated visual responses.</p>",
         "<p>The popular labels date from the 2000s.</p>",
         "<p>It is claimed to bring mystical experience, \"oneness\" and states above gamma.</p>",
         "<p>No research. At 200 Hz the pulses blend into a buzzy tone rather than a rhythm. <b>Speculative.</b></p>")
    };
    i(format!("<p><b>{}</b>: {} This session glides from a gentle tuning tone to <b>{} Hz</b> over two minutes, then holds it for 30 minutes.</p>{}", name, what, hz, BB_WHAT),
      format!("<p><b>The {} band.</b></p>{}{}", band, origin, BB_HISTORY),
      era, claims, format!("{}{}", research, BB_RESEARCH))
}

// ---------------------------------------------------------------- Schumann
fn schumann(mode: u32) -> Info {
    let hz = [7.83, 14.3, 20.8, 27.3, 33.8].get(mode.wrapping_sub(1) as usize).copied();
    let what = match hz {
        Some(h) => format!("<p><b>Schumann mode {}</b>, about <b>{} Hz</b>, played as a binaural beat on a 194.18 Hz carrier. The Schumann resonances are global electromagnetic standing waves in the cavity between Earth's surface and the ionosphere, about 60–90 km up.</p>", mode, h),
        None => "<p>The Schumann resonances are global electromagnetic standing waves in the cavity between Earth's surface and the ionosphere, about 60–90 km up.</p>".to_string(),
    };
    i(format!("{}<p>About 50 lightning strikes every second around the world keep \"ringing\" this cavity like a bell. The fundamental (about 7.83 Hz) corresponds to a wavelength equal to Earth's circumference. Higher modes sit near 14.3, 20.8, 27.3 and 33.8 Hz. They are <b>electromagnetic</b>, not sound, and extremely weak (picotesla).</p>", what),
      "<p>German physicist <b>Winfried Otto Schumann</b> predicted the resonances mathematically in <b>1952</b> at the Technical University of Munich. <b>Balser &amp; Wagner</b> first measured them clearly in <b>1960</b>. Schumann's colleague Herbert König noticed that the fundamental is close to human alpha/theta EEG. Rütger Wever at the Max Planck Institute ran bunker studies (1960s–70s), reporting that shielding people from fields disturbed their daily rhythms, which made the frequency famous.</p>",
      "<p>They were predicted in 1952, measured in 1960 and adopted by wellness culture from the 1990s.</p>",
      "<p>Popular claims: the \"heartbeat of the Earth\"; listening grounds and heals you; NASA puts Schumann generators in spacecraft (not documented); the frequency is \"rising\" toward 13 Hz or more as humanity \"awakens\".</p>",
      "<p>The resonances are real and closely monitored. They do <b>not</b> rise long-term: the \"rising Schumann\" charts show different measurements, such as spikes in intensity. Wever's bunker results were never firmly linked to Schumann frequencies specifically. Some correlational studies link geomagnetic activity to heart-rate variability (e.g. HeartMath's Global Coherence studies, not independently replicated). There is <b>no evidence</b> that hearing a 7.83 Hz <i>binaural beat</i> interacts with the electromagnetic resonance. These sessions are labelled <b>speculative</b>.</p>")
}

// ---------------------------------------------------------------- Solfeggio
fn solfeggio(hz: u32) -> Info {
    let (meaning, extra) = match hz {
        174 => ("foundation, natural anaesthetic", "<p>174 Hz is one of three tones (with 285 and 963) <b>added later</b> to the original six, so it has no link even to the hymn story.</p>"),
        285 => ("tissue restoration, energy fields", "<p>285 Hz is one of the later additions to the original six.</p>"),
        396 => ("<i>UT</i>: liberating guilt and fear", "<p>Paired with the syllable <b>UT</b> (\"<i>Ut</i> queant laxis\").</p>"),
        417 => ("<i>RE</i>: undoing situations, facilitating change", "<p>Paired with <b>RE</b> (\"<i>Re</i>sonare fibris\").</p>"),
        528 => ("<i>MI</i>: transformation and \"DNA repair\", the \"love frequency\"", "<p>Paired with <b>MI</b> (\"<i>Mi</i>ra gestorum\"). The 528 Hz \"DNA repair\" claim came from Leonard Horowitz and biochemist Glen Rein's informal experiments. Akimoto et al. (2018, <i>Health</i>) studied 9 people listening to music at 528 Hz and reported reduced stress markers (cortisol). It was a tiny study in a low-impact journal. No study shows DNA repair.</p>"),
        639 => ("<i>FA</i>: connecting and relationships", "<p>Paired with <b>FA</b> (\"<i>Fa</i>muli tuorum\").</p>"),
        741 => ("<i>SOL</i>: awakening intuition, expression, \"detox\"", "<p>Paired with <b>SOL</b> (\"<i>Sol</i>ve polluti\").</p>"),
        852 => ("<i>LA</i>: returning to spiritual order", "<p>Paired with <b>LA</b> (\"<i>La</i>bii reatum\").</p>"),
        963 => ("crown, \"oneness\", pineal activation", "<p>963 Hz is a later addition and is often linked online to the \"pineal gland\" and \"God frequency\".</p>"),
        _ => ("", ""),
    };
    let what = if hz > 0 {
        format!("<p>A <b>{} Hz</b> carrier tone, one of the modern \"Solfeggio frequencies\", with a 6 Hz theta beat. Traditional meaning: <b>{}</b>.</p>{}", hz, meaning, extra)
    } else { String::new() };
    i(format!("{}<p>The Solfeggio set is nine tones (174, 285, 396, 417, 528, 639, 741, 852, 963 Hz) promoted in sound healing.</p>{}", what, BB_WHAT),
      "<p>The name borrows from real music history. The syllables <i>ut–re–mi–fa–sol–la</i> come from the medieval hymn <b>\"Ut queant laxis\"</b> to St John the Baptist (text attributed to <b>Paul the Deacon</b>, 8th century). Around <b>1025</b>, monk <b>Guido of Arezzo</b> used its syllables to teach singing. The <i>frequencies</i>, though, were invented much later. Around the 1970s, Dr <b>Joseph Puleo</b> said he found them by applying numerology (\"Pythagorean\" digit reduction) to verse numbers in the Book of Numbers. <b>Leonard Horowitz</b> popularised them in <i>Healing Codes for the Biological Apocalypse</i> (<b>1999</b>). Medieval music had no fixed pitch standard, so the hymn never used these frequencies.</p>",
      "<p>The hymn is from the 8th century and Guido's syllables from about 1025. The frequencies were derived in the 1970s, popularised in 1999, and spread on YouTube in the 2010s.</p>",
      "<p>Claims: these were \"lost ancient sacred frequencies\" used in Gregorian chant; they heal DNA (528), dissolve fear (396), cleanse toxins (741) and awaken the pineal gland (963).</p>",
      "<p>No historical evidence supports an ancient origin, and Gregorian chant had no fixed Hz tuning. Research on Solfeggio tones is very limited: a few small studies (e.g. Akimoto 2018 on 528 Hz, n = 9) and no replicated health effects. Any calming effect is likely due to relaxation and expectation. This session gives the tone a gentle theta beat. Labelled <b>traditional</b> (modern tradition).</p>")
}

// ---------------------------------------------------------------- Chakras
fn chakra(hz: u32, name: &str) -> Info {
    let detail = match hz {
        396 => "<p><b>Muladhara</b> (\"root support\"), at the base of the spine. Element: earth. Colour: red. Seed syllable (bija): <i>LAM</i>. Linked to stability, safety and survival.</p>",
        417 => "<p><b>Svadhisthana</b> (\"one's own seat\"), in the lower abdomen. Element: water. Colour: orange. Bija: <i>VAM</i>. Linked to emotion, pleasure and creativity.</p>",
        528 => "<p><b>Manipura</b> (\"city of jewels\"), at the navel or solar plexus. Element: fire. Colour: yellow. Bija: <i>RAM</i>. Linked to will, energy and digestion.</p>",
        639 => "<p><b>Anahata</b> (\"unstruck\"; refers to the unstruck sound), at the heart. Element: air. Colour: green. Bija: <i>YAM</i>. Linked to love and compassion. This session uses the 7.83 Hz Schumann beat.</p>",
        741 => "<p><b>Vishuddha</b> (\"especially pure\"), at the throat. Element: space/ether. Colour: blue. Bija: <i>HAM</i>. Linked to expression and communication.</p>",
        852 => "<p><b>Ajna</b> (\"command\"), between the eyebrows: the \"third eye\". Colour: indigo. Bija: <i>OM</i>. Linked to intuition and insight.</p>",
        963 => "<p><b>Sahasrara</b> (\"thousand-petalled\"), at or above the crown. Colour: violet/white. Linked to pure consciousness and liberation.</p>",
        _ => "<p>The seven chakras: Muladhara (root), Svadhisthana (sacral), Manipura (solar plexus), Anahata (heart), Vishuddha (throat), Ajna (third eye) and Sahasrara (crown).</p>",
    };
    let what = if hz > 0 { format!("<p><b>{}</b>, played on a {} Hz carrier (the common Solfeggio pairing) with a matching beat.</p>{}", name, hz, detail) } else { detail.to_string() };
    i(format!("{}<p><b>Chakras</b> (Sanskrit <i>cakra</i>, \"wheel\") are centres of subtle energy in yogic and tantric anatomy, strung along a central channel (<i>sushumna nadi</i>) through which <i>kundalini</i> energy is said to rise.</p>{}", what, BB_WHAT),
      "<p>Chakras appear in medieval Hindu and Buddhist <b>tantric</b> texts. Different texts give different numbers (4, 5, 6, 7 or more). The familiar seven-chakra system comes mainly from the <i>Ṣaṭ-cakra-nirūpaṇa</i> (\"Description of the Six Centres\", <b>1577</b>, by Pūrṇānanda). <b>Sir John Woodroffe</b> (Arthur Avalon) translated it as <i>The Serpent Power</i> (<b>1919</b>), bringing it to the West. The rainbow colours and the psychological meanings were added in the 20th century (C.W. Leadbeater 1927; Western New Age writers from the 1970s). The pairing of chakras with specific <b>Solfeggio frequencies</b> is a 21st-century internet invention.</p>",
      "<p>The tantric texts are from about the 8th–16th centuries, with the key text from 1577. The system reached the West in 1919, and the colour and frequency pairings date from the 1970s–2010s.</p>",
      "<p>Claims: each chakra can become \"blocked\" or \"balanced\"; specific tones open or align it, leading to physical and emotional healing.</p>",
      "<p>There is no anatomical or scientific evidence that chakras exist as physical structures, and no research on chakra frequencies. Some researchers note that the chakra locations roughly match major nerve plexuses. Meditation focused on body areas (interoception) has real calming benefits. Labelled <b>traditional</b>.</p>")
}

// ---------------------------------------------------------------- Cosmic Octave
fn planet(hz: f64, name: &str) -> Info {
    let body = name.split(" — ").next().unwrap_or(name);
    let period = match body {
        "Earth Year / Om" => "Earth's orbit: 365.24 days (tropical year), raised 32 octaves → 136.10 Hz, about C♯. This is the famous \"Om\" tuning used for chanting and by many gong and tuning-fork makers.",
        "Earth Day" => "Earth's rotation: one 24-hour day, raised 24 octaves → 194.18 Hz, about G. It is said to be energising and dynamic.",
        "Sun" => "Not an orbit. Cousto based it on the combined planetary periods/\"Sun tone\" (126.22 Hz, about B).",
        "Moon (synodic)" => "The synodic month from new moon to new moon, 29.53 days, raised 29 octaves → 210.42 Hz, about G♯. Linked to emotion, cycles and sexuality.",
        "Mercury" => "Orbit: 87.97 days → 141.27 Hz, about C♯/D. Linked to communication and intellect.",
        "Venus" => "Orbit: 224.70 days → 221.23 Hz, about A. Linked to love, beauty and harmony.",
        "Mars" => "Orbit: 686.98 days → 144.72 Hz, about D. Linked to will, energy and assertiveness.",
        "Jupiter" => "Orbit: 11.86 years → 183.58 Hz, about F♯. Linked to growth, expansion and wisdom.",
        "Saturn" => "Orbit: 29.46 years → 147.85 Hz, about D. Linked to structure, discipline and time.",
        "Uranus" => "Orbit: 84.01 years → 207.36 Hz, about G♯. Linked to originality and sudden change.",
        "Neptune" => "Orbit: 164.8 years → 211.44 Hz, about G♯. Linked to dreams, intuition and the unconscious.",
        "Pluto" => "Orbit: about 248 years → 140.25 Hz, about C♯. Linked to transformation. (The IAU reclassified Pluto as a dwarf planet in 2006.)",
        _ => "",
    };
    i(format!("<p><b>{} — {} Hz</b>. {} Played here with a 6 Hz theta beat.</p><p>The <b>Cosmic Octave</b> method takes a real astronomical period (such as a year or an orbit), converts it to a frequency (1 ÷ period), then doubles that frequency again and again (raising it by octaves) until it falls into the audible range. Because octave doubling keeps the \"pitch class\", the tone is said to \"resonate\" with the planet.</p>{}", body, hz, period, BB_WHAT),
      "<p>Swiss mathematician and musicologist <b>Hans Cousto</b> developed the method in <i>Die kosmische Oktave</i> (<b>1978</b>; English <i>The Cosmic Octave</i>, 1988). It updates the ancient idea of the <b>\"music of the spheres\"</b> (Pythagoras, c. 500 BCE; Johannes Kepler's <i>Harmonices Mundi</i>, 1619). Cousto's tones are used in Planetware tuning forks, planetary gongs (Don Conreaux, Paiste) and sound therapy.</p>",
      "<p>The idea goes back to about 500 BCE (Pythagoras) and 1619 (Kepler). Cousto's calculations are from 1978, and they spread through sound healing from the 1980s.</p>",
      "<p>Claims: each planetary tone carries that planet's astrological qualities and helps you \"tune in\" to cosmic rhythms. 136.1 Hz is said to be deeply relaxing and heart-centred.</p>",
      "<p>The <b>maths is real</b>: the frequencies are correct octave transpositions of real periods. But there is <b>no evidence</b> that these tones have planet-specific effects, and astrological meanings are not science. Choosing octaves (rather than other intervals) is an arbitrary convention. Labelled <b>traditional</b>; enjoy it as a pleasant tonal meditation.</p>")
}

// ------------------------------------------------- Sound Systems Library & Astral Projection
const OBE_CLAIMS: &str = "<p>Practitioners claim that consciousness can leave the physical body, travel, and visit \"astral planes\" or other people. Some say the experience proves life after death. None of this has been shown in controlled tests.</p>";
const OBE_RESEARCH: &str = "<p>Roughly 10% of people report at least one out-of-body experience. The brain basis is well studied: <b>Blanke et al. (2002, <i>Nature</i>)</b> triggered OBE-like feelings by stimulating the right temporo-parietal junction; <b>Ehrsson (2007, <i>Science</i>)</b> produced them with a camera-and-headset illusion. Induced OBEs mostly happen at the edge of REM sleep, overlapping with <b>lucid dreams and sleep paralysis</b>, which explains the vibrations, buzzing and \"pulled out\" feelings. No study has shown that people bring back verifiable information. Sleep paralysis is harmless, but if it frightens you, stop the practice.</p>";
const OBE_ERA: &str = "<p>Soul-travel beliefs are ancient. \"Astral projection\" is a 19th-century Theosophical term; modern techniques date from 1929 (Muldoon) to the 2010s (Raduga).</p>";

fn extra(id: &str) -> Option<Info> {
    let sys = |what: &str, origin: &str, era: &str, claims: &str, research: &str| Some(i(what, origin, era, claims, research));
    let obe = |what: &str, origin: &str| Some(i(format!("{}{}", what, BB_WHAT), origin, OBE_ERA, OBE_CLAIMS, OBE_RESEARCH));
    match id {
        "sys_binaural" => sys(BB_WHAT,
            "<p>Discovered by Prussian physicist <b>Heinrich Wilhelm Dove</b> in 1839. Biophysicist <b>Gerald Oster</b> revived it in \"Auditory Beats in the Brain\" (<i>Scientific American</i>, 1973), and Robert Monroe built Hemi-Sync on it in the 1970s.</p>",
            "<p>1839 discovery; mainstream wellness use since the 1970s; app boom since the 2010s.</p>",
            "<p>Claimed to steer brainwaves to any state: sleep, focus, meditation, even out-of-body experiences.</p>",
            "<p>Real but modest. A meta-analysis (Garcia-Argibay et al., 2019) found small-to-moderate effects on anxiety, attention and memory. EEG evidence that the whole brain \"follows\" the beat is mixed (Ingendoh et al., 2023 systematic review).</p>"),
        "sys_monaural" => sys("<p><b>Monaural beats</b> mix two close tones into one signal, so the beat is a real rise and fall in loudness that each ear hears directly. They work on speakers.</p>",
            "<p>Beats between two tones were described by Joseph Sauveur around 1700 and are how musicians tune instruments. Their use for brain entrainment grew alongside binaural beats from the 1970s.</p>",
            "<p>Known for centuries; used for entrainment since the late 20th century.</p>",
            "<p>Claimed to entrain the brain more strongly than binaural beats.</p>",
            "<p>EEG studies do find a stronger auditory steady-state response to monaural than binaural beats (Schwarz &amp; Taylor, 2005), but a stronger response in the hearing cortex doesn't by itself mean a different mood or state. Behavioural studies are few.</p>"),
        "sys_isochronic" => sys("<p><b>Isochronic tones</b> are a single tone turned on and off at an even rate (iso = equal, chronos = time). Soft edges avoid clicks. They need no headphones.</p>",
            "<p>The term is credited to <b>Arturo Manns</b> (1980s). It builds on Neher's 1960s work on rhythmic sound and the 1981 discovery of the 40 Hz auditory steady-state response (Galambos et al.).</p>",
            "<p>1980s onward; very common in YouTube and app tracks since the 2010s.</p>",
            "<p>Claimed to be the \"strongest\" entrainment method.</p>",
            "<p>Clear, repeatable brain responses at the pulse rate, strongest near 40 Hz. The 40 Hz \"GENUS\" studies in mice (Martorell et al., 2019, <i>Cell</i>) and early human trials use this kind of sound. Evidence for mood or focus effects in people is still limited.</p>"),
        "sys_am" => sys("<p><b>Amplitude-modulated (AM) tones</b> rise and fall smoothly in loudness like a sine wave, instead of switching on and off. It's the gentlest pulsed method and works on speakers.</p>",
            "<p>AM is basic signal processing (AM radio, 1900s). Hearing scientists have used AM tones to measure brain responses since the 1980s. Companies such as Brain.fm later built \"functional music\" around it.</p>",
            "<p>Lab tool since the 1980s; consumer \"focus music\" since the late 2010s.</p>",
            "<p>Claimed to boost focus, relaxation or sleep depending on the modulation rate.</p>",
            "<p><b>Woods et al. (2024, <i>Communications Biology</i>)</b> found that beta-rate amplitude-modulated music improved sustained attention and changed brain connectivity, with larger benefits for people with more ADHD symptoms. That study was funded by a music company and needs independent replication.</p>"),
        "sys_hybrid" => sys("<p><b>Hybrid beats</b> combine a binaural pair (different tone in each ear) with isochronic pulsing at the same rate.</p>",
            "<p>A commercial innovation from the 2000s–2010s download and YouTube era of brainwave tracks.</p>",
            "<p>About 2005 to the present.</p>",
            "<p>Claimed to combine the best of both methods for stronger entrainment.</p>",
            "<p>No study has compared hybrid beats with either method alone. Expect roughly what each method gives: a clear pulse plus a binaural wobble.</p>"),
        "sys_bilateral" => sys("<p><b>Bilateral stimulation</b> alternates a sound (or touch, or eye movement) between the left and right side, usually about once a second.</p>",
            "<p>Psychologist <b>Francine Shapiro</b> developed EMDR (Eye Movement Desensitization and Reprocessing) in 1987 after noticing that eye movements eased distressing thoughts. Therapists later used alternating tones and taps as alternatives.</p>",
            "<p>1987 onward; recommended for PTSD by the WHO (2013).</p>",
            "<p>Claimed to calm the nervous system and help process memories; some apps sell it for anxiety relief.</p>",
            "<p>EMDR as a full therapy is effective for PTSD. Whether the left–right part itself matters is debated; reviews suggest it adds a modest benefit by taxing working memory, which makes memories less vivid (Lee &amp; Cuijpers, 2013). Bilateral audio on its own is <b>not therapy</b>; if you are working with trauma, use a qualified therapist.</p>"),
        "sys_spatial" => sys("<p><b>Spatial (\"8D\") audio</b> moves a sound around your head by changing its balance between the ears. Here it glides smoothly side to side.</p>",
            "<p>Stereo panning dates to the 1930s (Alan Blumlein). Rotating \"8D audio\" remixes went viral on YouTube and WhatsApp in 2018.</p>",
            "<p>2018 internet trend; built on 1930s stereo technology.</p>",
            "<p>Claimed to feel immersive, hypnotic or even mind-altering.</p>",
            "<p>No clinical research on 8D audio as entrainment. Moving sound does capture attention strongly, which can make relaxation more absorbing. Keep the volume low; some people find it dizzying.</p>"),
        "sys_layered" => Some(gw("<p><b>Layered beats</b> stack several binaural pairs on different carriers so the brain hears a chord of rates at once.</p>", "")),
        "sys_descend" => Some(bb("<p><b>Descending layered carriers</b>: binaural beats that step down from alpha through theta to delta, the format used by programs such as Holosync.</p>",
            "<p>Bill Harris's <b>Centerpointe Research Institute</b> launched Holosync in 1989, building on Monroe's layered-tone idea and selling a graded series of \"levels\".</p>",
            "<p>1989 to the present.</p>",
            "<p>Claimed to give \"deep meditation at the touch of a button\" and to raise your threshold for stress over months.</p>",
            "<p>No independent peer-reviewed trials of Holosync exist.</p>")),
        "sys_noise" => sys("<p><b>Coloured noise</b> is random sound shaped by frequency: white (flat), pink (softer highs, like rain) and brown (deep rumble, like surf). No beats.</p>",
            "<p>The colour names come from light: white light has every colour equally. Pink noise (\"1/f noise\") was described in electronics in the 1920s–1930s (Johnson, Schottky).</p>",
            "<p>Sound-masking machines since the 1960s; \"brown noise\" went viral on TikTok in 2022.</p>",
            "<p>Claimed to help sleep, focus and ADHD.</p>",
            "<p>Steady noise masks disruptive sounds. Reviews find low-quality, mixed evidence for sleep (Riedy et al., 2021). Some studies find white noise helps attention in people with ADHD symptoms (Söderlund et al., 2007). Timed pink-noise pulses during deep sleep are a separate, better-supported method (see Slow-Wave Sleep Pulses).</p>"),
        "sys_drum" => sys("<p><b>Rhythmic auditory driving</b>: a steady drum, rattle or chant, typically 4–7 beats per second, used to induce trance.</p>",
            "<p>Found in shamanic and ritual practice on every inhabited continent. Andrew Neher (1962) proposed that drumming drives brain rhythms; Michael Harner's <i>The Way of the Shaman</i> (1980) popularised 4–4.5 beats per second in the West.</p>",
            "<p>Prehistoric origins; studied scientifically since the 1960s.</p>",
            "<p>Claimed to open a doorway to the spirit world or the \"lower world\" of shamanic journeying.</p>",
            "<p>Hove et al. (2016, <i>Cerebral Cortex</i>) found that drum trance changed brain network connectivity in experienced practitioners. Neher's original EEG claims were poorly controlled. The experience is real; spirit-world claims cannot be tested.</p>"),

        "ap_monroe" => obe("<p>Monroe's <b>vibrational state and roll-out</b> method over Gateway-style layered tones.</p>",
            "<p>Radio executive <b>Robert Monroe</b> began having spontaneous OBEs in 1958 and described this method in <i>Journeys Out of the Body</i> (1971), which made the term \"out-of-body experience\" popular. He later founded the Monroe Institute.</p>"),
        "ap_rope" => obe("<p>The <b>rope technique</b>: climb an imagined rope hand over hand to create a pulling sensation, here on hybrid beats.</p>",
            "<p>Australian author <b>Robert Bruce</b> described it in <i>Astral Dynamics</i> (1999). It is one of the most-shared beginner techniques online.</p>"),
        "ap_muldoon" => obe("<p>The <b>target technique</b>: memorise a place in detail and rehearse being there with every sense.</p>",
            "<p><b>Sylvan Muldoon</b> and psychical researcher <b>Hereward Carrington</b> wrote <i>The Projection of the Astral Body</i> (1929), the first practical manual in English. Muldoon reported spontaneous projections from age 12.</p>"),
        "ap_phase" => obe("<p>The <b>phase</b> method: lie still immediately after waking and try to \"separate\" before you fully wake up.</p>",
            "<p>Russian author <b>Michael Raduga</b> coined \"the phase\" (2008) to cover OBEs, lucid dreams and sleep paralysis as one state, and founded the Out-of-Body Experience Research Center.</p>"),
        "ap_wbtb" => obe("<p><b>Wake-back-to-bed</b> with an intention and soft chime cues, aimed at REM-sleep projections.</p>",
            "<p>WBTB comes from lucid-dream research by <b>Stephen LaBerge</b> at Stanford (1980s). It is among the most effective induction methods in controlled studies (Stumbrys et al., 2012; Aspy et al., 2017).</p>"),
        "ap_theta_gamma" => obe("<p>A <b>theta–gamma stack</b>: 6.3 Hz \"astral\" theta plus a quiet 40 Hz gamma beat.</p>",
            "<p>A recipe from YouTube and forum culture of the 2010s. The 6.3 Hz figure has no traceable scientific source. The gamma layer is inspired by findings that lucid dreaming involves more 40 Hz activity (Voss et al., 2009).</p>"),
        "ap_float" => obe("<p>A <b>floating</b> visualisation over slow spatial (8D) panning of the tone.</p>",
            "<p>The floating and lifting imagery is shared by Muldoon, Monroe and Bruce. Slow spatial movement of sound adds a sense of drift; the combination is original to this app.</p>"),
        "ap_bilateral" => obe("<p><b>Bilateral calming</b> first, then body-asleep imagery.</p>",
            "<p>Uses EMDR-style alternating tones (Shapiro, 1987) for relaxation, then the \"mind awake, body asleep\" approach from Monroe's work. The combination is original to this app.</p>"),
        "ap_hypnotic" => obe("<p>A <b>hypnotic induction</b> followed by suggestions of leaving the body and seeing it from above.</p>",
            "<p>Hypnotically suggested out-of-body experiences were studied by <b>Enrico Facco</b> and colleagues at the University of Padua (2019). Hypnosis itself goes back to Franz Mesmer in the 1770s and James Braid, who coined the word in 1841.</p>"),
        "ap_imagined" => obe("<p>An <b>imagined out-of-body viewpoint</b>: picturing your body from above, without hypnosis.</p>",
            "<p>The comparison condition of Facco et al. (2019). Imagined changes of viewpoint are also used in neuroscience studies of self-location (e.g. Arzy et al., 2006, <i>Journal of Neuroscience</i>).</p>"),
        "ap_kinesthetic" => obe("<p><b>Kinesthetic floating</b>: feeling, rather than picturing, a second body moving above your own.</p>",
            "<p>From the fMRI case study by <b>Andra Smith and Claude Messier</b> at the University of Ottawa (2014) of a student who could bring on such experiences at will.</p>"),
        "ap_cycling" => obe("<p><b>Technique cycling</b>: short alternating attempts (phantom wiggling, watching images, rotation, listening in) in the first moments after waking.</p>",
            "<p>Taught by <b>Michael Raduga</b> in his books on \"the phase\" (from 2008) and studied by his Out-of-Body Experience Research Center.</p>"),
        "ap_lab" => obe("<p>A <b>laboratory-style induction</b>: progressive relaxation, a Ganzfeld, then imagery of leaving the body.</p>",
            "<p>Parapsychology laboratories of the 1970s, notably <b>John Palmer</b>'s studies, combined relaxation, Ganzfeld stimulation and suggestion to try to produce OBEs on demand.</p>"),
        "ap_swing" => obe("<p>The <b>swing technique</b>: balance (vestibular) imagery of swinging higher until you float free, over side-to-side panning sound.</p>",
            "<p>A modern technique shared in OBE books and forums. Its rationale is the strong link between balance signals and OBEs found by neurologists such as <b>Olaf Blanke</b>.</p>"),
        "ap_christos" => obe("<p>The <b>Christos technique</b>: foot and forehead massage, imagined stretching and expansion, then a flight to a new place.</p>",
            "<p>Described by Australian author <b>G. M. Glaskin</b> in <i>Windows of the Mind</i> (1974) and later books, and widely used in personal-growth circles in the 1970s and 1980s.</p>"),
        "ap_rising" => obe("<p><b>Rising on the planes</b>: a vertical ascent through layers of light, from the Western esoteric tradition.</p>",
            "<p>Taught in the <b>Hermetic Order of the Golden Dawn</b> (1888) and later by writers such as Israel Regardie and Dion Fortune.</p>"),
        "ap_tattva" => obe("<p><b>Tattva vision</b>: gazing at an elemental symbol and using its afterimage as a doorway for guided imagery.</p>",
            "<p>The five tattvas come from Hindu Samkhya philosophy; the Golden Dawn adapted them in the 1890s as \"tattva cards\" for what it called scrying in the spirit vision.</p>"),
        "ap_phowa" => obe("<p><b>Phowa</b>: visualising awareness rising through the central channel and out through the crown.</p>",
            "<p>One of the <b>Six Yogas of Naropa</b>, an 11th-century Indian Buddhist system carried to Tibet by Marpa, and taught today in several Tibetan schools.</p>"),
        "ap_dreamyoga" => obe("<p><b>Dream yoga</b>: a bedtime practice for recognising dreams as dreams.</p>",
            "<p>Also one of the Six Yogas of Naropa, with parallel teachings in the Bön tradition, brought to Western readers by Tenzin Wangyal Rinpoche (1998) and others.</p>"),
        "ap_upper" => obe("<p>A drum-driven <b>upper-world journey</b> in the core-shamanism style.</p>",
            "<p>Anthropologist <b>Michael Harner</b> distilled \"core shamanism\" from many cultures in <i>The Way of the Shaman</i> (1980) and founded the Foundation for Shamanic Studies.</p>"),
        "ap_middle" => obe("<p>A drum-driven <b>middle-world journey</b> across the landscape of this world.</p>",
            "<p>From <b>Michael Harner</b>'s core shamanism (1980), which describes lower, middle and upper worlds reached by drumming.</p>"),
        "ap_ssild" => obe("<p><b>SSILD</b>: cycling attention through sight, sound and body sensation after an early-morning awakening, then falling asleep.</p>",
            "<p>Developed online by a lucid dreamer known as CosmicIron around 2013, and tested in the International Lucid Dream Induction Study (Aspy et al., 2020).</p>"),
        "ap_ground" => sys("<p>A short <b>grounding</b> session that brings you back from deep, dreamy states to full alertness.</p>",
            "<p>\"Grounding\" after inner journeys appears in shamanic, yogic and Western esoteric traditions; Monroe's tapes end with a counted return for the same reason.</p>",
            "<p>Traditional; modern form from 20th-century guided meditation.</p>",
            "<p>Claimed to \"anchor\" energy back into the body after astral work.</p>",
            "<p>Physiologically, a short period of rising alertness helps clear sleep inertia (grogginess), which can last up to 30 minutes after deep relaxation or naps (Tassi &amp; Muzet, 2000). Don't drive straight after deep sessions.</p>"),
        _ => None,
    }
}

// ------------------------------------------------- Spiritual & Mystical Experiences
const MY_CLAIMS: &str = "<p>Practitioners describe union with God or the absolute, deep peace, insight, love and a sense of the sacred, and some traditions describe visions, inner sounds or light. Each tradition interprets these experiences in its own terms; science cannot test what, if anything, they reveal beyond the mind.</p>";
const MY_RESEARCH: &str = "<p>Mystical experiences can be measured: Hood's Mysticism Scale (1975) and the Mystical Experience Questionnaire capture unity, sacredness, a sense of truth, positive mood, going beyond time and space, and ineffability. Brain-imaging studies of prayer and meditation (Newberg and colleagues, from 2001) find changes in attention and self-location areas, and long-term meditators show distinctive patterns (Lutz et al., 2004; Josipovic et al., 2012). Evidence that any single practice reliably produces mystical states is limited, and most studies are small. Intense practice can occasionally bring up distress; stop if anything feels overwhelming.</p>";

fn my_info(id: &str) -> Option<Info> {
    let m = |what: &str, origin: &str, era: &str| Some(i(what, origin, era, MY_CLAIMS, MY_RESEARCH));
    match id {
        "my_centering" => m("<p><b>Centering prayer</b>: silent Christian contemplative prayer with a sacred word.</p>",
            "<p>Developed at St Joseph's Abbey in Spencer, Massachusetts, by Trappist monks William Meninger, Basil Pennington and <b>Thomas Keating</b>, drawing on the 14th-century <i>Cloud of Unknowing</i>; spread worldwide by Keating's Contemplative Outreach.</p>",
            "<p>1970s, with roots in medieval Christian mysticism.</p>"),
        "my_tibetan" => m("<p><b>Image visualisation</b>, a core method of Tibetan Vajrayana meditation.</p>",
            "<p>Tibetan Buddhism, where practitioners visualise a deity or symbol in detail and then dissolve it into emptiness; studied by Andrew Newberg's group at the University of Pennsylvania.</p>",
            "<p>Practised for over a thousand years; brain-scanned in 2001.</p>"),
        "my_vocal" => m("<p><b>Free vocalisation</b> in the style of glossolalia (speaking in tongues).</p>",
            "<p>Glossolalia appears in the New Testament (Acts 2) and is central to Pentecostal and charismatic Christianity, which grew from the Azusa Street revival in Los Angeles (1906); similar ecstatic vocalisation appears in many religions.</p>",
            "<p>Ancient; the modern Pentecostal form since 1906.</p>"),
        "my_jhana" => m("<p><b>Jhāna</b>: progressive absorption, from rapture to equanimity.</p>",
            "<p>Described in the Pali Canon of Theravada Buddhism, and taught in the West by teachers such as Ajahn Brahm, Leigh Brasington and Shaila Catherine.</p>",
            "<p>About 2,500 years old; brain-scanned in 2013.</p>"),
        "my_nondual" => m("<p><b>Nondual awareness</b>: resting as awareness itself, without dividing experience into subject and object.</p>",
            "<p>Central to Tibetan Dzogchen and Mahamudra, Indian Advaita Vedanta, and Chan and Zen; studied at New York University by Zoran Josipovic.</p>",
            "<p>Ancient; neuroscience studies since about 2010.</p>"),
        "my_inquiry" => m("<p><b>Self-inquiry</b>: tracing every thought back to the sense of I.</p>",
            "<p>Taught by <b>Ramana Maharshi</b> at Tiruvannamalai in South India from the early 1900s, and later by teachers such as Nisargadatta Maharaj and Papaji.</p>",
            "<p>Early 20th century, with roots in ancient Advaita Vedanta.</p>"),
        "my_mantra" => m("<p><b>Effortless mantra meditation</b>: a silent sound, thought without concentration.</p>",
            "<p>Mantra meditation is ancient in Hinduism and Buddhism. <b>Maharishi Mahesh Yogi</b> introduced Transcendental Meditation worldwide from 1958, and the Beatles' visit in 1968 made it famous.</p>",
            "<p>Ancient roots; TM since 1958.</p>"),
        "my_float" => Some(i("<p>A <b>dry floatation-REST</b>: darkness, stillness and imagined weightlessness.</p>",
            "<p>Neuroscientist <b>John C. Lilly</b> built the first isolation tank at the US National Institute of Mental Health in 1954. Commercial float tanks appeared in the 1970s, and the Laureate Institute for Brain Research in Tulsa leads much of the modern research.</p>",
            "<p>1954 onward.</p>",
            "<p>Floating is claimed to relieve stress, pain and anxiety, and to open the way to deep meditation, out-of-body and mystical states.</p>",
            "<p>Feinstein et al. (2018, <i>PLOS ONE</i>) found large short-term drops in anxiety after a single float in 50 people with anxiety and stress-related disorders. This dry version is an imitation: lying in a dark, quiet room also reduces stimulation, but without the weightlessness of a real float. Never try it in a bath or any water.</p>")),
        "my_arc" => m("<p>A <b>sober mystical-experience arc</b> modelled on the structure of psychedelic research sessions, without any drug.</p>",
            "<p>The Johns Hopkins psilocybin studies (Roland Griffiths, William Richards and colleagues, from 2006) built on Walter Pahnke's 1962 \"Good Friday Experiment\" and on W. T. Stace's <i>Mysticism and Philosophy</i> (1960).</p>",
            "<p>Ideas from the 1960s; modern research since 2006.</p>"),
        "my_dhikr" => m("<p><b>Dhikr</b>: rhythmic remembrance of God with breath and phrase.</p>",
            "<p>Rooted in the Qur'an's call to remember God often, and developed by Sufi orders, from the silent dhikr of the Naqshbandi to the spoken and sung dhikr of the Qadiri and Mevlevi orders.</p>",
            "<p>Since the early centuries of Islam.</p>"),
        "my_jesus" => m("<p>The <b>Jesus Prayer</b>: a short prayer joined to the breath.</p>",
            "<p>From the Desert Fathers of 4th- and 5th-century Egypt; developed by the hesychast monks of Mount Athos, defended by Gregory Palamas in the 14th century, and collected in the <i>Philokalia</i> (1782).</p>",
            "<p>Since about the 5th century.</p>"),
        "my_nada" => m("<p><b>Nāda yoga</b>: following sound inward to the inner sound.</p>",
            "<p>Described in medieval hatha yoga texts such as the <i>Hatha Yoga Pradipika</i> (15th century) and the <i>Nāda Bindu Upanishad</i>; related to the \"sound current\" teachings of Sant Mat.</p>",
            "<p>Medieval India, with older roots.</p>"),
        "my_vipassana" => m("<p><b>Vipassana</b> body sweeping with equanimity.</p>",
            "<p>From the Burmese Theravada revival: Ledi Sayadaw, then Sayagyi U Ba Khin, who taught <b>S. N. Goenka</b>; Goenka's ten-day courses began in India in 1969.</p>",
            "<p>Ancient roots; the modern lay form from the 20th century.</p>"),
        "my_psychomanteum" => Some(i("<p>The <b>psychomanteum</b>: mirror gazing in a dim room to reconnect with someone who has died.</p>",
            "<p><b>Raymond Moody</b>, known for <i>Life After Life</i> (1975) on near-death experiences, modelled it on the ancient Greek oracles of the dead and described it in <i>Reunions</i> (1993).</p>",
            "<p>Ancient roots; the modern form since 1993.</p>",
            "<p>People report seeing, hearing or sensing the person who died, and feeling comforted. Whether these are contacts or vivid imagery is a matter of belief.</p>",
            "<p>Hastings et al. (2002, <i>Omega</i>) reported reduced grief in bereaved volunteers after a psychomanteum session. Gazing at a mirror in dim light produces illusory faces and distortions on its own (Caputo, 2010, <i>Perception</i>), which helps explain the visions. Grief can be intense; if you are struggling, talk to someone you trust or a bereavement service.</p>")),
        "my_orbit" => m("<p>The <b>microcosmic orbit</b>: circulating attention and breath along the spine and the front of the body.</p>",
            "<p>Taoist inner alchemy (<i>neidan</i>), developed over more than a thousand years in China, and taught in the West by Mantak Chia from the 1980s.</p>",
            "<p>Over a thousand years old.</p>"),
        "my_hitbodedut" => m("<p><b>Hitbodedut</b>: daily secluded, spontaneous prayer in your own words.</p>",
            "<p>Taught by <b>Rebbe Nachman of Breslov</b> (1772–1810), great-grandson of the Baal Shem Tov, the founder of Hasidism.</p>",
            "<p>Late 18th and early 19th century.</p>"),
        "my_guide" => m("<p>A guided-imagery journey to meet an <b>inner guide</b>.</p>",
            "<p>Spirit guides appear in shamanic, Spiritualist and New Age traditions; Carl Jung developed active imagination in the 1910s; the inner advisor technique comes from Martin Rossman and David Bresler's guided-imagery work.</p>",
            "<p>Ancient; the modern psychological form since the 1970s and 1980s.</p>"),
        _ => None,
    }
}

// ------------------------------------------------- Light Journeys (flicker)
// ---------------------------------------------------------------- world traditions (v1.7)
const CHANT_RESEARCH: &str = "<p>Repeating a mantra quiets widespread areas of the cortex, including the default-mode network linked to mind-wandering (<b>Berkovich-Ohana et al., 2015, <i>Brain and Behavior</i></b>). Rosary prayer and yoga mantras recited at about six breaths a minute slowed breathing and raised heart-rate variability and baroreflex sensitivity (<b>Bernardi et al., 2001, <i>BMJ</i></b>). A small fMRI study of chanting Om found reduced activity in emotion-related regions (Kalyani et al., 2011, <i>International Journal of Yoga</i>). Most studies are small, and none can test the spiritual claims.</p>";
const RITE_CLAIMS: &str = "<p>Practitioners describe journeys to other worlds, meetings with spirit helpers, healing, guidance and a strong sense of energy or presence. Each tradition explains these experiences in its own terms; science cannot test what, if anything, they contact beyond the mind.</p>";
const RITE_RESEARCH: &str = "<p>Steady drumming at about four beats a second changes how brain networks connect: in experienced practitioners it coupled the default-mode hub with control regions and turned attention inward (<b>Hove et al., 2016, <i>Cerebral Cortex</i></b>). Repetitive ritual can also lower anxiety (<b>Brooks et al., 2016, <i>Organizational Behavior and Human Decision Processes</i></b>). Claims about particular postures, spirits or healing powers have not been tested in controlled studies. Trance can bring up strong emotions; stop if anything feels overwhelming.</p>";
const WEST_CLAIMS: &str = "<p>Western esoteric traditions teach that the human being mirrors the cosmos, that the mind can rise through levels of reality toward the divine, and that ritual, sound and visualisation awaken subtle forces. These are claims of the traditions, not scientific findings.</p>";
const WEST_RESEARCH: &str = "<p>There is little direct research on these practices. Their ingredients are better understood: vivid mental imagery uses much of the same brain machinery as seeing (<b>Pearson et al., 2015, <i>Trends in Cognitive Sciences</i></b>), slow chanting and intoning slow the breath, and ritual can lower anxiety (<b>Brooks et al., 2016</b>). The anthropologist Tanya Luhrmann studied how modern magicians learn to have vivid inner experiences (<i>Persuasions of the Witch's Craft</i>, 1989).</p>";
const TRANCE_CLAIMS: &str = "<p>Hypnotists say trance makes the mind more open to helpful suggestion, which can ease pain, anxiety and unwanted habits and help with sleep and performance. Stage shows suggest that people lose control under hypnosis; in fact you stay aware and can come out at any time.</p>";
const TRANCE_RESEARCH: &str = "<p>Hypnosis is among the best-studied altered states. Brain imaging shows reduced activity in a hub of the salience network and changed connections between control, body-awareness and default-mode regions (<b>Jiang et al., 2017, <i>Cerebral Cortex</i></b>), and a meta-analysis of 85 controlled trials found that hypnotic suggestion reliably reduced pain (<b>Thompson et al., 2019, <i>Neuroscience &amp; Biobehavioral Reviews</i></b>). People differ: roughly one in ten is highly hypnotisable, and most people respond better with practice.</p>";
const QI_CLAIMS: &str = "<p>Qigong teachers say these practices gather and circulate <i>qi</i>, the life energy, along the body's channels, and link each sound or posture to the organs and elements of traditional Chinese medicine. These ideas come from traditional theory, not from anatomy or physiology.</p>";
const QI_RESEARCH: &str = "<p>Qigong and tai chi have been tested in many trials. A large review found benefits for blood pressure, balance, physical function, mood and quality of life (<b>Jahnke et al., 2010, <i>American Journal of Health Promotion</i></b>), though many studies are small or of mixed quality. Slow breathing with long out-breaths raises heart-rate variability (Zaccaro et al., 2018, <i>Frontiers in Human Neuroscience</i>).</p>";
const GAZE_RESEARCH: &str = "<p>Fixed gazing at a still image has well-understood effects: an afterimage when you close your eyes, and <b>Troxler fading</b> (first described in 1804), in which unchanging parts of the view fade from awareness. Long, steady fixation can also bring Ganzfeld-like imagery. Claims that kasina practice leads to deep absorption or special powers have not been tested in controlled studies.</p>";

fn med_info(id: &str) -> Option<Info> {
    let m = |what: &str, origin: &str, era: &str| Some(i(what, origin, era, MY_CLAIMS, MY_RESEARCH));
    let ch = |what: &str, origin: &str, era: &str| Some(i(what, origin, era, MY_CLAIMS, CHANT_RESEARCH));
    let rite = |what: &str, origin: &str, era: &str| Some(i(what, origin, era, RITE_CLAIMS, RITE_RESEARCH));
    let west = |what: &str, origin: &str, era: &str| Some(i(what, origin, era, WEST_CLAIMS, WEST_RESEARCH));
    let trance = |what: &str, origin: &str, era: &str| Some(i(what, origin, era, TRANCE_CLAIMS, TRANCE_RESEARCH));
    let qi = |what: &str, origin: &str, era: &str| Some(i(what, origin, era, QI_CLAIMS, QI_RESEARCH));
    match id {
        // ---- Christian mysticism
        "cm_lectio" => m("<p><b>Lectio divina</b> (\"divine reading\"): slow, prayerful reading of scripture in four steps: reading, meditation, prayer and contemplation.</p>",
            "<p>A monastic practice since the Desert Fathers and the Rule of St Benedict (6th century). The Carthusian prior <b>Guigo II</b> named its four rungs in <i>The Ladder of Monks</i>.</p>",
            "<p>Benedictine roots in the 6th century; Guigo's ladder about 1150.</p>"),
        "cm_maranatha" => m("<p><b>Christian Meditation</b>: silently repeating one prayer word, <i>Maranatha</i> (\"Come, Lord\"), from beginning to end.</p>",
            "<p>The Benedictine monk <b>John Main</b> (1926–1982) found the practice in the <i>Conferences</i> of John Cassian (about 420), who learned it from the Desert Fathers of Egypt. The World Community for Christian Meditation continues his teaching.</p>",
            "<p>4th–5th century; revived from the 1970s.</p>"),
        "cm_ignatian" => m("<p><b>Ignatian contemplation</b>: imagining yourself into a Gospel scene with all your senses.</p>",
            "<p>From the <i>Spiritual Exercises</i> of <b>Ignatius of Loyola</b> (1491–1556), founder of the Jesuits, who asked retreatants to \"see the place\" and enter the story.</p>",
            "<p>Written 1522–1548; still the basis of Jesuit retreats.</p>"),
        "cm_castle" => m("<p>Walking inward through the mansions of the soul to the <b>prayer of quiet</b>.</p>",
            "<p><b>Teresa of Ávila</b> (1515–1582), Carmelite reformer and Doctor of the Church, pictured the soul as a crystal castle with God dwelling at its centre in <i>The Interior Castle</i>.</p>",
            "<p>Written in 1577.</p>"),
        "cm_cloud" => m("<p>Contemplative prayer beyond images and ideas, reaching toward God with one short word.</p>",
            "<p><i>The Cloud of Unknowing</i>, written in Middle English by an anonymous English mystic, teaches piercing the cloud between us and God with a \"sharp dart of longing love\".</p>",
            "<p>Late 14th century.</p>"),
        "cm_taize" => ch("<p>Short sung prayers repeated many times, alternating with silence.</p>",
            "<p>The ecumenical <b>Taizé</b> community in Burgundy, France, founded by Brother Roger Schutz in 1940. Its simple meditative chants are sung by young pilgrims from around the world.</p>",
            "<p>Community since 1940; the chants since the 1970s.</p>"),
        "cm_quaker" => m("<p><b>Expectant waiting</b>: gathered silence, open to the \"inner light\".</p>",
            "<p>The Religious Society of Friends (Quakers), founded by <b>George Fox</b> in England, worships in silence without clergy, and anyone may speak when moved.</p>",
            "<p>Since the 1650s.</p>"),
        "cm_vigil" => Some(i("<p>Prayer or quiet reflection in the natural wakeful spell between a \"first\" and a \"second\" sleep.</p>",
            "<p>Monastic night offices (Rule of St Benedict, chapter 8) and the pre-industrial habit of segmented sleep documented by the historian A. Roger Ekirch.</p>",
            "<p>Medieval monastic practice; common in households until the 19th century.</p>", MY_CLAIMS,
            "<p><b>Ekirch (2001, <i>American Historical Review</i>)</b> gathered hundreds of historical references to first and second sleep. <b>Wehr (1992, <i>Journal of Sleep Research</i>)</b> found that volunteers given 14 hours of darkness a night drifted into two sleeps with a calm waking spell between them, marked by high prolactin, which he compared to the state of meditation. Whether modern sleepers gain from segmented sleep is not established. For insomnia, cognitive behavioural therapy for insomnia (CBT-I) is the best-supported treatment.</p>")),
        "cm_bestill" => m("<p>A breath prayer on Psalm 46:10, \"Be still, and know that I am God\", shortened word by word until only \"Be\" remains.</p>",
            "<p>A modern contemplative practice built on an ancient verse. Short breath prayers echo the Jesus Prayer of the Eastern Church.</p>",
            "<p>Ancient psalm; the shortening method is modern.</p>"),
        "cm_visio" => m("<p><b>Visio divina</b>: praying with an icon or sacred image.</p>",
            "<p>Eastern Orthodox icon veneration, affirmed by the Second Council of Nicaea in 787, joined to the four steps of lectio divina.</p>",
            "<p>Icons since early Christianity; visio divina as a named method since the late 20th century.</p>"),
        "cm_rosary" => ch("<p>The <b>Rosary</b>: decades of Hail Marys on beads while contemplating the mysteries of Christ's life.</p>",
            "<p>A Catholic devotion that grew from medieval psalters of 150 prayers. Pope Pius V set its form in 1569, and Pope John Paul II added the Luminous Mysteries in 2002.</p>",
            "<p>Medieval; standard form since 1569.</p>"),
        // ---- Tibetan Buddhism and Bön
        "tb_shamatha" => Some(i("<p><b>Shamatha</b> (calm abiding): training attention to be both stable and clear.</p>",
            "<p>Indian Buddhist texts by Asanga and Kamalashila map its nine stages; it is taught in every Tibetan school, and in the West by <b>B. Alan Wallace</b>.</p>",
            "<p>Indian sources from the 4th–8th centuries.</p>", MY_CLAIMS,
            "<p>In the <b>Shamatha Project</b>, people on a three-month retreat improved in perceptual sensitivity and sustained attention compared with a waiting-list group (<b>MacLean et al., 2010, <i>Psychological Science</i></b>), and much of the attention gain was still present years later (Zanesco et al., 2018, <i>Journal of Cognitive Enhancement</i>). Sahdra et al. (2011, <i>Emotion</i>) reported better emotional functioning. Short daily practice has smaller effects.</p>")),
        "tb_tonglen" => m("<p><b>Tonglen</b> (\"sending and taking\"): breathing in others' suffering as dark smoke and breathing out relief as light.</p>",
            "<p>Part of the Tibetan <b>lojong</b> (mind training) tradition set out in Geshe Chekawa's <i>Seven-Point Mind Training</i>, from teachings attributed to Atisha; taught widely by Pema Chödrön.</p>",
            "<p>12th century, from earlier Indian teachings.</p>"),
        "tb_mani" => ch("<p>Chanting <b>Om Mani Padme Hum</b>, the mantra of Chenrezig (Avalokiteśvara), the bodhisattva of compassion.</p>",
            "<p>The mantra first appears in the <i>Kāraṇḍavyūha Sūtra</i>. It is the most recited mantra in Tibet, carved on stones and turned in prayer wheels, and the Dalai Lamas are regarded as emanations of Chenrezig.</p>",
            "<p>About 1,500 years old.</p>"),
        "tb_tara" => ch("<p>Mantra and visualisation of <b>Green Tara</b>, the swift protector who frees from fear.</p>",
            "<p>Tara devotion arose in Indian Buddhism and was carried to Tibet by Atisha in the 11th century; she is among the most loved figures of Tibetan Buddhism.</p>",
            "<p>Indian roots by about the 6th century; in Tibet since the 11th.</p>"),
        "tb_medicine" => ch("<p>Visualising the <b>Medicine Buddha</b> (Bhaiṣajyaguru), whose body is deep-blue lapis light, and reciting his mantra.</p>",
            "<p>From the <i>Medicine Buddha Sūtra</i>, central to Tibetan medicine and practised across Mahayana Buddhism.</p>",
            "<p>The sutra was translated into Chinese by the 7th century.</p>"),
        "tb_vajra" => m("<p><b>Vajra recitation</b>: silently joining the syllables Om, Ah and Hum to the in-breath, the pause and the out-breath.</p>",
            "<p>A breathing practice from the Indian Buddhist tantras, especially the <i>Guhyasamāja</i> tradition, taught across the Tibetan schools.</p>",
            "<p>Indian tantras of about the 8th–9th centuries.</p>"),
        "tb_phat" => m("<p>The Dzogchen practice of a sudden <b>PHAT!</b> that cuts through thought, so the nature of mind can be recognised.</p>",
            "<p>From <b>Garab Dorje</b>'s \"Three Words that Strike the Vital Point\", with the famous commentary by Patrul Rinpoche (1808–1887), <i>The Special Teaching of the Wise and Glorious King</i>.</p>",
            "<p>Traditionally traced to Garab Dorje; Patrul Rinpoche's commentary is from the 19th century.</p>"),
        "tb_tilopa" => m("<p>Resting the mind with <b>Tilopa</b>'s six words of advice, then in silence.</p>",
            "<p>Tilopa (988–1069), the Indian mahasiddha at the head of the Kagyu lineage, gave this advice on Mahamudra, the \"great seal\" of the mind's nature.</p>",
            "<p>11th century.</p>"),
        "tb_chod" => m("<p><b>Feeding your demons</b>: giving a fear or struggle a form, asking what it needs, and feeding it with compassion until it becomes an ally.</p>",
            "<p>Developed by <b>Lama Tsultrim Allione</b> (<i>Feeding Your Demons</i>, 2008) from the <b>Chöd</b> of the Tibetan yogini <b>Machig Labdrön</b> (1055–1149), who taught feeding rather than fighting what troubles us.</p>",
            "<p>Chöd from the 11th–12th centuries; the five-step method from 2008.</p>"),
        "tb_bardo" => m("<p>A rehearsal of dying: the dissolution of the elements and the dawning of the clear light.</p>",
            "<p>The Tibetan <b>bardo</b> teachings, including the <i>Bardo Thödol</i> (\"Tibetan Book of the Dead\") revealed by Karma Lingpa, describe earth dissolving into water, water into fire, fire into air and air into consciousness.</p>",
            "<p>14th century, from older Indian tantric teachings.</p>"),
        "tb_gyuto" => Some(i("<p>A deep drone modelled on the <b>one-voice chord</b> of the Gyuto monks, who sing a very low tone with a strong high overtone above it.</p>",
            "<p>The Gyuto Tantric College, founded in Tibet in 1474 and re-established in India after 1959, is famed for this style of chant.</p>",
            "<p>Gyuto founded 1474; the overtone style first analysed in 1967.</p>", MY_CLAIMS,
            "<p>Acousticians first analysed the chant in 1967 (<b>Smith, Stevens &amp; Tomlinson, <i>Journal of the Acoustical Society of America</i></b>), showing how the singers shape the vocal tract to boost one high harmonic above a very low fundamental. No studies have tested its effects on listeners.</p>")),
        "tb_sleepyoga" => Some(i("<p><b>Sleep yoga</b>: falling asleep with awareness resting at the heart, aiming to recognise the clear light of deep sleep.</p>",
            "<p>Taught in both Bön and Nyingma Dzogchen; Tenzin Wangyal Rinpoche's <i>The Tibetan Yogas of Dream and Sleep</i> (1998) describes it for Western students.</p>",
            "<p>Over a thousand years old in Tibet.</p>", MY_CLAIMS,
            "<p>There are no trials of sleep yoga itself. Experienced meditators show more fast gamma activity during deep non-REM sleep than non-meditators (<b>Ferrarelli et al., 2013, <i>PLOS ONE</i></b>), which hints that practice can change sleep, but whether anyone stays aware in dreamless sleep is still debated.</p>")),
        "tb_sky" => m("<p><b>Sky gazing</b>: resting an open gaze in clear space to mix awareness with the sky.</p>",
            "<p>A Dzogchen practice of the Nyingma and Bön traditions, taught by teachers such as Namkhai Norbu and Tenzin Wangyal.</p>",
            "<p>Traditional Dzogchen practice, many centuries old.</p>"),
        // ---- Buddhist meditation
        "bu_metta" => Some(i("<p><b>Mettā</b>: offering wishes of goodwill in widening circles.</p>",
            "<p>Taught by the Buddha in the <i>Karaṇīya Mettā Sutta</i>. Buddhaghosa set out the order of persons in the <i>Visuddhimagga</i>, and Sharon Salzberg made it popular in the West.</p>",
            "<p>About 2,500 years old.</p>", MY_CLAIMS,
            "<p><b>Fredrickson et al. (2008, <i>Journal of Personality and Social Psychology</i>)</b>: seven weeks of loving-kindness meditation increased daily positive emotions, which built resources such as mindfulness, purpose and social support and raised life satisfaction. <b>Hutcherson, Seppala &amp; Gross (2008, <i>Emotion</i>)</b>: a few minutes increased feelings of connection toward strangers. A review (Hofmann, Grossman &amp; Hinton, 2011, <i>Clinical Psychology Review</i>) found promising effects, though many studies are small.</p>")),
        "bu_brahma" => m("<p>Radiating the four <b>brahmavihāras</b>, loving-kindness, compassion, appreciative joy and equanimity, in all directions.</p>",
            "<p>Described again and again in the Pali suttas, for example the <i>Tevijja Sutta</i>; many Theravada teachers use it as a path to absorption.</p>",
            "<p>About 2,500 years old.</p>"),
        "bu_anapana" => m("<p><b>Ānāpānasati</b>: mindfulness of breathing in sixteen steps.</p>",
            "<p>The <i>Ānāpānasati Sutta</i> (Majjhima Nikāya 118) of the Pali Canon; modern teachers include Buddhadāsa Bhikkhu and Thich Nhat Hanh.</p>",
            "<p>About 2,500 years old.</p>"),
        "bu_noting" => m("<p><b>Noting</b>: lightly labelling each experience as it arises.</p>",
            "<p>Developed by the Burmese monk <b>Mahasi Sayadaw</b> (1904–1982) from the <i>Satipaṭṭhāna Sutta</i>; it spread worldwide through insight meditation centres.</p>",
            "<p>1940s–1950s, from early Buddhist sources.</p>"),
        "bu_mu" => m("<p>Working with the koan <b>Mu</b>, case 1 of <i>The Gateless Gate</i>.</p>",
            "<p>The Chinese master Zhaozhou (778–897) gave the answer, and Wumen Huikai made it the first case of the <i>Wumenguan</i>. It is the classic first koan of Rinzai Zen.</p>",
            "<p>9th-century exchange; collected in 1228.</p>"),
        "bu_shikantaza" => m("<p><b>Shikantaza</b>: just sitting, with no object and no goal.</p>",
            "<p>Taught by <b>Eihei Dōgen</b> (1200–1253), founder of Japanese Sōtō Zen, in his <i>Fukanzazengi</i>, from the \"silent illumination\" of Chinese Caodong Chan.</p>",
            "<p>13th century.</p>"),
        "bu_nembutsu" => ch("<p>The <b>nembutsu</b>: chanting <i>Namu Amida Butsu</i>, calling on Amida Buddha.</p>",
            "<p>Pure Land Buddhism, from Chinese masters such as Shandao (613–681) to Hōnen and Shinran in Japan.</p>",
            "<p>China from the 6th–7th centuries; Japan from the 12th.</p>"),
        "bu_daimoku" => ch("<p><b>Daimoku</b>: chanting <i>Nam-myoho-renge-kyo</i>, the title of the Lotus Sutra.</p>",
            "<p>Taught by <b>Nichiren</b> (1222–1282) in Japan from 1253; today the main practice of the Nichiren schools and Soka Gakkai International.</p>",
            "<p>Since 1253.</p>"),
        "bu_kasina" => Some(i("<p><b>Kasiṇa</b>: gazing at a coloured disc until a clear inner image forms, as a way into absorption.</p>",
            "<p>Mentioned in the Pali suttas and described in detail in Buddhaghosa's <i>Visuddhimagga</i>; revived by some modern retreat teachers.</p>",
            "<p>Early Buddhist; described in detail in the 5th century.</p>", MY_CLAIMS, GAZE_RESEARCH)),
        "bu_ajikan" => m("<p><b>Ajikan</b>: contemplating the Sanskrit letter A within a moon disc.</p>",
            "<p>The core meditation of <b>Shingon</b> esoteric Buddhism, brought from China to Japan by <b>Kūkai</b> (774–835), who founded the monastery on Mount Kōya.</p>",
            "<p>9th century.</p>"),
        // ---- Hindu and yogic meditation
        "hy_trataka" => Some(i("<p><b>Trāṭaka</b>: steady gazing at a candle flame or point, then holding its afterimage.</p>",
            "<p>One of the six cleansing actions (<i>ṣaṭkarma</i>) of hatha yoga, described in Svātmārāma's <i>Hatha Yoga Pradīpikā</i>.</p>",
            "<p>15th-century text, from older yogic practice.</p>",
            "<p>The <i>Hatha Yoga Pradīpikā</i> says trāṭaka removes eye diseases and sloth; modern teachers say it builds concentration and memory and awakens intuition. The eye-disease claims are not supported, and gazing should never strain the eyes.</p>",
            "<p>In a small randomised trial in older adults, <b>Talwadkar, Jagannathan &amp; Raghuram (2014, <i>International Journal of Yoga</i>)</b> reported better scores on several cognitive tests after about a month of daily trāṭaka. Other studies are small, so the evidence is preliminary. Fixed gazing also produces afterimages and Troxler fading.</p>")),
        "hy_soham" => m("<p><b>So'ham</b>: noticing the mantra \"I am That\" in the natural breath.</p>",
            "<p>Called <i>ajapa japa</i> or <i>hamsa</i> in Hindu tantra and yoga texts, and taught by modern teachers such as Swami Muktananda and the Bihar School of Yoga.</p>",
            "<p>Medieval yoga texts; widely taught since the 20th century.</p>"),
        "hy_vbt" => m("<p>Five of the 112 <b>dhāraṇās</b> (contemplations) of the <i>Vijñāna Bhairava Tantra</i>.</p>",
            "<p>A text of Kashmir Shaivism, the non-dual tradition later expounded by Abhinavagupta; the version in Paul Reps's <i>Zen Flesh, Zen Bones</i> (1957) made it known in the West.</p>",
            "<p>About the 8th century.</p>"),
        "hy_bija" => ch("<p>Chanting the <b>bīja</b> (seed) mantras of the chakras, from LAM at the base of the spine to OM at the brow.</p>",
            "<p>The tantric chakra system as set out in texts such as Pūrṇānanda's <i>Ṣaṭ-cakra-nirūpaṇa</i>, translated by Arthur Avalon (Sir John Woodroffe) in <i>The Serpent Power</i> (1919).</p>",
            "<p>Medieval tantra; this six-chakra form by the 16th century.</p>"),
        "hy_shiva" => ch("<p><b>Japa</b> of <i>Om Namah Shivaya</i> on a mala of 108 beads.</p>",
            "<p>The five-syllable mantra (<i>Pañcākṣara</i>) appears in the <i>Śrī Rudram</i> of the Yajur Veda and is the central mantra of Shaivism.</p>",
            "<p>Vedic, first millennium BCE.</p>"),
        "hy_gayatri" => ch("<p>Reciting the <b>Gāyatrī mantra</b> to the divine light of the sun at the junctions of the day.</p>",
            "<p>Rig Veda 3.62.10, attributed to the sage Viśvāmitra; the most revered verse of the Vedas, recited daily in the <i>sandhyāvandana</i> rite.</p>",
            "<p>Rig Veda, 2nd millennium BCE.</p>"),
        "hy_mahamantra" => ch("<p>Chanting the <b>Hare Krishna mahā-mantra</b>, quietly on beads and in joyful group song.</p>",
            "<p>Found in the <i>Kali-santaraṇa Upaniṣad</i>; spread by Chaitanya Mahāprabhu (1486–1534) in Bengal and worldwide by ISKCON from 1966.</p>",
            "<p>16th-century bhakti; worldwide since the 1960s.</p>"),
        "hy_antarmouna" => m("<p><b>Antar mouna</b>: witnessing sounds and thoughts in stages until inner silence opens.</p>",
            "<p>Developed by <b>Swami Satyananda Saraswati</b> (1923–2009) of the Bihar School of Yoga from tantric <i>pratyāhāra</i> (sense-withdrawal) methods.</p>",
            "<p>1960s–1970s.</p>"),
        "hy_mandukya" => m("<p>Chanting <b>AUM</b> sound by sound to explore the four states of consciousness.</p>",
            "<p>The <i>Māṇḍūkya Upaniṣad</i> and Gauḍapāda's verses on it, founding texts of Advaita Vedanta, with commentary by Shankara.</p>",
            "<p>The Upaniṣad is roughly 2,000 years old; Gauḍapāda wrote about the 6th century.</p>"),
        // ---- Sufi mysticism
        "su_sema" => m("<p>A seated, listening form of the Mevlevi <b>sema</b>: attention turns with a sound that circles the head.</p>",
            "<p>The Mevlevi order, founded in Konya by Rumi's son Sultan Walad and his followers after Rumi's death in 1273. UNESCO lists the sema ceremony as intangible cultural heritage.</p>",
            "<p>Since the 13th century.</p>"),
        "su_muraqaba" => m("<p><b>Murāqaba</b>: watchful waiting on the heart.</p>",
            "<p>Practised across the Sufi orders, especially the Naqshbandi, named after Bahāʾ al-Dīn Naqshband (1318–1389) of Bukhara, known for silent remembrance.</p>",
            "<p>Early Sufism; Naqshbandi form from the 14th century.</p>"),
        "su_lataif" => m("<p>Awakening the five <b>laṭāʾif</b>, subtle centres in the chest, with silent remembrance of God.</p>",
            "<p>Set out in the Naqshbandi-Mujaddidi order of Shaykh Aḥmad Sirhindī (1564–1624), building on earlier Sufi psychology.</p>",
            "<p>17th century, from earlier roots.</p>"),
        "su_breaths" => Some(i("<p>The four <b>elemental purification breaths</b>: five slow breaths each for earth, water, fire and air.</p>",
            "<p>Taught by <b>Hazrat Inayat Khan</b> (1882–1927), the Indian musician and Chishti Sufi who brought Sufism to the West, and continued by the Inayatiyya and the Sufi Ruhaniat.</p>",
            "<p>Early 20th century.</p>", MY_CLAIMS, BREATH_RES)),
        "su_names" => ch("<p><b>Wazīfa</b>: repeating some of the ninety-nine Beautiful Names of God.</p>",
            "<p>Rooted in the Qur'an's \"to God belong the most beautiful names\" (7:180) and a hadith listing ninety-nine names; practised across the Sufi orders.</p>",
            "<p>Since the early centuries of Islam.</p>"),
        // ---- Jewish mysticism and Kabbalah
        "jk_shema" => m("<p>Meditating on the <b>Shema</b>, drawing out the word <i>echad</i>, \"one\".</p>",
            "<p>Deuteronomy 6:4, recited twice daily. The Talmud (Berakhot 13b) describes covering the eyes and lengthening <i>echad</i>, and Kabbalists made it a meditation on divine unity.</p>",
            "<p>Biblical; the Talmudic practice by about 500 CE.</p>"),
        "jk_abulafia" => m("<p><b>Letter combination</b>: chanting a Hebrew letter with each vowel on the breath.</p>",
            "<p>Abraham Abulafia (1240–after 1291), founder of prophetic Kabbalah, in works such as <i>Or ha-Sekhel</i> (\"Light of the Intellect\"); studied by Moshe Idel and presented for modern readers by Aryeh Kaplan (<i>Meditation and Kabbalah</i>, 1982).</p>",
            "<p>13th century.</p>"),
        "jk_tree" => m("<p>Drawing divine light through the ten <b>sefirot</b> of the Tree of Life, mapped onto the body.</p>",
            "<p>The sefirot appear in the <i>Sefer Yetzirah</i> and the <i>Bahir</i> and are developed in the <b>Zohar</b> (Castile, late 13th century) and by Isaac Luria in 16th-century Safed.</p>",
            "<p>12th–16th centuries.</p>"),
        "jk_ayin" => m("<p><b>Bittul</b>: dissolving the sense of self into <i>ayin</i>, divine nothingness.</p>",
            "<p>Early Hasidism, especially Dov Ber, the Maggid of Mezritch (d. 1772), successor of the Baal Shem Tov.</p>",
            "<p>18th century.</p>"),
        "jk_merkavah" => m("<p>A visionary ascent through the seven heavenly palaces to the divine throne.</p>",
            "<p>The <b>Hekhalot</b> texts of late antiquity, such as <i>Hekhalot Rabbati</i>, rooted in Ezekiel's chariot vision; Gershom Scholem's <i>Major Trends in Jewish Mysticism</i> (1941) brought them to wide attention.</p>",
            "<p>About the 3rd–8th centuries CE.</p>"),
        "jk_niggun" => ch("<p>Singing a wordless <b>niggun</b> over a drone.</p>",
            "<p>Hasidic worship since the Baal Shem Tov (about 1698–1760); each Hasidic dynasty keeps its own melodies.</p>",
            "<p>18th century to today.</p>"),
        // ---- Taoist inner cultivation
        "ta_zuowang" => m("<p><b>Zuowang</b>: sitting and forgetting the body, the senses and knowing.</p>",
            "<p>Chapter 6 of the <i>Zhuangzi</i>; made into a method by Sima Chengzhen (647–735) of the Shangqing school.</p>",
            "<p>4th–3rd century BCE; method texts from the Tang dynasty.</p>"),
        "ta_xinzhai" => m("<p><b>Xinzhai</b>: the fasting of the heart-mind, listening with qi instead of the ears and the mind.</p>",
            "<p>Chapter 4 of the <i>Zhuangzi</i>, one of the founding texts of Daoism.</p>",
            "<p>4th–3rd century BCE.</p>"),
        "ta_standing" => qi("<p><b>Zhan zhuang</b>: standing still in a tree-hugging posture to build relaxed strength and calm.</p>",
            "<p>Long part of internal martial-arts training; popularised by Wang Xiangzhai (1885–1963), founder of Yiquan, and now taught in many qigong schools.</p>",
            "<p>Old roots; the modern form from the 1920s–40s.</p>"),
        "ta_sixsounds" => qi("<p>The <b>six healing sounds</b>: six slow, whispered out-breaths, each linked to an organ.</p>",
            "<p>First recorded by Tao Hongjing (456–536) in the <i>Yangxing yanming lu</i>; standardised as Health Qigong Liu Zi Jue by the Chinese Health Qigong Association in 2003.</p>",
            "<p>About 1,500 years old.</p>"),
        // ---- Shamanic and ancestral rituals
        "sr_lower" => rite("<p>A drum journey to the <b>lower world</b> to meet a power animal.</p>",
            "<p>Core shamanism, developed by the anthropologist <b>Michael Harner</b> (1929–2018) from fieldwork with peoples such as the Shuar and Conibo and from cross-cultural study; described in <i>The Way of the Shaman</i> (1980).</p>",
            "<p>Modern method since 1980, drawing on very old practices.</p>"),
        "sr_posture" => rite("<p>Holding a <b>ritual body posture</b> for fifteen minutes to a fast rattle.</p>",
            "<p>Proposed by the anthropologist <b>Felicitas Goodman</b> (1914–2005), who founded the Cuyamungue Institute in New Mexico and described the postures in <i>Where the Spirits Ride the Wind</i> (1990).</p>",
            "<p>1970s–1990s.</p>"),
        "sr_dance" => rite("<p><b>Trance dance</b>: free movement to a drum that rises to a peak and returns to stillness.</p>",
            "<p>Ecstatic dance is found worldwide, from the all-night healing dance of the Kalahari San to Sufi and Siberian ceremonies; modern ecstatic-dance gatherings spread from the 1970s.</p>",
            "<p>Probably among the oldest human rituals; San rock paintings in southern Africa appear to show it.</p>"),
        // ---- Western esoteric and classical
        "we_incubation" => Some(i("<p><b>Dream incubation</b>: falling asleep with a chosen theme, prompted again and again at the edge of sleep.</p>",
            "<p>The healing temples of Asclepius in ancient Greece, such as Epidaurus; brought into the laboratory at the MIT Media Lab as targeted dream incubation.</p>",
            "<p>From the 5th century BCE; the lab method since 2020.</p>",
            "<p>In antiquity, temple dreams were believed to bring healing or divine advice. Today people use incubation to steer dreams toward a problem, a creative project or a feeling.</p>",
            "<p><b>Haar Horowitz et al. (2020, <i>Consciousness and Cognition</i>)</b>: prompting a theme at sleep onset and briefly waking people to report made the theme appear in their hypnagogic dreams far more often than in controls. <b>Haar Horowitz et al. (2023, <i>Scientific Reports</i>)</b>: incubated naps improved later scores on creative tasks. Their device timed prompts with sleep sensors; timed prompts are a looser approximation. The edge of sleep is a creative \"sweet spot\" in other studies too (Lacaux et al., 2021, <i>Science Advances</i>).</p>")),
        "we_plotinus" => m("<p>A Neoplatonic ascent from the image of the cosmos to Intellect and the One.</p>",
            "<p><b>Plotinus</b> (204/5–270), whose treatises his student Porphyry edited as the <i>Enneads</i>; his ideas shaped Christian, Jewish and Islamic mysticism.</p>",
            "<p>3rd century.</p>"),
        "we_above" => west("<p>A Hermetic contemplation: expanding awareness from the body to the whole cosmos.</p>",
            "<p>The Greek <i>Corpus Hermeticum</i> and the Arabic <i>Emerald Tablet</i>. Marsilio Ficino's Latin translation of the Corpus in 1463 made Hermeticism central to Renaissance thought.</p>",
            "<p>2nd–3rd century; revived in the Renaissance.</p>"),
        "we_middlepillar" => west("<p>Building five spheres of light along the body's central column, each vibrated with a divine name.</p>",
            "<p>The Hermetic Order of the Golden Dawn (London, 1888); published by Israel Regardie (1907–1985) in <i>The Middle Pillar</i> (1938).</p>",
            "<p>Late 19th century; published 1938.</p>"),
        "we_pentagram" => west("<p>The <b>Lesser Banishing Ritual of the Pentagram</b>, a short ritual for centring and clearing space.</p>",
            "<p>The Hermetic Order of the Golden Dawn; published by Aleister Crowley in <i>Liber O</i> (1909) and by Israel Regardie in <i>The Golden Dawn</i> (1937–40).</p>",
            "<p>Late 19th century.</p>"),
        "we_bards" => Some(i("<p>Composing poems or ideas in darkness, eyes covered, with a weight on the belly.</p>",
            "<p>The Gaelic poets of Scotland and Ireland, described by Martin Martin (1703) and in accounts of the Irish bardic schools, which faded out in the 17th century.</p>",
            "<p>Medieval to the 17th–18th centuries.</p>",
            "<p>The bardic schools held that darkness concentrated the mind and drew down inspiration, called <i>imbas</i> in Irish and <i>awen</i> in Welsh tradition.</p>",
            "<p>The bardic method itself has not been studied. Darkness and sensory reduction make spontaneous imagery more likely (see the Ganzfeld and floatation research), and the drowsy edge of sleep is a creative sweet spot (Lacaux et al., 2021, <i>Science Advances</i>).</p>")),
        "we_galdr" => west("<p><b>Galdr</b>: chanting rune names with long, resonant vowels.</p>",
            "<p>Old Norse magical chant mentioned in the Eddas and sagas, such as the <i>Hávamál</i>; practised today as a modern reconstruction in contemporary heathenry and rune work.</p>",
            "<p>Viking Age traditions written down in the 13th century; modern revival since the 20th century.</p>"),
        // ---- Trance and hypnosis techniques
        "tr_autogenic" => Some(i("<p><b>Autogenic training</b>: six self-suggestion formulas for heaviness, warmth, heartbeat, breathing, belly warmth and a cool forehead.</p>",
            "<p>Developed by the German psychiatrist <b>Johannes Heinrich Schultz</b> (1884–1970) from Oskar Vogt's research on hypnosis; published as <i>Das autogene Training</i> (1932) and later extended by Wolfgang Luthe.</p>",
            "<p>1920s; published 1932.</p>",
            "<p>Schultz held that the formulas train the body's own self-regulation, producing a calm, hypnosis-like state that helps with stress, sleep and stress-related complaints.</p>",
            "<p>A meta-analysis of 60 studies (<b>Stetter &amp; Kupper, 2002, <i>Applied Psychophysiology and Biofeedback</i></b>) found medium-to-large effects for tension headache, migraine, mild high blood pressure, anxiety, mild depression and functional sleep problems. A systematic review (Ernst &amp; Kanji, 2000, <i>Complementary Therapies in Medicine</i>) found it helpful for stress and anxiety.</p>")),
        "tr_erickson" => trance("<p>The <b>3-2-1 method</b>: narrowing attention through sights, sounds and feelings into self-hypnosis.</p>",
            "<p>Taught by Elizabeth (Betty) Erickson, wife and collaborator of the psychiatrist Milton H. Erickson (1901–1980), whose indirect, conversational hypnosis shaped modern hypnotherapy.</p>",
            "<p>Second half of the 20th century.</p>"),
        "tr_braid" => trance("<p><b>Eye fixation</b>: the classic hypnotic induction, fixing the gaze until the eyelids close by themselves.</p>",
            "<p>The Scottish surgeon <b>James Braid</b> (1795–1860) began experimenting after watching a stage mesmerist in Manchester in 1841, and coined \"hypnotism\" in <i>Neurypnology</i> (1843).</p>",
            "<p>1841–1843.</p>"),
        "tr_fractionation" => trance("<p><b>Fractionation</b>: deepening trance by coming partly out and going back in, several times.</p>",
            "<p>Developed by the German neurologist <b>Oskar Vogt</b> (1870–1959); Schultz's autogenic training grew out of Vogt's work.</p>",
            "<p>1890s.</p>"),
        _ => None,
    }
}

const LJ_WHAT: &str = "<p><b>Flicker-light stimulation</b>: bright light flashing evenly on closed eyes makes the visual cortex fire in rhythm with it. Within seconds most people see vivid colours and moving geometric patterns that aren't there, called <b>stroboscopic</b> or <b>flicker-induced hallucinations</b>. In this app the flash rate follows the session's binaural beat, so eyes and ears get the same rhythm (audio-visual entrainment). It only runs after you accept the warning screen.</p>";
const LJ_ORIGIN: &str = "<p>Jan Purkinje described patterns from flickering sunlight through spread fingers in <b>1819</b>. Neuroscientist W. Grey Walter studied flicker with EEG in the 1940s–50s (<i>The Living Brain</i>, 1953). Artist <b>Brion Gysin</b> and Ian Sommerville built the <b>Dreamachine</b> in 1959. 1980s–90s \"mind machines\" combined LED goggles with binaural beats; Lumenate brought it to phone flashlights in the 2020s.</p>";
const LJ_ERA: &str = "<p>Described since 1819; studied with EEG since the 1940s; phone apps since the 2020s.</p>";
const LJ_CLAIMS: &str = "<p>Claimed to give psychedelic-like experiences without drugs, deepen meditation, and reduce stress. Some users report emotional release or insight.</p>";
const LJ_RESEARCH: &str = "<p>The visuals are well established. <b>Allefeld et al. (2011, <i>Consciousness and Cognition</i>)</b> found pattern and colour experiences depend on rate, strongest around 10 Hz. <b>Bartossek et al. (2021, <i>PLOS ONE</i>)</b> found 10 Hz flicker produced stronger altered-state ratings than 3 Hz. <b>Amaya et al. (2023)</b>, from the Dreamachine team, found rhythmic flicker beat arrhythmic. Ermentrout &amp; Cowan (1979) explained the geometry as waves across the visual cortex. Claims about lasting psychological benefits are not yet well tested.</p><p><b>Risks:</b> flashing light at 3–30 Hz (peak risk 15–25 Hz) can trigger seizures in people with photosensitive epilepsy, about 1 in 4,000 people, sometimes without any previous history. This app caps the rate at 12 Hz, needs warnings accepted, ramps in, and stops instantly on any tap or key.</p>";

fn lj_info(id: &str) -> Option<Info> {
    let x = |what: &str| Some(i(format!("{}{}", what, LJ_WHAT), LJ_ORIGIN, LJ_ERA, LJ_CLAIMS, LJ_RESEARCH));
    match id {
        "lj_alpha" => x("<p>A <b>10 Hz light journey</b>: the rate most studies find gives the strongest visuals.</p>"),
        "lj_sweep" => x("<p>A <b>sweet-spot sweep</b> from 8 to 12 Hz to find the rate that works best for you.</p>"),
        "lj_geometry" => x("<p><b>12 Hz geometry</b>: upper-alpha flicker for crisp patterns such as checkerboards, spirals and honeycombs (Klüver's form constants, 1926).</p>"),
        "lj_descent" => x("<p>An <b>alpha-to-theta descent</b>: strong patterns first, then slower flicker for softer, dreamlike imagery.</p>"),
        "lj_dream" => x("<p>A <b>Dreamachine-inspired</b> wandering rhythm between 8 and 12 Hz.</p>"),
        "lj_ganzflicker" => x("<p><b>Ganzflicker</b>: full red and black at about 10 Hz (Shenyan et al., 2024; Königsmark et al., 2021).</p>"),
        "lj_bart10" => x("<p>The <b>10 Hz condition</b> of Bartossek et al. (2021): 20 minutes of white light, lit for 50% of each cycle, on closed eyes.</p>"),
        "lj_bart3" => x("<p>The <b>3 Hz condition</b> of Bartossek et al. (2021): the same set-up with slow flicker.</p>"),
        "lj_steady" => x("<p>The <b>constant-light control</b> of Bartossek et al. (2021): the same light, never flashing.</p>"),
        "lj_amaya" => x("<p>The <b>frequency and rhythm series</b> of Amaya et al. (2023): 2-minute blocks, light on for 30% of each cycle, rhythmic and arrhythmic.</p>"),
        "lj_ganzopen" => x("<p>The original <b>eyes-open Ganzflicker</b>: the whole screen alternating red and black at about 10 Hz for 10 minutes.</p>"),
        "lj_scan" => x("<p>A <b>frequency scan</b> from 3 to 12 Hz, one minute per rate, to map how the imagery changes.</p>"),
        "lj_lucia" => x("<p><b>Hypnagogic light</b>: flicker alternating with steady glow, in the style of the Lucia N°03 lamp.</p>"),
        "lj_ave" => x("<p><b>Audio-visual entrainment</b>: light flashes and sound pulses at the same rate, as in AVE \"mind machines\".</p>"),
        "lj_phone" => x("<p>A <b>phone-flashlight journey</b> in the style of apps such as Lumenate.</p>"),
        _ => None,
    }
}

// ------------------------------------------------- Infrasound & Breathwork (opt-in)
fn ib_info(id: &str) -> Option<Info> {
    const INF_WHAT: &str = "<p><b>Infrasound</b> is sound below about 20 Hz, too low to hear as a pitch. At high levels it can be felt as pressure or vibration. Most headphones and almost all phone speakers reproduce very little of it, so you may notice nothing at all. Don't raise the volume to try to hear it. This layer is capped well below the tones and stops instantly when you turn it off.</p>";
    const INF_ORIGIN: &str = "<p>Infrasound was studied in the 1960s (Vladimir Gavreau in France; NASA for rocket launches). Vic Tandy's 1998 \"ghost in the machine\" paper and the 2003 \"Soundless Music\" concert brought it into the paranormal debate. Wind turbines raised new questions in the 2000s.</p>";
    const INF_RESEARCH: &str = "<p>Lab studies of infrasound at levels people meet in everyday life mostly find <b>no</b> reliable effects on mood or the body once people don't know when it's on (e.g. Crichton et al., 2014, <i>Health Psychology</i>, which found expectations caused the symptoms). Very loud infrasound (above about 110 dB, not possible here) can cause pressure, nausea and fatigue. This app's layer is quiet; any effect is likely expectation plus the beat underneath.</p>";
    const BR_ORIGIN: &str = "<p>Fast, deep breathing appears in yogic <i>kapalabhati</i> and <i>bhastrika</i> and Tibetan tummo. In the West: rebirthing (Leonard Orr, 1970s), <b>Holotropic Breathwork</b> (Stanislav and Christina Grof, 1970s) and the <b>Wim Hof Method</b> (2010s).</p>";
    const BRS_RESEARCH: &str = "<p>Slow breathing with long breaths out is one of the best-supported ways to calm the body: it raises heart-rate variability, strengthens the baroreflex and lowers arousal (Lehrer &amp; Gevirtz, 2014; Zaccaro et al., 2018 systematic review). These techniques involve no hyperventilation and no long holds, so they suit almost everyone; if you feel light-headed, simply breathe normally.</p>";
    const BR_RESEARCH: &str = "<p>Breathing fast blows off carbon dioxide (hypocapnia). That narrows blood vessels in the brain and causes tingling, light-headedness, cramping hands, vivid sensations and emotions. That's why it's used in EEG labs to <b>provoke seizures</b>. Kox et al. (2014, <i>PNAS</i>) showed trained Wim Hof breathers could boost adrenaline and dampen inflammation. Small studies suggest breathwork can lift mood (e.g. Fincham et al., 2023, <i>Scientific Reports</i> meta-analysis on breathwork and stress, small effects). <b>Breath holding after hyperventilation can cause blackout without warning</b>; there have been drownings when people practised in water.</p>";
    let s = |what: &str, origin: &str, era: &str, claims: &str, research: &str| Some(i(what, origin, era, claims, research));
    match id {
        "inf_ghost" | "inf_concert" | "inf_7" | "inf_schumann" => s(INF_WHAT, INF_ORIGIN,
            "<p>Studied since the 1960s; paranormal link from 1998.</p>",
            "<p>Claimed to cause unease, awe, chills, \"presences\" and even ghost sightings, or (for 7 Hz) to resonate with organs.</p>", INF_RESEARCH),
        "br_power" => s("<p><b>Power breathing rounds</b>: about 30 deep, quick breaths, an empty-lung hold until you need to breathe, then a 15-second recovery hold, repeated for 3 rounds.</p>", BR_ORIGIN,
            "<p>Ancient roots; the Wim Hof Method since the 2010s.</p>",
            "<p>Claimed to boost energy, immunity, cold tolerance and mood, and to give euphoric or mystical states.</p>", BR_RESEARCH),
        "br_kox" => s("<p><b>The Kox 2014 protocol</b>: about 30 deep breaths, an empty-lung hold for as long as comfortable, then a 10-second recovery hold, in rounds.</p>", BR_ORIGIN,
            "<p>Wim Hof Method from the 2000s; studied in 2014.</p>",
            "<p>Claimed to strengthen the immune system, give control over the stress response and build cold tolerance.</p>", BR_RESEARCH),
        "br_cyclic" => s("<p><b>Cyclic hyperventilation with retention</b>: three rounds of 30 breaths, each followed by a 15-second empty-lung hold.</p>", BR_ORIGIN,
            "<p>Studied at Stanford in 2023.</p>",
            "<p>Claimed to energise and lift mood.</p>", BR_RESEARCH),
        "br_tummo" => s("<p><b>Forceful vase breathing</b> with inner-fire visualisation.</p>",
            "<p>G-tummo is one of the Six Yogas of Naropa, practised in Tibetan Buddhism for about a thousand years. Herbert Benson's Harvard team measured it in Himalayan monks (1982, <i>Nature</i>), and Maria Kozhevnikov's team in 2013.</p>",
            "<p>About a thousand years old; studied since 1982.</p>",
            "<p>Claimed to make enough inner heat to dry wet sheets in freezing air, and to lead to bliss and insight.</p>",
            "<p>Benson et al. (1982) recorded finger and toe temperatures rising by several degrees. Kozhevnikov et al. (2013) found core temperature rose only with the forceful breathing, and stayed raised when it was combined with the visualisation. Forceful holds with muscle contraction act like straining (a Valsalva manoeuvre) and raise blood pressure.</p>"),
        "br_kapal" => s("<p><b>Kapalabhati</b>: rapid, forceful breaths out at about one a second.</p>",
            "<p>One of the six cleansing techniques (<i>shatkarmas</i>) of the <i>Hatha Yoga Pradipika</i> (15th century); studied extensively by Shirley Telles and colleagues in India.</p>",
            "<p>Medieval; studied since the 2000s.</p>",
            "<p>Claimed to cleanse the airways and the mind, raise energy and sharpen attention.</p>",
            "<p>Telles et al. (2011) found it reduced vagal (calming) activity during and after practice; other studies found brief improvements on some attention tasks. It raises breathing to about 60 breaths a minute, which can cause light-headedness. Yoga teachers advise against it in pregnancy and with hernia, high blood pressure, heart disease or epilepsy.</p>"),
        "br_bhastrika" => s("<p><b>Bhastrika</b>: forceful breaths in and out, like a bellows.</p>",
            "<p>Described in the <i>Hatha Yoga Pradipika</i> and the <i>Gheranda Samhita</i>.</p>",
            "<p>Medieval hatha yoga.</p>",
            "<p>Claimed to heat the body, awaken energy (kundalini) and clear the mind.</p>", BR_RESEARCH),
        "br_fire" => s("<p><b>Breath of fire</b>: rapid, light, equal breaths through the nose.</p>",
            "<p>Kundalini yoga as taught by Yogi Bhajan in the United States from 1969.</p>",
            "<p>1969 onward.</p>",
            "<p>Claimed to energise, cleanse and strengthen the nervous system.</p>", BR_RESEARCH),
        "br_sky" => s("<p><b>Cyclic breathing</b> inspired by Sudarshan Kriya: ocean breath, bellows breath, Om, then slow, medium and fast cycles, then rest.</p>",
            "<p>Sudarshan Kriya was introduced by <b>Sri Sri Ravi Shankar</b> in 1982 and is taught by the Art of Living Foundation.</p>",
            "<p>1982 onward.</p>",
            "<p>Claimed to release stress and negative emotions and to raise energy.</p>",
            "<p>Small trials report less depression and anxiety (for example Janakiramaiah et al., 2000; reviewed by Zope &amp; Zope, 2013), and a randomised trial with university students found benefits for well-being (Seppälä et al., 2020). Many studies involve the Art of Living organisation. The fast cycles are a form of hyperventilation.</p>"),
        "br_rebirth" => s("<p><b>Rebirthing breathwork</b>: relaxed, connected breathing with no pauses.</p>", BR_ORIGIN,
            "<p>1970s onward.</p>",
            "<p>Claimed to release birth trauma and suppressed emotions. Not to be confused with the restraint-based \"rebirthing therapy\" used on children, which is dangerous and has been banned in some US states.</p>", BR_RESEARCH),
        "br_sigh" => s("<p><b>Cyclic sighing</b>: a double breath in, then a long breath out, for five minutes.</p>",
            "<p>The sigh is a natural reflex studied by Jack Feldman's lab at UCLA; Stanford's Huberman and Spiegel labs tested cyclic sighing as a daily practice (Balban et al., 2023).</p>",
            "<p>Tested as a practice in 2023.</p>",
            "<p>Promoted as the fastest way to calm down in real time.</p>", BRS_RESEARCH),
        "br_box" => s("<p><b>Box breathing</b>: equal counts of breathing in, holding, breathing out and holding, sized by a CO₂ tolerance test.</p>",
            "<p>Used in military and first-responder stress training and popularised by former Navy SEAL Mark Divine; tested at Stanford in 2023.</p>",
            "<p>Modern form since the 2010s, based on older pranayama.</p>",
            "<p>Claimed to give calm focus under pressure.</p>", BRS_RESEARCH),
        "br_478" => s("<p>The <b>4-7-8 breath</b>: in for 4, hold for 7, out for 8.</p>",
            "<p>Taught by integrative-medicine physician <b>Andrew Weil</b> from the 1990s, adapted from yogic pranayama.</p>",
            "<p>1990s onward.</p>",
            "<p>Claimed to act as a natural tranquilliser and to help people fall asleep quickly.</p>", BRS_RESEARCH),
        "br_coherent" => s("<p><b>Coherent breathing</b> at about five breaths a minute.</p>",
            "<p>Named by <b>Stephen Elliott</b> (2005) and taught by Richard Brown and Patricia Gerbarg; resonance-frequency breathing was developed in biofeedback by Paul Lehrer and Evgeny Vaschillo.</p>",
            "<p>2000s onward.</p>",
            "<p>Claimed to balance the nervous system and ease anxiety, depression and PTSD.</p>", BRS_RESEARCH),
        "br_bhramari" => s("<p><b>Bhramari</b>: humming on the breath out.</p>",
            "<p>Named after the black bee (<i>bhramara</i>); described in the <i>Hatha Yoga Pradipika</i>.</p>",
            "<p>Medieval hatha yoga.</p>",
            "<p>Claimed to calm the mind, relieve anger and anxiety, and help sleep.</p>", BRS_RESEARCH),
        "br_nadi" => s("<p><b>Nadi shodhana</b>: alternate-nostril breathing.</p>",
            "<p>A classical pranayama of hatha yoga, linked to the yogic idea of the <i>ida</i> and <i>pingala</i> energy channels.</p>",
            "<p>Medieval hatha yoga, with older roots.</p>",
            "<p>Claimed to balance the two sides of the brain and the body's energy channels.</p>", BRS_RESEARCH),
        "br_vase" => s("<p><b>Gentle vase breathing</b> with a warmth visualisation.</p>",
            "<p>The gentle form of g-tummo in Tibetan Buddhism.</p>",
            "<p>About a thousand years old.</p>",
            "<p>Claimed to maintain inner heat and bliss.</p>", BRS_RESEARCH),
        "br_ratio" => s("<p><b>Ratio breathing</b> with retention, 1:4:2.</p>",
            "<p>Classical <i>kumbhaka</i> in the <i>Hatha Yoga Pradipika</i>, codified for modern students by B. K. S. Iyengar.</p>",
            "<p>Medieval hatha yoga.</p>",
            "<p>Claimed to increase prana (life force) and steady the mind.</p>", BRS_RESEARCH),
        "br_slowbh" => s("<p><b>Slow-pace bhastrika</b>: deep breaths at six a minute.</p>",
            "<p>A slowed form of the classical bellows breath, studied by Tapas Pramanik and colleagues (2009).</p>",
            "<p>Studied since 2009.</p>",
            "<p>Claimed to calm and balance the body.</p>", BRS_RESEARCH),
        "br_connected" => s("<p><b>Conscious connected breathing</b>: continuous, fuller-than-normal breaths with no pause between in and out, for 15 minutes.</p>", BR_ORIGIN,
            "<p>1970s onward.</p>",
            "<p>Claimed to release stored trauma and emotions, and to give non-ordinary states comparable to psychedelics.</p>", BR_RESEARCH),
        _ => None,
    }
}
