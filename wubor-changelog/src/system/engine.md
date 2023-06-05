# Engine Changes

## Input Buffer

Reduced Tap Buffer from 9 frames to 6 frames.

## Crouch

It is now possible to crouch during the following states:
* Dash
* Run
* Run Brake/Skid

## Jump Squat

All jump squats have been increased from 3 to 4 frames (except for Kazuya, who is unchanged).
Up Smash is no longer prioritized when attempting to perform a buffered Up Air.

## Jumping

Jumping no longer has accelerated rising momentum, matching the momentum from previous games.

## Shield

### Shield Mechanics
* Parrying has been moved to Shield Startup instead of Shield Release.
* The Parry window was reduced from 5 to 3 frames.
  * These two changes should make it possible, but difficult to parry multi-hit moves, as many slower multi-hits will require manual represses of Shield.
* Shield Pushback was also increased by roughly 1.3x.
  * Shield Pushback while Parrying was reduced to 0.
* Shield Release frames were reduced from 11 frames to 5.
* It is impossible to shield for 11 frames after the start of Shield Release (6 frames after you're actionable from Shield Release).
* The natural Shield HP decrease was increased from 0.15 HP/frame to 0.18 HP/frame.
* Shields can no longer be tilted.
* Shield Lock (Siheld + Special or Shield + Shield) only prevents Rolls and Spotdodges from coming out.
* Shield shrinks while losing HP, however a more transparent shield will always be there to show how much space your shield covers.
  * Instead of the above, Yoshi's Shield still fades to black, but it stops before turning black completely.
* It is no longer possible to Shield Break yourself by holding Shield. Your shield can only break if the opponent breaks it.

### Actions Into Shield
* It is now possible to Shield during Dash and Back Dash (Ryu/Ken/Terry/Kazuya).

## Actions Out of Shield
Out-of-Shield options have been significantly changed.

* It is now possible to perform a Platform Drop like in previous games.
  * It is also possible to perform a Platform Drop while in Shield Lock.
* You can no longer directly perform actions out of shield. Performing any action while holding Shield will instead force a Shield Drop first, and then buffer the action.
  * Spotdodges, Rolls, and Platform Dropping can still be directly performed.
  * In addition, Grab, Dash Grab, and Turn Grab can still be performed directly while putting up Shield. Performing Grab otherwise will force a Shield Drop first.
* Using the C-Stick will drop shield and perform whatever the C-Stick is bound to.
  * Performing angled Tilts or Smash Attacks will require moving the left stick up or down after inputting the C-Stick horizontally.

## Grab

* The Grab Lockout Timer no longer exists.
* Grab-techs are now active throughout the entire startup of grab, not just while the grab box is currently active.
* All non-standard Grabs can no longer be grab-teched.
* It is now possible to buffer Dash and Pivot Grabs.

## Spot Dodge

* Removed Spot Dodge Canceling.
* All Spot Dodges have a base FAF of 27.
* All Spot Dodges have intangibility from frames 3 to 18.
* All Spot Dodges stale at the same rate.
* On the last frame of invulnerability, Spot Dodge can be canceled into a fully-staled roll with frame 1 intangibility.

## Directional Air Dodge

See [Air Dash](./mechanics.md#air-dash)

## Ledge Slipoffs

It is now possible to slip off of ledges during the following states:
* Dash
* Back Dash (Ryu/Ken/Terry/Kazuya)
* Normal/Light/Air Dodge Landing
* Special Fall/Freefall Landing
* Taunt

## Knockback

* The spinning knockback animation no longer is randomly applied, instead being applied at a consistent `character weight + 33` damage threshold, meant to signify average kill percent.
* Balloon Knockback has been reworked.
  * Balloon Knockback now applies based on Launch Speed instead of total Hitstun frames.
  * Maximum Balloon Knockback magitude has been reduced (6 > 4), making it have less of, but still a noticable effect.

## Directional Influence

* Directional Influence has been increased (9.74 degrees / 0.17 radians > 12.0321 degrees / 0.21 radians).
* Launch Speed Influence has been removed.
* The DI Guide Line has been removed.

## Teching

* The tap tech window has been increased from 12 to 20.
* The tap window now applies to all states where it is possible tech.
* Untechables no longer exist except for on moves where it is purposefully impossible to tech.

## Ledges

* Dropping from ledge no longer clears your intangibility.
* Intangibility gets cleared if you are no longer in the falling or jumping states.

In addition, the Ledge Intangibility formula has been changed:

<datatable>

|                                  | Vanilla         | WuBor           |
|:-------------------------------- |:--------------- |:--------------- |
| Ledge Grab Invuln (Inactionable) | 19              | 19 (Unchanged)  |
| Invuln from Airtime              | Up to 60 Frames | Up to 30 Frames |
| Invuln from Damage Taken         | Up to 44 Frames | Up to 30 Frames |
| Min. Potential Intangibility     | 4 Frames        | 11 Frames       |
| Max. Potential Intangibility     | 123 Frames      | 79 Frames       |

</datatable>

This means an immediate ledge grab, from something like a ledge trump for example, at 0% will give you 30 invulnerable frames to work with. At maximum, from being off stage for 5 seconds and being at 0%, you will have 60 invulnerable frames to work with. While this seems like a lot, this scenario is very unlikely.

On the other hand, being at around 120% and immediately grabbing the ledge will give the minimum 11 frames of invulnerability, which should be enough to go for an immediate edge guard, but not much else.

## Stale-Move Negation

Staling has been overhauled. The queue has been effectively shortened from 9 to 7, Freshness Bonus was removed, and many values have been reworked.<br>
The values shown in the following table are multipliers to the move's base damage that get added during damage calculation.<br>
If a move is in multiple places in the Stale Move list, all slots are added together.

<datatable>

| Stale Move Slot     | Vanilla    | WuBor    |
|:------------------- |:---------- |:-------- |
| 0 (Freshness Bonus) | + 0.05x    | + 0.0x   |
| 1                   | - 0.09x    | - 0.125x |
| 2                   | - 0.08545x | - 0.1x   |
| 3                   | - 0.07635x | - 0.075x |
| 4                   | - 0.0679x  | - 0.05x  |
| 5                   | - 0.05945x | - 0.05x  |
| 6                   | - 0.05053x | - 0.05x  |
| 7                   | - 0.04255x | - 0.05x  |
| 8                   | - 0.03345x | - 0.0x   |
| 9                   | - 0.025x   | - 0.0x   |

</datatable>

## 1v1 Damage Multiplier

Removed.

## Rage

Removed.

## Attack Canceling

Removed.

## Sliding Normals

All grounded normals (Jab, Tilts, Smash Attacks) will carry your momentum from your previous action.
* The exception is Forward Smash, which will not carry momentum if performed during the initial frames of Dash.

## Jab

Only Neutral Attack inputs will be valid when performing Jab combos. Pressing Tilts, Smash attacks, etc. will not count as a Jab input.

## Aerials

* Pressing Jump and Attack at the same time no longer forces a short hop aerial.
* Short Hop Aerials no longer deal less damage than usual.
* Aerial Landing Lag is increased by 4 frames if you generated a hitbox and did not connect with anything. This does not apply to moves that only generate projectiles. For example:
  * Mario presses Neutral Air.
    * If Mario lands before a hitbox is generated, he takes 8 frames of landing lag.
    * If Mario generates a hitbox, does not hit anything, and lands, he takes 12 frames of landing lag.
    * If Mario hits a fighter or a Shield before landing, he takes 8 frames of landing lag.
  * Mega Man presses Up Air.
    * Because Mega Man does not generate a hitbox that's directly connected to him, he will always take 20 frames of landing lag.

## Respawning

Instead of already being in your idle animation, characters now start laying down, only to get up before the platform stops moving. (Did you know this was a thing in Smash 64?)

In addition, it is now possible to Taunt while on the respawn platform.
* Due to… surprising complexity, Sora’s Up Taunt will not produce any effects.