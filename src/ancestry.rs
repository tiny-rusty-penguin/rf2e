use crate::{AbilityScores, Sizes, Specials, Traits};
use crate::language::Languages;
use strum::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Ancestries {
    Dwarf,
    Elf,
    Gnome,
    Goblin,
    Halfling,
    Human,
    Leshy,
    Orc,
}

pub struct Ancestry {
    pub ancestry: Ancestries,
    pub traits: Vec<Traits>,
    pub hp: u8,
    pub adult_hood: f32,
    pub life_expectancy: i16,
    pub average_height: f32,
    pub size: Sizes,
    pub speed: u8,
    pub ability_boosts: Vec<AbilityScores>,
    pub ability_flaws: Vec<AbilityScores>,
    pub free_boosts: u8,
    pub languages: Vec<Languages>,
    pub free_languages: u8,
    pub specials: Vec<Specials>,
    pub description: &'static str,
    pub you_might: Vec<&'static str>,
    pub others_probaly: Vec<&'static str>,
    pub physical_description: &'static str,
    pub society: &'static str,
    pub beliefs: Option<&'static str>,
}

impl Ancestry {
    pub fn all() -> Vec<Self> {
        vec![
            Self::human(),
            Self::elf(),
            Self::gnome(),
            Self::goblin(),
            Self::halfling(),
            Self::human(),
            Self::leshy(),
            Self::orc(),
        ]
    }

    pub fn str_to_ancestry(s: &str) -> Option<Ancestry> {
        match s {
            "dwarf" =>  Some(Self::dwarf()),
            "elf" =>  Some(Self::elf()),
            "gnome" =>  Some(Self::gnome()),
            "goblin" =>  Some(Self::goblin()),
            "halfling" => Some(Self::halfling()),
            "human" => Some(Self::human()),
            "leshy" =>  Some(Self::leshy()),
            "orc" => Some(Self::orc()),
            _ => None,
        }
    }
    fn dwarf() -> Self {
        Ancestry {
            ancestry: Ancestries::Dwarf,
            traits: vec![Traits::Dwarf, Traits::Humanoid],
            hp: 10,
            adult_hood: 25.0,
            life_expectancy: 350,
            average_height: 4.5,
            size: Sizes::Small,
            speed: 20,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            ability_flaws: vec![AbilityScores::Charisma],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Dwarven],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::ClanDagger],
            description: "Orcs are proud, strong people with hardened physiques who value physical might and glory in combat.

Orcs are forged in the fires of violence and conflict, often from the moment they are born. As they live lives that are frequently cut brutally short, orcs revel in testing their strength against worthy foes, often by challenging a higher-ranking member of their community for dominance. Orcs often struggle to gain acceptance among other communities, who frequently see them as brutes. Those who earn the loyalty of an orc friend, however, soon learn that an orc's fidelity and honesty are unparalleled. Orc culture teaches that they are shaped by the challenges they survive, and the most worthy survive the most hardships. Orcs who attain both a long life and great triumphs command immense respect.",
            you_might: vec!["Strive to uphold your personal honor and refuse to back down", "Appreciate quality craftsmanship in all forms and insist upon it for all your gear."],
            others_probaly: vec!["See you as stubborn, though whether this is an asset or a detriment changes from moment to moment.", "Recognize the deep connection you have with your family, heritage, and friends."],
            physical_description: "Dwarves are short and stocky, standing about a foot shorter than most humans. They have wide, compact bodies and burly frames. Dwarves of all genders pride themselves on the length of their hair and beards, which they often braid into intricate patterns, some of which represent specific clans. A long beard is a sign of maturity and honor among many dwarf clans.

Dwarves typically reach physical adulthood around the age of 25, though their traditionalist culture places more value on completing coming-of-age ceremonies unique to each clan than reaching a certain age. A typical dwarf can live to around 350 years old.",
            society: "Though the ancient dwarven empire fell long ago, overwhelmed by orc and goblinoid enemies, dwarves today retain many of the qualities that once propelled them to greatness: fierceness, gumption, and stubbornness in their endeavors. Dwarves live within mountain Sky Citadels dotted over the surface, which can create vast cultural divides between dwarf clans. Yet nearly all dwarven peoples share a passion for stonework, metalwork, and family.

Few dwarves are seen without their clan dagger strapped to their belt. This dagger is forged just before a dwarf's birth and is distinctive to their clan. Dwarves honor their children with names of ancestors or dwarven heroes. When introducing themselves, dwarves tend to list their family and clan, plus any number of other familial connections and honorifics.",
            beliefs: Some("Dwarves tend to value honor and closely follow the traditions of their clans and kingdoms. They have a strong sense of friendship and justice, though they are often very particular about who they consider a friend. They work hard and play harder—especially when strong ale is involved. Torag, god of dwarvenkind, is dwarves' primary deity, though worship of Torag's family members is also common.

Popular Edicts create art with utility, guard your community against those who would harm it, keep your clan dagger close
Popular Anathema leave an activity or promise uncompleted, forsake your family"),
        }
    }
    fn elf() -> Self {
        Ancestry {
            ancestry: Ancestries::Elf,
            traits: vec![Traits::Elf, Traits::Humanoid],
            hp: 6,
            adult_hood: 20.0,
            life_expectancy: 600,
            average_height: 6.5,
            size: Sizes::Medium,
            speed: 30,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Intelligence],
            ability_flaws: vec![AbilityScores::Constitution],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Elven],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
            description: "Elves are a tall, long-lived people with a strong tradition of art and magic.

As an ancient people, elves have seen great change and have the perspective that can come only from watching the arc of history. After leaving Golarion in ancient times, they returned to a changed land, and they still struggle to reclaim their ancestral homes. Elves value kindness, intellect, and beauty, with many elves striving to improve their manners, appearance, and culture. Their studies delve into a level of detail that most shorter-lived peoples find excessive or inefficient. Elves are often rather private people, steeped in the secrets of their groves and kinship groups. They're slow to build friendships outside their kinsfolk, as elves who spend their lives among shorter-lived peoples often become morose after watching generations of companions age and die. These elves are known as Forlorn among their fellow elves.",
            you_might: vec!["Carefully curate your relationships with people with shorter lifespans.", "Adopt specialized or obscure interests simply for the sake of mastering them."],
            others_probaly: vec!["Focus on your appearance, either admiring your grace or treating you as if you're physically fragile.", "Worry that you privately look down on them, or feel like you're condescending and aloof."],
            physical_description: "While generally taller than humans, elves possess a fragile grace, accentuated by long features and sharply pointed ears. Their eyes are wide and rounded, featuring large and often vibrantly colored pupils that make up the entire visible portion of the eye. These pupils give them an alien look and allow them to see sharply even in very little light.

Elves gradually adapt to their environment and their companions, and they often take on physical traits reflecting their surroundings. An elf who has dwelled in primeval forests for centuries, for example, might exhibit verdant hair and gnarled fingers, while one who's lived in a desert might have golden pupils and skin. Elves reach physical adulthood around the age of 20, though they aren't considered to be fully emotionally mature by other elves until closer to the passing of their first century. A typical elf can live to around 600 years old.",
            society: "he inborn patience and intellectual curiosity of elves make them excellent sages, philosophers, and wizards, and their societies are built upon their inherent sense of wonder and knowledge.

Elves hold deeply seated ideals of individualism, allowing each elf to explore multiple occupations before alighting on a particular pursuit or passion that suits her best. Elves bear notorious grudges against rivals, which elves call ‘ilduliel,' but these antagonistic relationships can sometimes blossom into friendships over time. An elf keeps their personal name secret among their family, while giving a nickname when meeting other people. This nickname can change over time, due to events in the elf's life or even on a whim. A single elf might be known by many names by associates of different ages and regions.

Elven names consist of multiple syllables and are meant to flow lyrically—at least in the Elven tongue.",
            beliefs: Some("Elves are often emotional and capricious, yet they hold high ideals close to their hearts. They prefer deities who share their love of all things mystic and artistic. Desna and Shelyn are particular favorites, the former for her sense of wonder and the latter for her appreciation of artistry. Calistria is the most notorious of elven deities, as she represents many of the elven ideals taken to the extreme."),
        }
    }

    fn gnome() -> Self {
        Ancestry {
            ancestry: Ancestries::Gnome,
            traits: vec![Traits::Gnome, Traits::Humanoid],
            hp: 8,
            adult_hood: 18.0,
            life_expectancy: 400,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            ability_flaws: vec![AbilityScores::Strength],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey, Languages::Gnomish],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
            description: "Gnomes are short and hardy folk, with an unquenchable curiosity and eccentric habits.

Long ago, early gnome ancestors emigrated from the First World, realm of the fey. While it's unclear why the first gnomes wandered to Golarion, this lineage manifests in modern gnomes as bizarre reasoning, eccentricity, obsessive tendencies, and what some see as naivete.

Always hungry for new experiences, gnomes constantly wander both mentally and physically, attempting to stave off a terrible ailment that threatens all of their people. This affliction, known as the Bleaching, strikes gnomes who fail to dream, innovate, and take in new experiences. The Bleaching slowly drains the color— literally—from gnomes, and it plunges those affected into states of deep depression that eventually claim their lives. Very few gnomes survive this scourge, becoming deeply morose and wise survivors known as bleachlings.",
            you_might: vec!["Embrace learning and hop from one area of study to another without warning.", "Speak, think, and move quickly, and lose patience with those who can't keep up."],
            others_probaly: vec!["Appreciate your enthusiasm and the energy with which you approach new situations.", "Struggle to understand your motivations or adapt to your rapid changes of direction."],
            physical_description: "Most gnomes stand just over 3 feet in height and weigh little more than a human child. They exhibit a wide range of natural skin, hair, and eye colors. For gnomes that haven't begun the Bleaching, nearly any hair and eye color other than white is possible, with vibrant colors most frequent, while skin tones span a slightly narrower spectrum and tend toward earthy tones and pinkish hues, though occasionally green, black, or pale blue.

Gnomes typically reach physical maturity at the age of 18, though many gnomes maintain a childlike curiosity about the world even into adulthood. A gnome can theoretically live to any age if they can stave off the Bleaching indefinitely, but in practice gnomes rarely live longer than around 400 years.",
            society: "While most gnomes adopt some of the cultural practices of the region in which they live, they tend to pick and choose, adjusting their communities to fit their own fey logic. This often leads to majority gnome communities eventually consisting almost entirely of gnomes, as other people, bewildered by gnomish political decisions, choose to move elsewhere. Gnomes have little culture that they would consider entirely their own. Exceptionally few gnome kingdoms or nations exist on the surface of Golarion, and most gnomes wouldn't know what to do with such a state if they had one.

By necessity, few gnomes marry for life, instead allowing relationships to run their course before amicably moving on, the better to stave off the Bleaching with new experiences. Though gnome families tend to be small, many gnome communities raise children communally, with fluid family boundaries. Gnome names can get quite complex and polysyllabic. Gnomes rarely concern themselves with how easy their names are to pronounce, and they often go by shorter nicknames. Among gnomes, the shorter the name, the more feminine it's considered to be.",
            beliefs: Some("Though gnomes are impulsive tricksters with inscrutable motives and confusing methods, many at least attempt to make the world a better place. They are prone to fits of powerful emotion and are rarely shy about helping those they believe deserve it. Gnomes most commonly worship deities that value individuality and nature, such as Cayden Cailean, Desna, Gozreh, and Shelyn.",)
        }
    }

    fn goblin() -> Self {
        Ancestry {
            ancestry: Ancestries::Goblin,
            traits: vec![Traits::Goblin, Traits::Humanoid],
            hp: 6,
            adult_hood: 4.5,
            life_expectancy: 20,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: vec![AbilityScores::Wisdom],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Goblin],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            description: "Goblins are a short, scrappy, energetic people who have spent millennia maligned and feared.

The convoluted histories other people cling to don't interest goblins. These small folk live in the moment, and they prefer tall tales over factual records. Goblin virtues are about being present, creative, and honest. They strive to lead fulfilled lives, rather than worrying about how their journeys will end. To tell stories, not nitpick the facts. To be small, but dream big. Many goblins enjoy simpler delights like songs, fire, and eating, and hate reading, dogs, and horses. Other goblins might have more complex pursuits, though, such as tinkering with scraps or concocting snacks and explosives from most anything.",
            you_might: vec!["Strive to prove that you have a place among other civilized peoples, perhaps even to yourself.", "Lighten the heavy emotional burdens others carry (and amuse yourself) with antics and pranks."],
            others_probaly: vec!["Work to ensure you don't accidentally (or intentionally) set too many things on fire.", "Wonder how you survive given your ancestry's typical gastronomic choices, reckless behavior, and love of fire."],
            physical_description: "Goblins are stumpy humanoids with large bodies, scrawny limbs, and massively oversized heads with large ears and beady red eyes. Their skin ranges from green to gray to blue, and they often bear scars, boils, and rashes. Goblins average 3 feet tall. Most are bald, with little or no body hair. Their jagged teeth fall out and regrow constantly, and their fast metabolism means they eat constantly and nap frequently. Mutations are also more common among goblins than other peoples, and goblins usually view particularly salient mutations as a sign of power or fortune.

Goblins reach adolescence by the age of 3 and adulthood 4 or 5 years later. Goblins can live 50 years or more, but without anyone to protect them from each other or themselves, few live past 20 years of age.",
            society: "Goblins tend to flock to strong leaders, forming small tribes. These tribes rarely number more than a hundred, though the larger a tribe is, the more diligent the leader must be to keep order—a notoriously difficult task. Play and creativity matter more to goblins than productivity or study, and their encampments erupt with songs and laughter.

Goblins bond closely with their allies, fiercely protecting those companions who have protected them or offered a sympathetic ear. Goblins tend to assume for their own protection that members of taller ancestries, which goblins often refer to colloquially as “longshanks,” won't treat them kindly. Learning to trust longshanks is difficult for a goblin, and it's been only in recent years that such a partnership has even been an option. Goblins keep their names simple. A good name should be easy to pronounce, short enough to shout without getting winded, and taste good to say. The namer often picks a word that rhymes with something they like so that writing songs is easier.",
            beliefs: Some("Even the most well-intentioned goblins have trouble following the rules, meaning goblin adventurers are often unsure whether they're on the right side of the law. Organized worship likewise confounds goblins, and most of them would rather pick their own deities, choosing powerful monsters, natural wonders, or anything else they find fascinating— sometimes even attributing deific status to fellow goblins of note. Goblins who spend time around people of other ancestries might adopt some of their beliefs, though, and many goblin adventurers adopt the worship of Cayden Cailean."),
        }
    }

    fn halfling() -> Self {
        Ancestry {
            ancestry: Ancestries::Halfling,
            traits: vec![Traits::Halfling, Traits::Humanoid],
            hp: 6,
            adult_hood: 20.0,
            life_expectancy: 150,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Wisdom],
            ability_flaws: vec![AbilityScores::Strength],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Halfling],
            free_languages: 0,
            specials: vec![Specials::KeenEyes],
            description: "Halflings are a short, resilient people who exhibit remarkable curiosity and humor.

Claiming no place as their own, halflings control few settlements larger than villages. Instead, they frequently live among humans within larger cities, carving out small communities alongside taller folk. Optimistic, cheerful, and driven by powerful wanderlust, halflings make up for their short stature with an abundance of bravado. At once excitable and easygoing, halflings are the best kind of opportunists, and their passions favor joy over violence. While their curiosity sometimes drives them toward adventure, halflings also carry strong ties to house and home.",
            you_might: vec!["Get along well with a wide variety of people and enjoy meeting new friends.", "Find it difficult to resist indulging your curiosity, even when you know it's going to lead to trouble."],
            others_probaly: vec!["Appreciate your ability to always find a silver lining or something to laugh about, no matter how dire the situation.", "Think you bring good luck with you."],
            physical_description: "Halflings are short humanoids who look vaguely like smaller humans. They rarely grow to be more than 3 feet in height. Halfling proportions vary, with some looking like shorter adult humans with slightly larger heads and others having proportions closer to those of a human child. Most halflings prefer to walk barefoot rather than wear shoes, and those who do so develop roughly calloused soles on their feet over time. Tufts of thick, often-curly hair warm the tops of their broad, tanned feet. Halfling skin tones tend toward rich, tawny shades like amber or oak, and their hair color ranges from a light golden blond to raven black.

Halflings reach physical adulthood around the age of 20. A typical halfling can live to be around 150 years old.",
            society: "Despite their jovial and friendly nature, halflings don't usually tend to congregate. They have few cultural centers in the Inner Sea region, and they instead tend to weave themselves throughout the societies of the world. Halflings eke out whatever living they can manage, many performing menial labor or holding simple service jobs. Some halflings reject city life, instead turning to the open road and traveling from place to place in search of fortune and fame. These nomadic halflings often travel in small groups, sharing hardships and simple pleasures among close friends and family. Halfling names are usually two to three syllables, with a gentle sound that avoids hard consonants. Preferring their names to sound humble, halflings see overly long or complex names as a sign of arrogance for their people. However, they understand that elves and humans might have longer names to suit their own aesthetics.",
            beliefs: Some("Halflings are loyal to their friends and their family, but they aren't afraid to do what needs to be done in order to survive. Wherever halflings go, they seamlessly blend into the society they find themselves in, adapting to the culture and beliefs of the predominant ancestry around them and adding their uniquely halfling twists, creating a blend of cultural diffusion that enriches both cultures. Halflings favor gods that either grant luck, like Desna, or encourage guile, like Norgorber, and many appreciate Cayden Cailean's role as a liberator, as well as any religions common among other ancestries around them."),
        }
    }

    fn human() -> Self {
        Ancestry {
            ancestry: Ancestries::Human,
            traits: vec![Traits::Human, Traits::Humanoid],
            hp: 8,
            adult_hood: 15.0,
            life_expectancy: 90,
            average_height: 5.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![],
            ability_flaws: vec![],
            free_boosts: 2,
            languages: vec![Languages::Common],
            free_languages: 1,
            specials: vec![],
            description: "Humans are diverse and adaptable people with wide potential and deep ambitions.

As unpredictable and varied as any of Golarion's peoples, humans have exceptional drive and the capacity to endure and expand. Though many civilizations thrived before humanity rose to prominence, humans have built some of the greatest and the most terrible societies throughout the course of history, and today they are the most populous people in the realms around the Inner Sea.

Humans' ambition, versatility, and exceptional potential have led to their status as the world's predominant ancestry. Their empires and nations are vast, sprawling things, and their citizens carve names for themselves with the strength of their sword arms and the power of their spells. Humanity is diverse and tumultuous, running the gamut from nomadic to imperial, sinister to saintly. Many of them venture forth to explore, to map the expanse of the multiverse, to search for long-lost treasure, or to lead mighty armies to conquer their neighbors—for no better reason than because they can.",
            you_might: vec!["Strive to achieve greatness, either in your own right or on behalf of a cause.", "Seek to understand your purpose in the world.", "Cherish your relationships with family and friends."],
            others_probaly: vec!["Respect your flexibility, your adaptability, and—in most cases—your open-mindedness.", "Distrust your intentions, fearing you seek only power or wealth", "Aren't sure what to expect from you and are hesitant to assume your intentions."],
            physical_description: "Humans' physical characteristics are as varied as the world's climes. Humans have a wide variety of skin and hair colors, body types, and facial features. Generally speaking, their skin has a darker hue the closer to the equator they or their ancestors lived.

Humans reach physical adulthood around the age of 15, though mental maturity occurs a few years later. A typical human can live to be around 90 years old. Humans have exceptionally mutable physical characteristics compared to other ancestries, with greater variance in height, weight, and other physical parameters.",
            society: "Human variety also manifests in terms of their governments, attitudes, and social norms. Though the oldest of human cultures can trace their shared histories thousands of years into the past, when compared to the societies of the elves or dwarves, human civilizations seem in a state of constant flux as empires fragment and new kingdoms subsume the old.",
            beliefs: None,
        }
    }

    fn leshy() -> Self {
        Ancestry {
            ancestry: Ancestries::Leshy,
            traits: vec![Traits::Leshy, Traits::Plant],
            hp: 8,
            adult_hood: 0.0,
            life_expectancy: -1,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Wisdom, AbilityScores::Constitution],
            ability_flaws: vec![AbilityScores::Intelligence],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::PlantNourishment],
            description: "Leshies are immortal nature spirits placed in small plant bodies, seeking to experience the world.

Leshies are immortal spirits of nature temporarily granted physical forms. As guardians and emissaries of the environment, leshies are “born” when a skilled druid or other master of primal magic conducts a ritual to create a suitable vessel, and then a spirit chooses that vessel to be their temporary home. Leshies are self-sufficient from the moment the ritual ends, though it isn't uncommon for leshies to maintain lifelong bonds with their creators. Many leshies relish the opportunity to interact with the physical world. While most leshy spirits are ancient, they rarely recall past lifetimes and see their new life as a chance to experience the wonders of the world once more.",
            you_might: vec!["Act as a traveling agent for natural guardians who are unable to leave their territories.", "Encourage civilizations to cooperate with nature and build their cities in ecologically friendly ways."],
            others_probaly: vec!["Think you are a curiosity due to your spiritual origins.", "Assume you know only about nature and are unfamiliar with civilization and society."],
            physical_description: "Leshies are as varied as the material used to create their vessels, usually appearing as a bizarre mishmash of various plants or fungi. Their bodies are vaguely humanoid in shape, with numerous characteristics of the plant or fungus from which they were made. A typical leshy is about 3 feet tall. Leshies begin their lives as adults and don't age.",
            society: "To most leshies, the concept of family is not a matter of birth, but rather determined by bonds of loyalty and friendship. Leshies are dedicated allies, but they have little tolerance for those who would despoil nature. As much as they are happy to accept someone who earns their trust into their family, they expect family members to look out for them and their natural wards in return.
Leshies are grouped into categories akin to ethnicities, but these are not connected to physical features; rather, they represent broad categories of characteristics of their spirits. Certain spirits are more likely to gravitate toward particular physical bodies, though this predisposition is far from absolute. Leshies' genders are determined by the spirits that inhabit their bodies. Some leshies are exclusively male or female, while many consider themselves both. Others, particularly fungus leshies, tend toward far more complex expressions of gender, or eschew the concept entirely. Leshies choose and change their names multiple times throughout their lives.",
            beliefs: Some("Leshies' beliefs generally focus on the natural world. Those with a philosophical bent lean toward the Green Faith, and Gozreh is the most popular deity among faithful leshies. Some leshies also venerate green men, powerful spirits of nature."),
        }
    }

    fn orc() -> Self {
        Ancestry {
            ancestry: Ancestries::Orc,
            traits: vec![Traits::Orc, Traits::Humanoid],
            hp: 10,
            adult_hood: 17.0,
            life_expectancy: 60,
            average_height: 7.0,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Constitution],
            ability_flaws: vec![AbilityScores::Intelligence],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Orcish],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            description: "Orcs are proud, strong people with hardened physiques who value physical might and glory in combat.

Orcs are forged in the fires of violence and conflict, often from the moment they are born. As they live lives that are frequently cut brutally short, orcs revel in testing their strength against worthy foes, often by challenging a higher-ranking member of their community for dominance. Orcs often struggle to gain acceptance among other communities, who frequently see them as brutes. Those who earn the loyalty of an orc friend, however, soon learn that an orc's fidelity and honesty are unparalleled. Orc culture teaches that they are shaped by the challenges they survive, and the most worthy survive the most hardships. Orcs who attain both a long life and great triumphs command immense respect.",
            you_might: vec!["Eagerly meet any chance to prove your strength in a physical contest.", "View dying in glorious combat as preferable to a mundane death from old age or illness."],
            others_probaly: vec!["See you as violent or lacking in discipline.", "Admire your forthrightness and blunt honesty."],
            physical_description: "Orcs are tall and powerfully built, with long arms and stocky legs. Many orcs top 7 feet in height, though they tend to adopt broad, almost bow-legged stances and slouch forward at the shoulders. Orcs have rough skin, thick bones, and rock-hard muscles, making them suited to war and other physically demanding tasks. Orc skin color is typically some shade of green, though some orcs have other skin colors that reflect adaptations to their environments.

Orcs consider powerful builds, heavily scarred skin, large tusks, and tattoos attractive, regardless of gender. Orcs reach physical adulthood around the age of 17, with many orcs living to be up to 60 years old.",
            society: "Most orc communities—known as holds—define themselves through two things: pain and glory. Each earns respect in near equal measure, so long as the pain is borne with stoicism. An orc with many scars who walks uncomplaining with a broken leg draws as much admiration as one who wins a great victory on the battlefield. Additionally, power defines the dynamics among families and holds. Weaker orcs work at the behest of the strong, with power constantly shifting between orcs that prove their might. Orcs tend to share in familial duties, raising children as a community and sharing responsibilities among the entire hold.",
            beliefs: Some("A common orc saying is “you are the scars that shape you.” Violent, chaotic lives in violent, chaotic lands mean that most orcs tend to expect and accept violence. Lamashtu, and Rovagug are commonly worshiped among more war-minded orc communities, while less violent holds worship gods like Sarenrae, whose tenets of fire, redemption, and glory all hold some appeal to orc sensibilities.

While there are orc deities, their worship is surprisingly uncommon among orcs. Orcs believe that if a creature has a face and a name, it can be killed, and so their own deities are often targets, rather than objects of reverence. Some orc holds teach that the greatest members of the hold can earn a chance to challenge the orc deities for a place amid the pantheon."),
        }
    }
}
