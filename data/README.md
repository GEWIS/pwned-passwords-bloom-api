# RedisBloom Data
Since June 2024, we ourselves no longer use the [RedisBloom dump](https://scotthelme.co.uk/re-bloom-pwned-passwords-v8/) (or Pwned Passwords v8) from Scott Helme. Instead, we are using the [`haveibeenpwned-downloader` tool](https://github.com/HaveIBeenPwned/PwnedPasswordsDownloader) to be able to create the bloom filter in Redis ourselves. 

As nearly 100 million new hashes have been added in the past 2 years, the old parameters no longer work properly. We see that if we keep the memory usage and the number of hash functions the same, the probability of a false positive increases significantly:

```shell
m = 32482743068 # (3.78GiB)
n = 936491757 # (passwords in the download of June 5, 2024)
k = 27 # (hash functions)
p = 0.000000062 # (1 in 16,087,630)
```

We think this probability of false-positives is too high, and we therefore still aim for a probability of about 1 in 100 million. This can be achieved in two ways, by increasing memory usage or by increasing the number of hash functions:

```shell
m = 35905312644 # (4.18GiB)
n = 936491757 # (passwords in the download of June 5, 2024)
k = 27 # (hash functions)
p = 0.00000001 # (1 in 99,857,412)
```

```shell
m = 32482743068 # (3.78GiB)
n = 936491757 # (passwords in the download of June 5, 2024)
k = 30 # (hash functions)
p = 0.000000009 # (1 in 101,674,820)
```

In the application of pwned password detection, we need all the speed we can get. Increasing the number of hash functions will result in it taking (slightly) longer to determine whether a password is in the filter or not. Therefore, we chose not to increase the number of hash functions, but instead increased the filter by about 0.5GB. This means our parameters are as follows (the same as the first option above):

```shell
m = 35905312644 # (4.18GiB)
n = 936491757 # (passwords in the download of June 5, 2024)
k = 27 # (hash functions)
p = 0.00000001 # (1 in 99,857,412)
```

---

The Rust script provided in this subdirectory is able to parse and add approximately 1 million hashes to the bloom filter every 1.5s. With nearly 1 billion hashes in the list of June 2024 this takes almost 25 minutes on a Windows 10 notebook with an i7-11800H and 32GB RAM (in WSL). Depending on your available system memory you may have to decrease the `batch_size` to something smaller than 1 million hashes. Subsequently, you can simply run the Rust script using:

```shell
cargo run --release
```

Scott Helme [reported](https://scotthelme.co.uk/re-bloom-pwned-passwords-v8/#generating-a-new-bloom-filter) needing approximately 17.5 hours for the filter containing `847223402` passwords. It should be noted that he did it using PHP (see [the repository](https://github.com/ScottHelme/when-pwned-passwords-bloom/)), so that carries significant overhead compared to Rust.


## Creating your own RedisBloom filter and dump

If you want to create your own bloom filter, you can no longer download one of the original Pwned Passwords file from [Have I Been Pwned?](https://haveibeenpwned.com/Passwords). Instead, you have to use the [`haveibeenpwned-downloader` tool](https://github.com/HaveIBeenPwned/PwnedPasswordsDownloader). You can adjust the parameters `m`, `k`, and `p` to obtain your desired probability of a false-positive outcome occurring. Please note that increasing the number of hashes (`k`) and size of the bloom filter (`m`) can negatively impact the performance.

Then you can use the Rust script to create the actual bloom filter in Redis. Depending on the parameters you use to generate the bloom filter, this can take a long time to complete.

## Original RedisBloom dump
The original RedisBloom dump from Scott Helme was generated with the following parameters in mind:

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

---

The bloom filter is keyed in Redis as `pwned-bloom`.