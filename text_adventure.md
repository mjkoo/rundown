# Supernatural Day at the Precinct

```rundown
fun restart() {
    goto "supernatural-day-at-the-precinct";
}
```

You've been sitting at an officer's desk in the precinct for hours at this point.
Thankfully, the office has a great view of the impound lot.
You can even see your own car out there; just waiting for this misunderstanding to be resolved so you can go home.

Suddenly, you hear a crash from behind you.
As if a table full of laptops and books just fell over.

Your draw the officers attention.
"WHAT ARE YOU DOING OVER THERE?"
"I TOLD YOU TO REMAIN SILENT!"
"DON'T MAKE ME COME OVER THERE"

1. Insist it wasn't your
1. Look out the window

```rundown
let response = read();
if (response == "1") {
    goto "insist-it-wasnt-you";
} else {
    goto "look-out-the-window";
}
```

# Insist it wasnt you

You speak up.
"It wasn't me. That sound from outside"

The once noisy room falls into silence
"WHAT DID YOU SAY, CRIMINAL?"

"That it wasn't me. That came from outside"

The disgusted officer leaves the watercooler and heads back, face red in anger.

1. Put your head down and stare at the floor
1. Look out the window for the source of the sound

```rundown
let response = read();
if (response == "1") {
    goto "head-down";
} else {
    goto "look-out-the-window";
}
```

# Look out the window

You look out the window.
Outside you see absolute chaos.

The impound lot seems to be exploding.
Cars are being tossed in the air.
You see your own car, tossed up, and ripped in half.

Whatever is destorying those cars is headed this way.

1. Take cover under your desk
1. Watch the chaos unfold

```rundown
let response = read();
if (response == "1") {
    goto "take-cover";
} else {
    goto "watch-the-chaos-unfold";
}
```


# Take Cover

You hide under your desk while the precint continues the loud hum.
The windoe above you shatters, and rains down onto your desk.
You sustain only minor injuries, with some glass cutting you slightly.

The room errupts into chaos.

1. Jump out the window
1. Stay hidden

```rundown
let response = read();
if (response == "1") {
    goto "jump-out-the-window";
} else {
    goto "stay-hidden";
}
```

# Stay Hidden

You stay under the desk.
The Cops scramble, grabbing weapon and all their SWAT equipment thinking it is an invasion.
Not realizing the supernatural situation they have found themselves in.

You stay hidden the whole time.

The cops start firing at something.
You hear the bullets richochet off something.

You feel something hot, and painful.
It all goes black.

```rundown
let response = read();
if (response == "1") {
    restart();
}
```

# Watch the chaos unfold

You stare, entranced by the destruction taking place outside.
You watch and watch.
A crowd has built up by this point.

The destructive force changes directions and heads straight for your window.

The glass bursts with a strong force.

You stagger, trying to move away before it attacks again.
Your legs give out, and you collapse amongst the glass and debris.

```rundown
let response = read();
if (response == "1") {
    restart();
}
```

# Head down

The cop arrives.

"That's more like it, criminal"
"Keep your head down, I don't want to hear another PEEP out of you."
"DO you UNDERSTAND ME?"

1. Yes I do.
1. Nod silently


```rundown
let response = read();
if (response == "1") {
    goto "yes-i-do";
} else {
    goto "nod";
```


# Yes I Do

"I told you to stay silent!"
"jjCome with me".
The officer takes roughly pulls you out of the chair by your cuffs, and hauls you into the processing room.

The room has a series of lines on the back wall, marked out for every foot.
Across is a camera person barking orders.

Front

Side

Other side.

The camera man leaves, called by someone from outside.
They leave you here for 15 minutes.

Suddenly the earth rends open and you and the police station fall deep into the new chasm.

1. Restart

```rundown
let response = read();
if (response == "1") {
    restart();
}
```

# Nod

"Good choice."

The officer turns and heads back to his conversation.
The Window behind you explodes inwards.

Shattered glass rains down on your cutting up your limbs with hundreds of tiny cuts.
The officer turns around sharply.

"WHAT THE HELL DID YOU DO NOW, CRIMINAL?"

1. Ask the officer for a first aid kit
1. Jump out the window

```rundown
let response = read();
if (response == "1") {
    goto "first-aid";
} else {
    goto "jump-out-the-window";

}
```

# First aid

"HA! Come with me we will fix you right up."

The officer leads you to an interrogation room, where he promptly chains you to the desk.
You bang on the desk, begging for first aid.
Everthing goes black.

You wake up briefly.
The room is turned sideways.

Minutes pass.

The horrible sound of the building rending apart

Hours pass, it is silent again.

Days pass, and you are drifting in and out of conciousness.

1. Restart

```rundown
let response = read();
if (response == "1") {
    restart();
}
```

# Jump Out The Window

You leap out the window.
Thankfully, this precinct is only 1 story.

As you exit, everything seems calm for the moment.
You see a Police Car near your, tossed open.

1. Search car for first aid
1. Search car for weapon

```rundown
let response = read();
if (response == "1") {
    goto "search-for-first-aid";
} else {
    goto "search-for-shotgun";
}
```

# Search For first aid

You rummage around the truck looking for first aid.
You see it, in the truck, but it is jammed in tight by the crushed sides.

You tug, and tug and tug.
An officer shouts at you to stop.

You rip the first aid kit free, but the officer only sees you pull something from the trunk.

The officer fires.

1. Restart

```rundown
let response = read();
if (response == "1") {
    restart();
}
```

# Search for shotgun

The police car has a shotgun sitting on top.
You easily grab it and get away from the scene.

As you step off out of the parking lot, a hulking creature appears before you.
It screams.

You feel your fear rising.

1. Fire gun
1. Drop gun and run

```rundown
let response = read();
if (response == "1") {
    goto "fire-gun";
} else {
    goto "drop-gun";
}
```

# Fire Gun

You pull the trigger.

The beast reacts violently.

1. Restart

```rundown
let response = read();
if (response == "1") {
    restart();
}
```

# Drop Gun

You drop the gun and run as fast as possible.

Shots ring out, the cops are shooting the beast from the windows.
It crashes through the wall, leaving you an opportunity to escape on foot.

Bloodied, and bruised, you walk for hours out of the city.
Once in the woods you take a nap to recover.
Tomorrow will be better.

**The End**

