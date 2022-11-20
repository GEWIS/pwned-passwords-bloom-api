# Pwned Passwords Bloom API
This repository provides a (micro) webservice in Rust that queries Redis loaded with a [RedisBloom](https://redis.io/docs/stack/bloom/) instance of [Troy Hunt's](https://www.troyhunt.com/) [Pwned Passwords](https://haveibeenpwned.com/Passwords) list.

This is based on the work of [Scott Helme](https://scotthelme.co.uk/): "[When Pwned Passwords Bloom!](https://scotthelme.co.uk/when-pwned-passwords-bloom/)" and "[Re-bloom! Pwned Passwords v8](https://scotthelme.co.uk/re-bloom-pwned-passwords-v8/)".

