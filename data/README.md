# RedisBloom Data
The default instance of this project uses the [RedisBloom dump](https://scotthelme.co.uk/re-bloom-pwned-passwords-v8/) (of Pwned Passwords v8) from Scott Helme. It was generated with the following parameters in mind:

```shell
m = 32482743068 # (3.78GiB)
n = 847223402 # (passwords in v8)
k = 27 # (hash functions)
p = 0.00000001 # (1 in 99,857,412)

p = pow(1 - exp(-k / (m / n)), k)
p = pow(1 - exp(-27 / (32482743068 / 847223402)), 27)
p = 0.00000001
```

This means that the chance of a false-positive outcome for a query occurring is approximately 1 in a hundred million.

If you want to create your own bloom filter, you can download one of the original Pwned Passwords file from [Have I Been Pwned?](https://haveibeenpwned.com/Passwords). You can adjust the parameters `m`, `k`, and `p` to obtain your desired probability of a false-positive outcome occurring. Please note that increasing the number of hashes (`k`) and size of the bloom filter (`m`) can negatively impact the performance.

Depending on your machine and the chosen values, generating a bloom filter may take up to a day. 

---

The bloom filter is keyed in Redis as `pwned-bloom`.