//! Long-form background for every session: what it is, where it comes from,
//! when it began, what is claimed, and what research actually found.
//! Text is trusted HTML authored here (never user input).
use crate::plan::{Info, Preset};

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
        "ap_ground" => sys("<p>A short <b>grounding</b> session that brings you back from deep, dreamy states to full alertness.</p>",
            "<p>\"Grounding\" after inner journeys appears in shamanic, yogic and Western esoteric traditions; Monroe's tapes end with a counted return for the same reason.</p>",
            "<p>Traditional; modern form from 20th-century guided meditation.</p>",
            "<p>Claimed to \"anchor\" energy back into the body after astral work.</p>",
            "<p>Physiologically, a short period of rising alertness helps clear sleep inertia (grogginess), which can last up to 30 minutes after deep relaxation or naps (Tassi &amp; Muzet, 2000). Don't drive straight after deep sessions.</p>"),
        _ => None,
    }
}
