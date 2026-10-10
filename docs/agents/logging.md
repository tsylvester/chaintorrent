# Logging

Logging is how the agent sees what is actually happening. It is added to observe, not guessed around, and never removed unless the user asks.

Applies whenever debugging or editing. This is a Process topic — it governs all other work.

## Debug by logging, not by guessing

- The first step to debugging is to add logging. Do not guess at the cause — add logging to see what is actually happening, then read it.
- Do not remove logging unless the user explicitly instructs you to.
- Console logs added solely for troubleshooting are exempt from the TDD and workplan obligations — but the exemption covers only the logging statements themselves, not any other change made alongside them.

## Believe the signal literally

Failing tests, linter flags, and user-reported errors are literal. Believe the stated condition and fix **that** before chasing a deeper or more interesting cause. The reported symptom is the truth to act on, not a starting point to reinterpret.

A user's flag of instruction noncompliance is handled as [discovery-halt](discovery-halt.md#do-not-ruminate-on-workarounds) states.


