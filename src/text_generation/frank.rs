use super::actor::Actor;

pub const FRANK: Actor = Actor {
    name: "frank",
    lines: &FRANK_LINES,
    markov_order: 2,
};

const FRANK_LINES: [&str; 780] = [
  "I got no leg room back here.",
  "Move your seat forward.",
  "There's a mechanism.",
  "You just pull it, and throw your body weight.",
  "If you want the leg room, say you want the leg room!",
  "Don't blame the mechanism!",
  "Like an animal.",
  "Because of her, I have to sit here like an animal!",
  "Serenity now!",
  "Serenity now!",
  "Doctor gave me a relaxation cassette.",
  "When my blood pressure gets too high, the man on the tape tells me to say, 'Serenity now!'",
  "The man on the tape wasn't specific.",
  "Serenity nowww!",
  "George, as you may be aware, your mother and I are not moving to Del Boca Vista, Florida.",
  "So, I was wondering, would it be okay if I turned your room into a billiard parlor?",
  "Regulation table, the hi-fi, maybe even a bar..",
  "Give it real authenticity..",
  "(Interested) There's a Costanza in Tuscany?",
  "(Elaine nods) Did he look like me?",
  "Did you talk to him?",
  "I gotta get that picture - it could be my cousin, Carlo.",
  "I played with him every day until the age of four - and then we separated.",
  "Today, I went record shopping in Greenwich Village.",
  "I bought this record, but I can't seem to find the hi-fi.",
  "No.",
  "That's why I can never be president..",
  "It always irked me.",
  "That's why, even at an early age, I had no interest in politics.",
  "I refuse to vote.",
  "(Yelling out) They don't want me, I don't want them!",
  "Didn't I give you my old record player?",
  "(Cutting him off) Don't bring me down.",
  "(To Elaine) Do you have another copy of that photo?",
  "Cosmo?",
  "Who's Cosmo?",
  "The Maestro?",
  "What Maestro?",
  "Well, I want it back.",
  "I wanna listen to that cha-cha record.",
  "Who is this?",
  "(putting down the bowl) Lemme change my shirt.",
  "(hanging up) Sorry, George, our Chinese food just came.",
  "Talk to you later.",
  "So, what do you think?",
  "Your old man can look pretty good when he wants to, huh?",
  "Serenity now!",
  "What's the matter with this tie?",
  "I've hardly worn it.",
  "Will you put her to rest for me?",
  "How do you know what kind of ties they wear?",
  "Blow out the candles!",
  "Blow out the candles, I said!",
  "Blow out the damn candles!",
  "Oh, get the hell outta here.",
  "7th Avenue.",
  "Blow out the candles!!",
  "Huh?",
  "My computers.",
  "I've been selling them for two months now.",
  "Shut up!",
  "We gotta stop off and pick up a marble rye from Schnitzer's.",
  "Two months ago, I saw a provocative movie on cable TV.",
  "It was called The Net, with that girl from the bus.",
  "I did a little reading, and I realize, it wasn't that farfetched.",
  "Why do you need all that ketchup for?",
  "No!",
  "We have to go to Schnitzer's!",
  "I'll show these people something about taste!",
  "Yes, I do.",
  "That's why I got a secret weapon...",
  "my son.",
  "So I, I talked to Phil Casacof today.",
  "Yeah, you know my friend, the bra salesman.",
  "He says they are looking maybe to put somebody on so I got you an interview next Friday with his boss.",
  "2 o'clock.",
  "You can look for sneakers the next day!",
  "Well, it wouldn't hurt to go in the and be able to discuss it intelligently.",
  "Maybe you should take a look at a few bras?",
  "(to Estelle) Where is you bra?",
  "Give him a bra to look.",
  "Why not?",
  "Why, so he'll go to the interview and he wouldn't know what he's talking about!?!",
  "Well, we got to get moving.",
  "You don't even know what they're made from.",
  "We have a catered affair.",
  "Get out of here!",
  "Lycra-spandex?",
  "(leaving the kitchen) It's creative black tie.",
  "Move, woman.",
  "I once talked to the reverend Yung Son Moon.",
  "He bought two Jesus statues from me.",
  "He's a hell of a nice guy.",
  "I was out for five minutes before, I couldn't feel my extremities.",
  "Wanna bet?",
  "How much you wanna bet?",
  "Ever see that face on him?",
  "Like a Biiig apple pie.",
  "Eight ball.",
  "Take a look.",
  "No one is touching my feet.",
  "Between you and me, Elaine, I think I've got a foot odour problem.",
  "You like to gamble, Cosmo?",
  "(Yelling to Estelle as she leaves)I don't know what the big problem is getting a bra?!",
  "Five dollars a game, huh?",
  "Look at this, George.",
  "(Takes a coin out of his pocket) You ever seen a silver dollar?",
  "I'm not saying go to the library and read the whole history, but it wouldn't kill you to know a little bit about it.",
  "What is this thing anyway?",
  "Okay.",
  "How long it takes to find a bra?",
  "What's going on in there?",
  "You ask me to get a pair of underwear, I'm back in two seconds...you know about the uh cup sizes and all?",
  "They have different cups.",
  "What is that, like a little chicken?",
  "Would you believe when I was 18, I had a ssssilver dollar collection?",
  "You got the A, the B, the C and the D.",
  "That's the biggest.",
  "I had an affair with a Korean woman.",
  "Gamebird?",
  "No, I feel I need to unburden myself.",
  "I loved her very deeply.",
  "But the clash of cultures was too much.",
  "Her family would not accept me.",
  "(Still looking at his coin) You know, I couldn't bring myself to spend one of these.",
  "I got some kind of a-a-a-a-a phobia.",
  "Let me see it.",
  "What do you mean?",
  "Like, you - you hunt it?",
  "Maybe it was because I refused to take off my shoes.",
  "Again, the foot odour problem.",
  "Her father would look at me and say, ' eno enoa juang '.",
  "Which means, 'this guy - this is not my kind of guy'.",
  "Let me see it.",
  "How hard could it be to kill this thing?",
  "What are you wearing, an athletic sweat suit?",
  "I bet that collection would be worth a lot of money today.",
  "(does a form of magic trick making the coin disappear as he shows George an empty hand)",
  "Hmm, that's surprising.",
  "All right, what else?",
  "You got the cups in the front, two loops in the back.",
  "All right, a guess that's about it.",
  "I don't like this waiter.",
  "(Holds up his hand to get the waiters attention - starts snapping) Look at him..",
  "He sees us..",
  "he doesn't want to come over.",
  "Yeah.",
  "I'll take some.",
  "We still haven't finished the first game.",
  "Cook?",
  "No, I don't know any cooks.",
  "I don't know anything about cooking!",
  "Thank you.",
  "(To Estelle) he knows the Maestro.",
  "He could have the picture..",
  "I'm not 'dad' in the workplace.",
  "My professional name is Mr.",
  "Costanza, and I will refer to you as 'Costanza'.",
  "Morning, Braun.",
  "(Yelling out) You don't know that!",
  "(Estelle leaves, slamming the door) We're gonna go see him, huh?",
  "Tommy Tune is a very good dancer.",
  "(hits George on the head with what seems to be the tvguide) You ever see Tommy Tune dance?",
  "Let me understand, you got the hen, the chicken and the rooster.",
  "The rooster goes with the chicken.",
  "So, who's having sex with the hen?",
  "(Sensing the game is going to last a long time) Oh boy.",
  "Your mother recommended him.",
  "But you see my point here?",
  "You only hear of a hen, a rooster and a chicken.",
  "Something's missing!",
  "I didn't read his resume.",
  "Tap dancing.",
  "Anyone can tap dance.",
  "It's all in those shoes.",
  "That's perverse.",
  "(Drawing a zero under George's name) This is your lagging.",
  "Good work, Braun.",
  "Tommy Tune is very tall.",
  "That helps.",
  "It makes him lankier.",
  "I'll take a manicure.",
  "I don't take my shoes off for anyone.",
  "Hey!",
  "Hey!",
  "Come on!",
  "Come on!",
  "I haven't seen it yet.",
  "Hey!",
  "Lloyd!",
  "What was that?",
  "You come here.",
  "Still!",
  "Still!",
  "I like to go in fresh!",
  "They made a derogatory comment about me.",
  "What?",
  "(looks up to heaven) She's deceased.",
  "How are you doing, Lloyd?",
  "I hear you're a big advisor for Dinkins now.",
  "Get the hell out of here.",
  "He's getting married?",
  "(looks to Estelle) She had problems.",
  "(Estelle nods) Internal.",
  "Eight years have I had this car.",
  "Not a scratch on it!",
  "Eight years!",
  "Thank you, Kramer.",
  "To a woman?",
  "A beautiful Mercury!",
  "I special-ordered that bench seat!",
  "Okay, THAT'S IT!",
  "'oki on awa' Where's my tail?",
  "I heard every word you said.",
  "You got some nerve.",
  "And who doesn't serve cake after a meal?",
  "What kind of people?",
  "Would it kill them to put out a pound cake?",
  "Something!",
  "Also internal problems.",
  "(indicating a chair) Put it over there.",
  "I'm sure she's pretty gorgeous.",
  "You're late again, Costanza, so listen up.",
  "Starting tonight, we're having a little sales contest.",
  "The loser gets fired, the winner gets a Waterpik.",
  "I don't know what you mean.",
  "This is your dressing room?",
  "They treat you like Toscanini.",
  "What about your side?",
  "Your cousin Hennie.",
  "(animated) She was sickly from the moment I met her!",
  "(to George) You know, my insurance doesn't cover this?",
  "The whole thing is a total loss!",
  "Tired.",
  "(from other room) I never seen people treated like this!",
  "Not impolite...it's stupid, that's what it is.",
  "You gotta be stupid to do something like that!",
  "Serenity now!",
  "George, we've had it with you.",
  "Understand?",
  "We love you like a son, but even parents have limits.",
  "No it isn't!",
  "How did you know?",
  "So far.",
  "I guess your mother was right.",
  "You never could compete with Lloyd Braun!",
  "(Lloyd rings his sale bell and smiles)",
  "What my wife is trying to say is that this is supposed to be our time.",
  "Kim?",
  "She's not pretty?",
  "Your arm moves like this?",
  "(does the nudging motion)",
  "(looks down, and indicates his chest) Up here?",
  "She'd never make it.",
  "We're cuttin' you lose.",
  "Where the hell is my paper?",
  "No, I brought it in.",
  "They never put it out.",
  "(continues to move his arm) I never seen your arm move like this.",
  "You want me to wear a bra?!",
  "Now, if you'll excuse me, I'm going to make love to your mother.",
  "Ya know what I like about Manhattan?",
  "There's no mosquitoes.",
  "Not really.",
  "It's like some kind of aaaaa (snapping his fingers) spasm.",
  "Queens is full of mosquitoes.",
  "Are you telling me there's not one condo available in all of Del Boca Vista?",
  "Gnats, too.",
  "If I'm not mistaken.",
  "What do you mean stole?",
  "It's my bread.",
  "They didn't eat it.",
  "Why should I leave it there?",
  "How'd you get yours?",
  "You mentioned George's name to Mayor Dinkins?",
  "You discussed George with the mayor of New York?",
  "(angry) Your mother has to tell you every move I make?!",
  "Apparently, it wasn't good enough for them to serve.",
  "Are you trying to keep us out of Del Boca Vista?!",
  "Okay...",
  "(grabbing the card) Hey, Mayor Dinkins set this up for you.",
  "You know what kind of a doctor this must be if Dinkins knows him?",
  "They didn't say hello?",
  "Can I talk to her, please?",
  "They couldn't just say hello?!...",
  "Oh, to hell with them.",
  "Elaine, I can see, not sayin' hello.",
  "She's very--what's the word--uh, supercilious.",
  "Aw, they didn't forget to put it out!",
  "It's deliberate!",
  "Deliberate, I tell ya!",
  "Kramer, I got your message.",
  "I haven't celebrated Festivus in years!",
  "What is your interest?",
  "(shouting and clapping hands in anger) How could Jerry not say Hello?!!",
  "His name was Carlo Costanza.",
  "We played together everyday until I was four.",
  "If I could just look through your photographs, maybe I could recognize him.",
  "Many Christmases ago, I went to buy a doll for my son.",
  "I reach for the last one they had - but so did another man.",
  "As I rained blows opon him, I realized there had to be another way!",
  "They're not coming?",
  "It was destroyed.",
  "But out of that, a new holiday was born.",
  "'A Festivus for the rest of us!'",
  "(Trying to concentrate) Yes, It's, uh, it's..",
  "uh, uh..",
  "What do you call it, Kramer?",
  "She was.",
  "Well what difference does it make?",
  "They wouldn't lie to us, they're are dear friends.",
  "No, not billiard..",
  "(Scolding) Not billiards..",
  "it was..",
  "Come on, already.",
  "Come on..",
  "We call it..",
  "the, uh..",
  "Maybe?",
  "In my mind, there's a war still going on.",
  "The place to be!",
  "Yes!",
  "It's the place to be.",
  "Why wouldn't they like us?",
  "(tastes the Paella; disgusted) Again with the pepper?",
  "What do you gotta use all the pepper for?",
  "Inchon, Korea, 1950.",
  "I was the best cook Uncle Sam ever saw, slinging hash for the Fighting 103rd.",
  "As we marched north, our supply lines were getting thin.",
  "One day a couple of GIs found a crate, inside were six hundred pounds of prime Texas steer.",
  "At least it once was prime.",
  "The Use date was three weeks past, but I was arrogant, I was brash, I thought if I used just the right spices, cooked it long enough...",
  "What are you trying to set my mouth on fire?",
  "I went too far.",
  "I over seasoned it.",
  "Men were keeling over all around me.",
  "I can still hear the retching, the screaming.",
  "I sent sixteen of my own men to the latrines that night.",
  "They were just boys.",
  "Tell that to Bobby Colby.",
  "All that kid wanted to do was go home.",
  "Well he went home alright, with a crater in his colon the size of a cutlet.",
  "Had to sit him on a cork the eighteen-hour flight home!",
  "And at the Festivus dinner, you gather your family around, and you tell them all the ways they have disappointed you over the past year.",
  "No.",
  "No, I'll never cook again!",
  "Never!",
  "Now get out of my house!!",
  "Get out.",
  "Go.",
  "No.",
  "Instead, there's a pole.",
  "It requires not decoration.",
  "I find tinsel distracting.",
  "Let's do it then!",
  "Festivus is back!",
  "I'll get the pole out of the crawl space.",
  "(Turns to leave, meets up with Elaine)",
  "Hello, woman.",
  "(leaves)",
  "Where's the mail? ",
  "You've been in there an hour. ",
  "I couldn't because I had a potential foot problem.",
  "(Looking up from his paper) Who the hell did'ya ever mention his hands to?",
  "I wiped them for two minutes on the mat.",
  "I don't know why your father had to make a federal case out of it.",
  "You never mentioned them to me!",
  "Between you and me I think your country is placing a lot of importance on shoe removal.",
  "Is there anything wrong with getting a receipt at a toll booth? ",
  "George, open the door. ",
  "No!",
  "You never said milky white!",
  "(examining mail) This stack should be bigger, where's the TV Guide? ",
  "I'm missing TV Guide volume forty-one, number thirty-one. ",
  "We're having an upscale dinner.",
  "Elaine took it? ",
  "Has nothing to do with the weather, it's because of the Seinfelds.",
  "So, we're blowing it all.",
  "(holding out his tie) Look George, it's a Pierre Cardin.",
  "(To Estelle, referring to the Jell-O) Why'd you put the bananas in there?!",
  "(shouting) How could you let her take the TV Guide?! ",
  "They don't want us there, so we're going.",
  "We're moving right into Del Boca Vista!",
  "This feels very comfortable.",
  "(Trying to match her tone) So let him have bananas on the side!",
  "Absolutely.",
  "No one tells Frank Costanza what to do!",
  "The nerve of that woman.",
  "Walking into my house, stealing my collectible! ",
  "I feel ten years younger.",
  "(taking the packet) What is this?",
  "(accusingly to George) A prophylactic wrapper?! ",
  "And I can breathe easier, too.",
  "Of course.",
  "Sid Farkus.",
  "He's the best in the business.",
  "You were having sex on our bed?! ",
  "Let's do it!",
  "Except, we gotta do something about the name.",
  "Get outta here!",
  "We have a booth.",
  "No, bro's no good.",
  "Too ethnic.",
  "Order a hot dish.",
  "Your bed is too small?",
  "I'm gone two weeks and you turn our house into, into Bourbon Street! ",
  "How 'bout uh...",
  "the mansiere?",
  "It's dry.",
  "(Yelling) That's not a booth!",
  "That's right.",
  "A brassiere for a man.",
  "The mansiere, get it?",
  "Well it sucks.",
  "(Loud shouting) I didn't take the subway all the way to New York to sit at a table like that!",
  "(Gestures to the table)",
  "George, Festivus is your heritage - it's part of who you are.",
  "Hey George, what d'you like better?",
  "The bro, or the mansiere?",
  "Your meatloaf is mushy, your salmon croquettes are oily and your eggplant parmesan is a disgrace to this house!",
  "That's it!",
  "You're grounded! ",
  "Now, George, what do you want to know about your childhood?",
  "Not any more!",
  "Gimme that spatula!",
  "I'm back, baby!",
  "George, you're forgetting how much Festivus has meant to us all.",
  "I brought one of the casette tapes.",
  "(Franks pushes play, George as a child celebrating Festivus is heard)",
  "You wanna live here?",
  "You respect the rules of our house.",
  "(yells) You're grounded! ",
  "(Looks up) Where's that breeze coming from?",
  "Read that poem.",
  "It's him!",
  "(Standing up) It's Carlo Costanza!",
  "You don't need glasses, you're just weak!",
  "You're weak!",
  "I'd know him anywhere.",
  "Alright, George.",
  "It's time for the feats of strength.",
  "I still say we're related.",
  "We had some good times.",
  "(Complaining) I can't make anything..",
  "Follow through?",
  "What do you mean?",
  "You think you could keep us out of Florida?",
  "We're moving in lock, stock and barrel.",
  "We're gonna be in the pool.",
  "We're gonna be in the clubhouse.",
  "We're gonna be all over that shuffleboard court!",
  "And I dare you to keep me out!",
  "It's a little unnatural, but I think I'm getting the hang of it.",
  "You still need a cook?",
  "Ya got T-Fal?",
  "No!",
  "Follow me.",
  "You're not the only one improving yourself.",
  "I worked out with a dumbbell yesterday.",
  "I feel *vigorous*.",
  "Or, the mansiere.",
  "I got things to do, too.",
  "(on TV) You step on it and it flushes.",
  "I like mansiere.",
  "Getting an eye job like some Manhattanite, huh?",
  "That's your mother's new car.",
  "A woman?",
  "What are you out of your mind? ",
  "Kramer made a pass at you?",
  "You're crazy.",
  "It's a Coupe de Elegance.",
  "Wait.",
  "Is this the group that goes around mutilating squirrels? ",
  "(to Kramer) That's what we figured, huh?",
  "He stopped short?",
  "That's my move.",
  "I'm gonna kill him!",
  "Your mother changed her mind...",
  "(tries to catch a fly with his hand)",
  "I'm calling my lawyer.",
  "It might not be too late to get out of this. ",
  "Yeah, we worked it out.",
  "(vehement) I wouldn't be caught dead in Banlon.",
  "What, they brainwashed you? ",
  "He was my lawyer.",
  "Hey, Braun, Costanza's kicking your butt!",
  "Definitely velcro.",
  "You're not performing any rituals in this house. ",
  "Yeah.",
  "So what?",
  "Costanza, you're white hot!",
  "(claps his hands) You can drop a grand in Disneyworld, like that.",
  "And stay away from those squirrels. ",
  "He's very independent; he doesn't follow the trends.",
  "Hey, Braun, I got good news and bad news.",
  "And they're both the same: you're fired.",
  "Costanza, you've won the water pik!",
  "How do you just walk into a house and take a TV Guide?",
  "How does she expect you to watch TV?",
  "(doorbell rings) Am I just supposed to turn it on and wander aimlessly around the dial? ",
  "(shouting) You have no eye for fashion!!!",
  "You wanna bet?",
  "Serenity now, serenity now!",
  "(shouting) And she's not welcome in this house! ",
  "(yelling) That's my TV Guide!",
  "Ripped to shreds!",
  "She gave that to you?! ",
  "Oh, thank you, Sid, but that's all in the past.",
  "I'm ready to move on.",
  "(anger) You wanna go out with my wife?!",
  "(rage) Where do you get the nerve to ask me something like that?!",
  "(Holding up a picture) Here, take a look at this.",
  "I know what you're saying, and I know what you're thinking!!",
  "It's Carlo.",
  "I found him!",
  "Where's the powdered sugar?",
  "C'mon, Cosmo, I'm not doing business with this guy.",
  "(Yelling out) You never support me!",
  "Let's see what George says about this..",
  "Where're my pants?",
  "(Takes his pair off a rack and leaves)",
  "Of course they're coming, they're leaving soon.",
  "If they don't come tonight they might not see us.",
  "No breaks.",
  "I fell reborn, I'm like a Phoenix rising from Arizona.",
  "Well...",
  "thank you very much!",
  "I admire Morty and Helen going to France.",
  "We should take a trip, maybe a cruise.",
  "Jerry, how come you didn't say hello to me the other day, huh?!",
  "He took it back?",
  "Didn't you tell him I was using it?",
  "What the hell did you trade Jay Buener for?!?",
  "He had 30 home runs, and over 100 RBIs last year.",
  "He's got a rocket for an arm - - you don't know what the hell you're doin'!!",
  "Yes?",
  "(to George, pointing) I knew it was Elaine!!",
  "Where am I supposed to sleep?",
  "Under arrest?",
  "What for?",
  "Jerry, it's Frank Costanza, Mr.",
  "Steinbrenner's here, George is dead, call me back!",
  "What stain?",
  "What's that?",
  "(jumping to his feet) OK that's it!",
  "We're moving!",
  "Noooo!!!",
  "Don't eat it!",
  "No good!'",
  "Don't get in trouble with the Yankees.",
  "You be nice.",
  "(Slaps George's forehead)",
  "(disbelief) You had me sleeping on a pee-stained couch?",
  "George!",
  "George!",
  "I will not tolerate infestation.",
  "(anger) But, the very idea.",
  "you had me lying in urine!!",
  "Don't you understand the very thought, the very idea, I'll never be comfortable again.",
  "(coming upon the van) Good.",
  "He left the door unlocked.",
  "Assman?",
  "I'll get him, Assman!",
  "Isn't it obvious?",
  "There are no parking meters out here.",
  "Those.",
  "I've been saving those from the beginning. ",
  "Will you stop it?",
  "(reclining the seats in the van to a bed) Hey, look at this.",
  "Hoochie mama!",
  "Here, Cosmo...",
  "...You can have the hi-fi.",
  "(hands it over) I don't need it now...",
  "Why'd you take my TV Guide? ",
  "Where's your friend Kramer?",
  "(Holding up the picture) Take a look at this.",
  "Doesn't that look like my flesh and blood?",
  "Of course, your mother- (His attention is drawn over to Susan's doll.",
  "Like George did earlier, he starts to imagine that the doll is scolding him as his wife would)",
  "...I got one at home.",
  "Because I'm looking for him.",
  "That's why.",
  "He stopped short.",
  "(Walking toward the doll) Ridiculous?!",
  "I'll show you ridiculous!",
  "(Struggles with Susan for possession of the doll) Come here!",
  "That's right.",
  "That's me on the left.",
  "We'll go out for dinner tonight.",
  "(examining magazine) What is this?",
  "You got stains all over it!",
  "What the hell'd you do? ",
  "In a car, with my wife.",
  "He stopped short.",
  "You think I don't know what that's about?",
  "That's my old move!",
  "I used it on Estelle forty years ago!",
  "I told everybody about it!",
  "Everybody knows!",
  "(Demonstrates) Hmmph!",
  "I stopped short.",
  "(Holding out the head in his hand, he addresses it) There!",
  "Now what have you got to say for yourself?!",
  "Thirty years ago, we came to an agreement.",
  "It was the only way I could get some rest.",
  "What d'you mean, busy?",
  "(yelling) I'll talk to her any way I want! ",
  "You're not kidding it's a good move!",
  "(Sets a gift he's brought down) Carlo!",
  "It's me, Frank!",
  "(Attempts to hug the guy, but he resists - pushing Frank away.",
  "He scolds Frank in another language) I'm your cousin, Frank!",
  "Aren't you Carlo?",
  "(confidential) Estelle's got the (jerks his elbow) jimmy arms.",
  "With whom?",
  "(Realizing) What do you know..",
  "Alright.",
  "(Picking up his present) I guess I was wrong.",
  "(Walks off)",
  "Like you wouldn't believe.",
  "(anger) Sid Farkus?!",
  "You're not having dinner with a bra salesman.",
  "Don't Frank me!",
  "I know what you did.",
  "How dare you stop short with my wife!",
  "Okay, that's it!",
  "I'm not coming home!",
  "You think I don't know, Assman?!!",
  "To think I almost split the profits on the Manssierre with you.",
  "George?",
  "This is a surprise.",
  "(Looking at Kruger) Who's the suit?",
  "We'll work something out.",
  "You single-handedly brought Costanza and Son to the brink of bankruptcy.",
  "Manssierre!",
  "Okay, where's my boy?",
  "Have you seen the pole, Kruger?",
  "He's crazy.",
  "His phone wasn't even hooked up.",
  "He just liked ringing that bell.",
  "Manssierre!",
  "You...!",
  "I'm sitting at home, reading a periodical, and this is the call I get?",
  "My son is a bootlegger?",
  "(He hits George in the head)",
  "He's gonna see it.",
  "Aah!!!",
  "This is a place of business.",
  "I told you never to come in here.",
  "Serenity now!",
  "You're supposed to your face there!",
  "Do you see your face in there?",
  "Who put you up to this, was it her?",
  "It's too late.",
  "We bought a condo at Del Boca Vista.",
  "We're leaving tonight.",
  "Yeah?",
  "...oh really?...oh...",
  "how about that?...",
  "Right down a hill huh?",
  "Okay!",
  "Alight!",
  "Bye!",
  "George, forget about the shoes.",
  "Want you to do something for me (scribbles something on a piece of paper).",
  "This handicapped woman had an accident.",
  "Somebody gave her a used wheelchair with defective brakes.",
  "My George isn't clever enough to hatch a scheme like this.",
  "So, what am I supposed to say?",
  "Anyway, I want you to pick up this big screen TV, and deliver it to her.",
  "(shouts after them) You want a divorce?!!?",
  "You got one!!!",
  "What the hell does that mean?",
  "It's made from aluminum.",
  "Very high strength-to-weight ratio.",
  "No you're not!",
  "Hoochie mama!",
  "Hoochie mama!",
  "Do you think you can handle it?",
  "You sayin' you want a piece of me?",
  "He's not family.",
  "It's different, psychologically.",
  "You wanna piece of me?",
  "You got it!",
  "(They begin to fight)",
  "Take my swim trunks.",
  "I won't need them.",
  "What you saw in that van was a natural expression of a man's love for his lady.",
  "Why should they go to waste?!?",
  "Let's begin.",
  "And it was safe.",
  "It was a million to one shot, Doc.",
  "Million to one.",
  "Welcome, new comers.",
  "The tradition of Festivus begins with the airing of grievances.",
  "I got a lot of problems with you people!",
  "And now you're gonna hear about it!",
  "You, Kruger.",
  "My son tells me your company stinks!",
  "Now if you'll excuse me.",
  "Once again, your mother and I...",
  "I fell on some Fusilli.",
  "(To George) Quiet, you'll get yours in a minute.",
  "Kruger, you couldn't smooth a silk sheet if you had a hot date with a babe..",
  "I lost my train of thought.",
  "You know, the corkscrew pasta.",
  "It was a Fusilli Jerry.",
  "It got stuck in me.",
  "Had to go to the proctologist.",
  "Yeah.",
  "I don't think it's so bad.",
  "People should wear name tags.",
  "Everyone would be a lot friendlier.",
  "'Hello, Sam.' 'How are you doing, Joe?' (George's arm moves and hits the lamp) Hey, your arm.",
  "It moved again.",
  "I thought you said it went away.",
  "Kasha?",
  "And now as Festivus rolls on, we come to the feats of strength.",
  "What do you mean you felt the material?",
  "What, with your fingers like this?",
  "This year, the honor goes to Mr.",
  "Kramer.",
  "What ever happened to 'Why, that's a lovely dress you have on.",
  "May I have this dance?'!!",
  "Good thinking, Kruger.",
  "Until you pin me, George, Festivus is not over!",
  "(Taking off his sweater) Let's rumble!",
  "Stop crying, and fight your father!",
  "This is the best Festivus ever!",
  "(coming away from the window) I saw a bum sleeping in a Cadillac the other day.",
  "They don't nap.",
  "They make it their home.",
  "They urinate in there!",
  "(standing) That's it, we're going back to Queens.",
  "(claps hands) Where's my hat?",
  "First Kramer, then Elaine?",
  "It's a slap in the face.",
  "What are they too good for us?",
  "A raincoat salesman, I could buy and sell 'em like that.",
  "Let's forget about it.",
  "We're going on a beautiful vacation.",
  "(sits down in his chair)",
  "You're mother and I are planning on taking a cruise.",
  "But I can't find any vacation clothes.",
  "They were in the attic.",
  "How can I go on a cruise with out my cabana wear?",
  "I love those, those clothes.",
  "(Looks down yells) AH!",
  "(jumps out of his chair) A mouse!",
  "I saw a mouse!",
  "(takes off into another room with glass doors on it and shuts the door)",
  "(looking from the room; you can see him through the glass) Where the hell are my clothes?",
  "I love those clothes.",
  "I just don't understand how all those clothes can disappear.",
  "Moths, ate three boxes?",
  "That shirt, where'd you get that shirt?",
  "That's my cabana shirt, you stole my shirt you son of a bitch!",
  "(really fast) George you let your friends go up in my attic and steal my clothes?",
  "(grabbing at the shirt) Gimme that back",
  "Who's Rudy?",
  "What clothes?",
  "You sold my clothes (smacks George on the forehead) what do you mean you sold my clothes?",
  "It's cruise wear!",
  "THAT'S BECAUSE IT'S MINE!!",
  "Who's this Rudy?",
  "Morty Seinfeld?",
  "He's a bum.",
  "Tomorrow I'm going straight down to this Rudy and get my clothes.",
  "You burned them?",
  "Those clothes are not yours to burn.",
  "I'm the father.",
  "He said I was dead?",
  "That's what my life is worth to him?",
  "Twenty-five dollars.",
  "Oh, I just want to you know I'm retracting our dinner invitation.",
  "I'm retracting that it was ever offered.",
  "Oh, you trying to unload some of that junk of yours?",
  "That's another one of my shirts!!",
  "My clothes don't have moths!",
  "I just hope those exterminators know what they're doing."
];
