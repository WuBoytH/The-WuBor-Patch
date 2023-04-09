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

## Short Hop Aerial Macro

No longer forces a short hop. Release the jump button to perform a Short Hop, or hold it to Full Hop.

## Shield

* It is now possible to Shield during Dash and Back Dash (Ryu/Ken/Terry/Kazuya).
* You can no longer directly use Up Special while in Shield.
* Parrying has been moved to Shield Startup instead of Shield Release.
* The Parry window was reduced from 5 to 3 frames.
  * These two changes should make it possible, but difficult to parry multi-hit moves.
* Shield Pushback was also increased by roughly 1.3x.
  * Shield Pushback while Parrying was reduced to 0.

## Grab

* The Grab Lockout Timer no longer exists.
* All non-standard Grabs can no longer be grab-teched.
* It is now possible to buffer Dash and Pivot Grabs.
* You can manually Grab-Release an opponent by pressing Taunt.

## Spot Dodge

* Removed Spot Dodge Canceling.
* All Spot Dodges have a base FAF of 27.
* All Spot Dodges have intangibility from frames 3 to 18.
* All Spot Dodges stale at the same rate.
* On the last frame of invulnerability, Spot Dodge can be canceled into a fully-staled roll with frame 1 intangibility.

## Directional Air Dodge

[See New Mechanics/Air Dash](./mechanics.md#air-dash)

## Platform Dropping in Shield

It is not possible to Shield Drop on a Platform either by tilting down the Control Stick, like in previous Smash titles, or by pressing Taunt.

## Ledge Slipoffs

It is now possible to slip off of ledges during the following states:
* Dash
* Back Dash (Ryu/Ken/Terry/Kazuya)
* Normal/Light/Air Dodge Landing
* Special Fall/Freefall Landing
* Taunt

## Teching

* The tap tech window has been increased from 12 to 20.
* The tap window now applies to all states where it is possible tech.
* Untechables no longer exist except for on moves where it is purposefully impossible to tech.

## Geting Launched

* The spinning knockback animation no longer is randomly applied, instead being applied at a consistent `character weight + 33` damage threshold, meant to signify average kill percent.
* Balloon Knockback has been reworked so that it applies based on Launch Speed instead of total Hitstun frames.

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

## Jab

Only Neutral Attack inputs will be valid when performing Jab combos. Pressing Tilts, Smash attacks, etc. will not count as a Jab input.

## Aerials

Aerial Landing Lag is increased by 4 frames if you generated a hitbox and did not connect with anything. This does not apply to moves that only generate projectiles.
For example:
* Mario presses Neutral Air.
  * If Mario lands before a hitbox is generated, he takes 8 frames of landing lag.
  * If Mario generates a hitbox, does not hit anything, and lands, he takes 12 frames of landing lag.
  * If Mario hits a fighter or a Shield before landing, he takes 8 frames of landing lag.
* Mega Man presses Up Air.
  * Because Mega Man does not generate a hitbox that's directly connected to him, he will always take 20 frames of landing lag.
